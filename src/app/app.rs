use tui_tree_widget::TreeState;
use crate::ui::NodeId;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub track_id: i64,
    pub title: String,
    pub position: Duration,
    pub duration: Duration,
    pub playing: bool,
}

pub struct App {
    pub tree_state: TreeState<NodeId>,
    pub tree_items: Vec<tui_tree_widget::TreeItem<'static, NodeId>>,
    pub now_playing: Option<NowPlaying>,
}

impl App {
    pub fn new(tree_items: Vec<tui_tree_widget::TreeItem<'static, NodeId>>) -> Self {
        Self {
            tree_state: TreeState::default(),
            tree_items,
            now_playing: None,
        }
    }
}