use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod recorce;
mod rts_camera;
mod seenbuild;
mod slime;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(recorce::ResorcePlugin)
        .add_plugin(WorldInspectorPlugin::new())
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
        .camera_state(true)
        .build(&mut commands, &mut meshes, &mut materials);
}
