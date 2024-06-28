use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

pub struct Paths {
    pub config_path: String,
    pub data_path: String,
    pub tmp_path: String,
    pub history_path: String,
    pub sync_path: String,
    pub list_path: String,
    pub plug_path: String,
}

impl Paths {
    // Method to convert the Paths struct to a HashMap
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("config_path".to_string(), self.config_path.clone());
        map.insert("data_path".to_string(), self.data_path.clone());
        map.insert("tmp_path".to_string(), self.tmp_path.clone());
        map.insert("history_path".to_string(), self.history_path.clone());
        map.insert("sync_path".to_string(), self.sync_path.clone());
        map.insert("list_path".to_string(), self.list_path.clone());
        map.insert("plug_path".to_string(), self.plug_path.clone());
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

    let config_path = format!("{}{}msailor", user_config_path, MAIN_SEPARATOR);
    let data_path = format!("{}{}msailor", user_data_path, MAIN_SEPARATOR);
    let tmp_path = format!("{}{}msailor", user_tmp_path, MAIN_SEPARATOR);
    let history_path = format!("{}{}history", data_path, MAIN_SEPARATOR);
    let sync_path = format!("{}{}sync", data_path, MAIN_SEPARATOR);
    let list_path = format!("{}{}list", config_path, MAIN_SEPARATOR);
    let plug_path = format!("{}{}plug", data_path, MAIN_SEPARATOR);

    // Create directories if they don't exist
    fs::create_dir_all(&config_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::create_dir_all(&data_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    fs::create_dir_all(&user_tmp_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    Paths {
        config_path,
        data_path,
        tmp_path,
        history_path,
        sync_path,
        list_path,
        plug_path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths() {
        let paths = get_default_paths();

        println!("Config Path:  {}", paths.config_path);
        println!("Data Path:    {}", paths.data_path);
        println!("Temp Path:    {}", paths.tmp_path);
        println!("History Path: {}", paths.history_path);
        println!("Sync Path:    {}", paths.sync_path);
        println!("List Path:    {}", paths.list_path);
        println!("Plug Path:    {}", paths.plug_path);

        // Check if paths are not empty
        assert!(!paths.config_path.is_empty(), "Config path is empty");
        assert!(!paths.data_path.is_empty(), "Data path is empty");
        assert!(!paths.tmp_path.is_empty(), "Tmp path is empty");
        assert!(!paths.history_path.is_empty(), "History path is empty");
        assert!(!paths.sync_path.is_empty(), "Sync path is empty");
        assert!(!paths.list_path.is_empty(), "List path is empty");
        assert!(!paths.plug_path.is_empty(), "Plug path is empty");

        // Check if directories exist
        assert!(fs::metadata(&paths.config_path).is_ok(), "Config path does not exist");
        assert!(fs::metadata(&paths.data_path).is_ok(), "Data path does not exist");
        assert!(fs::metadata(&paths.tmp_path).is_ok(), "Tmp path does not exist");
    }

    #[test]
    fn test_paths_to_hash_map() {
        let paths = Paths {
            config_path: "config.cfg".to_string(),
            data_path: "/var/data".to_string(),
            tmp_path: "/tmp".to_string(),
            history_path: "/var/history".to_string(),
            sync_path: "/var/sync".to_string(),
            list_path: "/var/list".to_string(),
            plug_path: "/var/plug".to_string(),
        };

        let map = paths.to_hash_map();

        let expected_map: HashMap<String, String> = [
            ("config_path", "config.cfg"),
            ("data_path", "/var/data"),
            ("tmp_path", "/tmp"),
            ("history_path", "/var/history"),
            ("sync_path", "/var/sync"),
            ("list_path", "/var/list"),
            ("plug_path", "/var/plug"),
        ].iter().cloned()
         .map(|(k, v)| (k.to_string(), v.to_string()))
         .collect();

        // Check if the map has the same entries as the expected_map
        assert_eq!(map, expected_map);
    }
}
