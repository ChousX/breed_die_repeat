use bevy::prelude::*;
use super::{MobMoveEvent, Speed, mResorce};
pub enum TagType {
    Food,
    Danger,
}

#[derive(Component)]
pub struct Vition(f32);
impl Default for Vition {
    fn default() -> Self {
        Self(10.0)
    }
}

pub fn sight(
    mut vition: Query<(&Vition, &Transform, Entity)>,
    mut all: Query<(&Transform, Entity)>,
    mut output: EventWriter<PerseptionEvent>,
) {
    for (Vition(range), transform, entity) in vition.iter() {
        let (x, z) = (transform.translation.x, transform.translation.z);
        let grater: (f32, f32) = (x + range, z + range);
        let lesser: (f32, f32) = (x - range, z - range);
        //this is really bad no good no good must fix
        //fix chunk entitys and only have to look at most 4 chunks
        //chunks should be of a size to reduse the change of 4 way chunk but also should be of a practigle size
        //the golden ration may work(may also be solved)
        let mut seen = Vec::new();
        let _ = all.iter().map(|(tran, e)| {
            let (tx, tz) = (tran.translation.x, tran.translation.z);
            //this is a box witch is fine but not a cirle witch would be more intutive
            if entity != e && tx <= grater.0 && tx >= lesser.0 && tz <= grater.1 && tz >= lesser.1 {
                let tag_type = if let Ok(_) = all.get_component::<mResorce>(e) {
                    TagType::Food
                } else {
                    TagType::Danger
                };
                seen.push((e, tag_type));
            }
        });
        output.send(PerseptionEvent::TrueSight(seen, entity))
    }
}

pub enum PerseptionEvent {
    //add
    TrueSight(Vec<(Entity, TagType)>, Entity),
}

pub fn perseption_event_handler(
    mut events: EventReader<PerseptionEvent>,
    mut output: EventWriter<MobMoveEvent>,
    mut transform: Query<&Transform>,
    mut speed: Query<&Speed>,
) {
    //so I would like to add in some entity matching but this is a todo
    for PerseptionEvent::TrueSight(things, entity) in events.iter() {
        if let Ok(from) = transform.get_component::<Transform>(*entity) {
            for (e, tt) in things.iter() {
                match tt {
                    TagType::Food => {
                        if let Ok(to) = transform.get_component::<Transform>(*e) {
                            let norm = move_towrds(from.translation, to.translation);
                            let speed = if let Ok(Speed(speed)) = speed.get(*entity) {
                                *speed
                            } else {
                                0.0f32
                            };
                            output.send(MobMoveEvent(*entity, norm * speed));
                            break;
                        }
                    }
                    TagType::Danger => {}
                }
            }
        }
    }
}

fn move_towrds(from: Vec3, to: Vec3) -> Vec3 {
    (to - from).normalize_or_zero()
}