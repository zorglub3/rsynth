use crate::synth_math::SynthMath;
use alloc::vec;
use alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone)]
pub enum Instr {
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,

    Call(Function),
    Const(f32),
    State(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Tanh,

    Ln,
    Exp,
    Logistic,

    Abs,
    Min,
    Max,
    Lerp,
}

#[derive(Debug, PartialEq)]
pub enum ExecError {
    StackOverflow,
    StackUnderflow,
    StateOutOfBounds(usize),
}

// TODO static analysis for stack programs:
// 1. Make sure all `State` instructions are valid
// 2. Make sure stack under- and over- flow is impossible
// 3. Make sure all programs return just one value on the stack
// 4. All runtime checks for validity should be redundant

#[derive(PartialEq, Debug, Clone)]
pub struct StackProgram {
    pub code: Vec<Instr>,
    pub stack_size: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ProgramError {
    StackUnderflow,
    InvalidStateIndex(usize),
}

/*
pub fn compute_stack_size(code: &Vec<Instr>) -> usize {
    let mut stack_size: usize = 0;
    let mut stack_max_size: usize = 0;

    for instr in code {
        use Instr::*;

        match instr {
            Add | Subtract | Multiply | Divide => stack_size -= 1,
            Negate => {} // No change
            Const(_) | State(_) => stack_size += 1,
            Call(f) => {
                use Function::*;

                match f {
                    Min | Max => stack_size -= 1,
                    _ => {} // no change
                }
            }
        }

        stack_max_size = stack_max_size.max(stack_size);
    }

    stack_max_size
}
*/

pub fn static_check(code: &[Instr], state_size: usize) -> Result<usize, ProgramError> {
    let mut stack_top = 0_i32;
    let mut stack_size = 0_i32;

    for instr in code {
        use Instr::*;

        let stack_delta: i32;
        let stack_consume: i32;

        match instr {
            Add | Subtract | Multiply | Divide => {
                stack_delta = -1;
                stack_consume = 2;
            }
            Negate => {
                stack_delta = 0;
                stack_consume = 1;
            }
            Const(_) => {
                stack_delta = 1;
                stack_consume = 0;
            }
            State(i) => {
                stack_delta = 1;
                stack_consume = 0;
                if *i >= state_size {
                    return Err(ProgramError::InvalidStateIndex(*i));
                }
            }
            Call(f) => {
                use Function::*;

                match f {
                    Min | Max => {
                        stack_delta = -1;
                        stack_consume = 2;
                    }
                    Lerp => {
                        stack_delta = -2;
                        stack_consume = 3;
                    }
                    _ => {
                        stack_delta = 0;
                        stack_consume = 1;
                    }
                }
            }
        }

        if stack_top < stack_consume {
            return Err(ProgramError::StackUnderflow);
        }

        stack_top += stack_delta;
        stack_size = stack_size.max(stack_top);
    }

    Ok(stack_size as usize)
}

impl StackProgram {
    pub fn new_with_stack_size(code: Vec<Instr>, stack_size: usize) -> Self {
        Self { code, stack_size }
    }

    /*
    pub fn new(code: Vec<Instr>) -> Self {
        let stack_size = compute_stack_size(&code);

        Self { code, stack_size }
    }
    */

    pub fn zero() -> Self {
        Self {
            code: vec![Instr::Const(0.)],
            stack_size: 1,
        }
    }

    pub fn constant(v: f32) -> Self {
        Self {
            code: vec![Instr::Const(v)],
            stack_size: 1,
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            code: vec![Instr::State(index)],
            stack_size: 1,
        }
    }

    pub fn eval(&self, state_values: &[f32], stack: &mut [f32]) -> f32 {
        let mut stack_top = 0;

        for instr in &self.code {
            use Instr::*;
            match instr {
                Add => {
                    let a = stack[stack_top];
                    stack_top -= 1;
                    stack[stack_top] += a;
                    // let b = stack[stack_top];
                    // stack[stack_top] = a + b;
                }
                Subtract => {
                    let a = stack[stack_top];
                    stack_top -= 1;
                    let b = stack[stack_top];
                    stack[stack_top] = a - b;
                }
                Multiply => {
                    let a = stack[stack_top];
                    stack_top -= 1;
                    let b = stack[stack_top];
                    stack[stack_top] = a * b;
                }
                Divide => {
                    let a = stack[stack_top];
                    stack_top -= 1;
                    let b = stack[stack_top];
                    stack[stack_top] = a / b;
                }
                Negate => {
                    let v = stack[stack_top];
                    stack[stack_top] = v * -1.;
                }
                Call(f) => {
                    use Function::*;
                    let v = stack[stack_top];
                    let r = match f {
                        Sin => v.sin(),
                        Cos => v.cos(),
                        Tan => v.tan(),
                        Tanh => v.tanh(),
                        Abs => v.abs(),
                        Ln => v.ln(),
                        Exp => v.exp(),
                        Min => {
                            stack_top -= 1;
                            let u = stack[stack_top];
                            v.min(u)
                        }
                        Max => {
                            stack_top -= 1;
                            let u = stack[stack_top];
                            v.max(u)
                        }
                        Lerp => {
                            stack_top -= 1;
                            let u = stack[stack_top];
                            stack_top -= 1;
                            let x = stack[stack_top];
                            u * (1. - x) + v * x
                        }
                        Logistic => todo!(),
                    };
                    stack[stack_top] = r;
                }
                Const(v) => {
                    stack_top += 1;
                    stack[stack_top] = *v;
                }
                State(i) => {
                    stack_top += 1;
                    stack[stack_top] = state_values[*i];
                }
            }
        }

        stack[1]
    }

