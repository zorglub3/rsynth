use crate::modules::ModuleError;

pub enum InputSpecTerm {
    Constant(f32),
    Term {
        module_name: String,
        module_field: String,
        scale: f32,
    },
}

impl InputSpecTerm {
    pub fn parse(s: &str, complete: &str) -> Result<Self, ModuleError> {
        // TODO ignore whitespace characters
        if let Ok(v) = s.parse::<f32>() {
            return Ok(InputSpecTerm::Constant(v));
        } else {
            let mut split = s.split(':');

            let Some(module_name) = split.next().map(|s| s.to_string()) else {
                return Err(ModuleError::MalformedInputSpec(complete.to_string()));
            };

            let Some(module_field) = split.next().map(|s| s.to_string()) else {
                return Err(ModuleError::MalformedInputSpec(complete.to_string()));
            };

            let Some(Ok(scale)) = split.next().map(|s| s.parse::<f32>()) else {
                return Err(ModuleError::MalformedInputSpec(complete.to_string()));
            };

            if split.next().is_none() {
                return Ok(InputSpecTerm::Term {
                    module_name,
                    module_field,
                    scale,
                });
            } else {
                return Err(ModuleError::MalformedInputSpec(complete.to_string()));
            }
        }
    }
}

pub struct InputSpec(pub(crate) Vec<InputSpecTerm>);

impl InputSpec {
    pub fn zero() -> Self {
        Self(vec![])
    }

    pub fn new(terms: Vec<InputSpecTerm>) -> Self {
        Self(terms)
    }

    pub fn parse(s: &str) -> Result<Self, ModuleError> {
        let mut result = vec![];

        for x in s.split(',') {
            let term = InputSpecTerm::parse(x, s)?;
            result.push(term);
        }

        Ok(Self(result))
    }
}
