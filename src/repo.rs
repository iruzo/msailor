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
    writeln!(config_file, "config content")?;

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
