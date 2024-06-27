use crossterm::event;
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mock_event_receiver: Option<std::sync::mpsc::Receiver<Event>>,
) -> Result<(), io::Error> {
    let mut input_buffer = String::new();

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
            let main_box = Block::default().title("Main Box").borders(Borders::ALL);
            f.render_widget(main_box, vertical_chunks[0]);

            // Bottom bar
            let bottom_paragraph =
            Paragraph::new(Text::from(input_buffer.as_str())).block(Block::bordered());
            f.render_widget(bottom_paragraph, vertical_chunks[1]);

            // Right panel
            let right_panel = Block::default().title("Right Panel").borders(Borders::ALL);
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
                KeyCode::Enter => {
                    if input_buffer == ":q" {
                        break;
                    }
                    // Event::FocusGained => todo!(),
                    // Event::FocusLost => todo!(),
                    // Event::Mouse(_) => todo!(),
                    // Event::Paste(_) => todo!(),
                    // Event::Resize(_, _) => todo!(),
                    input_buffer.clear();
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
                _ => {}
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
