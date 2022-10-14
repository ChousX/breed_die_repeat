use super::*;
use bevy::prelude::*;

pub fn _perseption_reader(
    mut events: EventReader<PerseptionEvent>,
    mut movement: EventWriter<MobMoveEvent>,
    transforms: Query<&Transform, Without<DontView>>,
) {
    for e in events.iter() {
        match &*e {
            PerseptionEvent::TrueSight { id, seen } => {
                'main: for preseved in seen {
                    if let Preseved::Food(en) = preseved {
                        if let Ok(p) = transforms.get(*en) {
                            if let Ok(position) = transforms.get(*id) {
                                movement.send(MobMoveEvent::new(
                                    move_towrds(position.translation, p.translation),
                                    1.0,
                                    *id,
                                ));
                                break 'main;
                            }
                        }
                    } else if let Preseved::Friend(en) = preseved {
                    }
                }
            }
        }
    }
}

fn move_towrds(from: Vec3, to: Vec3) -> Vec3 {
    (to - from).normalize_or_zero()
}
