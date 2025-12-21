use std::io;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, enable_raw_mode},

};

use std::time::Duration;
use crossterm::event::{poll, read, Event};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use std::sync::{Arc, Mutex};

use crate::app::{App};
use crate::ui::render::render_ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Artists,
    Albums { artist_id: i64 },
    Tracks { album_id: i64 },
}


pub mod render;
pub mod input;
pub mod build;



pub fn run(app: Arc<Mutex<App>>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    {
        let mut app = app.lock().unwrap();
        app.load_artists();
    }

    loop {
        // Draw UI
        {
            let mut app = app.lock().unwrap();
            terminal.draw(|f| render_ui(f, &mut app))?;
        }

        // Non-blocking input with timeout
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                let mut app = app.lock().unwrap();
                input::handle_key(key_event.code, &mut app);
            }
        }

        // Now the loop can continue even if no input
        // This allows the timer thread to update app.now_playing.elapsed
    }
}

