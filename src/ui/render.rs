use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::Block,
    widgets::Borders,
    widgets::Paragraph,
    layout::{Layout, Constraint, Direction, Rect},
    text::{Span, Line},
    style::{Style, Modifier},    
    Frame,
};

use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::app::{App, NowPlaying};
use crate::ui::NodeId;

pub fn render_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(7),
        ])
        .split(f.area());

    let tree = Tree::new(&app.tree_items)
        .unwrap()
        .block(Block::default().borders(Borders::ALL).title("Music Library"))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(tree, chunks[0], &mut app.tree_state);

    render_player(f, chunks[1], &app.now_playing);
}



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
            .highlight_symbol(" ");

        f.render_stateful_widget(tree, area, &mut state);
    })?;

    Ok(())
}

fn render_player(f: &mut Frame, area: Rect, now: &Option<NowPlaying>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Now Playing");

    let lines = match now {
        Some(track) => vec![
            Line::from(Span::styled(
                &track.title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(format!(
                "{} / {}",
                format_time(track.position),
                format_time(track.duration),
            )),
            Line::from("[⏯ Space]  [⏮ Prev]  [⏭ Next]"),
        ],
        None => vec![Line::from("No track playing")],
    };

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

fn format_time(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}