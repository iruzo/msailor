use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

pub struct Paths {
    pub config_dir: String,
    pub config_file: String,
    pub quickmarks: String,
    pub data_dir: String,
    pub tmp_dir: String,
    pub history: String,
    pub sync_dir: String,
    pub list_dir: String,
    pub plug_dir: String,
}

impl Paths {
    // Method to convert the Paths struct to a HashMap
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("path.config_dir".to_string(), self.config_dir.clone());
        map.insert("path.config_file".to_string(), self.config_file.clone());
        map.insert("path.quickmarks".to_string(), self.quickmarks.clone());
        map.insert("path.data".to_string(), self.data_dir.clone());
        map.insert("path.tmp".to_string(), self.tmp_dir.clone());
        map.insert("path.history".to_string(), self.history.clone());
        map.insert("path.sync".to_string(), self.sync_dir.clone());
        map.insert("path.list".to_string(), self.list_dir.clone());
        map.insert("path.plug".to_string(), self.plug_dir.clone());
        map
    }
}

fn home_dir() -> PathBuf {
    match env::var("HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            if cfg!(target_os = "windows") {
                PathBuf::from(env::var("USERPROFILE").unwrap())
            } else {
                panic!("Cannot determine home directory")
            }
        }
    }
}

pub fn get_default_paths() -> Paths {
    let (user_config_path, user_data_path, user_tmp_path) = if cfg!(target_os = "linux") {
        let user_config_path = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let mut path = home_dir();
            path.push(".config");
            path.to_str().unwrap().to_string()
        });

        let user_data_path = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            let mut path = home_dir();
            path.push(".local");
            path.push("share");
            path.to_str().unwrap().to_string()
        });

        let user_tmp_path = env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
            let mut path = home_dir();
            path.push(".cache");
            path.to_str().unwrap().to_string()
        });

        (user_config_path, user_data_path, user_tmp_path)
    } else if cfg!(target_os = "windows") {
        let user_config_path = env::var("APPDATA").expect("APPDATA not set");
        let user_data_path = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not set");
        let user_tmp_path = env::var("TMP").expect("TMP not set");
        (user_config_path, user_data_path, user_tmp_path)
    } else if cfg!(target_os = "macos") {
        let user_config_path = env::var("HOME").unwrap_or_else(|_| panic!("HOME not set"));
        let user_data_path = env::var("HOME").unwrap_or_else(|_| panic!("HOME not set"));
        let user_tmp_path = env::var("TMPDIR").unwrap_or_else(|_| {
            let mut path = home_dir();
            path.push("Library");
            path.push("Caches");
            path.push("msailor");
            path.to_str().unwrap().to_string()
        });

        let user_config_path = format!("{}{}", user_config_path, "/Library/Application Support/msailor");
        let user_data_path = format!("{}{}", user_data_path, "/Library/Application Support/msailor");

        (user_config_path, user_data_path, user_tmp_path)
    } else {
        panic!("Unsupported platform");
    };

    let config_dir = format!("{}{}msailor", user_config_path, MAIN_SEPARATOR);
    let config_file = format!("{}{}config", config_dir, MAIN_SEPARATOR);
    let quickmarks = format!("{}{}quickmarks", config_dir, MAIN_SEPARATOR);
    let data_dir = format!("{}{}msailor", user_data_path, MAIN_SEPARATOR);
    let tmp_dir = format!("{}{}msailor", user_tmp_path, MAIN_SEPARATOR);
    let history = format!("{}{}history", data_dir, MAIN_SEPARATOR);
    let sync_dir = format!("{}{}sync", data_dir, MAIN_SEPARATOR);
    let list_dir = format!("{}{}list", config_dir, MAIN_SEPARATOR);
    let plug_dir = format!("{}{}plug", data_dir, MAIN_SEPARATOR);

    // Create directories if they don't exist
    fs::create_dir_all(&config_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::create_dir_all(&data_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::create_dir_all(&tmp_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    Paths {
        config_dir,
        config_file,
        quickmarks,
        data_dir,
        tmp_dir,
        history,
        sync_dir,
        list_dir,
        plug_dir,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths() {
        let paths = get_default_paths();

        println!("Config Path:  {}", paths.config_dir);
        println!("Data Path:    {}", paths.data_dir);
        println!("Temp Path:    {}", paths.tmp_dir);
        println!("History Path: {}", paths.history);
        println!("Sync Path:    {}", paths.sync_dir);
        println!("List Path:    {}", paths.list_dir);
        println!("Plug Path:    {}", paths.plug_dir);

        // Check if paths are not empty
        assert!(!paths.config_dir.is_empty(), "Config path is empty");
        assert!(!paths.data_dir.is_empty(), "Data path is empty");
        assert!(!paths.tmp_dir.is_empty(), "Tmp path is empty");
        assert!(!paths.history.is_empty(), "History path is empty");
        assert!(!paths.sync_dir.is_empty(), "Sync path is empty");
        assert!(!paths.list_dir.is_empty(), "List path is empty");
        assert!(!paths.plug_dir.is_empty(), "Plug path is empty");

        // Check if directories exist
        assert!(fs::metadata(&paths.config_dir).is_ok(), "Config path does not exist");
        assert!(fs::metadata(&paths.data_dir).is_ok(), "Data path does not exist");
        assert!(fs::metadata(&paths.tmp_dir).is_ok(), "Tmp path does not exist");

        fs::remove_dir(&paths.tmp_dir).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    #[test]
    fn test_paths_to_hash_map() {
        let paths = Paths {
            config_dir: "config.cfg".to_string(),
            config_file: "config_file.cfg".to_string(),
            quickmarks: "quickmarks".to_string(),
            data_dir: "/var/data".to_string(),
            tmp_dir: "/tmp".to_string(),
            history: "/var/history".to_string(),
            sync_dir: "/var/sync".to_string(),
            list_dir: "/var/list".to_string(),
            plug_dir: "/var/plug".to_string(),
        };

        let map = paths.to_hash_map();

        let expected_map: HashMap<String, String> = [
            ("path.config_dir", "config.cfg"),
            ("path.config_file", "config_file.cfg"),
            ("path.quickmarks", "quickmarks"),
            ("path.data", "/var/data"),
            ("path.tmp", "/tmp"),
            ("path.history", "/var/history"),
            ("path.sync", "/var/sync"),
            ("path.list", "/var/list"),
            ("path.plug", "/var/plug"),
        ].iter().cloned()
         .map(|(k, v)| (k.to_string(), v.to_string()))
         .collect();

        // Check if the map has the same entries as the expected_map
        assert_eq!(map, expected_map);
    }
}
