use std::fs;
use std::io;
use std::path::Path;
use git2::{Repository, Signature, IndexAddOption};
use tokio::task;

pub async fn sync_repos(repos: Vec<&str>, target_path: &str) -> io::Result<()> {
    if !repos.is_empty() {
        if Path::new(target_path).exists() {
            fs::remove_dir_all(target_path)?;
        }

        fs::create_dir(target_path)?;

        let mut handles = vec![];

        for repo in repos {
            let repo_name = repo.split('/').last().unwrap().trim().to_string();
            let repo_path = format!("{}{}{}", target_path, std::path::MAIN_SEPARATOR, repo_name);
            let repo_clone = repo.to_string();
            let handle = tokio::spawn(async move {
                task::spawn_blocking(move || {
                    match Repository::clone(&repo_clone, &repo_path) {
                        Ok(_) => println!("Successfully cloned {}", repo_clone),
                        Err(e) => eprintln!("Failed to clone {}: {}", repo_clone, e),
                    }
                }).await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    Ok(())
}

pub async fn push_config_repo(config_path: &str) -> Result<(), git2::Error> {
    // Check if the directory is a valid repository
    let repo = match Repository::open(config_path) {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("The directory at {} is not a valid Git repository", config_path);
            return Err(git2::Error::from_str("Not a valid Git repository"));
        }
    };

    // Stage all changes
    {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;
    }

    // Commit changes
    let signature = Signature::now("msailor", "msailor@example.com")?;
    let oid = {
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let parent_commit = repo.head()?.peel_to_commit()?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "msailor auto push",
            &tree,
            &[&parent_commit],
        )?
    };

    println!("New commit: {}", oid);

    // Push changes
    let mut remote = repo.find_remote("origin")?;
    remote.connect(git2::Direction::Push)?;

    let mut push_opts = git2::PushOptions::new();
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_opts))?;

    println!("Pushed changes to remote");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Write;

    #[tokio::test]
    async fn test_sync_repos() {
        let temp_dir = env::temp_dir();
        let sync_path = temp_dir.join("sync_test");

        // Define a list of repositories
        let repos = vec![
            "https://github.com/libgit2/libgit2",
            "https://github.com/catppuccin/catppuccin",
            "https://github.com/iruzo/pxalarm",
        ];

        // Call sync_repos
        let result = sync_repos(repos, sync_path.to_str().unwrap()).await;

        // Check if sync_repos executed successfully
        assert!(result.is_ok());

        // Check if the repositories were cloned
        assert!(sync_path.join("libgit2").exists());
        assert!(sync_path.join("catppuccin").exists());
        assert!(sync_path.join("pxalarm").exists());

        // Clean up
        fs::remove_dir_all(&sync_path).unwrap();
    }

    // Tests fails since there is no upstream to push
    //
    // #[tokio::test]
    // async fn test_push_config_repo() {
    //     let temp_dir = env::temp_dir();
    //     let config_path = temp_dir.join("config_repo");

    //     // Create a new repository
    //     let _repo = Repository::init(&config_path).unwrap();

    //     // Create a new file in the repository
    //     let file_path = config_path.join("test.txt");
    //     let mut file = fs::File::create(&file_path).unwrap();
    //     writeln!(file, "Hello, world!").unwrap();

    //     // Stage, commit, and push changes
    //     let result = push_config_repo(config_path.to_str().unwrap()).await;

    //     // Check if push_config_repo executed successfully
    //     assert!(result.is_ok());

    //     // Clean up
    //     fs::remove_dir_all(&config_path).unwrap();
    // }

}
