extern crate peg;
use crate::modules::SynthSpec;
use peg::parser;
use peg::str::LineCol;
use synth_engine::stack_program::*;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseLocation(String, LineCol);

#[derive(Error, Debug, PartialEq)]
pub enum ExprError {
    #[error("Unrecognized function: {0}")]
    UnrecognizedFunction(String),
    #[error("Parse error when parsing {0}: {1:?}")]
    ParseError(String, peg::error::ParseError<LineCol>),
    #[error("Missing module field. Module: {0}, field: {1}")]
    MissingField(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(f32),
    OutputState(String, String),
    BinOp(BinaryOperator, Box<Expr>, Box<Expr>),
    FunCall(String, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Inverse,
}

impl Expr {
    pub fn parse(s: &str) -> Result<Self, ExprError> {
        arithmetic::expression(s).map_err(|err| ExprError::ParseError(s.to_string(), err))
    }

    pub fn zero() -> Self {
        Expr::Number(0.)
    }

    pub fn constant(v: f32) -> Self {
        Expr::Number(v)
    }
}

parser! {
    grammar arithmetic() for str {
        pub rule expression() -> Expr = precedence!{
            x:(@) "+" y:@ { Expr::BinOp(BinaryOperator::Add, Box::new(x), Box::new(y)) }
            x:(@) "-" y:@ { Expr::BinOp(BinaryOperator::Subtract, Box::new(x), Box::new(y)) }
            --
            x:(@) "*" y:@ { Expr::BinOp(BinaryOperator::Multiply, Box::new(x), Box::new(y)) }
            x:(@) "/" y:@ { Expr::BinOp(BinaryOperator::Divide, Box::new(x), Box::new(y)) }
            --
            _ n:number() _ { n }
            _ f:function() _ { f }
            _ o:output() _ { o }
            _ "(" e:expression() ")" _ { e }
        }

        rule _ = [' ' | '\n']*

        rule number() -> Expr
            = n:$("-"? ['0'..='9']+ "." ['0'..='9']*) { Expr::Number(n.parse::<f32>().unwrap()) }

        rule function() -> Expr
            = f:$(['a'..='z'|'A'..='Z'|'_']+) "(" args:(expression() ** ",") ")" { Expr::FunCall(f.to_string(), args) }

        rule output() -> Expr
            = a:$(['a'..='z'|'A'..='Z'|'_']+) "." b:$(['a'..='z'|'A'..='Z'|'_']+) { Expr::OutputState(a.to_string(), b.to_string()) }
    }
}

impl Expr {
    pub fn compile(&self, synth_spec: &SynthSpec) -> Result<StackProgram, ExprError> {
        let mut program: Vec<Instr> = Vec::new();

        self.compile_helper(synth_spec, &mut program)?;

        let stack_size = compute_stack_size(&program);

        Ok(StackProgram::new(program, stack_size))
    }

    fn compile_helper(
        &self,
        synth_spec: &SynthSpec,
        program: &mut Vec<Instr>,
    ) -> Result<(), ExprError> {
        use Expr::*;

        match self {
            BinOp(op, e1, e2) => {
                e2.compile_helper(synth_spec, program)?;
                e1.compile_helper(synth_spec, program)?;
                let op_instr = match op {
                    BinaryOperator::Add => Instr::Add,
                    BinaryOperator::Subtract => Instr::Subtract,
                    BinaryOperator::Multiply => Instr::Multiply,
                    BinaryOperator::Divide => Instr::Divide,
                };
                program.push(op_instr);
            }
            Number(n) => program.push(Instr::Const(*n)),
            OutputState(m, n) => match synth_spec.input_state_index(m.as_str(), n.as_str()) {
                Ok(index) => program.push(Instr::State(index)),
                Err(_) => return Err(ExprError::MissingField(m.to_string(), n.to_string())),
            },
            FunCall(f, args) => {
                // TODO check len() of args matches what's required by function
                for expr in args {
                    expr.compile_helper(synth_spec, program)?;
                }

                let fun = match f.as_str() {
                    "sin" => Function::Sin,
                    "cos" => Function::Cos,
                    "tan" => Function::Tan,
                    "tanh" => Function::Tanh,
                    "abs" => Function::Abs,
                    "min" => Function::Min,
                    "max" => Function::Max,
                    "ln" => Function::Ln,
                    "exp" => Function::Exp,
                    "logistic" => Function::Logistic,
                    "lerp" => Function::Lerp,
                    _ => return Err(ExprError::UnrecognizedFunction(f.to_string())),
                };

                program.push(Instr::Call(fun));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::modules::NoiseGeneratorModuleSpec;
    use synth_engine::simulator::state::State as SimulatorState;
    use Expr::*;

    #[test]
    fn parse_arithmetic() {
        let input = "4.+3.2*xyZ.Zyz";
        let expected = BinOp(
            BinaryOperator::Add,
            Box::new(Number(4.)),
            Box::new(BinOp(
                BinaryOperator::Multiply,
                Box::new(Number(3.2)),
                Box::new(OutputState("xyZ".to_string(), "Zyz".to_string())),
            )),
        );

        assert_eq!(Expr::parse(input), Ok(expected));
    }

    #[test]
    fn expression_with_whitespace() {
        let input = "  4. + 3.2          * xyZ.Zyz  + sin(a.b)";
        let expected = BinOp(
            BinaryOperator::Add,
            Box::new(BinOp(
                BinaryOperator::Add,
                Box::new(Number(4.)),
                Box::new(BinOp(
                    BinaryOperator::Multiply,
                    Box::new(Number(3.2)),
                    Box::new(OutputState("xyZ".to_string(), "Zyz".to_string())),
                )),
            )),
            Box::new(FunCall(
                "sin".to_string(),
                vec![OutputState("a".to_string(), "b".to_string())],
            )),
        );

        assert_eq!(Expr::parse(input), Ok(expected));
    }

    #[test]
    fn negative_constant() {
        let input = "-2. * a.b";
        let expected = BinOp(
            BinaryOperator::Multiply,
            Box::new(Number(-2.)),
            Box::new(OutputState("a".to_string(), "b".to_string())),
        );

        assert_eq!(Expr::parse(input), Ok(expected));
    }

    #[test]
    fn compile() {
        let mut synth_spec = SynthSpec::new();

        let _ = synth_spec
            .add_module(Box::new(NoiseGeneratorModuleSpec::new("noise", 0)))
            .unwrap();

        let input = "2. * noise.signal_output";
        let expected =
            StackProgram::new(vec![Instr::State(0), Instr::Const(2.), Instr::Multiply], 2);

        assert_eq!(
            Expr::parse(input).unwrap().compile(&synth_spec).unwrap(),
            expected
        );
    }

    #[test]
    fn compile_fun_call() {
        let mut synth_spec = SynthSpec::new();

        let _ = synth_spec
            .add_module(Box::new(NoiseGeneratorModuleSpec::new("noise", 0)))
            .unwrap();

        let input = "sin(2. * noise.signal_output)";
        let expected = StackProgram::new(
            vec![
                Instr::State(0),
                Instr::Const(2.),
                Instr::Multiply,
                Instr::Call(Function::Sin),
            ],
            2,
        );

        assert_eq!(
            Expr::parse(input).unwrap().compile(&synth_spec).unwrap(),
            expected
        );
    }

    #[test]
    fn test_functions() {
        let mut synth_spec = SynthSpec::new();
        let mut stack = vec![0.; 32];
        let state = SimulatorState::new_with_values(&vec![0.; 32]);

        let input1 = "lerp(0.0, 1.0, 2.0)";
        let output1 = 1.0;

        let input2 = "lerp(0.5, 1.0, 2.0)";
        let output2 = 1.5;

        let input3 = "logistic(0.0, 2.0, 1.0, 0.0)";
        let output3 = 1.0;

        assert_eq!(
            Expr::parse(input1)
                .unwrap()
                .compile(&synth_spec)
                .unwrap()
                .run(&state, &mut stack),
            Ok(output1)
        );
        assert_eq!(
            Expr::parse(input2)
                .unwrap()
                .compile(&synth_spec)
                .unwrap()
                .run(&state, &mut stack),
            Ok(output2)
        );
        assert_eq!(
            Expr::parse(input3)
                .unwrap()
                .compile(&synth_spec)
                .unwrap()
                .run(&state, &mut stack),
            Ok(output3)
        );
    }
}
