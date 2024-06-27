use std::process::exit;

mod modules;

#[tokio::main]
async fn main() {

    let default_paths = modules::path::get_default_paths();
    // println!("Plug Path: {}", paths.plug_path);

    // Define the path to the configuration file
    let config_path = "/path/to/config.cfg";

    let _ = modules::tui::tui();

    exit(0);

    // Parse the configuration file
    match modules::config::parse_config_file(config_path, Some(default_paths.to_hash_map())) {
        Ok(config_map) => {
            for (key, value) in &config_map {
                println!("{}: {}", key, value);
            }
        },
        Err(e) => eprintln!("Error parsing config file: {}", e),
    }

    // Define a list of repositories
    let repos = vec![
        "https://github.com/libgit2/libgit2",
        "https://github.com/catppuccin/catppuccin",
        "https://github.com/iruzo/pxalarm",
    ];

    // // Call sync_repos function
    let sync_path = default_paths.sync_path;
    if let Err(e) = modules::git::sync_repos(repos.clone(), &sync_path).await {
        eprintln!("Error during repo sync: {}", e);
    }

    // Define the plug path
    let plug_path = default_paths.plug_path;

    // Call sync_plug function
    if let Err(e) = modules::git::sync_repos(repos, &plug_path).await {
        eprintln!("Error during plug sync: {}", e);
    }

    // // Define the config path
    let config_path = "/path/to/config";
    //
    // // Call push_config_repo function
    if let Err(e) = modules::git::push_config_repo(config_path).await {
        eprintln!("Error during config repo push: {}", e);
    }

    // Define the repository path
    let repo_path = "/path/to/sample_repo";

    // Create a sample repo
    if let Err(e) = modules::repo::create_sample_repo(repo_path) {
        eprintln!("Error creating sample repo: {}", e);
    }

    // Define the necessary paths
    let sync_path = "/path/to/sync";
    let list_path = "/path/to/list";
    let config_path = "/path/to/config";
    let plug_path = "/path/to/plug";

    // Generate menu content
    match modules::menu::generate_menu_content(sync_path, list_path, config_path, plug_path) {
        Ok(menu_content) => {
            for item in menu_content {
                println!("{}", item);
            }
        },
        Err(e) => eprintln!("Error generating menu content: {}", e),
    }

    // ------------------------ file download --------------------

    let urls = vec![
        ("https://raw.githubusercontent.com/iruzo/msailor/main/Cargo.toml".to_string(), "output_file_1.txt".to_string()),
        ("https://raw.githubusercontent.com/iruzo/msailor/main/README.md".to_string(), "output_file_2.txt".to_string()),
    ];

    let _download_files = modules::dwnl::download_files(urls);

    println!("Files downloaded successfully.");

}
