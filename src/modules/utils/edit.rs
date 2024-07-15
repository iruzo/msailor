use std::process::Command;
use std::io;

use kiro_editor::{self as kiro, Editor, StdinRawMode};

fn default_editor(files: Vec<String>) -> kiro::Result<()> {
    let input = StdinRawMode::new()?.input_keys();
    Editor::open(input, io::stdout(), None, &files)?.edit()
}

fn execute(command: &str, arg: &str) {
    match Command::new(command).arg(arg).spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    println!("Process exited with: {}", status);
                }
                Err(e) => {
                    eprintln!("Failed to wait on child process: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to spawn process: {}", e);
        }
    }
}

pub fn edit(editor_config_value: Option<&str>, editor_envvar_value: Option<&str>, path: &str) {
    if let Some(editor_config_value) = editor_config_value {
        if !editor_config_value.is_empty() {
            execute(editor_config_value, path);
            return;
        }
    }

    if let Some(editor_envvar_value) = editor_envvar_value {
        if !editor_envvar_value.is_empty() {
            execute(editor_envvar_value, path);
            return;
        }
    }

    let _ = default_editor(vec![String::from(path)]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_edit_with_config_value() {
        // Setup: Create a temporary file path
        let temp_file_path = "temp_file.txt";
        fs::write(temp_file_path, "Temporary file for testing").unwrap();

        // Action: Call the `edit` function with editor_config_value
        let editor_config_value = Some("echo"); // Using `echo` as a mock editor command
        edit(editor_config_value, None, temp_file_path);

        // Teardown: Remove the temporary file
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_edit_with_envvar_value() {
        // Setup: Create a temporary file path
        let temp_file_path = "temp_file.txt";
        fs::write(temp_file_path, "Temporary file for testing").unwrap();

        // Setup: Mock the environment variable
        env::set_var("MOCK_EDITOR", "echo");

        // Action: Call the `edit` function with editor_envvar_value
        let editor_envvar_value = env::var("MOCK_EDITOR").ok();
        edit(None, editor_envvar_value.as_deref(), temp_file_path);

        // Teardown: Remove the temporary file
        fs::remove_file(temp_file_path).unwrap();

        // Teardown: Remove the mocked environment variable
        env::remove_var("MOCK_EDITOR");
    }

    // #[test]
    // fn test_edit_with_default_editor() {
    //     // Setup: Create a temporary file path
    //     let temp_file_path = "temp_file.txt";
    //     fs::write(temp_file_path, "Temporary file for testing").unwrap();
    //
    //     // Action: Call the `edit` function with no config or envvar values
    //     edit(None, None, temp_file_path);
    //
    //     // TODO Try to simulate pressing Ctrl-q after opening the editor
    //
    //     // Teardown: Remove the temporary file
    //     fs::remove_file(temp_file_path).unwrap();
    // }

}

