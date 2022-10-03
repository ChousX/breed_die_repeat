use bevy::prelude::*;

fn main() {}

// Breed die repeat
//

pub struct Mob;

#[derive(Default)]
pub struct MobBuilder {
    position: Option<(f32, f32)>,
    size: Option<f32>,
}

impl MobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.position = Some((x, y));
        self
    }

    pub fn add_size(&mut self, size: f32) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn run(self, commands: &mut Commands) -> Entity {
        todo!()
    }
}

/// for prototyping
/// Requires: DefaultPlugins
/// will spawn a plain a light and a camera
#[derive(Default)]
pub struct SeenBuilder {
    plane_size: Option<f32>,
    plane_position: Option<(f32, f32, f32)>, // todo
    plane_color: Option<Color>,

    light_intensity: Option<f32>,
    shadows: bool,
    light_position: Option<(f32, f32, f32)>,

    camera_position: Option<(f32, f32, f32)>,
    camera_looking_at: Option<(Vec3, Vec3)>,
}

impl SeenBuilder {
    fn get_camera_transform(&self) -> Transform {
        let (x, y, z) = self.camera_position.unwrap_or((-2.0, 2.5, 5.0));
        let (target, up) = self.camera_looking_at.unwrap_or((Vec3::ZERO, Vec3::Y));
        Transform::from_xyz(x, y, z).looking_at(target, up)
    }

    fn get_light_pos(&self) -> Transform {
        let (x, y, z) = self.light_position.unwrap_or((4.0, 8.0, 4.0));
        Transform::from_xyz(x, y, z)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_size(&mut self, size: f32) -> &mut Self {
        self.plane_size = Some(size);
        self
    }

    pub fn add_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.plane_position = Some((x, y, z));
        self
    }

    pub fn add_color(&mut self, red: f32, green: f32, blue: f32) -> &mut Self {
        self.plane_color = Some(Color::rgb(red, green, blue));
        self
    }

    pub fn build(
        self,
        commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: self.plane_size.unwrap_or(5.0),
            })),
            material: materials.add(self.plane_color.unwrap_or(Color::rgb(0.2, 0.2, 0.4)).into()),
            ..default()
        });

        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 15000.0,
                shadows_enabled: true,
                ..default()
            },

            transform: self.get_light_pos(),
            ..default()
        });
    }
}

#[cfg(test)]
mod tests {}
