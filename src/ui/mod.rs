use std::io;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},

};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::app::App;
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



pub fn start() -> io::Result<()> {
    enable_raw_mode()?;

    

    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?; // ← FIX IS HERE

    let mut app = App::new();
    app.load_artists();

    
    
    loop {
        terminal.draw(|f| {
            render_ui(f, &mut app);
        })?;

        if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
            
            input::handle_key(key_event.code, &mut app);
        }

        app.update();
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
