use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Amplifier {
    input_index: usize,
    output_index: usize,
    lin_control_index: usize,
    exp_control_index: usize,
}

impl Amplifier {
    pub fn new(
        input_index: usize,
        output_index: usize,
        lin_control_index: usize,
        exp_control_index: usize,
    ) -> Self {
        Self {
            input_index,
            output_index,
            lin_control_index,
            exp_control_index,
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
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let input = state.get(self.input_index);
        let m = amplifier_amount(
            state.get(self.lin_control_index),
            state.get(self.exp_control_index),
        );
        update.set(self.output_index, input * m, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
