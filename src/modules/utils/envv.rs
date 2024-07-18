use std::collections::HashMap;
use std::env;

pub fn get_env_vars() -> HashMap<String, String> {
    let keys = [
        "MSAILOR_TEST_ENV_VAR",
        "VISUAL",
        "EDITOR",
    ];
    let mut env_vars = HashMap::new();

    for key in &keys {
        if let Ok(value) = env::var(key) {
            env_vars.insert(key.to_string(), value);
        }
    }

    env_vars
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_env_vars() {
        // Set up the environment variable for the test
        let test_key = "MSAILOR_TEST_ENV_VAR";
        let test_value = "vim";
        env::set_var(test_key, test_value);

        // Call the function
        let env_vars = get_env_vars();

        // Check that the environment variable was correctly retrieved
        assert_eq!(env_vars.get(test_key), Some(&test_value.to_string()));

        // Clean up the environment variable
        env::remove_var(test_key);
    }

    #[test]
    fn test_get_env_vars_missing_key() {
        // Make sure the environment variable is not set
        let test_key = "MSAILOR_NON_EXISTENT_TEST_ENV_VAR";
        env::remove_var(test_key);

        // Call the function
        let env_vars = get_env_vars();

        // Check that the environment variable is not in the HashMap
        assert!(!env_vars.contains_key(test_key));
    }

}
