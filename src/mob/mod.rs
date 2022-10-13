use bevy::prelude::*;
mod movment;
use movment::*;

mod mass;
use mass::*;

mod perseption;
use perseption::*;

pub use perseption::DontView;

#[derive(Component, Default)]
pub struct Mob;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerseptionEvent>()
            .add_event::<MobMoveEvent>()
            .add_system(perseption_reader)
            .add_system(viewing)
            .add_system(mob_move);
    }
}

#[derive(Bundle, Default)]
pub struct MobBundle {
    pub mob: Mob,
    pub speed: Speed,
    pub vition: Vision,
}
