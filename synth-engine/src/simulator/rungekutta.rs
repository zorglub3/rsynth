use crate::event::ControllerEvent;
use crate::modules::SynthModule;
use crate::simulator::state::State;
use crate::simulator::state::StateUpdate;
use crate::simulator::Simulator;
use alloc::vec;
use alloc::vec::Vec;

// TODO detect when we are dealing with _stiff equations_ as described
// [here](https://en.wikipedia.org/wiki/Stiff_equation). This is eg when
// the cutoff frequency of a filter goes high. At some point the solver
// won't be able to give a good approximation.

pub struct RungeKutta<const STAGES: usize> {
    state: State,
    updates: [StateUpdate; STAGES],
    temp_states: [State; STAGES],
    a: [[f32; STAGES]; STAGES],
    b: [f32; STAGES],
    c: [f32; STAGES],
    modules: Vec<SynthModule>,
    stack: Vec<f32>,
}

impl RungeKutta<4> {
    pub fn rk4(state_size: usize, stack_size: usize) -> Self {
        Self {
            state: State::new(state_size),
            updates: [
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
            ],
            temp_states: [
                State::new(state_size),
                State::new(state_size),
                State::new(state_size),
                State::new(state_size),
            ],
            a: [
                [0.0, 0.0, 0.0, 0.0],
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            b: [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0],
            c: [0.0, 0.5, 0.5, 1.0],
            modules: Vec::new(),
            stack: vec![0.; stack_size],
        }
    }

    pub fn rk38(state_size: usize, stack_size: usize) -> Self {
        Self {
            state: State::new(state_size),
            updates: [
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
                StateUpdate::new(state_size),
            ],
            temp_states: [
                State::new(state_size),
                State::new(state_size),
                State::new(state_size),
                State::new(state_size),
            ],
            a: [
                [0., 0., 0., 0.],
                [1. / 3., 0., 0., 0.],
                [-1. / 3., 1., 0., 0.],
                [1., -1., 1., 0.],
            ],
            b: [1. / 8., 3. / 8., 3. / 8., 1. / 8.],
            c: [0., 1. / 3., 2. / 3., 1.],
            modules: Vec::new(),
            stack: vec![0.; stack_size],
        }
    }
}

impl RungeKutta<1> {
    pub fn euler(state_size: usize, stack_size: usize) -> Self {
        Self {
            state: State::new(state_size),
            updates: [StateUpdate::new(state_size)],
            temp_states: [State::new(state_size)],
            a: [[0.]],
            b: [1.],
            c: [0.],
            modules: Vec::new(),
            stack: vec![0.; stack_size],
        }
    }
}

impl RungeKutta<2> {
    pub fn second_order(_alpha: f32, _state_size: usize, _stack_size: usize) -> Self {
        todo!("Second order Runge Kutta method")
    }
}

impl<const STAGES: usize> Simulator for RungeKutta<STAGES> {
    fn set_modules(&mut self, modules: Vec<SynthModule>) {
        self.modules = modules;
    }

    fn step(&mut self, dt: f32) {
        // let mut updates = vec![];

        for stage in 0..STAGES {
            self.updates[stage].init(dt * self.c[stage], dt);
            self.temp_states[stage].copy_from(&self.state);
            // let mut update = self.state.update_data(dt * self.c[stage], dt);
            // let mut temp_state = self.state.clone();

            self.temp_states[stage].apply_updates(
                &self.updates,
                &self.a[stage],
                &self.c,
                dt,
                stage);
            // temp_state.apply_updates(&updates, &self.a[stage], &self.c, dt);

            for module in &self.modules {
                module.simulate(
                    &self.temp_states[stage], 
                    &mut self.updates[stage], 
                    &mut self.stack);
            }

            // updates.push(update);
        }

        self.state.apply_updates(&self.updates, &self.b, &self.c, dt, STAGES);
        // self.state.apply_updates(&updates, &self.b, &self.c, dt);

        for module in &mut self.modules {
            module.finalize(&mut self.state, dt, &mut self.stack);
        }
    }

    fn get_stereo_output(&self) -> (f32, f32) {
        (self.state.get_output(0), self.state.get_output(1))
    }

    fn process_event(&mut self, event: ControllerEvent) {
        for module in &mut self.modules {
            module.process_event(&event);
        }
    }
}
