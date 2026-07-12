use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    // let title_block = Block::default()
    //     .borders(Borders::ALL)
    //     .style(Style::default());

    // let title = Paragraph::new(Text::styled("Library", Style::default())).block(title_block);

    // frame.render_widget(title, chunks[0]);

    let library_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(chunks[0]);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::left(1))
        .style(Style::default());

    let list_items = app
        .library
        .iter()
        .map(|book| {
            ListItem::new(Line::from(Span::styled(
                String::from(&book.title),
                Style::default(),
            )))
        })
        .collect::<Vec<ListItem>>();

    let list = List::new(list_items).block(list_block);
    frame.render_widget(list, library_chunks[0]);

    let nav_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let nav_text = vec![
        Span::styled("R", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("egister | "),
        Span::styled("S", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("earch | "),
        Span::styled("C", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("heckout | "),
        Span::styled("Q", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("uit"),
    ];

    let nav = Paragraph::new(Line::from(nav_text)).block(nav_block);
    frame.render_widget(nav, library_chunks[1]);
}
