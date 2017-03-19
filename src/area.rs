use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Area {
    name: String,
    links: HashMap<String, Rc<RefCell<Area>>>,
}

impl Area {
    pub fn new(name: &str) -> Area {
        let links = HashMap::new();
        Area {
            name: name.to_string(),
            links: links,
        }
    }

    pub fn connect(first: Rc<RefCell<Area>>, exit_name: &str,
                   second: Rc<RefCell<Area>>, return_name: &str) {
        first.borrow_mut().add(exit_name.to_string(), second.clone());
        second.borrow_mut().add(return_name.to_string(), first);
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn add(&mut self, name: String, other: Rc<RefCell<Area>>) {
        self.links.entry(name).or_insert(other);
    }
}

