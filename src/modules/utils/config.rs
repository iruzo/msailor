use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Parses a key-value configuration file and returns the content as a `HashMap`.
pub fn parse_config_file(config_file_path: &str, default_paths: Option<HashMap<String, String>>) -> io::Result<HashMap<String, String>> {
    let path = Path::new(config_file_path);

    if !path.exists() {
        return Ok(HashMap::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut config_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        // Skip empty lines and comments
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        // Split the line into key and value
        if let Some((key, value)) = line.split_once('=') {
            config_map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // If default_paths is provided, insert only missing key-value pairs into config_map
    if let Some(defaults) = default_paths {
        for (key, value) in defaults {
            config_map.entry(key).or_insert(value);
        }
    }

    Ok(config_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;

    #[test]
    fn test_parse_config_file() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_config.cfg");

        // Create a sample config file
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "key1 = value1").unwrap();
        writeln!(file, "key2=value2").unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "key3 = value3").unwrap();
        writeln!(file, "   ").unwrap(); // Empty line

        // Parse the config file
        let result = parse_config_file(config_path.to_str().unwrap(), None);
        assert!(result.is_ok());

        let config_map = result.unwrap();
        assert_eq!(config_map.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config_map.get("key2"), Some(&"value2".to_string()));
        assert_eq!(config_map.get("key3"), Some(&"value3".to_string()));

        // Clean up
        std::fs::remove_file(config_path).unwrap();
    }
}
