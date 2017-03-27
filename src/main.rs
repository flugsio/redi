use std::{io, thread, time};
use std::io::Write;
use std::time::SystemTime;

use std::rc::Rc;
use std::cell::RefCell;

mod entity;
mod phrases;
mod area;
mod world;

use entity::Entity;
use entity::Item;
use phrases::Phrases;
use area::Area;

fn main() {
    let mut world = world::World::new();
    let book1 = entity::Book::new("The empty lake", 7);
    let items = vec!(Entity::Book(book1), Entity::Spell);
    let time = SystemTime::now();
    write_slowly(&random(Phrases::welcome_messages(), time));

    loop {
        let command = readln();
        // TODO: this should support multiple matches and find logic
        // TODO: add an engine to analyse requests to responses
        match command.split_whitespace().nth(0) {
            Some("list") => show_inventory(items.clone()),
            Some("items") => show_inventory(items.clone()),
            Some("help") => write_slowly(&random(Phrases::help_messages(), time)),
            Some("where") => write_slowly(format!(
                    "you are here: {}", world.location.borrow().name()).as_str()),
            Some("what") => describe_location(world.location.clone()),
            Some("go") => {
                if world.goto(command.split_whitespace().nth(1).unwrap().to_string()) {
                    write_slowly("You have arrived");
                } else {
                    write_slowly("No such location");
                }
            },
            Some("yes") => write_slowly("I think not."),
            Some("no") => write_slowly("Yes I agree"),
            //Some("search") => TODO search the current container, could take arguments
            Some("quit") => {
                write_slowly(&random(Phrases::quit_messages(), time));
                break
            },
            _ => write_slowly(&random(Phrases::what_messages(), time))
        }
    }
}

fn random(list: Vec<String>, time: SystemTime) -> String {
    // TODO, replace with real rand
    let i = time.elapsed().ok().unwrap().subsec_nanos() % list.len() as u32;
    list[i as usize].clone()
}

fn write_slowly(message: &str) {
    for letter in message.chars() {
        print!("{}", letter);
        io::stdout().flush().unwrap();
        let delay = match letter {
            '.' => 300,
            ',' => 100,
            ' ' => 130,
            _ => 25
        };
        thread::sleep(time::Duration::from_millis(delay));
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

fn describe_location(location: Rc<RefCell<Area>>) {
    for (path, link) in location.borrow().links.iter() {
        write_slowly(format!("{} leading to {}", path, link.borrow().name).as_str());
    }
}

fn readln() -> String {
    print!("# ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}
