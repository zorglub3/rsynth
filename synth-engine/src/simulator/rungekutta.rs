use crate::event::Event;
use crate::simulator::module::Module;
use crate::simulator::state::State;
use std::collections::HashMap;

pub struct RungeKutta {
    state: State,
    a: Vec<Vec<f32>>,
    b: Vec<f32>,
    c: Vec<f32>,
    stages: usize,
    modules: HashMap<String, Box<dyn Module>>,
}

impl RungeKutta {
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
            modules: HashMap::new(),
        }
    }

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
            modules: HashMap::new(),
        }
    }

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
            modules: HashMap::new(),
        }
    }

    pub fn second_order(_alpha: f32, _state_size: usize) -> Self {
        todo!("Second order Runge Kutta method")
    }

    pub fn with_modules(&mut self, modules: HashMap<String, Box<dyn Module>>) -> Self {
        let state_size = self.state.len();

        Self {
            state: State::new(state_size),
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            stages: self.stages,
            modules,
        }
    }

    pub fn step(&mut self, dt: f32) {
        let mut updates = vec![];

        for stage in 0..self.stages {
            let mut update = self.state.update_data(dt * self.c[stage]);
            let mut temp_state = self.state.clone();

            temp_state.apply_updates(&updates, &self.a[stage], dt);

            for (_id, module) in &self.modules {
                module.simulate(&temp_state, &mut update);
            }

            updates.push(update);
        }

        self.state.apply_updates(&updates, &self.b, dt);

        for (_ix, module) in &mut self.modules {
            module.finalize(&mut self.state);
        }
    }

    pub fn get_stereo_output(&self) -> (f32, f32) {
        (self.state.get_output(0), self.state.get_output(1))
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::MidiEvent { event, channel } => {
                for (_id, module) in self.modules.iter_mut() {
                    module.process_event(&event, channel);
                }
            }
            _ => {}
        }
    }

    pub fn get_state(&mut self) -> &mut State {
        &mut self.state
    }
}
