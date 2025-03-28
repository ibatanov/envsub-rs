use crate::errors::EnvSubstError;
use regex::Regex;

pub fn extract_variables(input: &str) -> Result<Vec<(String, Option<String>)>, EnvSubstError> {
    let re = Regex::new(r"\$\{([^}:]+)(?::-([^}]+))?\}|\$([^\s]+)").unwrap();
    let mut variables = Vec::new();
    for cap in re.captures_iter(input) {
        if let Some(var) = cap.get(1) {
            let default = cap.get(2).map(|m| m.as_str().to_string());
            variables.push((var.as_str().to_string(), default));
        } else if let Some(var) = cap.get(3) {
            variables.push((var.as_str().to_string(), None));
        }
    }

    for (var, default) in &variables {
        if var.contains(':') && default.is_none() {
            return Err(EnvSubstError::ParsingError(format!(
                "Invalid variable format: ${}",
                var
            )));
        }
    }

    Ok(variables)
}
