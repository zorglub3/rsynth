use core::ops::Range;

pub struct StateAllocator {
    next_state_index: usize,
    next_input_index: usize,
}

impl StateAllocator {
    pub fn new() -> Self {
        Self {
            next_state_index: 0,
            next_input_index: 0,
        }
    }

    pub fn allocate(&mut self, state_size: usize, input_size: usize) -> StateInputRange {
        let state_input_range = StateInputRange::new(
            self.next_state_index,
            self.next_state_index + state_size,
            self.next_input_index,
            self.next_input_index + input_size,
        );

        self.next_state_index += state_size;
        self.next_input_index += input_size;

        state_input_range
    }

    pub fn get_total_state_size(&self) -> usize {
        self.next_state_index
    }

    pub fn get_total_input_size(&self) -> usize {
        self.next_input_index
    }
}

pub struct StateInputRange {
    pub state_range: Range<usize>,
    pub input_range: Range<usize>,
}

impl StateInputRange {
    fn new(state_start: usize, state_end: usize, input_start: usize, input_end: usize) -> Self {
        Self {
            state_range: state_start..state_end,
            input_range: input_start..input_end,
        }
    }
}
/*

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
*/
