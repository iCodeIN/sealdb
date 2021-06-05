#![feature(generic_associated_types)]
#![allow(incomplete_features)]

pub mod models;

use std::io::{self, BufRead, Write};

use chrono::Date;
use sealdb::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PromptAction {
    List,
    Quit,
}

#[derive(Default)]
struct TaskBoard {
    pub boards: Table<models::Board>,
    pub labels: Table<models::Label>,
    pub lists: Table<models::List>,
    pub task_labels: Table<models::TaskLabel>,
    pub tasks: Table<models::Task>,
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

        // Check if we reached EOF
        if stdin.read_line(&mut line)? == 0 {
            println!();
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

fn list_items(store: &TaskBoard) {
    let completed_tasks: Vec<_> = store.tasks.filter(|task: &models::TaskFields<0>| task.completed).collect();
    dbg!(completed_tasks);

    todo!()
}

fn sample_data() -> TaskBoard {
    let store = TaskBoard::default();

    store
}
