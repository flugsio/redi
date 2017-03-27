use std::rc::Rc;
use std::cell::RefCell;

use area::Area;

pub struct World {
    pub location: Rc<RefCell<Area>>,
    pub previous: Rc<RefCell<Area>>,
}

impl World {
    pub fn new() -> World {
        let start = Rc::new(RefCell::new(Area::new("Floor 15")));

        let above = Rc::new(RefCell::new(Area::new("Floor 16")));
        Area::connect(start.clone(), "upstairs", above, "downstairs");

        let below = Rc::new(RefCell::new(Area::new("Floor 14")));
        Area::connect(start.clone(), "downstairs", below, "upstairs");

        World {
            location: start.clone(),
            previous: start,
        }
    }

     pub fn goto(&mut self, name: String) -> bool {
         let new = self.location.borrow().find(name);
         match new {
             Some(new) =>  {
                 self.previous = self.location.clone();
                 self.location = new;
                 true
             },
             None => false
         }
     }

}
