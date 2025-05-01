use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use crate::synth_math::SynthMath;

pub const STATE_SIZE: usize = 1;
pub const INPUT_SIZE: usize = 3;

// state/outputs
const SIGNAL_OUTPUT: usize = 0;

// inputs/controls
const SIGNAL_INPUT: usize = 0;
const LINEAR_CONTROL_INPUT: usize = 1;
const EXP_CONTROL_INPUT: usize = 2;

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
    let min: f32 = -5.0_f32.exp2();
    let scale: f32 = 1. / (1. - min);

    let exp_control = exp_control.clamp(0., 1.);
    let e = ((5. * (exp_control - 1.)).exp2() - min) * scale;
    (e + lin_control).max(0.)
}

impl Module for Amplifier {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        _state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let input = inputs[SIGNAL_INPUT];
        let m = amplifier_amount(
            inputs[LINEAR_CONTROL_INPUT],
            inputs[EXP_CONTROL_INPUT],
        );
        update[SIGNAL_OUTPUT] = input * m;
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[SIGNAL_OUTPUT] = UpdateType::Absolute;
    }

    /*
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
    */
}
