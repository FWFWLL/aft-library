use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;

use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

use crate::book::{Book, BookBuilder};
use crate::ui::ui;

#[derive(Default)]
pub enum CurrentScreen {
    #[default]
    Library,
    Registration,
    Search,
}

pub enum CurrentField {
    Title,
    Author,
    Genre,
    Publication,
    Search,
}

#[derive(Default)]
pub struct RegistrationForm {
    pub title: Input,
    pub author: Input,
    pub genre: Input,
    pub publication: Input,
}

// Application state
#[derive(Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_field: Option<CurrentField>,
    pub library: Vec<Book>,
    pub library_state: ListState,
    pub prev_index: usize,
    pub registration_form: RegistrationForm,
    pub search_input: Input,
}

enum Message {
    Quit,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        terminal
            .draw(|f| ui(f, self))
            .expect("Failed to draw to terminal");
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
                        self.prev_index = self.library_state.selected().unwrap(); // Should never be `None`
                        self.library_state.select(None);
                        self.current_screen = CurrentScreen::Registration;
                        self.current_field = Some(CurrentField::Title);
                    },
                    _ => return None,
                },
                CurrentScreen::Registration => match key.code {
                    KeyCode::Esc => {
                        self.library_state.select(Some(self.prev_index));
                        self.current_screen = CurrentScreen::Library;
                        self.current_field = None;
                    },
                    KeyCode::Enter => {
                        match self.register_book() {
                            Ok(_) => {
                                self.library_state.select_last();
                                self.current_screen = CurrentScreen::Library;
                                self.current_field = None;

                                self.registration_form.title.reset();
                                self.registration_form.author.reset();
                                self.registration_form.genre.reset();
                                self.registration_form.publication.reset();
                            },
                            Err(_) => panic!("How?"),
                        };
                        
                    },
                    KeyCode::Tab | KeyCode::Down => match self.current_field {
                        Some(CurrentField::Title) => {
                            self.current_field = Some(CurrentField::Author)
                        },
                        Some(CurrentField::Author) => {
                            self.current_field = Some(CurrentField::Genre)
                        },
                        Some(CurrentField::Genre) => {
                            self.current_field = Some(CurrentField::Publication)
                        },
                        Some(CurrentField::Publication) => {
                            self.current_field = Some(CurrentField::Title)
                        },
                        _ => {},
                    },
                    KeyCode::BackTab | KeyCode::Up => match self.current_field {
                        Some(CurrentField::Title) => {
                            self.current_field = Some(CurrentField::Publication)
                        },
                        Some(CurrentField::Author) => {
                            self.current_field = Some(CurrentField::Title)
                        },
                        Some(CurrentField::Genre) => {
                            self.current_field = Some(CurrentField::Author)
                        },
                        Some(CurrentField::Publication) => {
                            self.current_field = Some(CurrentField::Genre)
                        },
                        _ => {},
                    },
                    _ if let Some(current_field) = &self.current_field => match current_field {
                        CurrentField::Title => {
                            self.registration_form.title.handle_event(&event);
                        },
                        CurrentField::Author => {
                            self.registration_form.author.handle_event(&event);
                        },
                        CurrentField::Genre => {
                            self.registration_form.genre.handle_event(&event);
                        },
                        CurrentField::Publication => {
                            self.registration_form.publication.handle_event(&event);
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

    fn register_book(&mut self) -> Result<(), &'static str> {
        let title = self.registration_form.title.value();
        let author = self.registration_form.author.value();
        let genre = self.registration_form.genre.value();
        let publication = self.registration_form.publication.value();

        let book = Book::builder()
            .title(title)
            .author(author)
            .genre(genre)
            .publication(publication.parse().unwrap_or_default())
            .build()?;

        self.library.push(book);

        Ok(())
    }

    fn load_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: replace with actual loading logic
        self.library.push(
            Book::builder()
                .title("The Great Gatsby")
                .author("F. Scott Fitzgerald")
                .genre("Fiction")
                .publication(1925)
                .build()?,
        );
        self.library.push(
            Book::builder()
                .title("To Kill a Mockingbird")
                .author("Harper Lee")
                .genre("Fiction")
                .genre("Dystopian")
                .publication(1960)
                .build()?,
        );
        self.library.push(
            Book::builder()
                .title("The Rust Programming Language")
                .author("Steve Klabnik")
                .author("Carol Nichols")
                .genre("Programming")
                .publication(2018)
                .build()?,
        );

        let _ = self.library[2].check_out();

        self.library_state.select(Some(0));

        Ok(())
    }

    fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
