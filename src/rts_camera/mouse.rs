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
    for option in camera_options.iter() {
        if key.pressed(option.drag) || key.just_pressed(option.drag) {
            let mut velocity = Vec3::ZERO;
            for event in motion_event.iter() {
                let mut delta = event.delta;
                if option.invert_drag {
                    delta = delta.neg()
                }
                velocity += Vec3::new(delta.x, 0., delta.y);
            }
            if velocity != Vec3::ZERO {
                output.send(CameraMotionEvent::Move(velocity * option.drag_sensitivity));
            }
            continue;
        }
    }
}

pub fn rotate_camera_mouse(
    key: Res<Input<MouseButton>>,
    camera_options: Query<&RtsMouse>,
    mut motion_event: EventReader<MouseMotion>,
    mut output: EventWriter<CameraMotionEvent>,
) {
    for option in camera_options.iter() {
        if key.pressed(option.rotate) || key.just_pressed(option.rotate) {
            let mut velocity = 0.0;
            for event in motion_event.iter() {
                let mut delta = event.delta;
                if option.invert_drag {
                    delta = delta.neg()
                }
                velocity += delta.x + delta.y;
            }
            if velocity != 0.0 {
                output.send(CameraMotionEvent::Rotate(
                    velocity * option.rotate_sensitivity,
                ));
            }
            continue;
        }
    }
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
            if opt.invert_zoom { scroll = scroll.neg();}
            output.send(CameraMotionEvent::Zoom(scroll))
        }
    }
}

#[derive(Component)]
pub struct RtsMouse {
    pub rotate: MouseButton,
    pub drag: MouseButton,

    pub invert_drag: bool,
    pub invert_zoom: bool,

    pub rotate_sensitivity: f32,
    pub drag_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for RtsMouse {
    fn default() -> Self {
        Self {
            rotate: MouseButton::Middle,
            rotate_sensitivity: std::f32::consts::PI / 10.,
            drag: MouseButton::Right,
            drag_sensitivity: 5.,
            zoom_sensitivity: 5.,
            invert_drag: true,
            invert_zoom: true,
        }
    }
}
