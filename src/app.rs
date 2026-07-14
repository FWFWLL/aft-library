use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use anyhow::Result;

use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;

use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

use crate::book::Book;
use crate::ui::ui;

#[derive(Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Library,
    Registration,
    Edit,
    Search,
}

pub enum CurrentField {
    Title,
    Author,
    Genre,
    Year,
    Search,
}

#[derive(Default)]
pub struct EditorForm {
    pub title: Input,
    pub author: Input,
    pub genre: Input,
    pub year: Input,
}

// Application state
#[derive(Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_field: Option<CurrentField>,
    pub library: Vec<Book>,
    pub library_state: ListState,
    pub library_index: usize,
    pub editor_form: EditorForm,
    pub search_input: Input,
}

enum Message {
    Quit,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.load_state()?;

        loop {
            self.render(terminal);

            match self.update() {
                Some(Message::Quit) => break,
                _ => {},
            };
        }

        self.save_state()
    }

    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        terminal.draw(|f| ui(f, self)).expect("Failed to draw to terminal");
    }

    fn update(&mut self) -> Option<Message> {
        let event = event::read().expect("Failed to read event");
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Release {
                return None;
            }

            match self.current_screen {
                CurrentScreen::Library => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Some(Message::Quit),
                    KeyCode::Char('j') | KeyCode::Down => self.library_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.library_state.select_previous(),
                    KeyCode::Char('r') | KeyCode::Char('a') => {
                        self.library_index = self.library_state.selected().unwrap(); // Should never be `None` in `CurrentScreen::Library`
                        self.library_state.select(None);
                        self.current_screen = CurrentScreen::Registration;
                        self.current_field = Some(CurrentField::Title);
                    },
                    KeyCode::Char('e') => {
                        self.library_index = self.library_state.selected().unwrap(); // Should never be `None` in `CurrentScreen::Library`
                        self.current_screen = CurrentScreen::Edit;
                        self.current_field = Some(CurrentField::Title);

                        let book = self.library.get(self.library_index).unwrap();

                        self.editor_form.title = Input::new(book.title.clone());
                        self.editor_form.author = Input::new(book.author.clone());
                        self.editor_form.genre = Input::new(book.genre.clone());
                        self.editor_form.year = Input::new(book.year.to_string());
                    },
                    _ => return None,
                },
                CurrentScreen::Registration | CurrentScreen::Edit => match key.code {
                    KeyCode::Esc => {
                        if self.current_screen == CurrentScreen::Edit {
                            self.editor_form.title.reset();
                            self.editor_form.author.reset();
                            self.editor_form.genre.reset();
                            self.editor_form.year.reset();
                        }

                        self.library_state.select(Some(self.library_index));

                        self.current_screen = CurrentScreen::Library;
                        self.current_field = None;
                    },
                    KeyCode::Enter => {
                        if self.current_screen == CurrentScreen::Registration {
                            self.register_book();
                            self.library_state.select_last();
                        } else {
                            self.edit_book();
                        }

                        self.editor_form.title.reset();
                        self.editor_form.author.reset();
                        self.editor_form.genre.reset();
                        self.editor_form.year.reset();

                        self.current_screen = CurrentScreen::Library;
                        self.current_field = None;
                    },
                    KeyCode::Tab | KeyCode::Down => match self.current_field {
                        Some(CurrentField::Title) => self.current_field = Some(CurrentField::Author),
                        Some(CurrentField::Author) => self.current_field = Some(CurrentField::Genre),
                        Some(CurrentField::Genre) => self.current_field = Some(CurrentField::Year),
                        Some(CurrentField::Year) => self.current_field = Some(CurrentField::Title),
                        _ => {},
                    },
                    KeyCode::BackTab | KeyCode::Up => match self.current_field {
                        Some(CurrentField::Title) => self.current_field = Some(CurrentField::Year),
                        Some(CurrentField::Author) => self.current_field = Some(CurrentField::Title),
                        Some(CurrentField::Genre) => self.current_field = Some(CurrentField::Author),
                        Some(CurrentField::Year) => self.current_field = Some(CurrentField::Genre),
                        _ => {},
                    },
                    _ if let Some(current_field) = &self.current_field => match current_field {
                        CurrentField::Title => {
                            self.editor_form.title.handle_event(&event);
                        },
                        CurrentField::Author => {
                            self.editor_form.author.handle_event(&event);
                        },
                        CurrentField::Genre => {
                            self.editor_form.genre.handle_event(&event);
                        },
                        CurrentField::Year => {
                            self.editor_form.year.handle_event(&event);
                        },
                        _ => {},
                    },
                    _ => return None,
                },
                CurrentScreen::Search => todo!(),
            }
        }

        None
    }

    fn register_book(&mut self) {
        let title = self.editor_form.title.value();
        let author = self.editor_form.author.value();
        let genre = self.editor_form.genre.value();
        let year = self.editor_form.year.value();

        let book = Book::builder()
            .title(title)
            .author(author)
            .genre(genre)
            .year(year.parse().unwrap_or_default())
            .build()
            .unwrap();

        self.library.push(book);
    }

    fn edit_book(&mut self) {
        let title = self.editor_form.title.value();
        let author = self.editor_form.author.value();
        let genre = self.editor_form.genre.value();
        let publication = self.editor_form.year.value();

        match self.library.get_mut(self.library_index) {
            Some(book) => {
                book.title = title.to_string();
                book.author = author.to_string();
                book.genre = genre.to_string();
                book.year = publication.parse().unwrap_or_default();
            },
            None => unreachable!(),
        };
    }

    fn load_state(&mut self) -> Result<()> {
        let file = File::open("state.json")?;
        let reader = BufReader::new(file);

        self.library = serde_json::from_reader(reader)?;
        self.library_state.select_first();

        Ok(())
    }

    fn save_state(&self) -> Result<()> {
        let file = File::create("state.json")?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &self.library)?;

        writer.flush()?;

        Ok(())
    }
}
