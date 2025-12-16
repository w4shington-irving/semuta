


use crossterm::event::KeyCode;


use tui_tree_widget::TreeState;
use crate::ui::NodeId;

pub fn handle_key(key: KeyCode, state: &mut TreeState<NodeId>) -> bool {
    match key {
        KeyCode::Up => {
            state.key_up();
            true
        }
        KeyCode::Down => {
            state.key_down();
            true
        }
        KeyCode::Left | KeyCode::Right => {
            state.toggle_selected();
            true
        }
        KeyCode::Enter => {
            if let Some(NodeId::Track(track_id)) = state.selected().last() {
                println!("Playing track {}", track_id);
                // play_track(*track_id);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

