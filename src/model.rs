use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Available,
    CheckedOut(DateTime<Utc>),
}

impl Default for Status {
    fn default() -> Self {
        Status::Available
    }
}

// Every field is required (Might change later)
#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub title: String,
    pub author: Vec<String>,
    pub genre: String,    // Genres are arbitrary and thus difficult to enumerate
    pub publication: u32, // Year of publication
    pub status: Status,
}

impl Book {
    // Discouraged to use this constructor directly, use the builder instead
    fn new(title: String, author: Vec<String>, genre: String, publication: u32) -> Self {
        Book {
            title,
            author,
            genre,
            publication,
            status: Status::default(),
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
    author: Option<Vec<String>>,
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
        if self.author.is_none() {
            self.author = Some(vec![String::from(author)]);
        } else {
            self.author.as_mut().unwrap().push(String::from(author));
        }
        self
    }

    pub fn genre(mut self, genre: &str) -> Self {
        self.genre = Some(String::from(genre));
        self
    }

    pub fn publication(mut self, publication: u32) -> Self {
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
            vec![String::from("Steve Klabnik"), String::from("Carol Nichols")],
            String::from("Programming"),
            2018,
        );

        assert_eq!(book.title, "The Rust Programming Language");
        assert_eq!(
            book.author,
            vec![String::from("Steve Klabnik"), String::from("Carol Nichols")]
        );
        assert_eq!(book.genre, "Programming");
        assert_eq!(book.publication, 2018);
        assert_eq!(book.status, Status::Available);
    }

    #[test]
    fn test_book_check_out() {
        let mut book = Book::new(
            String::from("The Rust Programming Language"),
            vec![String::from("Steve Klabnik"), String::from("Carol Nichols")],
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
            .author("Steve Klabnik")
            .author("Carol Nichols")
            .genre("Programming")
            .publication(2018)
            .build();

        assert_eq!(book.is_ok(), true);

        let book = book.unwrap();

        assert_eq!(book.title, "The Rust Programming Language");
        assert_eq!(
            book.author,
            vec![String::from("Steve Klabnik"), String::from("Carol Nichols")]
        );
        assert_eq!(book.genre, "Programming");
        assert_eq!(book.publication, 2018);
        assert_eq!(book.status, Status::Available);
    }

    #[test]
    fn test_book_builder_error() {
        let book = BookBuilder::new()
            .author("Steve Klabnik")
            .author("Carol Nichols")
            .genre("Programming")
            .publication(2018)
            .build();

        assert_eq!(book.is_err(), true);
    }
}
