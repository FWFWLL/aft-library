use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::model::Book;
use crate::ui::ui;

pub enum CurrentScreen {
    Library,
    Registration,
    Search,
}

pub enum CurrentField {
    Title,
    Author,
    Genre,
    Publication,
}

// Application state
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_field: Option<CurrentField>,
    pub library: Vec<Book>,
    pub current_book: Option<Book>,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Library,
            current_field: None,
            library: Vec::new(),
            current_book: None,
        }
    }

    pub fn register_book(&mut self) -> Result<(), &'static str> {
        self.current_screen = CurrentScreen::Registration;
        self.current_field = Some(CurrentField::Title);

        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.load_state()?;

        loop {
            terminal
                .draw(|f| ui(f, self))
                .expect("Failed to draw to terminal");

            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match self.current_screen {
                    CurrentScreen::Library => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => {},
                    },
                    CurrentScreen::Registration => todo!(),
                    CurrentScreen::Search => todo!(),
                }
            }
        }

        self.save_state()
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

        Ok(())
    }

    fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
