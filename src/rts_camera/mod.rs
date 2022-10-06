use bevy::prelude::*;
mod keyboard;
mod mouse;
use keyboard::{RtsKeyboard, move_camera_keybored, rotate_camera_keybored};
use mouse::{RtsMouse, move_camera_mouse, rotate_camera_mouse};

pub fn build_camera(commands: &mut Commands, transform: Transform) {
    commands
        .spawn_bundle(RtsCameraRigBundle { ..default() })
        .insert_bundle(Camera3dBundle {
            transform,
            ..default()
        });
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
            .add_event::<CameraMotionEvent>();
    }
}

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
    Rotate(),
    Zoom(),
}

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
            CameraMotionEvent::Rotate() => {
                todo!()
            }
            CameraMotionEvent::Zoom() => {
                todo!()
            }
        }
    }

}






