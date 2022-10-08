use super::CameraMotionEvent;
use bevy::prelude::*;
pub fn move_camera_keybored(
    keyboard_input: Res<Input<KeyCode>>,
    q: Query<&RtsKeyboard>,
    mut event: EventWriter<CameraMotionEvent>,
) {
    let mut velocity = Vec3::ZERO;
    for options in q.iter() {
        let x = options.move_sensitivity;
        if let Some(count) = pressed(&options.forward, &keyboard_input) {
            velocity.z -= count as f32 * x;
        }
        if let Some(count) = pressed(&options.backward, &keyboard_input) {
            velocity.z += count as f32 * x;
        }

        if let Some(count) = pressed(&options.left, &keyboard_input) {
            velocity.x -= count as f32 * x;
        }
        if let Some(count) = pressed(&options.right, &keyboard_input) {
            velocity.x += count as f32 * x;
        }
    }

    if velocity != Vec3::ZERO {
        event.send(CameraMotionEvent::Move(velocity));
    }
}

pub fn rotate_camera_keybored(
    keyboard_input: Res<Input<KeyCode>>,
    q: Query<&RtsKeyboard>,
    mut event: EventWriter<CameraMotionEvent>,
) {
    for options in q.iter(){
        let mut rotation = 0.0;
        let sensitivity = options.rotate_sensitivity;
        if let Some(count) = pressed(&options.rotait_left, &keyboard_input) {
            rotation -= count as f32 * sensitivity;
        }
        if let Some(count) = pressed(&options.rotait_right, &keyboard_input) {
            rotation += count as f32 * sensitivity;
        }
        if rotation != 0.0 {
            event.send(CameraMotionEvent::Rotate(rotation));
        }
    }
}

type KeyBinding = Box<[KeyCode]>;
fn pressed(binding: &KeyBinding, input: &Res<Input<KeyCode>>) -> Option<u8> {
    let mut acum = 0u8;
    for key in binding.iter() {
        if input.pressed(*key) {
            acum += 1;
        }
    }
    if acum > 0 {
        Some(acum)
    } else {
        None
    }
}

#[derive(Component)]
pub struct RtsKeyboard {
    pub forward: KeyBinding,
    pub backward: KeyBinding,
    pub left: KeyBinding,
    pub right: KeyBinding,

    pub rotait_left: KeyBinding,
    pub rotait_right: KeyBinding,

    pub move_sensitivity: f32,
    pub rotate_sensitivity: f32,
}

impl Default for RtsKeyboard {
    fn default() -> Self {
        Self {
            forward: Box::new([KeyCode::W, KeyCode::Up]),
            backward: Box::new([KeyCode::S, KeyCode::Down]),
            left: Box::new([KeyCode::A, KeyCode::Left]),
            right: Box::new([KeyCode::D, KeyCode::Right]),

            rotait_left: Box::new([KeyCode::Q]),
            rotait_right: Box::new([KeyCode::E]),

            move_sensitivity: 5.0,
            rotate_sensitivity: 5.0,
        }
    }
}
