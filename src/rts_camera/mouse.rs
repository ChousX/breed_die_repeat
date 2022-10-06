use bevy::prelude::*;
use super::CameraMotionEvent;

pub fn move_camera_mouse(
    mouse_input: Res<Input<MouseButton>>,
    q: Query<&RtsMouse>,
    event: EventWriter<CameraMotionEvent>,
) {
    
}

pub fn rotate_camera_mouse() {
   
}

#[derive(Component)]
pub struct RtsMouse {
    pub rotate: MouseButton,
    pub drag: MouseButton,

    pub rotate_sensitivity: f32,
    pub drag_sensitivity: (f32, f32),
    pub zoom_sensitivity: f32,
}

impl Default for RtsMouse {
    fn default() -> Self {
        Self {
            rotate: MouseButton::Right,
            rotate_sensitivity: std::f32::consts::PI / 1000.,
            drag: MouseButton::Left,
            drag_sensitivity: (1., std::f32::consts::PI / 1000.),
            zoom_sensitivity: 1.,
        }
    }
}