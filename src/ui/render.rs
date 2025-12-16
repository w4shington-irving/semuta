use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::Block,
};

use tui_tree_widget::{Tree, TreeItem, TreeState};

type NodeId = u32;


pub fn render_tree(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    items: &[TreeItem<'static, NodeId>],
    mut state: &mut TreeState<NodeId>,
) -> std::io::Result<()> {
    terminal.draw(|f| {
        let area = f.area();

        let tree = Tree::new(&items).unwrap()   // unwrap Result
            .block(Block::bordered().title("Music Library"))
            .highlight_symbol("▶ ");

        f.render_stateful_widget(tree, area, &mut state);
    })?;

    Ok(())
}
