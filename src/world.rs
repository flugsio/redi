use std::rc::Rc;
use std::cell::RefCell;

use area::Area;

pub struct World {
    pub location: Rc<RefCell<Area>>,
    pub previous: Rc<RefCell<Area>>,
}

impl World {
    pub fn new() -> World {
        let start = Self::build_tower();
        let nowhere = Rc::new(RefCell::new(Area::new("in complete darkness")));

        World {
            location: start,
            previous: nowhere,
        }
    }

     pub fn goto(&mut self, name: &str) -> bool {
         let new = match name {
             "back" => Some(self.previous.clone()),
             name => self.location.borrow().find(name)
         };
         match new {
             Some(new) =>  {
                 self.previous = self.location.clone();
                 self.location = new;
                 true
             },
             None => false
         }
     }

    fn build_tower() -> Rc<RefCell<Area>> {
        let start = Rc::new(RefCell::new(Area::new("Floor 15")));

        let above = Rc::new(RefCell::new(Area::new("Floor 16")));
        Area::connect(start.clone(), "upstairs", above, "downstairs");

        let below = Rc::new(RefCell::new(Area::new("Floor 14")));
        Area::connect(start.clone(), "downstairs", below, "upstairs");

        start
    }
}
