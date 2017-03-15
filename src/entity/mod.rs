
mod item;
mod action;
mod book;

pub use self::item::Item;
pub use self::action::Action;
pub use self::book::Book;

#[derive(Clone)]
pub enum Entity {
    Book(Book),
    Spell
}

