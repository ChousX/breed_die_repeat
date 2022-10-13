use crate::mob::*;
use crate::recorce::ResorceSpawnEvent;
use bevy::prelude::*;
use rand::prelude::*;

pub struct GameGovenerPlugin;
impl Plugin for GameGovenerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResorceSpawnTimer::default())
            .add_system(spawn_random_recorse)
            .add_startup_system(spawn_slimes);
    }
}

#[derive(DerefMut, Deref)]
struct ResorceSpawnTimer(Timer);
fn spawn_random_recorse(
    mut timer: ResMut<ResorceSpawnTimer>,
    mut output: EventWriter<ResorceSpawnEvent>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    let mut rng = thread_rng();
    if timer.just_finished() {
        //span a recorce
        output.send(ResorceSpawnEvent {
            amount: rng.gen_range((1.0)..(2.5)),
            resorce_type: crate::recorce::ResorceType::Plant,
            position: (
                rng.gen_range((-50.0)..(50.0)),
                rng.gen_range((-50.0)..(50.0)),
            ),
        })
    }
}
impl Default for ResorceSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, true))
    }
}

fn spawn_slimes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1., 1., 1.),
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                rings: 0,
                ..default()
            })),
            material: materials.add(Color::BLUE.into()),
            ..default()
        })
        .insert_bundle(MobBundle { ..default() });
}
