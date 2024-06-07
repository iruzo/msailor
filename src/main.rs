use std::process::exit;

mod path;
mod git;
mod repo;
mod menu;
mod config;

#[tokio::main]
async fn main() {

    exit(0);

    let paths = path::get_paths();
    // println!("Plug Path: {}", paths.plug_path);

    // Define a list of repositories
    let repos = vec![
        "https://github.com/libgit2/libgit2",
        "https://github.com/catppuccin/catppuccin",
        "https://github.com/iruzo/pxalarm",
    ];

    // // Call sync_repos function
    let sync_path = paths.sync_path;
    if let Err(e) = git::sync_repos(repos.clone(), &sync_path).await {
        eprintln!("Error during repo sync: {}", e);
    }

    // Define the plug path
    let plug_path = paths.plug_path;

    // Call sync_plug function
    if let Err(e) = git::sync_repos(repos, &plug_path).await {
        eprintln!("Error during plug sync: {}", e);
    }

    // // Define the config path
    let config_path = "/path/to/config";
    //
    // // Call push_config_repo function
    if let Err(e) = git::push_config_repo(config_path).await {
        eprintln!("Error during config repo push: {}", e);
    }

    // Define the repository path
    let repo_path = "/path/to/sample_repo";

    // Create a sample repo
    if let Err(e) = repo::create_sample_repo(repo_path) {
        eprintln!("Error creating sample repo: {}", e);
    }

    // Define the necessary paths
    let sync_path = "/path/to/sync";
    let list_path = "/path/to/list";
    let config_path = "/path/to/config";
    let plug_path = "/path/to/plug";

    // Generate menu content
    match menu::generate_menu_content(sync_path, list_path, config_path, plug_path) {
        Ok(menu_content) => {
            for item in menu_content {
                println!("{}", item);
            }
        },
        Err(e) => eprintln!("Error generating menu content: {}", e),
    }

    // Define the path to the configuration file
    let config_path = "/path/to/config.cfg";

    // Parse the configuration file
    match config::parse_config_file(config_path) {
        Ok(config_map) => {
            for (key, value) in &config_map {
                println!("{}: {}", key, value);
            }
        },
        Err(e) => eprintln!("Error parsing config file: {}", e),
    }

}
