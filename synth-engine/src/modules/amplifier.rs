use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

pub struct Amplifier {
    signal_input: StackProgram,
    output_index: usize,
    lin_control_input: StackProgram,
    exp_control_input: StackProgram,
}

impl Amplifier {
    pub fn new(
        signal_input: StackProgram,
        output_index: usize,
        lin_control_input: StackProgram,
        exp_control_input: StackProgram,
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
    let min: f32 = 2.0_f32.powf(-5.0);
    let scale: f32 = 1. / (1. - min);

    let exp_control = exp_control.max(0.).min(1.);
    let e = (2.0_f32.powf(5.0 * (exp_control - 1.)) - min) * scale;
    (e + lin_control).max(0.)
}

impl Module for Amplifier {
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
