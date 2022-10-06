use bevy::prelude::*;

pub fn build_camera() {}

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
pub struct RtsCamera {}

fn move_camera_keybored() {}
fn move_camera_mouse() {}

fn rotate_camera_mouse() {}
fn rotate_camera_keybored() {}

pub enum CameraMotionEvent {
    Move(),
    Rotate(),
}

fn camera_motion(
    mut events: EventReader<CameraMotionEvent>,
    mut q: Query<(&RtsCamera, &mut Transform)>,
) {
    let (camera_options, mut transform) = q.single_mut();
    for event in events.iter() {
        match event {
            CameraMotionEvent::Move() => {}
            CameraMotionEvent::Rotate() => {}
        }
    }
}
