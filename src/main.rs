use std::{io, thread, time};
use std::io::Write;

mod entity;
mod phrases;

use entity::Entity;
use entity::Item;
use phrases::Phrases;

fn main() {
    let book1 = entity::Book::new("The empty lake", 7);
    let items = vec!(Entity::Book(book1), Entity::Spell);
    write_slowly(&Phrases::random_welcome());

    loop {
        let command = readln();
        // TODO: this should support multiple matches and find logic
        // TODO: add an engine to analyse requests to responses
        match command.split_whitespace().nth(0) {
            Some("list") => show_inventory(items.clone()),
            Some("help") => println!("Try list or use"),
            Some("quit") => {
                ragequit();
                break
            },
            _ => println!("try again")
        }
    }
}

fn write_slowly(message: &str) {
    for word in message.split_whitespace() {
        for letter in word.chars() {
            print!("{}", letter);
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(20));
        }
        print!(" ");
        thread::sleep(time::Duration::from_millis(100));
    }
    println!("");
}

fn show_inventory(items: Vec<Entity>) {
    write_slowly("This is what you carry:");
    if items.len() == 0 {
        // TODO: randomize different responses
        write_slowly("  You rummage through your pockets, but thing nothing");
    } else {
        for item in items {
            write_slowly(match item {
                Entity::Book(b) => format!(" * {}: {}, {}", b.what(), b.name(), b.describe()),
                Entity::Spell => format!(" * one crappy spell"),
            }.as_str());
        }
    }
    println!("");
}

fn ragequit() {
    write_slowly(&Phrases::random_ragequit());
}

fn readln() -> String {
    print!("# ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}
