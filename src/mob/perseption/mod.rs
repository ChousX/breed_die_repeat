use bevy::prelude::*;

mod vision;
use super::MobMoveEvent;
pub use vision::*;


mod cognision;
pub use cognision::*;

pub enum PerseptionEvent {
    //I think for testing only
    TrueSight {
        id: Entity,
        seen: Vec<Preseved>,
    },
}

pub enum Preseved {
    Food(Entity),
    Friend(Entity),
    Fow(Entity),
}


