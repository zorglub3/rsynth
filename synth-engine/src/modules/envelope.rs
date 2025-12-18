use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;
use core::f32::consts::PI;

#[allow(dead_code)]
fn hamming(x: f32) -> f32 {
    let x = x.clamp(0., 1.);

    0.54 - 0.46 * (2. * PI * x).cos()
}

#[allow(dead_code)]
fn blackman(x: f32) -> f32 {
    let x = x.clamp(0., 1.);
    0.42 - 0.5 * (2. * PI * x).cos() + 0.08 * (4. * PI * x).cos()
}

#[allow(dead_code)]
fn triangle(x: f32) -> f32 {
    let x = x.clamp(0., 1.);

    1. - (2. * x - 1.).abs()
}

const MIN_TIME: f32 = 0.01_f32; // 10 ms

#[derive(Debug)]
enum EnvState {
    Finished,
    Attack,
    Hold,
    Decay,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum EnvType {
    AttackDecay,
    AttackRelease,
    Cyclic,
}

pub struct Envelope {
    // signal_input: StackProgram,
    // attack_input: StackProgram,
    // decay_input: StackProgram,
    // shape_select: StackProgram,
    // output_index: usize,
    // cycle_state: usize,
    env_state: EnvState,
    env_type: EnvType,
}

impl Envelope {
    pub const STATE_SIZE: usize = 2;
    pub const INPUT_SIZE: usize = 4;

    // state/outputs
    pub const CONTROL_OUTPUT: usize = 0;
    pub const CYCLE_STATE: usize = 1;

    // inputs
    pub const SIGNAL_INPUT: usize = 0;
    pub const ATTACK_INPUT: usize = 1;
    pub const DECAY_INPUT: usize = 2;
    pub const SHAPE_SELECT: usize = 3;

    pub fn new(
        // signal_input: StackProgram,
        // attack_input: StackProgram,
        // decay_input: StackProgram,
        // shape_select: StackProgram,
        // output_index: usize,
        // cycle_state: usize,
        env_type: EnvType,
    ) -> Self {
        Self {
            // signal_input,
            // attack_input,
            // decay_input,
            // shape_select,
            // output_index,
            // cycle_state,
            env_state: EnvState::Finished,
            env_type,
            // env_type: EnvType::AttackRelease,
        }
    }
}

fn rise_decay(t: f32) -> f32 {
    let t = t.max(MIN_TIME);

    1. / t
}

fn output_value(cycle_index: f32, shape: f32) -> f32 {
    let cycle_index = (cycle_index * 0.5).clamp(0., 0.5);
    let shape = shape.clamp(0., 1.);
    blackman(cycle_index) * shape + triangle(cycle_index) * (1. - shape)
}

impl Module for Envelope {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        updates: &mut [f32],
        _dt: f32,
    ) {
        let attack = inputs[Envelope::ATTACK_INPUT];
        let decay = inputs[Envelope::DECAY_INPUT];
        
        // println!("attack {}, decay {}", attack, decay);
        match self.env_state {
            EnvState::Attack => {
                let delta = rise_decay(attack);
                updates[Envelope::CYCLE_STATE] = delta;
            }
            EnvState::Decay => {
                let delta = -rise_decay(decay);
                updates[Envelope::CYCLE_STATE] = delta;
            }
            _ => { /* do nothing */ }
        }

        updates[Envelope::CONTROL_OUTPUT] =
            output_value(
                state[Envelope::CYCLE_STATE],
                inputs[Envelope::SHAPE_SELECT],
            );
    }

    fn finalize(&mut self, inputs: &[f32], state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let input_state = inputs[Envelope::SIGNAL_INPUT];
        let output_state = state[Envelope::CONTROL_OUTPUT];
        let cycle = state[Envelope::CYCLE_STATE];

        use EnvState::*;
        use EnvType::*;

        match (&self.env_state, &self.env_type) {
            (Attack, AttackRelease) if cycle >= 1. => self.env_state = Hold,
            (Attack, AttackDecay|Cyclic) if cycle >= 1. => self.env_state = Decay,
            (Hold, AttackRelease) if input_state < 0.5 => self.env_state = Decay,
            (Hold, AttackDecay|Cyclic) => self.env_state = Decay,
            (Decay, Cyclic) if cycle <= 0. => self.env_state = Attack,
            (Decay, AttackRelease|AttackDecay) if input_state >= 0.5 => self.env_state = Attack,
            (Decay, AttackRelease|AttackDecay) if cycle <= 0. => self.env_state = Finished,
            (Finished, Cyclic) => self.env_state = Attack,
            (Finished, AttackRelease|AttackDecay) if input_state > 0.5 => self.env_state = Attack,
            _ => { /* do nothing */ }
        }

        // println!("state {:?}, {}, {}", &self.env_state, cycle, output_state);

        state[Envelope::CONTROL_OUTPUT] = output_state.clamp(0., 1.);
        state[Envelope::CYCLE_STATE] = cycle.clamp(0., 1.);
    }

    fn get_input_size(&self) -> usize {
        Envelope::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Envelope::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Envelope::CYCLE_STATE] = UpdateType::Differentiable;
        update_types[Envelope::CONTROL_OUTPUT] = UpdateType::Absolute;
    }
}
