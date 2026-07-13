use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph};

use crate::app::App;
use crate::book::Book;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let [left_area, right_area] = frame.area().layout(&Layout::horizontal([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ]));
    let [lib_area, nav_area] = left_area.layout(&Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
    ]));
    let [search_area, details_area] = right_area.layout(&Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
    ]));

    render_library(frame, lib_area, &app.library, &mut app.library_state);
    render_nav(frame, nav_area);
    render_search_bar(frame, search_area);
    render_book_details(frame, details_area);
}

fn render_library(
    frame: &mut Frame,
    area: Rect,
    library: &Vec<Book>,
    library_state: &mut ListState,
) {
    let library_block = Block::bordered().style(Style::default());

    let book_titles: Vec<_> = library
        .iter()
        .map(|book| book.title.as_str())
        .collect();

    let library = List::new(book_titles)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(library_block);

    frame.render_stateful_widget(library, area, library_state);
}

fn render_nav(frame: &mut Frame, area: Rect) {
    let nav_block = Block::bordered()
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
    frame.render_widget(nav, area);
}

fn render_search_bar(frame: &mut Frame, area: Rect) {
    let search_block = Block::bordered()
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let search_bar = Paragraph::new(Span::styled("Search", Style::default().fg(Color::DarkGray)))
        .block(search_block);

    frame.render_widget(search_bar, area);
}

fn render_book_details(frame: &mut Frame, area: Rect) {
    let details_block = Block::bordered()
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let mut details_text = Vec::new();

    let details = Paragraph::new(Text::from(details_text)).block(details_block);
    frame.render_widget(details, area);
}
