use std::ops::Neg;

use super::CameraMotionEvent;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub fn move_camera_mouse(
    key: Res<Input<MouseButton>>,
    camera_options: Query<&RtsMouse>,
    mut output: EventWriter<CameraMotionEvent>,
    mut motion_event: EventReader<MouseMotion>,
) {
    //todo this works but it really all should be nested in the option loop
    let mut enabled = false;
    let mut invert_drag = false;
    for option in camera_options.iter() {
        invert_drag = option.invert_drag;
        if key.pressed(option.drag) || key.just_pressed(option.drag) {
            enabled = true;
            break;
        }
    }
    if enabled {
        let mut velocity = Vec3::ZERO;
        for event in motion_event.iter() {
            let mut delta = event.delta;
            if invert_drag {
                delta = delta.neg()
            }
            velocity += Vec3::new(delta.x, 0., delta.y);
        }
        if velocity != Vec3::ZERO {
            output.send(CameraMotionEvent::Move(velocity));
        }
    }
}

pub fn rotate_camera_mouse(
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut output: EventWriter<CameraMotionEvent>,
) {
}

pub fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut output: EventWriter<CameraMotionEvent>,
    camera_options: Query<&RtsMouse>,
) {
    for opt in camera_options.iter() {
        let mut scroll = 0.0;
        for ev in scroll_evr.iter() {
            scroll += ev.y * opt.zoom_sensitivity;
        }

        if scroll != 0.0 {
            output.send(CameraMotionEvent::Zoom(scroll))
        }
    }
}

#[derive(Component)]
pub struct RtsMouse {
    pub rotate: MouseButton,
    pub drag: MouseButton,

    pub invert_drag: bool,

    pub rotate_sensitivity: f32,
    pub drag_sensitivity: (f32, f32),
    pub zoom_sensitivity: f32,
}

impl Default for RtsMouse {
    fn default() -> Self {
        Self {
            rotate: MouseButton::Middle,
            rotate_sensitivity: std::f32::consts::PI / 1000.,
            drag: MouseButton::Right,
            drag_sensitivity: (10., std::f32::consts::PI / 1000.),
            zoom_sensitivity: 1.,
            invert_drag: true,
        }
    }
}
