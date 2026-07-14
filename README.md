# AFT Library

A terminal-based book library management system built in Rust using [Ratatui](https://ratatui.rs/). AFT Library provides a user-friendly TUI (Terminal User Interface) for managing your personal or small book collection.

## Features

- **Browse Library**: View all books in your library in an organized list
- **Add Books**: Register new books with title, author, genre, and publication year
- **Edit Books**: Modify book information
- **Delete Books**: Remove books from your library
- **Track Status**: Mark books as available or checked out with timestamps
- **Search**: Find books by title or author
- **Filter**: View books by availability status (All, Available, or Checked Out)
- **Persistent Storage**: Your library data is saved automatically in JSON format

## Building

Ensure you have Rust installed, then:

```bash
cargo build --release
```

The compiled binary will be in `target/release/`.

## Running

```bash
cargo run
```

Or run the release binary directly:

```bash
./target/release/aft-library
```

## Controls

### Main Library Screen

| Key | Action |
|-----|--------|
| **Q** or **Esc** | Quit the application |
| **J** or **↓** | Move down in book list |
| **K** or **↑** | Move up in book list |
| **R** or **A** | Register/Add a new book |
| **E** or **C** | Edit the selected book |
| **T** | Toggle book status (Available ↔ Checked Out) |
| **D** or **Delete** | Delete the selected book |
| **S** | Enter search mode |
| **F** | Cycle through filters (All → Available → Checked Out) |

### Book Registration/Edit Screen

| Key | Action |
|-----|--------|
| **Tab** or **↓** | Move to next field (Title → Author → Genre → Year) |
| **Shift+Tab** or **↑** | Move to previous field |
| **Enter** | Register/Save the book |
| **Esc** | Cancel and return to library |

### Search Screen

Type to search books by title or author. Press **Esc** or **Q** to exit search mode.

## Book Information

Each book in the library stores:

- **Title**: Book title
- **Author**: Author name
- **Genre**: Book genre/category
- **Year**: Publication year
- **Status**: 
  - *Available* (green indicator)
  - *Checked Out* with timestamp (red indicator)

## Data Storage

Your library is automatically saved to `library.json` in the application directory. This file contains all your books and their current status information.

## Project Structure

```
src/
├── main.rs      # Application entry point and terminal setup
├── app.rs       # Application state and event handling
├── book.rs      # Book data structures and builder pattern
└── ui.rs        # UI rendering and layout
```

## License

[Add your license information here]
