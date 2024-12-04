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
    Output(String, String),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    FunCall(String, Vec<Expr>),
}

impl Expr {
    pub fn simplify(self) -> Self {
        use Expr::*;

        match self {
            Sum(x) => match &x[..] {
                [y] => y.clone().simplify(),
                _ => Sum(x.into_iter().map(|e| e.simplify()).collect()),
            },
            Product(x) => match &x[..] {
                [y] => y.clone().simplify(),
                _ => Product(x.into_iter().map(|e| e.simplify()).collect()),
            },
            FunCall(f, args) => FunCall(f, args.into_iter().map(|e| e.simplify()).collect()),
            x => x,
        }
    }

    pub fn parse(s: &str) -> Result<Self, ExprError> {
        match arithmetic::expression(s) {
            Ok(e) => Ok(e.simplify()),
            Err(err) => Err(ExprError::ParseError(s.to_string(), err)),
        }
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
        pub rule expression() -> Expr
            = _ x:sum() _ { x }

        rule _ = [' ' | '\n']*

        rule sum() -> Expr
            = _ l:(term() ** "+") _ { Expr::Sum(l) }

        rule term() -> Expr
            = _ l:(atom() ** "*") _ { Expr::Product(l) }

        rule atom() -> Expr
            = _ n:number() _ { n }
            / _ "(" _ v:sum() _ ")" _ { v }
            / _ f:function() _ { f }
            / _ o:output() _ { o }

        rule number() -> Expr
            = n:$("-"? ['0'..='9']+ "." ['0'..='9']*) { Expr::Number(n.parse::<f32>().unwrap()) }

        rule function() -> Expr
            = f:$(['a'..='z'|'A'..='Z'|'_']+) "(" args:(expression() ** ",") ")" { Expr::FunCall(f.to_string(), args) }

        rule output() -> Expr
            = a:$(['a'..='z'|'A'..='Z'|'_']+) "." b:$(['a'..='z'|'A'..='Z'|'_']+) { Expr::Output(a.to_string(), b.to_string()) }
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
            Sum(args) => {
                for expr in args {
                    expr.compile_helper(synth_spec, program)?;
                }
                for _i in 0..(args.len() - 1).max(0) {
                    program.push(Instr::Add);
                }
            }
            Product(args) => {
                for expr in args {
                    expr.compile_helper(synth_spec, program)?;
                }
                for _i in 0..(args.len() - 1).max(0) {
                    program.push(Instr::Multiply);
                }
            }
            Number(n) => program.push(Instr::Const(*n)),
            Output(m, n) => match synth_spec.input_state_index(m.as_str(), n.as_str()) {
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
    use Expr::*;

    #[test]
    fn parse_arithmetic() {
        let input = "4.+3.2*xyZ.Zyz";
        let expected = Sum(vec![
            Number(4.),
            Product(vec![
                Number(3.2),
                Output("xyZ".to_string(), "Zyz".to_string()),
            ]),
        ]);

        assert_eq!(Expr::parse(input), Ok(expected));
    }

    #[test]
    fn expression_with_whitespace() {
        let input = "  4. + 3.2          * xyZ.Zyz  + sin(a.b)";
        let expected = Sum(vec![
            Number(4.),
            Product(vec![
                Number(3.2),
                Output("xyZ".to_string(), "Zyz".to_string()),
            ]),
            FunCall(
                "sin".to_string(),
                vec![Output("a".to_string(), "b".to_string())],
            ),
        ]);

        assert_eq!(Expr::parse(input), Ok(expected));
    }

    #[test]
    fn negative_constant() {
        let input = "-2. * a.b";
        let expected = Product(vec![Number(-2.), Output("a".to_string(), "b".to_string())]);
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
            StackProgram::new(vec![Instr::Const(2.), Instr::State(0), Instr::Multiply], 2);

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
                Instr::Const(2.),
                Instr::State(0),
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
}
