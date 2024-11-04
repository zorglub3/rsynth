use crate::midi::message::MidiMessage;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Amplifier {
    signal_input: InputExpr,
    output_index: usize,
    lin_control_input: InputExpr,
    exp_control_input: InputExpr,
}

impl Amplifier {
    pub fn new(
        signal_input: InputExpr,
        output_index: usize,
        lin_control_input: InputExpr,
        exp_control_input: InputExpr,
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
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let input = self.signal_input.from_state(state);
        let m = amplifier_amount(
            self.lin_control_input.from_state(state),
            self.exp_control_input.from_state(state),
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
