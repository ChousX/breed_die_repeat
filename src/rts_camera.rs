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

fn move_camera_keybored(
    keyboard_input: Res<Input<KeyCode>>,
    q: Query<(&RtsCamera, &RtsCameraKeyboard)>,
    event: EventWriter<CameraMotionEvent>,
) {

}
fn move_camera_mouse(    
    mouse_input: Res<Input<MouseButton>>,
    q: Query<(&RtsCamera, &RtsCameraMouse)>,
    event: EventWriter<CameraMotionEvent>,
) {}

fn rotate_camera_mouse() {}
fn rotate_camera_keybored() {}

#[derive(Component)]
pub struct RtsCamera{
    enabled: bool,
}

pub enum CameraMotionEvent {
    Move(),
    Rotate(),
    Zoom(),
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
            CameraMotionEvent::Zoom() => {}
        }
    }
}

type KeyBinding = Box<[KeyCode]>;
#[derive(Component)]
pub struct RtsCameraKeyboard{
    pub forward: KeyBinding,
    pub backward: KeyBinding,
    pub left: KeyBinding,
    pub right: KeyBinding,

    pub rotait_left: KeyBinding,
    pub rotait_right: KeyBinding,

    pub move_sensitivity: (f32, f32),
    pub rotate_sensitivity: f32,
}

impl Default for RtsCameraKeyboard{
    fn default() -> Self {
        Self{
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
pub struct RtsCameraMouse{
    pub rotate: MouseButton,
    pub drag: MouseButton,
    
    pub rotate_sensitivity: f32,
    pub drag_sensitivity: (f32, f32),
    pub zoom_sensitivity: f32,
}

impl Default for RtsCameraMouse{
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