    /*
        pub fn run(&self, state: &State, stack: &mut [f32]) -> Result<f32, ExecError> {
            #[inline]
            fn pop_stack(stack: &[f32], stack_ptr: &mut usize) -> Result<f32, ExecError> {
                if stack_ptr < &mut 1 {
                    Err(ExecError::StackUnderflow)
                } else {
                    let v = stack[*stack_ptr - 1];
                    *stack_ptr -= 1;
                    Ok(v)
                }
            }

            #[inline]
            fn push_stack(
                stack: &mut [f32],
                stack_ptr: &mut usize,
                value: f32,
            ) -> Result<(), ExecError> {
                if stack_ptr >= &mut stack.len() {
                    Err(ExecError::StackOverflow)
                } else {
                    stack[*stack_ptr] = value;
                    *stack_ptr += 1;
                    Ok(())
                }
            }

            let mut stack_ptr: usize = 0;

            for instr in &self.code {
                use Instr::*;

                match instr {
                    Add => {
                        let a = pop_stack(stack, &mut stack_ptr)?;
                        let b = pop_stack(stack, &mut stack_ptr)?;
                        push_stack(stack, &mut stack_ptr, a + b)?;
                    }
                    Subtract => {
                        let a = pop_stack(stack, &mut stack_ptr)?;
                        let b = pop_stack(stack, &mut stack_ptr)?;
                        push_stack(stack, &mut stack_ptr, a - b)?;
                    }
                    Multiply => {
                        let a = pop_stack(stack, &mut stack_ptr)?;
                        let b = pop_stack(stack, &mut stack_ptr)?;
                        push_stack(stack, &mut stack_ptr, a * b)?;
                    }
                    Divide => {
                        let a = pop_stack(stack, &mut stack_ptr)?;
                        let b = pop_stack(stack, &mut stack_ptr)?;
                        // TODO check for divide-by-zero (we shouldn't crash)
                        push_stack(stack, &mut stack_ptr, a / b)?;
                    }
                    Negate => {
                        let a = pop_stack(stack, &mut stack_ptr)?;
                        push_stack(stack, &mut stack_ptr, a * -1.)?;
                    }
                    Call(f) => {
                        use Function::*;

                        match f {
                            Sin => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.sin())?;
                            }
                            Cos => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.cos())?;
                            }
                            Tan => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.tan())?;
                            }
                            Tanh => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.tanh())?;
                            }
                            Abs => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.abs())?;
                            }
                            Min => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                let b = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.min(b))?;
                            }
                            Max => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                let b = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.max(b))?;
                            }
                            Ln => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.ln())?;
                            }
                            Exp => {
                                let a = pop_stack(stack, &mut stack_ptr)?;
                                push_stack(stack, &mut stack_ptr, a.exp())?;
                            }
                            Logistic => {
                                let x0 = pop_stack(stack, &mut stack_ptr)?;
                                let k = pop_stack(stack, &mut stack_ptr)?;
                                let l = pop_stack(stack, &mut stack_ptr)?;
                                let x = pop_stack(stack, &mut stack_ptr)?;
                                let v = l / (1. + (-k * (x - x0)).exp());
                                push_stack(stack, &mut stack_ptr, v)?;
                            }
                            Lerp => {
                                let hi = pop_stack(stack, &mut stack_ptr)?;
                                let lo = pop_stack(stack, &mut stack_ptr)?;
                                let x = pop_stack(stack, &mut stack_ptr)?.clamp(0., 1.);
                                let v = lo * (1. - x) + hi * x;
                                push_stack(stack, &mut stack_ptr, v)?;
                            }
                        }
                    }
                    Const(v) => {
                        push_stack(stack, &mut stack_ptr, *v)?;
                    }
                    State(i) => {
                        let v = state.get(*i);
                        push_stack(stack, &mut stack_ptr, v)?;
                        /*
                        if i >= &state.len() {
                            return Err(ExecError::StateOutOfBounds(*i));
                        } else {
                            let v = state.get(*i);
                            push_stack(stack, &mut stack_ptr, v)?;
                        }
                        */
                    }
                }
            }

            Ok(pop_stack(stack, &mut stack_ptr)?)
        }
    */
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::simulator::state::State as SimulatorState;

    #[test]
    fn simple_run() {
        let values = vec![0.; 10];
        let state = SimulatorState::new_with_values(&values);
        let mut stack = vec![0.; 2];

        use Instr::*;

        let prg = StackProgram::new(vec![Const(2.), Const(3.), Add]);

        assert_eq!(prg.run(&state, &mut stack), Ok(5.));
    }
}
*/
