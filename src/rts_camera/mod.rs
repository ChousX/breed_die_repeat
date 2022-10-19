use std::ops::Neg;

use bevy::prelude::*;
mod keyboard;
mod mouse;
use crate::mob::DontView;
use keyboard::{move_camera_keybored, rotate_camera_keybored, RtsKeyboard};
use mouse::{move_camera_mouse, rotate_camera_mouse, zoom_camera, RtsMouse};

pub fn build_camera(commands: &mut Commands, transform: Transform) {
    commands
        .spawn_bundle(RtsCameraRigBundle { ..default() })
        .insert_bundle(Camera3dBundle {
            transform,
            ..default()
        })
        .insert(DontView);
}

#[derive(Bundle, Default)]
pub struct RtsCameraRigBundle {
    pub camera: RtsCamera,
    pub keyboard: RtsKeyboard,
    pub mouse: RtsMouse,
}
pub struct RtsCameraPlugin;

impl Plugin for RtsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_camera_keybored)
            .add_system(move_camera_mouse)
            .add_system(rotate_camera_mouse)
            .add_system(rotate_camera_keybored)
            .add_system(camera_motion)
            .add_system(zoom_camera)
            .add_event::<CameraMotionEvent>();
    }
}

//todo finish adding some slide speed
#[derive(Component)]
pub struct RtsCamera {
    pub enabled: bool,
    pub velosity: Vec3,
    pub friction: f32,
    pub max_speed: f32,
}

impl Default for RtsCamera {
    fn default() -> Self {
        Self {
            enabled: true,
            velosity: Vec3::ZERO,
            friction: 0.2,
            max_speed: 30.0,
        }
    }
}

pub enum CameraMotionEvent {
    Move(Vec3),
    Rotate(f32),
    Zoom(f32),
}
//TODO: need to apply the rotation  to the movement
fn camera_motion(
    mut events: EventReader<CameraMotionEvent>,
    mut q: Query<(&mut RtsCamera, &mut Transform)>,
    time: Res<Time>,
) {
    let (mut camera_options, mut transform) = q.single_mut();
    for event in events.iter() {
        match event {
            CameraMotionEvent::Move(velocity) => {
                transform.translation += *velocity * time.delta_seconds();
                //I would like to add a percistent velocity with a claped to a max value and decay over time
            }

            CameraMotionEvent::Rotate(angle) => transform.rotate_y(*angle * time.delta_seconds()),
            CameraMotionEvent::Zoom(zoom) => {
                let zoom = *zoom * time.delta_seconds();
                transform.translation.y += zoom;
                transform.translation.z += zoom;
            }
        }
    }
}
