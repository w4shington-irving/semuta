

use crossterm::event::KeyCode;


use tui_tree_widget::TreeState;

type NodeId = u32;
pub fn handle_key(key: KeyCode, state: &mut TreeState<NodeId>) -> bool {
    match key {
        KeyCode::Up => { state.key_up(); true },
        KeyCode::Down => { state.key_down(); true },
        KeyCode::Left | KeyCode::Right => { state.toggle_selected(); true },
        _ => false,
    }
}
