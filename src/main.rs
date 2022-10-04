use bevy::prelude::*;
mod seenbuild;
mod camera;

fn main() {
    App::new()
        .add_startup_system(init)
        .add_plugins(DefaultPlugins)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    seenbuild::SeenBuilder::new().build(&mut commands, &mut meshes, &mut materials);
    MobBuilder::new().run(&mut commands, &mut meshes, &mut materials);
}

// Breed die repeat
//

#[derive(Component)]
pub struct Mob;

#[derive(Default)]
pub struct MobBuilder {
    position: Option<(f32, f32)>,
    color: Option<Color>,
    health: Option<u16>,
    speed: Option<f32>,
}

impl MobBuilder {
    fn transform(&self) -> Transform {
        let (x, y) = self.position.unwrap_or((0.0, 0.25));
        Transform::from_xyz(x, y, 0.0)
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

    pub fn run(
        self,
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
}

#[cfg(test)]
mod tests {}
