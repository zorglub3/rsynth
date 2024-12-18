use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use libm::exp2f;

pub struct Amplifier<'a> {
    signal_input: StackProgram<'a>,
    output_index: usize,
    lin_control_input: StackProgram<'a>,
    exp_control_input: StackProgram<'a>,
}

impl<'a> Amplifier<'a> {
    pub fn new(
        signal_input: StackProgram<'a>,
        output_index: usize,
        lin_control_input: StackProgram<'a>,
        exp_control_input: StackProgram<'a>,
    ) -> Self {
        Self {
            signal_input,
            output_index,
            lin_control_input,
            exp_control_input,
        }
    }
}

fn amplifier_amount(lin_control: f32, exp_control: f32) -> f32 {
    // TODO - these are constants - put them in the Amplifier struct
    // 2.0 and 5.0 should be arguments to `new`
    let min: f32 = exp2f(-5.0);
    let scale: f32 = 1. / (1. - min);

    let exp_control = exp_control.clamp(0., 1.);
    let e = (exp2f(5. * (exp_control - 1.)) - min) * scale;
    (e + lin_control).max(0.)
}

impl<'a> Module for Amplifier<'a> {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let input = self.signal_input.run(state, stack).unwrap_or(0.0);
        let m = amplifier_amount(
            self.lin_control_input.run(state, stack).unwrap_or(0.0),
            self.exp_control_input.run(state, stack).unwrap_or(0.0),
        );
        update.set(self.output_index, input * m, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
