use crate::{errors::EnvSubstError, parser::extract_variables};
use std::env;

pub fn substitute_variable(
    var: &str,
    default: Option<&str>,
    no_unset: bool,
    no_empty: bool,
) -> Result<String, EnvSubstError> {
    match env::var(var) {
        Ok(value) => {
            if no_empty && value.is_empty() && default.is_none() {
                Err(EnvSubstError::VariableEmpty(var.to_string()))
            } else {
                Ok(if value.is_empty() && default.is_some() {
                    default.unwrap().to_string()
                } else {
                    value
                })
            }
        }
        Err(_) if no_unset && default.is_none() => {
            Err(EnvSubstError::VariableNotSet(var.to_string()))
        }
        Err(_) => Ok(default.unwrap_or("").to_string()),
    }
}

pub fn substitute(input: &str, no_unset: bool, no_empty: bool) -> Result<String, EnvSubstError> {
    let variables = extract_variables(input)?;
    let mut result = input.to_string();

    for (var, default) in variables {
        let value = match env::var(&var) {
            Ok(v) => {
                if no_empty && v.is_empty() {
                    return Err(EnvSubstError::VariableEmpty(var.to_string()));
                }
                v
            }
            Err(_) => {
                if default.is_none() && (no_unset || no_empty) {
                    return Err(EnvSubstError::VariableNotSet(var.to_string()));
                }
                default.clone().unwrap_or_else(|| "".to_string())
            }
        };

        if let Some(default_val) = &default {
            let pattern = format!(
                "${{{var}:-{default_val}}}",
                var = var,
                default_val = default_val
            );
            result = result.replace(&pattern, &value);
        }

        result = result.replace(&format!("${{{}}}", var), &value);
        result = result.replace(&format!("${}", var), &value);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_var;

    #[test]
    fn test_substitute_variable_not_set() {
        // Переменная не установлена
        with_var("TEST_VAR", None::<String>, || {
            let result = substitute_variable("TEST_VAR", None, true, false);
            assert!(matches!(result, Err(EnvSubstError::VariableNotSet(_))));
        });
    }

    #[test]
    fn test_substitute_variable_empty() {
        // Переменная установлена, но пуста
        with_var("TEST_VAR", Some(""), || {
            let result = substitute_variable("TEST_VAR", None, false, true);
            assert!(matches!(result, Err(EnvSubstError::VariableEmpty(_))));
        });
    }

    #[test]
    fn test_substitute_variable_with_default() {
        // Переменная не установлена, но есть значение по умолчанию
        with_var("TEST_VAR", None::<String>, || {
            let result = substitute_variable("TEST_VAR", Some("default"), true, true);
            assert_eq!(result.unwrap(), "default");
        });

        // Переменная пуста, но есть значение по умолчанию
        with_var("TEST_VAR", Some(""), || {
            let result = substitute_variable("TEST_VAR", Some("default"), true, true);
            assert_eq!(result.unwrap(), "default");
        });
    }

    #[test]
    fn test_substitute_variable_with_default_and_no_empty() {
        // Переменная пуста, но есть значение по умолчанию, и установлен флаг --no-empty
        with_var("USER", Some(""), || {
            let result = substitute_variable("USER", Some("guest"), false, true);
            assert_eq!(result.unwrap(), "guest");
        });
    }

    #[test]
    fn test_substitute_no_empty_with_mixed_variables() {
        let input = "Test $USER and ${USER:-guest}";

        // Переменная USER пуста, но есть значение по умолчанию для ${USER:-guest}.
        // При флаге `--no-empty` должна быть ошибка из-за $USER.
        with_var("USER", Some(""), || {
            let result = substitute(input, false, true);
            assert!(matches!(result, Err(EnvSubstError::VariableEmpty(_))));
        });
    }

    #[test]
    fn test_substitute_variable_set() {
        // Переменная установлена
        with_var("TEST_VAR", Some("value"), || {
            let result = substitute_variable("TEST_VAR", None, false, false);
            assert_eq!(result.unwrap(), "value");
        });
    }

    #[test]
    fn test_substitute_with_flags() {
        let input = "Test $TEST_VAR and ${TEST_VAR} and ${TEST_VAR:-default}";

        // Тест с --no-unset
        with_var("TEST_VAR", None::<String>, || {
            let result = substitute(input, true, false);
            assert!(matches!(result, Err(EnvSubstError::VariableNotSet(_))));
        });

        // Тест с --no-empty
        with_var("TEST_VAR", Some(""), || {
            let result = substitute(input, false, true);
            assert!(matches!(result, Err(EnvSubstError::VariableEmpty(_))));
        });

        // Тест с установленной переменной
        with_var("TEST_VAR", Some("value"), || {
            let result = substitute(input, false, false).unwrap();
            assert_eq!(result, "Test value and value and value");
        });

        // Тест с не установленной переменной (и значением по умолчанию)
        with_var("TEST_VAR", None::<String>, || {
            let result = substitute(input, false, false).unwrap();
            assert_eq!(result, "Test  and  and default");
        });
    }
}
