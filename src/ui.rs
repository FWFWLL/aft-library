use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListState, Padding, Paragraph};

use crate::app::{App, CurrentField, CurrentScreen, StatusFilter};
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

    render_library(frame, lib_area, app);
    render_nav(frame, nav_area, app);

    match app.current_screen {
        CurrentScreen::Library => {
            let [search_area, details_area] = right_area.layout(&Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
            ]));

            render_search_bar(frame, search_area, &app);
            render_book_details(frame, details_area, &app.library, &app.library_state);
        },
        CurrentScreen::Registration | CurrentScreen::Edit => {
            render_book_editor_form(frame, right_area, app);
        },
        CurrentScreen::Search => {
            let [search_area, details_area] = right_area.layout(&Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
            ]));

            render_search_bar(frame, search_area, &app);
            render_book_details(frame, details_area, &app.library, &app.library_state);
        },
    }
}

fn render_library(frame: &mut Frame, area: Rect, app: &mut App) {
    let book_titles = app
        .library
        .iter()
        .filter(|book| match app.status_filter {
            StatusFilter::All => true,
            StatusFilter::Available => book.status == Status::Available,
            StatusFilter::CheckedOut => book.status != Status::Available,
        })
        .filter_map(|book| {
            let book_title = book.title.as_str();

            let search = app.search_input.value();
            if search.is_empty() {
                Some(book_title)
            } else {
                let book_author = &book.author;
                if book_title.find(search).is_some() || book_author.find(search).is_some() {
                    return Some(book_title);
                }

                None
            }
        })
        // .map(|book| book.title.as_str())
        .collect::<Vec<_>>();

    let library = List::new(book_titles)
        .highlight_style(Modifier::REVERSED)
        .block(
            Block::bordered()
                .title(Span::raw("Library").bold())
                .padding(Padding::horizontal(1)),
        );

    frame.render_stateful_widget(library, area, &mut app.library_state);
}

fn render_nav(frame: &mut Frame, area: Rect, app: &App) {
    let nav_block = Block::bordered()
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let mut nav_text = vec![
        Span::styled("Q", Style::default().underlined()),
        Span::raw("uit | "),
        Span::styled("R", Style::default().underlined()),
        Span::raw("egister New Book | "),
        Span::styled("S", Style::default().underlined()),
        Span::raw("earch | "),
        Span::styled("T", Style::default().underlined()),
        Span::raw("oggle Status | "),
        Span::styled("F", Style::default().underlined()),
        Span::raw("iltering for "),
    ];

    match app.status_filter {
        StatusFilter::All => nav_text.push(Span::styled("All", Style::default().italic())),
        StatusFilter::Available => nav_text.push(Span::styled("Available", Style::default().italic())),
        StatusFilter::CheckedOut => nav_text.push(Span::styled("Checked Out", Style::default().italic())),
    };

    let nav = Paragraph::new(Line::from(nav_text)).block(nav_block);
    frame.render_widget(nav, area);
}

fn render_search_bar(frame: &mut Frame, area: Rect, app: &App) {
    let search_bar_style = if app.current_field == Some(CurrentField::Search) {
        Style::default().blue()
    } else {
        Style::default()
    };

    let search_bar = Paragraph::new(app.search_input.value()).block(
        Block::bordered()
            .title(Span::raw("Search").bold())
            .padding(Padding::horizontal(1))
            .style(search_bar_style),
    );

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
            book_details.push(Line::from(Span::styled(
                book.title.as_str(),
                Style::default().bold(),
            )));
            book_details.push(Line::from(format!("Author: {}", book.author)));
            book_details.push(Line::from(format!("Genre: {}", book.genre)));
            book_details.push(Line::from(format!("Published in {}", book.year)));

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
                        date_time.format("%Y/%m/%d %T").to_string(),
                        Style::default().fg(Color::Red),
                    ));
                },
            }
            book_details.push(Line::from(book_status));
        }
    };

    let details =
        Paragraph::new(book_details).block(Block::bordered().padding(Padding::horizontal(1)));
    frame.render_widget(details, area);
}

fn render_book_editor_form(frame: &mut Frame, area: Rect, app: &mut App) {
    let block_title = match app.current_screen {
        CurrentScreen::Registration => "Registering New Book",
        CurrentScreen::Edit => "Editing Book",
        _ => unreachable!(),
    };

    let editor_block = Block::bordered().title(block_title);

    let [
        title_area,
        author_area,
        genre_area,
        year_area,
        _,
        instruction_area,
    ] = area.layout(
        &Layout::vertical([
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Percentage(100),
            Constraint::Length(3),
        ])
        .margin(1),
    );

    let mut title_block = Block::bordered().title("Title");
    let mut author_block = Block::bordered().title("Author");
    let mut genre_block = Block::bordered().title("Genre");
    let mut publicatoin_block = Block::bordered().title("Year of Publication");

    match app.current_field {
        Some(CurrentField::Title) => title_block = title_block.blue(),
        Some(CurrentField::Author) => author_block = author_block.blue(),
        Some(CurrentField::Genre) => genre_block = genre_block.blue(),
        Some(CurrentField::Year) => publicatoin_block = publicatoin_block.blue(),
        _ => {},
    }

    let title_input = Paragraph::new(app.editor_form.title.value()).block(title_block);
    let author_input = Paragraph::new(app.editor_form.author.value()).block(author_block);
    let genre_input = Paragraph::new(app.editor_form.genre.value()).block(genre_block);
    let year_input = Paragraph::new(app.editor_form.year.value()).block(publicatoin_block);

    frame.render_widget(editor_block, area);
    frame.render_widget(title_input, title_area);
    frame.render_widget(author_input, author_area);
    frame.render_widget(genre_input, genre_area);
    frame.render_widget(year_input, year_area);

    let cancel_text = "Cancel";
    let enter_text = match app.current_screen {
        CurrentScreen::Registration => "Register",
        CurrentScreen::Edit => "Save",
        _ => unreachable!(),
    };

    let [cancel_area, _, enter_area] = instruction_area.layout(&Layout::horizontal([
        Constraint::Length(cancel_text.len() as u16 + 4),
        Constraint::Fill(1),
        Constraint::Length(enter_text.len() as u16 + 4),
    ]));

    let cancel = Paragraph::new(Line::from(cancel_text)).block(
        Block::bordered()
            .title(Span::raw("Esc").bold())
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(cancel, cancel_area);

    let register = Paragraph::new(Line::from(enter_text).right_aligned()).block(
        Block::bordered()
            .title(Span::raw("Enter").bold())
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(register, enter_area);
}
