use crate::event::ControllerEvent;
use crate::modules::SynthModule;
use crate::simulator::state::{State, StateUpdate};

/*
 // TODO delete
#[cfg(any(features = allocator, test))]
use alloc::boxed::Box;

#[cfg(any(features = allocator, test))]
use alloc::vec;

#[cfg(any(features = allocator, test))]
use alloc::vec::Vec;
*/

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

    /*
    pub fn rk4(state_size: usize) -> Self {
        let a = vec![vec![], vec![0.5], vec![0.0, 0.5], vec![0.0, 0.0, 1.0]];

        let b = vec![1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0];

        let c = vec![0.0, 0.5, 0.5, 1.0];

        Self {
            state: State::new(state_size),
            a,
            b,
            c,
            stages: 4,
            modules: Vec::new(),
            stack: vec![0.0_f32; DEFAULT_STACK_SIZE],
        }
    }
    */

    /*
    pub fn rk38(state_size: usize) -> Self {
        let a = vec![
            vec![],
            vec![1.0 / 3.0],
            vec![-1.0 / 3.0, 1.0],
            vec![1.0, -1.0, 1.0],
        ];

        let b = vec![1.0 / 8.0, 3.0 / 8.0, 3.0 / 8.0, 1.0 / 8.0];

        let c = vec![0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];

        Self {
            state: State::new(state_size),
            a,
            b,
            c,
            stages: 4,
            modules: Vec::new(),
            stack: vec![0.0_f32; DEFAULT_STACK_SIZE],
        }
    }
    */

    /*
    pub fn euler(state_size: usize) -> Self {
        let a = vec![vec![]];
        let b = vec![1.0];
        let c = vec![0.0];

        Self {
            state: State::new(state_size),
            a,
            b,
            c,
            stages: 1,
            modules: Vec::new(),
            stack: vec![0.0_f32; DEFAULT_STACK_SIZE],
        }
    }
    */

    /*
    pub fn second_order(_alpha: f32, _state_size: usize) -> Self {
        todo!("Second order Runge Kutta method")
    }
    */

    /*
    pub fn with_modules(&mut self, modules: Vec<Box<dyn Module>>) -> Self {
        let state_size = self.state.len();

        Self {
            state: State::new(state_size),
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            stages: self.stages,
            modules,
            stack: self.stack.clone(),
        }
    }
    */

    // TODO cleanup
    pub fn step(&mut self, dt: f32, updates: &mut [StateUpdate], temp_states: &mut [State<'b>]) {
        // let mut updates = vec![];

        for stage in 0..STAGES {
            self.state
                .clear_update_data(&mut updates[stage], dt * self.c[stage], dt);
            // let mut update = self.state.update_data(dt * self.c[stage], dt);
            self.state.copy_values_to(&mut temp_states[stage]);
            // let mut temp_state = self.state.clone();

            temp_states[stage].apply_updates(updates, &self.a[stage], &self.c, dt, stage);
            // temp_state.apply_updates(&updates, &self.a[stage], &self.c, dt);

            for module in self.modules.iter() {
                module.simulate(&temp_states[stage], &mut updates[stage], &mut self.stack);
            }

            // updates.push(update);
        }

        self.state
            .apply_updates(updates, &self.b, &self.c, dt, STAGES);

        for module in self.modules.iter_mut() {
            module.finalize(&mut self.state, dt, &mut self.stack);
        }
    }

    pub fn get_stereo_output(&self) -> (f32, f32) {
        (self.state.get_output(0), self.state.get_output(1))
    }

    pub fn process_event(&mut self, event: ControllerEvent) {
        for module in self.modules.iter_mut() {
            module.process_event(&event);
        }
    }

    /*
     // TODO delete
    pub fn get_state(&mut self) -> &mut State {
        &mut self.state
    }
    */
}
