use std::ops::Neg;

use bevy::prelude::*;

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

fn move_camera_keybored(
    keyboard_input: Res<Input<KeyCode>>,
    q: Query<(&RtsKeyboard)>,
    mut event: EventWriter<CameraMotionEvent>,
) {
    let mut velocity = Vec3::ZERO;
    for options in q.iter() {
        let (x, _) = options.move_sensitivity;
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

fn move_camera_mouse(
    mouse_input: Res<Input<MouseButton>>,
    q: Query<(&RtsCamera, &RtsMouse)>,
    event: EventWriter<CameraMotionEvent>,
) {
}

fn rotate_camera_mouse() {}
fn rotate_camera_keybored() {}

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
    let mut changed = false;
    let (mut camera_options, mut transform) = q.single_mut();
    for event in events.iter() {
        match event {
            CameraMotionEvent::Move(velocity) => {
                camera_options.velosity = add(
                    &camera_options.velosity,
                    *velocity * time.delta_seconds(),
                    camera_options.max_speed,
                );
            }
            CameraMotionEvent::Rotate() => {}
            CameraMotionEvent::Zoom() => {}
        }
    }
    transform.translation += camera_options.velosity;
    camera_options.velosity = sub(
        &camera_options.velosity,
        camera_options.friction * time.delta_seconds(),
    )
}

fn add(current: &Vec3, add: Vec3, max: f32) -> Vec3 {
    Vec3::new(
        if current.x + add.x > max {
            max
        } else {
            current.x + add.x
        },
        if current.y + add.y > max {
            max
        } else {
            current.y + add.y
        },
        if current.z + add.x > max {
            max
        } else {
            current.z + add.z
        },
    )
}
fn sub(current: &Vec3, sub: f32) -> Vec3 {
    Vec3::new(
        if current.x > sub { current.x - sub } else { 0. },
        if current.y > sub { current.y - sub } else { 0. },
        if current.z > sub { current.z - sub } else { 0. },
    )
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

    pub move_sensitivity: (f32, f32),
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

            move_sensitivity: (2.0, 0.1),
            rotate_sensitivity: std::f32::consts::PI / 100.,
        }
    }
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
