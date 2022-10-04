// I used bevy_fly_camera as refrance for this 
// [https://github.com/mcpar-land/bevy_fly_camera]
use bevy::prelude::*;

/// Requres DefaultPlugins
pub struct CameraPlugin{}
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        
    }

    fn name(&self) -> &str {
        "FlyingCameraPlugin"
    }
}

#[derive(Component)]
struct MainCamera{
    pub enabled: bool,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_backward: KeyCode,
    pub key_forward: KeyCode,
}

impl Default for MainCamera{
    fn default() -> Self {
        Self { 
            enabled: true,

            //keys
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::W,
            key_down: KeyCode::S,
            key_backward: KeyCode::RShift,
            key_forward: KeyCode::RControl
        }
    }
} 

fn make(mut commands: Commands){
    commands
        .spawn_bundle(Camera3dBundle{
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
}

fn move_camera_position(
    time: Res<Time>,
    mut query: Query<(&mut MainCamera, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
)
{
    
    let (options, mut transform) = query.get_single_mut().unwrap();
    let (axis_h, axis_v, axis_float) = if options.enabled{
        (
            movement_axis(&keyboard_input, options.key_right, options.key_left),
            movement_axis(&keyboard_input, options.key_backward, options.key_forward),
            movement_axis(&keyboard_input, options.key_up, options.key_down),
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let rotation = transform.rotation;
    let acceleration: Vec3 = ()

    
}

fn movement_axis(
    input: &Res<Input<KeyCode>>,
    plus: KeyCode,
    minus: KeyCode
) -> f32
{
    let mut axis = 0.0;
	if input.pressed(plus) {
		axis += 1.0;
	}
    if input.pressed(minus) {
		axis -= 1.0;
	}
	axis
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
	// Rotate it 90 degrees to get the strafe direction
	Quat::from_rotation_y(90.0f32.to_radians())
		.mul_vec3(forward_walk_vector(rotation))
		.normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
	let f = forward_vector(rotation);
	let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
	f_flattened
}

fn forward_vector(rotation: &Quat) -> Vec3 {
	rotation.mul_vec3(Vec3::Z).normalize()
}