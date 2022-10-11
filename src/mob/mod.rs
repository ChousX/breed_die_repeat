use bevy::prelude::*;
mod movment;
use movment::*;

mod mass;
use mass::*;

pub struct Mob;

pub struct MobPlugin;
impl Plugin for MobPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<MobMoveEvent>()
            .add_system(mob_move)
        ;
    }
}

#[derive(Bundle, Default)]
pub struct MobBundle{
    pub mob: Mob,
    pub speed: Speed,
}

