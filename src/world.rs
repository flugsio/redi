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
}
