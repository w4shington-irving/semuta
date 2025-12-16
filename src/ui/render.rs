use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::Block,
};

use tui_tree_widget::{Tree, TreeItem, TreeState};


use crate::ui::NodeId;

pub fn render_tree(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    items: &Vec<TreeItem<'static, NodeId>>,
    mut state: &mut TreeState<NodeId>,
) -> std::io::Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let root_items: Vec<TreeItem<NodeId>> = items.clone();
        let tree = Tree::new(&root_items).unwrap()
            .block(Block::bordered().title("Music Library"))
            .highlight_symbol("▶ ");

        f.render_stateful_widget(tree, area, &mut state);
    })?;

    Ok(())
}
