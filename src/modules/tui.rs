use crossterm::event;
use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{Event, KeyCode},
};
use std::io;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mock_event_receiver: Option<std::sync::mpsc::Receiver<Event>>) -> Result<(), io::Error> {
    // Application loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Simple TUI")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        // Handle input
        if mock_event_receiver.is_some() {
            if let Ok(event) = mock_event_receiver.as_ref().unwrap().recv() {
                match event {
                    Event::Key(key) => {
                        if key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Mouse(_) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                }
            }
        }

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui::backend::TestBackend;
    use tui::Terminal;
    use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState, Event};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_run_app_exits_on_q() {
        // Create a TestBackend and Terminal
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        // Create an event channel
        let (event_sender, event_receiver) = mpsc::channel();

        // Simulate pressing 'q'
        let events = vec![
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            })
        ];

        thread::spawn(move || {
            for event in events {
                event_sender.send(event).unwrap();
            }
        });

        // Run the app
        let result = run_app(&mut terminal, Some(event_receiver));

        println!("yes");

        // Assert the app exited without error
        assert!(result.is_ok());
    }
}

