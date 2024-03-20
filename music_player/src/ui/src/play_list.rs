use tui::{backend::Backend, layout::{Alignment, Rect}, widgets::{Block, BorderType, Borders, List, ListItem}, Frame};
use app::App;

/// 绘制播放列表
pub fn draw_play_list<T>(app: &mut App, frame: &mut Frame<T>, area: Rect)
where
    T: Backend,
{
    let mut items = vec![];
    let player = &app.player;
    for item in &player.play_list.lists {
        items.push(ListItem::new(item.name.as_str()))
    }
    let list = List::new(items).block(
        Block::default().borders(Borders::ALL).title("播放列表").border_type(BorderType::Rounded).title_alignment(Alignment::Center),
    );
    frame.render_widget(list, area);
}