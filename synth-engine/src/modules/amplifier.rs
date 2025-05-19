use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;

pub struct Amplifier {
    min: f32,
    scale: f32,
    // signal_input: StackProgram,
    // output_index: usize,
    // lin_control_input: StackProgram,
    // exp_control_input: StackProgram,
}

impl Amplifier {
    pub const STATE_SIZE: usize = 1;
    pub const INPUT_SIZE: usize = 3;

    // state/outputs
    pub const SIGNAL_OUTPUT: usize = 0;

    // inputs/controls
    pub const SIGNAL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const EXP_CONTROL_INPUT: usize = 2;


    pub fn new(
        min: f32, // default: -5.0_f32.exp2()
        scale: f32, // default: 1. / (1. - min)
        // signal_input: StackProgram,
        // output_index: usize,
        // lin_control_input: StackProgram,
        // exp_control_input: StackProgram,
    ) -> Self {
        Self {
            min, 
            scale,
            // signal_input,
            // output_index,
            // lin_control_input,
            // exp_control_input,
        }
    }
}

fn amplifier_amount(
    min: f32,
    scale: f32,
    lin_control: f32, 
    exp_control: f32,
) -> f32 {
    // TODO - these are constants - put them in the Amplifier struct
    // 2.0 and 5.0 should be arguments to `new`
    // let min: f32 = -5.0_f32.exp2();
    // let scale: f32 = 1. / (1. - min);

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
        let input = inputs[Amplifier::SIGNAL_INPUT];
        let m = amplifier_amount(
            self.min,
            self.scale,
            inputs[Amplifier::LINEAR_CONTROL_INPUT],
            inputs[Amplifier::EXP_CONTROL_INPUT],
        );
        update[Amplifier::SIGNAL_OUTPUT] = input * m;
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        Amplifier::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Amplifier::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Amplifier::SIGNAL_OUTPUT] = UpdateType::Absolute;
    }
}
