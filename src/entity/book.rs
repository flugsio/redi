// use std::collections::BTreeSet;

use entity::Action;
use entity::Item;

#[derive(Clone)]
pub struct Book {
    name: String,
    //pages: BTreeSet<Page>,
    pages: Vec<Page>,
}

#[derive(Clone)]
struct Page {
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
        // TODO: implement
        "".to_string()
    }
}
