use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;

use crate::book::Book;
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

// Application state
#[derive(Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_field: Option<CurrentField>,
    pub library: Vec<Book>,
    pub library_state: ListState,
    pub current_search_text: Option<String>,
}

enum Message {
    Quit,
}

impl App {
    pub fn new() -> Self {
        App::default()
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
            self.render(terminal);

            match self.update() {
                Some(Message::Quit) => break,
                _ => {},
            };
        }

        self.save_state()
    }

    fn render<B: Backend>(&self, terminal: &mut Terminal<B>) {
        terminal
            .draw(|f| ui(f, self))
            .expect("Failed to draw to terminal");
    }

    fn update(&mut self) -> Option<Message> {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == event::KeyEventKind::Release {
                return None;
            }

            match self.current_screen {
                CurrentScreen::Library => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Some(Message::Quit),
                    KeyCode::Char('j') | KeyCode::Down => self.library_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.library_state.select_previous(),
                    _ => return None,
                },
                CurrentScreen::Registration => todo!(),
                CurrentScreen::Search => todo!(),
            }
        }

        None
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
