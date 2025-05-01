use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use crate::synth_math::SynthMath;
use core::f32::consts::PI;

pub const STATE_SIZE: usize = 2;
pub const INPUT_SIZE: usize = 2;

// state/outputs
const STATE_X: usize = 0;
const STATE_Y: usize = 1;

// inputs/controls
const EXP_CONTROL_INPUT: usize = 0;
const LINEAR_CONTROL_INPUT: usize = 1;

// TODO add to synth designer
pub struct QuadratureOscillator {
    f0: f32,
    state_x_index: usize,
    state_y_index: usize,
    control_input: StackProgram,
    linear_control: StackProgram,
}

impl QuadratureOscillator {
    pub fn new(
        f0: f32,
        state_x_index: usize,
        state_y_index: usize,
        control_input: StackProgram,
        linear_control: StackProgram,
    ) -> Self {
        Self {
            f0,
            state_x_index,
            state_y_index,
            control_input,
            linear_control,
        }
    }
}

impl Module for QuadratureOscillator {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        let omega = 2. * PI * control_to_frequency(
            self.f0,
            inputs[EXP_CONTROL_INPUT],
            inputs[LINEAR_CONTROL_INPUT],
        );

        let x = state[STATE_X];
        let y = state[STATE_Y];

        update[STATE_X] = omega * y;
        update[STATE_Y] = -omega * x;
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], outputs: &mut [f32], dt: f32) {
        let x = state[STATE_X];
        let y = state[STATE_Y];
        let s = (x * x + y * y).sqrt();

        if s < f32::EPSILON {
            state[STATE_X] = 0.;
            state[STATE_Y] = 1.;
        } else {
            state[STATE_X] /= s;
            state[STATE_Y] /= s;
        }
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[STATE_X] = UpdateType::Differentiable;
        update_types[STATE_Y] = UpdateType::Differentiable;
    }

    /*
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let omega = 2.
            * PI
            * control_to_frequency(
                self.f0,
                self.control_input.run(state, stack).unwrap_or(0.),
                self.linear_control.run(state, stack).unwrap_or(0.),
            );

        let x = state.get(self.state_x_index);
        let y = state.get(self.state_y_index);

        update.set(self.state_x_index, omega * y, UpdateType::Differentiable);
        update.set(self.state_y_index, -omega * x, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        let x = state.get(self.state_x_index);
        let y = state.get(self.state_y_index);
        let s = (x * x + y * y).sqrt();

        if s < f32::EPSILON {
            state.set(self.state_x_index, 0.);
            state.set(self.state_y_index, 1.);
        } else {
            state.set(self.state_x_index, x / s);
            state.set(self.state_y_index, y / s);
        }
    }
    */
}
