use super::utils::config;
use super::utils::menu;
use super::utils::path;
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
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::collections::HashMap;
use std::io;
use std::process::exit;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mock_event_receiver: Option<std::sync::mpsc::Receiver<Event>>,
) -> Result<(), io::Error> {
    let mut input_buffer = String::new();
    let paths = path::get_default_paths();
    let config: HashMap<String, String> = match config::parse_config_file(
        paths.config_file_path.as_str(),
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
        config_copy["sync_path"].as_str(),
        config_copy["list_path"].as_str(),
        config_copy["config_path"].as_str(),
        config_copy["plug_path"].as_str(),
    )?;
    let mut filtered_items = items.clone();
    let mut selected = filtered_items.len() - 1;

    loop {
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
            let main_box = Block::default().title("Menu").borders(Borders::ALL);
            let list_items: Vec<ListItem> = filtered_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if !selected < filtered_items.len() {
                        selected *= 0;
                    }
                    let content = if i == selected {
                        format!(" >> {}", item)
                    } else {
                        item.clone()
                    };
                    ListItem::new(Span::raw(content))
                })
                .collect();
            let list = List::new(list_items).block(main_box);
            f.render_widget(list, vertical_chunks[0]);

            // Bottom bar
            let bottom_paragraph = Paragraph::new(Text::from(input_buffer.as_str()))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(bottom_paragraph, vertical_chunks[1]);

            // Right panel
            let right_panel = Block::default().title("List").borders(Borders::ALL);
            f.render_widget(right_panel, chunks[1]);
        })?;

        // Handle input
        let event = if let Some(receiver) = &mock_event_receiver {
            receiver.recv().ok()
        } else {
            event::read().ok()
        };

        if let Some(Event::Key(key)) = event {
            match key.code {
                KeyCode::Char(c) => {
                    input_buffer.push(c);
                }
                KeyCode::Esc => {
                    input_buffer.clear();
                }
                KeyCode::Enter => {
                    if input_buffer == ":q" {
                        break;
                    }
                    if input_buffer.trim() != filtered_items[selected] {
                        input_buffer.clone_from(&filtered_items[selected]);
                    }
                    if input_buffer.trim() == filtered_items[selected] {
                        // execute
                    }
                    // Event::FocusGained => todo!(),
                    // Event::FocusLost => todo!(),
                    // Event::Mouse(_) => todo!(),
                    // Event::Paste(_) => todo!(),
                    // Event::Resize(_, _) => todo!(),
                }
                //     }
                // }
                //
                // if let Event::Key(key) = event::read()? {
                //     if key.code == KeyCode::Char('q') {
                //         break;
                KeyCode::Backspace => {
                    input_buffer.pop();
                }
                KeyCode::Tab => {
                    if selected == 0 {
                        selected = filtered_items.len();
                    }
                    selected -= 1;
                    if selected > filtered_items.len() -1 {
                        selected = filtered_items.len() - 1;
                    }
                    // if key.modifiers.contains(KeyModifiers::SHIFT) {
                    // }
                }
                _ => {}
            }

            // Update filtered items based on the input buffer
            let filters: Vec<String> = input_buffer
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .collect();
            let mut current: Vec<String> =
            items.iter().map(|item| item.replace("'\n'", "")).collect();

            for filter in filters {
                current.retain(|item| item.to_lowercase().contains(&filter));
            }

            if input_buffer.trim().is_empty() {
                current.push(String::new());
            }

            filtered_items = current;
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

        // Simulate pressing ':q'
        let events = vec![
            Event::Key(KeyEvent {
            code: KeyCode::Char(':'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            }),
            Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            }),
            Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            }),
        ];

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
