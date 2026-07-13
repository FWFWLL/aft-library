use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListState, Padding, Paragraph};

use crate::app::App;
use crate::book::{Book, Status};

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
    render_book_details(frame, details_area, &app.library, &app.library_state);
}

fn render_library(
    frame: &mut Frame,
    area: Rect,
    library: &Vec<Book>,
    library_state: &mut ListState,
) {
    let book_titles = library
        .iter()
        .map(|book| book.title.as_str())
        .collect::<Vec<_>>();

    let library = List::new(book_titles)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::bordered());

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

fn render_book_details(
    frame: &mut Frame,
    area: Rect,
    library: &Vec<Book>,
    library_state: &ListState,
) {
    let mut book_details = Vec::default();
    if let Some(library_index) = library_state.selected() {
        if let Some(book) = library.get(library_index) {
            book_details.push(Line::from(Span::styled(book.title.as_str(), Style::default().bold())));

            book_details.push(Line::from(vec![
                Span::raw("Author: "),
                Span::raw(book.author.join(", ")),
            ]));

            book_details.push(Line::from(vec![
                Span::raw("Year of Publication: "),
                Span::raw(book.publication.to_string()),
            ]));

            book_details.push(Line::from(vec![
                Span::raw("Genre: "),
                Span::raw(book.genre.as_str()),
            ]));

            let mut book_status = vec![Span::raw("Status: ")];
            match book.status {
                Status::Available => {
                    book_status.push(Span::styled("Available", Style::default().fg(Color::Green)))
                },
                Status::CheckedOut(date_time) => {
                    book_status.push(Span::styled(
                        "Checked out ",
                        Style::default().fg(Color::Red),
                    ));
                    book_status.push(Span::styled(
                        date_time.to_string(),
                        Style::default().fg(Color::Red),
                    ));
                },
            }
            book_details.push(Line::from(book_status));
        }
    };

    let details = Paragraph::new(book_details).block(Block::bordered());
    frame.render_widget(details, area);
}
