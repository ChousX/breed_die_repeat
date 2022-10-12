use super::PerseptionEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct Vision {
    range: f32,
}

impl Default for Vision {
    fn default() -> Self {
        Self { range: 25.0 }
    }
}
