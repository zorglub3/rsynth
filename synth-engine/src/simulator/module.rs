use crate::control_interface::ControlInterface;
use crate::simulator::state::UpdateType;

pub trait Module: Send {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    );
    fn finalize(&mut self, inputs: &[f32], state: &mut [f32], outputs: &mut [f32], dt: f32);
    fn get_input_size(&self) -> usize;
    fn get_state_size(&self) -> usize;
    fn set_update_type(&self, update_types: &mut [UpdateType]);
}
