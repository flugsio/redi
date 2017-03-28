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
        let mut last: Option<Rc<RefCell<Area>>> = None;
        for floor_nr in 1..20 {
            let floor = Area::new(&format!("Floor {}", floor_nr)).wrap();
            // TODO: add randomness
            for room_nr in 1..5 {
                Self::make_room(floor.clone(), &format!("{}{}", floor_nr, room_nr));
            };
            if let Some(last) = last {
                Area::connect(last.clone(), "upstasis", floor.clone(), "downstairs");
            };
            last = Some(floor);
        };
        last.unwrap()
    }

    fn make_room(out: Rc<RefCell<Area>>, name: &str) -> Rc<RefCell<Area>> {
        let room = Area::new(&format!("Room {}", name)).wrap();
        Area::connect(out, name, room.clone(), "out");
        room
    }
}
