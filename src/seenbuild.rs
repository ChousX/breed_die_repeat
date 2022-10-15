use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{mob::DontView, terrain::Chunk};

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

    camera: bool,
    camera_position: Option<(f32, f32, f32)>,
    camera_looking_at: Option<(Vec3, Vec3)>,
}

impl SeenBuilder {
    fn get_plain_transform(&self) -> Transform {
        let (x, y, z) = self.plane_position.unwrap_or((0.0, 0.0, 0.0));
        Transform::from_xyz(x, y, z)
    }

    fn get_light_pos(&self) -> Transform {
        let (x, y, z) = self.light_position.unwrap_or((4.0, 8.0, 4.0));
        Transform::from_xyz(x, y, z)
    }

    fn get_camera_transform(&self) -> Transform {
        let (x, y, z) = self.camera_position.unwrap_or((-2.0, 2.5, 5.0));
        let (target, up) = self.camera_looking_at.unwrap_or((Vec3::ZERO, Vec3::Y));
        Transform::from_xyz(x, y, z).looking_at(target, up)
    }
}

impl SeenBuilder {
    pub fn add_camera_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.camera_position = Some((x, y, z));
        self
    }

    pub fn add_camera_looking_at(&mut self, target: Vec3, up: Vec3) -> &mut Self {
        self.camera_looking_at = Some((target, up));
        self
    }
    pub fn add_light_intensity(&mut self, intensity: f32) -> &mut Self {
        self.light_intensity = Some(intensity);
        self
    }

    pub fn add_shadows(&mut self, shadows: bool) -> &mut Self {
        self.shadows = shadows;
        self
    }

    pub fn add_light_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.light_position = Some((x, y, z));
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_plain_size(&mut self, size: f32) -> &mut Self {
        self.plane_size = Some(size);
        self
    }

    pub fn add_(&mut self, size: f32) -> &mut Self {
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

    pub fn camera_state(&mut self, state: bool) -> &mut Self {
        self.camera = state;
        self
    }

    pub fn build(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let chunk = Chunk::rng(None);
        //spawining the plain
        commands
            .spawn_bundle(PbrBundle {
                // mesh: meshes.add(Mesh::from(shape::Plane {
                //     size: self.plane_size.unwrap_or(5.0),
                // })),
                mesh: meshes.add(chunk.march()),
                material: materials
                    .add(self.plane_color.unwrap_or(Color::rgb(0.2, 0.2, 0.4)).into()),
                transform: self.get_plain_transform(),
                ..default()
            })
            .insert(DontView);

        //spawning light
        commands
            .spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: self.light_intensity.unwrap_or(15000.0),
                    shadows_enabled: self.shadows,
                    ..default()
                },

                transform: self.get_light_pos(),
                ..default()
            })
            .insert(DontView);

        //spawing camera
        if self.camera {
            commands
                .spawn_bundle(Camera3dBundle {
                    transform: self.get_camera_transform(),
                    ..default()
                })
                .insert(DontView);
        }
    }
}
