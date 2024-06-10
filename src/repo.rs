use std::fs;
use std::io::Write;
use std::path::Path;
use git2::Repository;

pub fn create_sample_repo(repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(repo_path);

    // Create the main repository directory
    fs::create_dir_all(repo_path)?;

    // Create the .gitignore file
    let gitignore_path = repo_path.join(".gitignore");
    let mut gitignore_file = fs::File::create(gitignore_path)?;
    writeln!(gitignore_file, "/*")?;
    writeln!(gitignore_file, "!.gitignore")?;
    writeln!(gitignore_file, "!config")?;
    writeln!(gitignore_file, "!quickmark")?;
    writeln!(gitignore_file, "!source")?;
    writeln!(gitignore_file, "!list/")?;

    // Create the necessary directories and files
    // Directories
    fs::create_dir_all(repo_path.join("list"))?;

    // Files
    fs::create_dir_all(repo_path.join("config"))?;
    let mut config_file = fs::File::create(repo_path.join("config/config"))?;

    writeln!(config_file, "# Configuration file for the application")?;
    writeln!(config_file, "# Default paths (override by specifying new values):")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the configuration directory")?;
    writeln!(config_file, "# The default path is determined based on the OS:")?;
    writeln!(config_file, "# - On Linux: $XDG_CONFIG_HOME or ~/.config/msailor")?;
    writeln!(config_file, "# - On Windows: %APPDATA%\\msailor")?;
    writeln!(config_file, "# - On macOS: $HOME/Library/Application Support/msailor")?;
    writeln!(config_file, "config_path = /path/to/override/config")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the data directory")?;
    writeln!(config_file, "# The default path is determined based on the OS:")?;
    writeln!(config_file, "# - On Linux: $XDG_DATA_HOME or ~/.local/share/msailor")?;
    writeln!(config_file, "# - On Windows: %LOCALAPPDATA%\\msailor")?;
    writeln!(config_file, "# - On macOS: $HOME/Library/Application Support/msailor")?;
    writeln!(config_file, "data_path = /path/to/override/data")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the temporary directory")?;
    writeln!(config_file, "# The default path is determined based on the OS:")?;
    writeln!(config_file, "# - On Linux: $XDG_CACHE_HOME or ~/.cache/msailor")?;
    writeln!(config_file, "# - On Windows: %TEMP%\\msailor")?;
    writeln!(config_file, "# - On macOS: $HOME/Library/Caches/msailor")?;
    writeln!(config_file, "tmp_path = /path/to/override/tmp")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the history file")?;
    writeln!(config_file, "history_path = /path/to/override/history")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the sync directory")?;
    writeln!(config_file, "sync_path = /path/to/override/sync")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the list directory")?;
    writeln!(config_file, "list_path = /path/to/override/list")?;
    writeln!(config_file)?;
    writeln!(config_file, "# Path to the plugins directory")?;
    writeln!(config_file, "plug_path = /path/to/override/plug")?;
    writeln!(config_file)?;

    let mut quickmark_file = fs::File::create(repo_path.join("quickmark"))?;
    writeln!(quickmark_file, "quickmark content")?;

    let mut source_file = fs::File::create(repo_path.join("source"))?;
    writeln!(source_file, "source content")?;

    // Create a dummy file in the list directory to ensure it's not empty
    let list_file_path = repo_path.join("list/list1");
    fs::File::create(list_file_path)?;

    // Initialize a new Git repository
    let repo = Repository::init(repo_path)?;

    // Stage and commit the initial content
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = git2::Signature::now("Author", "author@example.com")?;
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_create_sample_repo() {
        let temp_dir = env::temp_dir();
        let repo_path = temp_dir.join("sample_repo");

        // Create the sample repo
        let result = create_sample_repo(repo_path.to_str().unwrap());
        assert!(result.is_ok());

        // Verify the structure and content of the created sample repo
        assert!(repo_path.exists());
        assert!(repo_path.join(".gitignore").exists());
        assert!(repo_path.join("config/config").exists());
        assert!(repo_path.join("quickmark").exists());
        assert!(repo_path.join("source").exists());
        assert!(repo_path.join("list/list1").exists());

        // Clean up
        fs::remove_dir_all(&repo_path).unwrap();
    }
}
