use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    // Frame chunks
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(frame.area())[..]
    else {
        unreachable!();
    };

    // Left chunks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(3)])
        .split(left);

    // Left chunk -> Library
    let library_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::left(1))
        .style(Style::default());

    let list_books = app
        .library
        .iter()
        .enumerate()
        .map(|(i, book)| {
            ListItem::new(Line::from(Span::styled(
                String::from(&book.title),
                Style::default(),
            )))
        })
        .collect::<Vec<ListItem>>();

    let library = List::new(list_books).block(library_block);
    frame.render_widget(library, left_chunks[0]);

    // Left chunk -> Navigation
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
    frame.render_widget(nav, left_chunks[1]);

    // Right chunks
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(right);

    // Right chunk -> Search
    let search_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let search_text;
    if let Some(current_search_text) = &app.current_search_text {
        search_text = Paragraph::new(current_search_text.clone());
    } else {
        search_text = Paragraph::new(Span::styled("Search", Style::default().fg(Color::DarkGray)));
    }

    let search_bar = search_text.block(search_block);
    frame.render_widget(search_bar, right_chunks[0]);

    // Right column -> Details
    let details_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let mut details_text = Vec::new();

    let details = Paragraph::new(Text::from(details_text)).block(details_block);
    frame.render_widget(details, right_chunks[1]);
}

fn render_library(frame: &mut Frame, area: Rect, list_state: &ListState) {}

fn render_nav(frame: &mut Frame, area: Rect) {}
