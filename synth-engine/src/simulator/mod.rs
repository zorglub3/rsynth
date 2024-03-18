use crate::event::Event;
use crate::modules::Module;

pub mod rk4;

pub trait Simulator: Send {
    fn step(&mut self, dt: f32);
    fn get_output(&self) -> (f32, f32);
    fn process_event(&mut self, event: Event);
    fn delete_module(&mut self, id: String);
    fn add_module(&mut self, id: String, module: Box<dyn Module>);
    fn connect_module(
        &mut self,
        source_id: String,
        source_index: usize,
        target_id: String,
        target_index: usize,
    );
}
