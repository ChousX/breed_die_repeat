use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
mod seenbuild;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(spawn_mob)
        .add_plugin(PlayerPlugin)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    seenbuild::SeenBuilder::new()
    .add_plain_size(1000.0)
    .build(&mut commands, &mut meshes, &mut materials);
}

pub fn spawn_mob(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                rings: 0,
                depth: 0.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.6, 0.4, 0.2).into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        })
        .insert(Mob);
}



#[derive(Component)]
pub struct Mob;
// Breed die repeat
/*

#[derive(Component)]
pub struct Mob;

#[derive(Default)]
pub struct MobBuilder {
    position: Option<(f32, f32, f32)>,
    color: Option<Color>,
    health: Option<u16>,
    speed: Option<f32>,
}

impl MobBuilder {
    fn transform(&self) -> Transform {
        let (x, y, z) = self.position.unwrap_or((0.0, 0.25, 0.0));
        Transform::from_xyz(x, y, z)
    }
}

impl MobBuilder {
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add_health(&mut self, health: u16) -> &mut Self {
        self.health = Some(health);
        self
    }

    pub fn add_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = Some(speed);
        self
    }

    pub fn add_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self{
        self.position = Some((x, y, z));
        self
    }

    pub fn run(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                material: materials.add(self.color.unwrap_or(Color::rgb(0.4, 0.2, 0.2)).into()),
                transform: self.transform(),
                ..default()
            })
            .insert(Mob)
            .id()
    }
}*/
