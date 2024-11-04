use crate::simulator::state::State;

pub struct InputExpr(Vec<InputTerm>);

pub enum InputTerm {
    Constant(f32),
    FromState(usize, f32),
}

impl InputTerm {
    pub fn constant(v: f32) -> Self {
        Self::Constant(v)
    }

    pub fn term(index: usize, weight: f32) -> Self {
        Self::FromState(index, weight)
    }
}

impl InputExpr {
    pub fn new(terms: Vec<InputTerm>) -> Self {
        Self(terms)
    }

    pub fn zero() -> Self {
        Self(vec![])
    }

    pub fn from_index(index: usize) -> Self {
        Self(vec![InputTerm::term(index, 1.)])
    }

    pub fn constant(v: f32) -> Self {
        Self(vec![InputTerm::constant(v)])
    }

    pub fn from_state(&self, state: &State) -> f32 {
        let mut acc = 0.;

        for term in self.0.iter() {
            match term {
                InputTerm::Constant(v) => acc += v,
                InputTerm::FromState(index, scale) => acc += state.get(*index) * scale,
            }
        }

        acc
    }
}
