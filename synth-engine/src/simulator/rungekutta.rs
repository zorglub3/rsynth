use crate::event::ControllerEvent;
use crate::modules::SynthModule;
use crate::simulator::state::{State, StateUpdate};
use crate::simulator::Simulator;

// TODO detect when we are dealing with _stiff equations_ as described
// [here](https://en.wikipedia.org/wiki/Stiff_equation). This is eg when
// the cutoff frequency of a filter goes high. At some point the solver
// won't be able to give a good approximation.

pub struct RungeKutta<'a, 'b, const STAGES: usize> {
    state: &'a mut State<'b>,
    a: [[f32; STAGES]; STAGES],
    b: [f32; STAGES],
    c: [f32; STAGES],
    modules: &'a mut [SynthModule<'a>],
    stack: &'a mut [f32],
}

impl<'a, 'b> RungeKutta<'a, 'b, 4_usize> {
    pub fn rk4(state: &'a mut State<'b>, stack: &'a mut [f32]) -> Self {
        Self {
            state,
            a: [
                [0.0, 0.0, 0.0, 0.0],
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            b: [1. / 6., 1. / 3., 1. / 3., 1. / 6.],
            c: [0., 0.5, 0.5, 1.],
            modules: &mut [],
            stack,
        }
    }

    pub fn rk38(state: &'a mut State<'b>, stack: &'a mut [f32]) -> Self {
        Self {
            state,
            a: [
                [0., 0., 0., 0.],
                [1./3., 0., 0., 0.],
                [-1./3., 1., 0., 0.],
                [1., -1., 1., 0.],
            ],
            b: [1./8., 3./8., 3./8., 1./8.],
            c: [0., 1./3., 2./3., 1.],
            modules: &mut [], 
            stack,
        }
    }
}

impl<'a, 'b> RungeKutta<'a, 'b, 1_usize> {
    pub fn euler(state: &'a mut State<'b>, stack: &'a mut [f32]) -> Self {
        Self {
            state,
            a: [[0.0]],
            b: [1.0],
            c: [0.0],
            modules: &mut [],
            stack,
        }
    }
}

impl<'a, 'b> RungeKutta<'a, 'b, 2_usize> {
    pub second_order(_alpha: f32, state: &'a mut State<'b>, stack: &'a mut [f32]) -> Self {
        todo!("Second order Runge Kutta method")
    }
}

impl<'a, 'b, const STAGES: usize> RungeKutta<'a, 'b, STAGES> {
    pub fn with_modules(self, modules: &'a mut [SynthModule<'a>]) -> Self {
        Self {
            state: self.state,
            a: self.a,
            b: self.b,
            c: self.c,
            modules,
            stack: self.stack,
        }
    }
}

impl<'a, 'b, const STAGES: usize> Simulator for RungeKutta<'a, 'b, STAGES> {
    fn step<'c>(&mut self, dt: f32, updates: &mut [StateUpdate<'c>], temp_states: &mut [State<'c>]) {
        for stage in 0..STAGES {
            self.state.clear_update_data(&mut updates[stage], dt * self.c[stage], dt);

            // TODO use `copy_values_to` on State (but fix lifetime)
            temp_states[stage].values.copy_from_slice(self.state.values);

            temp_states[stage].apply_updates(updates, &self.a[stage], &self.c, dt, stage);

            for module in self.modules.iter() {
                module.simulate(&temp_states[stage], &mut updates[stage], &mut self.stack);
            }
        }

        self.state
            .apply_updates(updates, &self.b, &self.c, dt, STAGES);

        for module in self.modules.iter_mut() {
            module.finalize(&mut self.state, dt, &mut self.stack);
        }
    }

    fn get_stereo_output(&self) -> (f32, f32) {
        (self.state.get_output(0), self.state.get_output(1))
    }

    fn process_event(&mut self, event: ControllerEvent) {
        for module in self.modules.iter_mut() {
            module.process_event(&event);
        }
    }
}
