
use entity::action::Action;

pub trait Item {
     fn what(&self) -> String;
     fn name(&self) -> String;
     fn actions(&self) -> Vec<Action>;
     fn trigger(&self, action: Action);
}

