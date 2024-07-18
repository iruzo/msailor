use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn generate_help_menu_content() -> Vec<String> {
    vec![
        String::from("q   => Exit"),
        String::from("e   => Edit"),
        String::from("k   => Go up"),
        String::from("j   => Go down"),
        String::from("g   => Go to top"),
        String::from("G   => Go to bottom"),
        String::from("s   => Sync plugins"),
        String::from("S   => Sync repositories"),
        String::from("/   => Enter filter mode"),
        String::from("Esc => Go back to normal mode from any other mode"),
    ]
}

pub fn generate_command_menu_content() -> Vec<String> {
    vec![
        String::from("create-sample-repo"),
        String::from("config-edit"),
        String::from("history-edit"),
        String::from("list-add"),
        String::from("indexwp"),
    ]
}

pub fn generate_menu_content(
    sync_path: &str,
    list_path: &str,
    config_path: &str,
) -> io::Result<Vec<String>> {
    let mut menu_content = Vec::new();

    // Synced repositories
    if Path::new(sync_path).exists() {
        for entry in fs::read_dir(sync_path)? {
            let synced_repo = entry?.file_name().into_string().unwrap();
            let repo_path = format!("{}{}{}", sync_path, std::path::MAIN_SEPARATOR, synced_repo);

            // Synced lists
            let list_repo_path = format!("{}{}{}", repo_path, std::path::MAIN_SEPARATOR, "list");
            if Path::new(&list_repo_path).exists() {
                for synced_list in fs::read_dir(&list_repo_path)? {
                    let synced_list = synced_list?.file_name().into_string().unwrap();
                    menu_content.push(format!("[list-{}] {}", synced_repo, synced_list));
                }
            }

            // Synced quickmarks
            let quickmarks_repo_path = format!("{}{}{}", repo_path, std::path::MAIN_SEPARATOR, "quickmarks");
            if Path::new(&quickmarks_repo_path).exists() {
                let quickmarks = fs::File::open(&quickmarks_repo_path)?;
                for quickmark in io::BufReader::new(quickmarks).lines() {
                    let quickmark = quickmark?;
                    menu_content.push(format!("[quickmark-{}] {}", synced_repo, quickmark));
                }
            }
        }
    }

    // Lists
    if Path::new(list_path).exists() {
        for entry in fs::read_dir(list_path)? {
            let list = entry?.file_name().into_string().unwrap();
            menu_content.push(format!("[list] {}", list));
        }
    }

    // Quickmarks
    let quickmark_path = format!("{}{}{}", config_path, std::path::MAIN_SEPARATOR, "quickmarks");
    if Path::new(&quickmark_path).exists() {
        let quickmarks = fs::File::open(&quickmark_path)?;
        for quickmark in io::BufReader::new(quickmarks).lines() {
            let quickmark = quickmark?;
            menu_content.push(format!("[quickmark] {}", quickmark));
        }
    }

    // Local files
    let file_path = format!("{}{}{}", config_path, std::path::MAIN_SEPARATOR, "file");
    if Path::new(&file_path).exists() {
        for entry in fs::read_dir(file_path)? {
            let file = entry?.file_name().into_string().unwrap();
            menu_content.push(format!("[file] {}", file));
        }
    }

    if cfg!(target_os = "windows") {
        // TODO: Implement an update method so windows users can update the app
        // menu_content.push("[command] update".to_string());
    }

    menu_content.extend_from_slice(&[
        "[config]".to_string(),
        "[history]".to_string(),
    ]);

    // remove empty lines
    menu_content.retain(|s| !s.is_empty());

    Ok(menu_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_generate_menu_content() {
        let temp_dir = env::temp_dir();
        let sync_path = temp_dir.join("sync_test");
        let list_path = temp_dir.join("list_test");
        let config_path = temp_dir.join("config_test");

        // Create test directories and files
        fs::create_dir_all(&sync_path).unwrap();
        fs::create_dir_all(&list_path).unwrap();
        fs::create_dir_all(&config_path).unwrap();

        // Create dummy sync repo
        let sync_repo = sync_path.join("repo1");
        fs::create_dir_all(&sync_repo).unwrap();
        let list_repo_path = sync_repo.join("list");
        fs::create_dir_all(&list_repo_path).unwrap();
        File::create(list_repo_path.join("list1")).unwrap();
        let quickmark_repo_path = sync_repo.join("quickmarks");
        let mut quickmark_file = File::create(quickmark_repo_path).unwrap();
        writeln!(quickmark_file, "quickmark1").unwrap();

        // Create dummy list files
        let list_file = list_path.join("list1");
        File::create(list_file).unwrap();

        // Create dummy quickmark file
        let quickmark_path = config_path.join("quickmarks");
        let mut quickmark_file = File::create(quickmark_path).unwrap();
        writeln!(quickmark_file, "quickmark2").unwrap();

        // Create dummy local files
        let file_path = config_path.join("file");
        fs::create_dir_all(&file_path).unwrap();
        let local_file = file_path.join("file1");
        File::create(local_file).unwrap();

        // Generate menu content
        let result = generate_menu_content(
            sync_path.to_str().unwrap(),
            list_path.to_str().unwrap(),
            config_path.to_str().unwrap(),
        ).unwrap();

        // Check if the generated menu content is correct
        assert!(result.contains(&"[list-repo1] list1".to_string()));
        assert!(result.contains(&"[quickmark-repo1] quickmark1".to_string()));
        assert!(result.contains(&"[list] list1".to_string()));
        assert!(result.contains(&"[quickmark] quickmark2".to_string()));
        assert!(result.contains(&"[file] file1".to_string()));
        assert!(result.contains(&"[config]".to_string()));
        assert!(result.contains(&"[history]".to_string()));

        // Clean up
        fs::remove_dir_all(&sync_path).unwrap();
        fs::remove_dir_all(&list_path).unwrap();
        fs::remove_dir_all(&config_path).unwrap();
    }
}
