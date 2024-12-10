use core::ops::Range;

pub struct StateAllocator(Range<usize>);

impl StateAllocator {
    pub fn new(size: usize) -> Self {
        let size = size.max(16);
        Self(0..size)
    }

    fn allocate_state_slot(&mut self) -> usize {
        if self.0.is_empty() {
            self.0 = Range {
                start: self.0.start,
                end: self.0.end * 2,
            };
        }

        let slot = self.0.start;

        self.0 = Range {
            start: self.0.start + 1,
            end: self.0.end,
        };

        slot
    }

    pub fn allocate(&mut self, state: &mut [usize]) {
        for i in 0..state.len() {
            state[i] = self.allocate_state_slot();
        }
    }
}
