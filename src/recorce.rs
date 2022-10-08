use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::rapier::crossbeam::epoch::Collector;
use std::{hash::Hasher, time::Duration};

pub struct mResorcePlugin;
impl Plugin for mResorcePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResorceSpawnEvent>()
            .add_system(resorce_spawner)
            .add_system(decay);
    }
}

#[derive(Component)]
pub struct mResorce {
    pub timer: Timer,
    pub amount: f32,
    pub resorce_type: ResorceType,
}

#[derive(Clone, Copy, Inspectable)]
pub enum ResorceType {
    Plant,
    Slime,
}
impl ResorceType{
    pub fn color(&self) ->Color{
        match *self{
            Self::Plant => Color::rgb(0.2, 0.6, 0.3),
            Self::Slime => Color::rgb(0.6, 0.2, 0.8)
        }
    }
}

impl Default for ResorceType {
    fn default() -> Self {
        Self::Slime
    }
}
pub struct ResorceSpawnEvent {
    pub amount: f32,
    pub resorce_type: ResorceType,
    pub position: (f32, f32),
}

impl Default for ResorceSpawnEvent{
    fn default() -> Self {
        Self{
            amount: 10.0,
            resorce_type: ResorceType::Slime,
            position: (0., 0.)
        }
    }
}

const RESORCE_LIFE_TIME_RATE: f32 = 10.0;

fn resorce_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<ResorceSpawnEvent>,
) {
    for event in events.iter() {
        //color should be resorce type dependent
        //position shoudle evenchualy be random aroun a small area
        let size = event.amount * 0.05;
        let translation = Vec3::new(event.position.0, size , event.position.1);
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: size })),
                material: materials.add(event.resorce_type.color().into()),
                transform: Transform::from_translation(translation),
                ..default()
            })
            .insert(mResorce {
                timer: Timer::from_seconds(RESORCE_LIFE_TIME_RATE * event.amount, false),
                amount: event.amount,
                resorce_type: event.resorce_type,
            });
    }
}

fn decay(
    mut commands: Commands,
    mut query: Query<(&mut mResorce, &mut Transform, Entity)>,
    time: Res<Time>,
) {

    for (mut resorce, mut transform, entity) in query.iter_mut() {
        resorce.timer.tick(time.delta());
        
        let quont = time.delta_seconds() * 0.001;
        resorce.amount -= quont;
        if transform.scale.x >= -0.99{
            transform.scale -= Vec3::splat(quont);
        }

        if resorce.timer.finished() {
            commands.entity(entity).despawn_recursive()
        }
    }
}