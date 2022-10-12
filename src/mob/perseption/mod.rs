use bevy::prelude::*;

mod vision;
use super::MobMoveEvent;
pub use vision::*;

pub enum PerseptionEvent {
    //I think for testing only
    TrueSight {
        id: Entity,
        position: Vec3,
        seen: Vec<(Vec3, Preseved)>,
    },
}

pub enum Preseved {
    Food(Entity),
    Friend(Entity),
    Fow(Entity),
}

pub fn perseption_reader(
    mut events: EventReader<PerseptionEvent>,
    mut movement: EventWriter<MobMoveEvent>,
) {
    for e in events.iter() {
        match &*e {
            PerseptionEvent::TrueSight { id, position, seen } => {
                'main: for (p, preseved) in seen {
                    if let Preseved::Food(_) = preseved {
                        movement.send(MobMoveEvent::new(move_towrds(*position, *p), 1.0, *id));
                        break 'main;
                    }
                }
            }
        }
    }
}

fn move_towrds(from: Vec3, to: Vec3) -> Vec3 {
    (to - from).normalize_or_zero()
}
