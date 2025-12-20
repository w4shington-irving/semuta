pub mod build;
pub mod render;
pub mod input;

use std::io;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

use tui_tree_widget::TreeState;
use crate::app::App;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeId {
    Artist(i64),
    Album(i64),
    Track(i64),
}

pub fn display_library() {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    let items = build::build_tree().expect("Failed to build tree");
    let mut state = TreeState::default();
    state.select_first(); // select root node

    loop {
        render::render_tree(&mut terminal, &items, &mut state).expect("Failed to render tree");

        if let Event::Key(key_event) = event::read().expect("Failed to read event") {
            if key_event.code == KeyCode::Char('q') {
                break;
            }
            input::handle_key(key_event.code, &mut state);
        }

        // 
    }

    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
}

pub fn display_ui() {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    
}
