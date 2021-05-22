mod models;

use std::io::{self, BufRead, Write};

use chrono::Date;
use sealdb::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PromptAction {
    List,
    Quit,
}

#[derive(Default)]
struct Bookstore {
    authors: Table<models::Author>,
    book_authors: Table<models::BookAuthor>,
    books: Table<models::Book>,
    listing_reviews: Table<models::ListingReview>,
    listings: Table<models::Listing>,
}

fn main() -> anyhow::Result<()> {
    let store = sample_data();

    loop {
        use PromptAction::*;
        match prompt()? {
            List => list_items(&store),

            Quit => break,
        }
    }

    Ok(())
}

fn prompt() -> anyhow::Result<PromptAction> {
    println!("Commands:");
    println!("* l/list - list all books");
    println!("* q/quit/exit - exit the program");

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    loop {
        print!("Enter a command: ");
        io::stdout().flush()?;

        if stdin.read_line(&mut line)? == 0 {
            // Reached EOF
            return Ok(PromptAction::Quit)
        }

        line.make_ascii_lowercase();
        return Ok(match line.trim() {
            "l" | "list" => PromptAction::List,
            "q" | "quit" | "exit" => PromptAction::Quit,

            _ => {
                println!("Unrecognized command");
                continue;
            },
        });
    }
}

fn list_items(store: &Bookstore) {
    todo!()
}

fn sample_data() -> Bookstore {
    let store = Bookstore::default();

    store
}
