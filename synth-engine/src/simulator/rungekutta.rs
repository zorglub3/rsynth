use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::modules::SynthModule;
use crate::simulator::state::State;
use crate::simulator::state::StateInput;
use crate::simulator::state::StateUpdate;
use crate::simulator::state::UpdateType;
use crate::simulator::Simulator;
use alloc::vec;
use alloc::vec::Vec;

// TODO detect when we are dealing with _stiff equations_ as described
// [here](https://en.wikipedia.org/wiki/Stiff_equation). This is eg when
// the cutoff frequency of a filter goes high. At some point the solver
// won't be able to give a good approximation.

struct ModuleEntry {
    module: SynthModule,
    input_offset: usize,
    input_end: usize,
    output_offset: usize,
    output_end: usize,
}

pub struct RungeKutta<const STAGES: usize> {
    state: State,
    state_input: StateInput,
    update_types: Vec<UpdateType>,
    updates: [StateUpdate; STAGES],
    temp_states: [State; STAGES],
    outputs: Vec<f32>,
    a: [[f32; STAGES]; STAGES],
    b: [f32; STAGES],
    c: [f32; STAGES],
    module_entries: Vec<ModuleEntry>,
    max_input: usize,
    max_state: usize,
}

impl RungeKutta<4> {
    pub fn rk4(
        state_size: usize,
        input_size: usize,
        output_size: usize,
        stack_size: usize,
    ) -> Self {
        Self {
            state: State::new(state_size),
            state_input: StateInput::new(input_size, stack_size),
            update_types: vec![UpdateType::default(); state_size],
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
            outputs: vec![0.; output_size],
            a: [
                [0.0, 0.0, 0.0, 0.0],
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            b: [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0],
            c: [0.0, 0.5, 0.5, 1.0],
            module_entries: Vec::new(),
            max_input: 0,
            max_state: 0,
        }
    }

    pub fn rk38(
        state_size: usize,
        input_size: usize,
        output_size: usize,
        stack_size: usize,
    ) -> Self {
        Self {
            state: State::new(state_size),
            state_input: StateInput::new(input_size, stack_size),
            update_types: vec![UpdateType::default(); state_size],
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
            outputs: vec![0.; output_size],
            a: [
                [0., 0., 0., 0.],
                [1. / 3., 0., 0., 0.],
                [-1. / 3., 1., 0., 0.],
                [1., -1., 1., 0.],
            ],
            b: [1. / 8., 3. / 8., 3. / 8., 1. / 8.],
            c: [0., 1. / 3., 2. / 3., 1.],
            module_entries: Vec::new(),
            max_input: 0,
            max_state: 0,
        }
    }
}

impl RungeKutta<1> {
    pub fn euler(
        state_size: usize,
        input_size: usize,
        output_size: usize,
        stack_size: usize,
    ) -> Self {
        Self {
            state: State::new(state_size),
            state_input: StateInput::new(input_size, stack_size),
            update_types: vec![UpdateType::default(); state_size],
            updates: [StateUpdate::new(state_size)],
            temp_states: [State::new(state_size)],
            outputs: vec![0.; output_size],
            a: [[0.]],
            b: [1.],
            c: [0.],
            module_entries: Vec::new(),
            max_input: 0,
            max_state: 0,
        }
    }
}

/*
impl RungeKutta<2> {
    pub fn second_order(_alpha: f32, _state_size: usize, _stack_size: usize) -> Self {
        todo!("Second order Runge Kutta method")
    }
}
*/

impl<const STAGES: usize> RungeKutta<STAGES> {
    pub fn add_module(&mut self, module: SynthModule) {
        let input_offset = self.max_input;
        let input_end = input_offset + module.get_input_size();
        let output_offset = self.max_state;
        let output_end = output_offset + module.get_state_size();

        self.max_input = input_end;
        self.max_state = output_end;

        module.set_update_type(&mut self.update_types[output_offset..output_end]);

        self.module_entries.push(ModuleEntry {
            module,
            input_offset,
            input_end,
            output_offset,
            output_end,
        });
    }
}

impl<const STAGES: usize> Simulator for RungeKutta<STAGES> {
    fn set_modules(&mut self, modules: Vec<SynthModule>) {
        for module in modules {
            self.add_module(module);
        }
    }

    fn step(&mut self, dt: f32, control_interface: &ControlInterface) {
        for stage in 0..STAGES {
            self.updates[stage].init(dt * self.c[stage], dt);
            self.temp_states[stage].copy_from(&self.state);

            self.temp_states[stage].apply_updates(
                &self.updates,
                &self.update_types,
                &self.a[stage],
                &self.c,
                dt,
                stage,
            );

            self.state_input
                .compute_inputs(self.temp_states[stage].borrow_all_values());

            for synth_module in &self.module_entries {
                synth_module.module.simulate(
                    control_interface,
                    self.state_input
                        .borrow_inputs(synth_module.input_offset, synth_module.input_end),
                    self.temp_states[stage]
                        .borrow_values(synth_module.output_offset, synth_module.output_end),
                    self.updates[stage]
                        .values_mut(synth_module.output_offset, synth_module.output_end),
                    self.c[stage], // TODO - or is it dt...? or dt * c?
                );
            }

            /*
            for module in &self.modules {
                module.simulate(
                    &self.temp_states[stage],
                    &mut self.updates[stage],
                    &mut self.stack,
                );
            }
            */
        }

        self.state.apply_updates(
            &self.updates,
            &self.update_types,
            &self.b,
            &self.c,
            dt,
            STAGES,
        );

        self.state_input
            .compute_inputs(self.state.borrow_all_values());

        for synth_module in &mut self.module_entries {
            synth_module.module.finalize(
                self.state_input
                    .borrow_inputs(synth_module.input_offset, synth_module.input_end),
                self.state
                    .values_mut(synth_module.output_offset, synth_module.output_end),
                &mut self.outputs,
                dt,
            );
            // module.finalize(&mut self.state, dt, &mut self.stack);
        }
    }

    fn get_stereo_output(&self) -> (f32, f32) {
        (self.outputs[0], self.outputs[1])
        // (self.state.get_output(0), self.state.get_output(1))
    }

    /*
    fn process_event(&mut self, event: ControllerEvent) {
        for module in &mut self.modules {
            module.process_event(&event);
        }
    }
    */
}
