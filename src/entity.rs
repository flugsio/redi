use std::collections::BTreeSet;

#[derive(Clone)]
pub enum Entity {
    Book(Book),
    Spell
}

pub enum Action {
    Read,
}

pub trait Item {
    fn what(&self) -> String;
    fn name(&self) -> String;
    fn actions(&self) -> Vec<Action>;
    fn trigger(&self, action: Action);
}

#[derive(Clone)]
pub struct Book {
    name: String,
    //pub pages: BTreeSet<Page>,
    pub pages: Vec<Page>,
}

#[derive(Clone)]
pub struct Page {
    page_number: i64,
}

impl Item for Book {
    fn what(&self) -> String {
        "Book".to_string()
    }

    fn actions(&self) -> Vec<Action> {
        vec!(Action::Read)
    }

    fn trigger(&self, action: Action) {
        match action {
            Action::Read => {
            }
            //_ => panic!("not implemented")
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Book {
    pub fn new(name: &str, number_of_pages: i64) -> Book {
        let mut pages = Vec::new();
        for i in 0..number_of_pages {
            pages.push(Page { page_number: i });
        }
        Book {
            name: name.to_string(),
            pages: pages,
        }
    }

    pub fn describe(&self) -> String {
        //if 
        "".to_string()
    }
}
