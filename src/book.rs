use chrono::DateTime;
use chrono::Utc;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Available,
    CheckedOut(DateTime<Utc>),
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String, // Books can have multiple genres
    pub year: u32,     // Year of publication
    pub status: Status,
}

impl Book {
    // Discouraged to use this constructor directly, use the builder instead
    fn new(title: String, author: String, genre: String, year: u32) -> Self {
        Book {
            title,
            author,
            genre,
            year,
            ..Default::default()
        }
    }

    pub fn builder() -> BookBuilder {
        BookBuilder::new()
    }

    pub fn check_out(&mut self) -> Result<DateTime<Utc>, &'static str> {
        match self.status {
            Status::Available => {
                let now = chrono::Utc::now();
                self.status = Status::CheckedOut(now);
                Ok(now)
            },
            Status::CheckedOut(_) => Err("Book is already checked out"),
        }
    }
}

// TODO: Implement error types for the builder
#[derive(Debug, Default)]
pub struct BookBuilder {
    title: Option<String>,
    author: Option<String>,
    genre: Option<String>,
    publication: Option<u32>,
}

impl BookBuilder {
    pub fn new() -> Self {
        BookBuilder::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(String::from(title));
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = Some(String::from(author));
        self
    }

    pub fn genre(mut self, genre: &str) -> Self {
        self.genre = Some(String::from(genre));
        self
    }

    pub fn year(mut self, publication: u32) -> Self {
        self.publication = Some(publication);
        self
    }

    pub fn build(self) -> Result<Book, &'static str> {
        let title = match self.title {
            Some(title) => title,
            None => return Err("Missing title"),
        };

        let author = match self.author {
            Some(author) => author,
            None => return Err("Missing author"),
        };

        let genre = match self.genre {
            Some(genre) => genre,
            None => return Err("Missing genre"),
        };

        let publication = match self.publication {
            Some(publication) => publication,
            None => return Err("Missing publication year"),
        };

        Ok(Book::new(title, author, genre, publication))
    }
}

#[cfg(test)]
mod test_book {
    use super::*;

    #[test]
    fn test_book_creation() {
        let book = Book::new(
            String::from("The Rust Programming Language"),
            String::from("Steve Klabnik, Carol Nichols"),
            String::from("Programming"),
            2018,
        );

        assert_eq!(book.title, "The Rust Programming Language");
        assert_eq!(book.author, "Steve Klabnik, Carol Nichols",);
        assert_eq!(book.genre, "Programming");
        assert_eq!(book.year, 2018);
        assert_eq!(book.status, Status::Available);
    }

    #[test]
    fn test_book_check_out() {
        let mut book = Book::new(
            String::from("The Rust Programming Language"),
            String::from("Steve Klabnik, Carol Nichols"),
            String::from("Programming"),
            2018,
        );

        let result = book.check_out();
        assert_eq!(result.is_ok(), true);
        assert_eq!(book.status, Status::CheckedOut(result.unwrap()));

        let result = book.check_out();
        assert_eq!(result.is_err(), true);
    }
}

#[cfg(test)]
mod test_book_builder {
    use super::*;

    #[test]
    fn test_book_builder() {
        let book = BookBuilder::new()
            .title("The Rust Programming Language")
            .author("Steve Klabnik, Carol Nichols")
            .genre("Programming")
            .year(2018)
            .build();

        assert_eq!(book.is_ok(), true);

        let book = book.unwrap();

        assert_eq!(book.title, "The Rust Programming Language");
        assert_eq!(book.author, "Steve Klabnik, Carol Nichols");
        assert_eq!(book.genre, "Programming");
        assert_eq!(book.year, 2018);
        assert_eq!(book.status, Status::Available);
    }

    #[test]
    fn test_book_builder_error() {
        let book = BookBuilder::new()
            .author("Steve Klabnik, Carol Nichols")
            .genre("Programming")
            .year(2018)
            .build();

        assert_eq!(book.is_err(), true);
    }
}
