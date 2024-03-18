use crate::event::Event;
use crate::modules::Module;
use crate::simulator::Simulator;
use crate::state::*;
use std::collections::HashMap;
use std::iter::zip;

const DEFAULT_STATE_SIZE: usize = 32;

pub struct Rk4 {
    state_type: Vec<OutputType>,
    current_state: Vec<f32>,
    k: [Vec<f32>; 4],
    a: [f32; 4],
    b: [f32; 4],
    modules: HashMap<String, Box<dyn Module>>,
    output_indices: (usize, usize),
}

impl Rk4 {
    fn modules_to_output_types(
        capacity: usize,
        modules: &Vec<(String, Box<dyn Module>)>,
    ) -> Vec<OutputType> {
        let mut out = vec![OutputType::Differentiable; capacity];

        for (_id, module) in modules {
            for (index, tpe) in zip(module.outputs(), module.output_types()) {
                out[index] = tpe
            }
        }

        out
    }

    pub fn with_modules(modules: Vec<(String, Box<dyn Module>)>) -> Self {
        let capacity = modules
            .iter()
            .map(|m| m.1.max_output_index())
            .max()
            .unwrap_or(DEFAULT_STATE_SIZE)
            + 1;

        Rk4 {
            state_type: Self::modules_to_output_types(capacity, &modules),
            current_state: vec![0.0; capacity],
            k: [
                vec![0.0; capacity],
                vec![0.0; capacity],
                vec![0.0; capacity],
                vec![0.0; capacity],
            ],
            a: [0.0, 0.5, 0.5, 1.0],
            b: [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0],
            modules: modules.into_iter().collect(),
            output_indices: (1, 2),
        }
    }

    pub fn new(capacity: usize) -> Self {
        Rk4 {
            state_type: vec![OutputType::Differentiable; capacity],
            current_state: vec![0.0; capacity],
            k: [
                vec![0.0; capacity],
                vec![0.0; capacity],
                vec![0.0; capacity],
                vec![0.0; capacity],
            ],
            a: [0.0, 0.5, 0.5, 1.0],
            b: [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0],
            modules: HashMap::new(),
            output_indices: (1, 2),
        }
    }
}

impl Simulator for Rk4 {
    fn step(&mut self, dt: f32) {
        let mut temp_state = vec![0.0; self.current_state.len()];

        copy_state(&self.current_state, &mut temp_state);

        for stage in 0..4 {
            self.k[stage].fill(0.);

            for (_id, module) in &self.modules {
                module.simulate(dt * self.a[stage], &temp_state, &mut self.k[stage]);
            }

            if stage < 3 {
                update_state(
                    &self.state_type,
                    dt * self.a[stage + 1],
                    &self.current_state,
                    &self.k[stage],
                    &mut temp_state,
                );
            }
        }

        let kk: [&[f32]; 4] = [&self.k[0], &self.k[1], &self.k[2], &self.k[3]];

        finalize_state(dt, &self.state_type, &self.b, &kk, &mut self.current_state);

        for (_id, module) in &self.modules {
            module.finalize(&mut self.current_state);
        }
    }

    fn get_output(&self) -> (f32, f32) {
        let (left_out, right_out) = self.output_indices;
        (self.current_state[left_out], self.current_state[right_out])
    }

    fn process_event(&mut self, event: Event) {
        match event {
            Event::MidiEvent { event, channel } => {
                for (_id, module) in self.modules.iter_mut() {
                    module.process_event(&event, channel);
                }
            }
            Event::SetState { index, value } => self.current_state[index] = value,
            Event::AddModule { id, module } => self.add_module(id, module),
            Event::DeleteModule { id } => self.delete_module(id),
            Event::ConnectModules {
                source_id,
                source_index,
                target_id,
                target_index,
            } => self.connect_module(source_id, source_index, target_id, target_index),
            Event::DisconnectModules { .. } => todo!("disconnect modules"),
            Event::SetConstant { .. } => todo!("Set constant value for module input"),
        }
    }

    fn delete_module(&mut self, id: String) {
        if let Some(_module) = self.modules.remove(&id) {
            todo!("delete/remap connections sourcing this module");
        }
    }

    fn add_module(&mut self, _id: String, _module: Box<dyn Module>) {
        todo!("Set the connections on the module");
        /*
        if self.modules.contains_key(&id) {
            eprintln!("Warning: there is already a module in the simulator with id {id}");
        } else {
            self.modules.insert(id, module);
        }
        */
    }

    fn connect_module(
        &mut self,
        source_id: String,
        _source_index: usize,
        target_id: String,
        _target_index: usize,
    ) {
        if let (Some(_source_module), Some(_target_module)) =
            (self.modules.get(&source_id), self.modules.get(&target_id))
        {
            todo!("set connections on source module");
        } else {
            eprintln!(
                "Tried to connect {source_id} to {target_id} but at least one module is missing"
            );
        }
    }
}
