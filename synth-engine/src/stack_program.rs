use crate::simulator::state::State;
use libm::{sinf, cosf, tanf, tanhf, logf, expf, fabsf};

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct StackProgram<'a> {
    pub code: &'a [Instr],
    pub stack_size: usize,
}

pub fn compute_stack_size(code: &[Instr]) -> usize {
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

impl<'a> StackProgram<'a> {
    pub fn new(code: &'a [Instr], stack_size: usize) -> Self {
        Self { code, stack_size }
    }

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

        for instr in self.code {
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

                    if fabsf(b) < f32::EPSILON {
                        push_stack(stack, &mut stack_ptr, 0.)?;
                    } else {
                        push_stack(stack, &mut stack_ptr, a / b)?;
                    }
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
                            push_stack(stack, &mut stack_ptr, sinf(a))?;
                        }
                        Cos => {
                            let a = pop_stack(stack, &mut stack_ptr)?;
                            push_stack(stack, &mut stack_ptr, cosf(a))?;
                        }
                        Tan => {
                            let a = pop_stack(stack, &mut stack_ptr)?;
                            push_stack(stack, &mut stack_ptr, tanf(a))?;
                        }
                        Tanh => {
                            let a = pop_stack(stack, &mut stack_ptr)?;
                            push_stack(stack, &mut stack_ptr, tanhf(a))?;
                        }
                        Abs => {
                            let a = pop_stack(stack, &mut stack_ptr)?;
                            push_stack(stack, &mut stack_ptr, fabsf(a))?;
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
                            push_stack(stack, &mut stack_ptr, logf(a))?;
                        }
                        Exp => {
                            let a = pop_stack(stack, &mut stack_ptr)?;
                            push_stack(stack, &mut stack_ptr, expf(a))?;
                        }
                        Logistic => {
                            let x0 = pop_stack(stack, &mut stack_ptr)?;
                            let k = pop_stack(stack, &mut stack_ptr)?;
                            let l = pop_stack(stack, &mut stack_ptr)?;
                            let x = pop_stack(stack, &mut stack_ptr)?;
                            let v = l / expf(1. + (-k * (x - x0)));
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
                    if i >= &state.len() {
                        return Err(ExecError::StateOutOfBounds(*i));
                    } else {
                        let v = state.get(*i);
                        push_stack(stack, &mut stack_ptr, v)?;
                    }
                }
            }
        }

        Ok(pop_stack(stack, &mut stack_ptr)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::simulator::state::State as SimulatorState;
    use alloc::vec;

    #[test]
    fn simple_run() {
        let values = vec![0.; 10];
        let state = SimulatorState::new_with_values(&values);
        let mut stack = vec![0.; 2];

        use Instr::*;

        let prg = StackProgram::new(vec![Const(2.), Const(3.), Add], 2);

        assert_eq!(prg.run(&state, &mut stack), Ok(5.));
    }
}
