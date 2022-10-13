use super::{PerseptionEvent, Preseved};
use crate::{rts_camera::RtsCamera, recorce::mResorce};
use bevy::prelude::*;

#[derive(Component)]
pub struct Vision {
    pub range: f32,
}

impl Default for Vision {
    fn default() -> Self {
        Self { range: 30.0 }
    }
}

pub fn viewing(
    q: Query<(&Transform, &Vision, Entity)>,
    things: Query<(&Transform, Entity), Without<DontView>>,
    recorces: Query<&mResorce>,
    mut output: EventWriter<PerseptionEvent>,
) {
    for (transform, vision, entity) in q.iter() {
        let (mx, my, mz) = (
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        );
        let mut seen = Vec::new();
        for (t, id) in things.iter() {
            let (x, y, z) = (t.translation.x, t.translation.y, t.translation.z);
            let r = vision.range;

            if  entity != id
                && x <= mx + r
                && x >= mx - r
                && y <= my + r
                && y >= my - r
                && z <= mz + r
                && y >= mz - r
            {
                if recorces.contains(id){
                    seen.push(Preseved::Food(id))
                }
            } 
        }
        if !seen.is_empty(){
            output.send(PerseptionEvent::TrueSight{
                id: entity,
                seen
            });
        }
    }
}

#[derive(Component)]
pub struct DontView;