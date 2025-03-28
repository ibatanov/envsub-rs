#[derive(Debug)]
pub enum EnvSubstError {
    VariableNotSet(String),
    VariableEmpty(String),
    ParsingError(String),
}

impl std::fmt::Display for EnvSubstError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EnvSubstError::VariableNotSet(var) => write!(f, "Variable {} not set", var),
            EnvSubstError::VariableEmpty(var) => write!(f, "Variable {} is empty", var),
            EnvSubstError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl std::error::Error for EnvSubstError {}
