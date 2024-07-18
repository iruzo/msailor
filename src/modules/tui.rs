use super::utils::config;
use super::utils::envv;
use super::utils::menu;
use super::utils::path;
use super::utils::edit;
use crossterm::event;
use crossterm::{
    // event::{Event, KeyCode, KeyModifiers},
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::collections::HashMap;
use std::io;
use std::process::exit;
use std::path::{PathBuf, MAIN_SEPARATOR};

#[derive(PartialEq)]
enum Mode {
    Normal,
    Command,
    Filter,
    Help,
}

fn open_editor(config: HashMap<String, String>, path: &str) {
    if config.contains_key("editor") {
        edit::edit(Some(config["editor"].as_str()), None, path);
    } else if envv::get_env_vars().get_key_value("VISUAL").is_some() {
        edit::edit(None, Some(envv::get_env_vars().get_key_value("VISUAL").unwrap().1.as_str()), path);
    } else if envv::get_env_vars().get_key_value("EDITOR").is_some() {
        edit::edit(None, Some(envv::get_env_vars().get_key_value("EDITOR").unwrap().1.as_str()), path);
    } else {
        edit::edit(None, None, path);
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mock_event_receiver: Option<std::sync::mpsc::Receiver<Event>>,
) -> Result<(), io::Error> {
    let mut input_buffer = String::new();
    let mut mode = Mode::Normal;
    let mut edit = false;
    let paths = path::get_default_paths();
    let config: HashMap<String, String> = match config::parse_config_file(
        paths.config_file.as_str(),
        Some(path::get_default_paths().to_hash_map()),
    ) {
            Ok(config_map) => config_map,
            Err(e) => {
                eprintln!("Error parsing config file on tui.rs: {}", e);
                exit(0);
            }
        };
    let config_copy = config.clone();
    let items: Vec<String> = menu::generate_menu_content(
        config_copy["path.sync"].as_str(),
        config_copy["path.list"].as_str(),
        config_copy["path.config_dir"].as_str()
    )?;
    let mut filtered_items = items.clone();
    let mut selected = filtered_items.len() - 1;
    let mut title = "NORMAL";
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    loop {

        if edit {
            terminal.clear().unwrap();
            disable_raw_mode()?;

            match filtered_items[selected].as_str() {
                s if s.contains("[list]") => open_editor(config.clone(), format!("{}{}{}", config_copy["path.list"], MAIN_SEPARATOR, s.split_once("[list]").map(|s| s.1.trim()).unwrap()).as_str()),
                s if s.contains("[history]") => open_editor(config.clone(), config_copy["path.history"].as_str()),
                s if s.contains("[config]") => open_editor(config.clone(), config_copy["path.config_file"].as_str()),
                s if s.contains("[quickmark]") => open_editor(config.clone(), config_copy["path.quickmarks"].as_str()),
                _ => {
                    // Handle the else case here
                }
            }

            enable_raw_mode()?;
            // Clear the screen
            terminal.clear().unwrap();

            edit = false;
        }

        terminal.draw(|f| {
            let size = f.size();

            // Layout constraints
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(80), // Main area takes 80% of the width
                        Constraint::Percentage(20), // Right panel takes 20% of the width
                    ]
                        .as_ref(),
                )
                .split(size);

            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(95), // Main box takes 95% of the height
                        Constraint::Percentage(5),  // Bottom bar takes 5% of the height
                    ]
                        .as_ref(),
                )
                .split(chunks[0]);

            // Main box
            let main_box = Block::default()
                // .title("Menu")
                .borders(Borders::ALL);
            let list_items: Vec<ListItem> = filtered_items
                .iter()
                .map(|item| ListItem::new(Span::raw(item.clone())))
                .collect();
            let list = List::new(list_items).block(main_box).highlight_style(
                Style::default()
                    // .bg(Color::Blue)
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_stateful_widget(list, vertical_chunks[0], &mut list_state);

            // Bottom bar
            if mode == Mode::Normal {
                title = "NORMAL";
            }
            if mode == Mode::Command {
                title = "COMMAND";
            }
            if mode == Mode::Filter {
                title = "FILTER";
            }
            if mode == Mode::Help {
                title = "HELP";
            }
            let bottom_paragraph = Paragraph::new(Text::from(input_buffer.as_str()))
                .block(Block::default().title(title).borders(Borders::ALL));
            f.render_widget(bottom_paragraph, vertical_chunks[1]);
            // Right panel
            let right_panel = Block::default()
                .title("Current playlist")
                .borders(Borders::ALL);
            f.render_widget(right_panel, chunks[1]);
        })?;

        // Handle input
        let event = if let Some(receiver) = &mock_event_receiver {
            receiver.recv().ok()
        } else {
            event::read().ok()
        };

        if let Some(Event::Key(key)) = event {
            match mode {
                Mode::Normal => match key.code {
                    KeyCode::Enter => {
                        if filtered_items.contains(&input_buffer)
                        && input_buffer.trim() == filtered_items[selected]
                        {
                            //execute
                            filtered_items.clone_from(&items);
                            input_buffer.clear();
                        } else {
                            input_buffer.clone_from(&filtered_items[selected]);
                        }
                    }
                    KeyCode::Char('e') => {
                        // edit selected
                        edit = true;
                    }
                    KeyCode::Char('j') => {
                        if selected < filtered_items.len() - 1 {
                            selected += 1;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('g') => {
                        selected = 0;
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('G') => {
                        selected = filtered_items.len() - 1;
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('/') => {
                        mode = Mode::Filter;
                        input_buffer.clear()
                    }
                    KeyCode::Char(':') => {
                        mode = Mode::Command;
                        input_buffer.clear();
                    }
                    KeyCode::Char('?') => {
                        mode = Mode::Help;
                        input_buffer.clear();
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Esc => {
                        filtered_items.clone_from(&items);
                        selected = filtered_items.len() - 1;
                        list_state.select(Some(selected));
                        input_buffer.clear();
                    }
                    _ => {}
                },
                Mode::Command => match key.code {
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Enter => {
                        // execute
                        mode = Mode::Normal;
                    }
                    KeyCode::Esc => {
                        filtered_items.clone_from(&items);
                        input_buffer.clear();
                        mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Filter => match key.code {
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Enter => {
                        input_buffer.clone_from(&filtered_items[selected]);
                        mode = Mode::Normal;
                    }
                    KeyCode::Esc => {
                        input_buffer.clear();
                        mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Help => {
                    if let KeyCode::Esc = key.code {
                        input_buffer.clear();
                        mode = Mode::Normal;
                        filtered_items.clone_from(&items);
                    }
                }
            }

            // Update filtered items based on the input buffer
            if mode == Mode::Filter {
                let filters: Vec<String> = input_buffer
                    .split_whitespace()
                    .map(|s| s.to_lowercase())
                    .collect();
                let mut current: Vec<String> =
                items.iter().map(|item| item.replace('\n', "")).collect();

                for filter in filters {
                    current.retain(|item| item.to_lowercase().contains(&filter));
                }

                // set selected to 0 when list is empty
                if filtered_items.len() != current.len() && !filtered_items.is_empty() {
                    selected = 0;
                }

                filtered_items = current;
                list_state.select(Some(selected));
            }

            // Command mode menu content
            if mode == Mode::Command {
                selected = 0;
                filtered_items = menu::generate_command_menu_content();
                list_state.select(Some(selected));
            }

            // Help mode menu content
            if mode == Mode::Help {
                selected = 0;
                filtered_items = menu::generate_help_menu_content();
                list_state.select(Some(selected));
            }
        }
    }

    Ok(())
}

pub fn tui() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, None);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_run_app_exits_on_q() {
        // Create a TestBackend and Terminal
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        // Create an event channel
        let (event_sender, event_receiver) = mpsc::channel();

        // Simulate pressing 'exit'
        let events = vec![Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
        })];

        thread::spawn(move || {
            for event in events {
                event_sender.send(event).unwrap();
            }
        });

        // Run the app
        let result = run_app(&mut terminal, Some(event_receiver));

        // Assert the app exited without error
        assert!(result.is_ok());
    }
}

