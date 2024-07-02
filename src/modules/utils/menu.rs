use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn generate_menu_content(
    sync_path: &str,
    list_path: &str,
    config_path: &str,
    plug_path: &str,
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
            let quickmark_repo_path = format!("{}{}{}", repo_path, std::path::MAIN_SEPARATOR, "quickmark");
            if Path::new(&quickmark_repo_path).exists() {
                let quickmarks = fs::File::open(&quickmark_repo_path)?;
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
    let quickmark_path = format!("{}{}{}", config_path, std::path::MAIN_SEPARATOR, "quickmark");
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

    // Plugins
    if Path::new(plug_path).exists() {
        for entry in fs::read_dir(plug_path)? {
            let file = entry?.file_name().into_string().unwrap();
            menu_content.push(format!("[plug] :{}", file));
        }
    }

    // Add options to menu
    menu_content.extend_from_slice(&[
        "[command] help".to_string(),
    ]);
    if cfg!(target_os = "windows") {
        menu_content.push("[command] update".to_string());
    }
    menu_content.extend_from_slice(&[
        "[command] create-sample-repo".to_string(),
        "[command] sync-repos".to_string(),
        "[command] sync-plugs".to_string(),
        "[command] push".to_string(),
        "[command] config-edit".to_string(),
        "[command] history-edit".to_string(),
        "[command] quickmark-add".to_string(),
        "[command] quickmark-edit".to_string(),
        "[command] list-add".to_string(),
        "[command] list-edit".to_string(),
        "[command] list-del".to_string(),
        "[command] indexwp".to_string(),
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
        let plug_path = temp_dir.join("plug_test");

        // Create test directories and files
        fs::create_dir_all(&sync_path).unwrap();
        fs::create_dir_all(&list_path).unwrap();
        fs::create_dir_all(&config_path).unwrap();
        fs::create_dir_all(&plug_path).unwrap();

        // Create dummy sync repo
        let sync_repo = sync_path.join("repo1");
        fs::create_dir_all(&sync_repo).unwrap();
        let list_repo_path = sync_repo.join("list");
        fs::create_dir_all(&list_repo_path).unwrap();
        File::create(list_repo_path.join("list1")).unwrap();
        let quickmark_repo_path = sync_repo.join("quickmark");
        let mut quickmark_file = File::create(quickmark_repo_path).unwrap();
        writeln!(quickmark_file, "quickmark1").unwrap();

        // Create dummy list files
        let list_file = list_path.join("list1");
        File::create(list_file).unwrap();

        // Create dummy quickmark file
        let quickmark_path = config_path.join("quickmark");
        let mut quickmark_file = File::create(quickmark_path).unwrap();
        writeln!(quickmark_file, "quickmark2").unwrap();

        // Create dummy local files
        let file_path = config_path.join("file");
        fs::create_dir_all(&file_path).unwrap();
        let local_file = file_path.join("file1");
        File::create(local_file).unwrap();

        // Create dummy plugin files
        let plug_file = plug_path.join("plugin1");
        File::create(plug_file).unwrap();

        // Generate menu content
        let result = generate_menu_content(
            sync_path.to_str().unwrap(),
            list_path.to_str().unwrap(),
            config_path.to_str().unwrap(),
            plug_path.to_str().unwrap()
        ).unwrap();

        // Check if the generated menu content is correct
        assert!(result.contains(&"[list-repo1] list1".to_string()));
        assert!(result.contains(&"[quickmark-repo1] quickmark1".to_string()));
        assert!(result.contains(&"[list] list1".to_string()));
        assert!(result.contains(&"[quickmark] quickmark2".to_string()));
        assert!(result.contains(&"[file] file1".to_string()));
        assert!(result.contains(&"[plug] :plugin1".to_string()));
        assert!(result.contains(&"[command] help".to_string()));

        // Clean up
        fs::remove_dir_all(&sync_path).unwrap();
        fs::remove_dir_all(&list_path).unwrap();
        fs::remove_dir_all(&config_path).unwrap();
        fs::remove_dir_all(&plug_path).unwrap();
    }
}
