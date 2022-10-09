use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod recorce;
mod rts_camera;
mod seenbuild;
mod slime;
mod game_govener;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(recorce::mResorcePlugin)
        .add_plugin(rts_camera::RtsCameraPlugin)
        .add_plugin(game_govener::GameGovenerPlugin)
        .add_startup_system(init)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    seenbuild::SeenBuilder::new()
        .add_plain_size(100.0)
        .camera_state(false)
        .build(&mut commands, &mut meshes, &mut materials);
    rts_camera::build_camera(
        &mut commands,
        Transform::from_xyz(-2.0, 2.5, 5.0).with_rotation(Quat::from_rotation_x(-0.5)),
    )
}
