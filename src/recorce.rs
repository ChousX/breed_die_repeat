use bevy::prelude::*;
use std::{time::Duration, hash::Hasher};
pub struct ResorcePlugin;
impl Plugin for ResorcePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResorceSpawnEvent>()
            .add_event::<ResorceDespawnEvent>()
            .add_system(resorce_spawner)
            .add_system(resorce_despawner)
            .add_system(decay)
        ;
    }
}

#[derive(Component)]
pub struct mResorce{
    pub timer: Timer,
    pub amount: f32,
    pub resorce_type: ResorceType,
}
pub enum ResorceType{
    Plant,
    Slime
}

impl Default for ResorceType{
    fn default() -> Self {
        Self::Slime
    }
}
pub struct ResorceSpawnEvent{
    pub quontity: f32,
    pub resorce_type: ResorceType,
    pub position: Vec3,
}

fn resorce_spawner(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<ResorceSpawnEvent>,
){
    for event in events{
        //color should be resorce type dependent
        //position shoudle evenchualy be random aroun a small area
        //TODO: add a despawn timer
        commands.spawn_bundle(PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Cube{ size: 0.01})),
            material: materials.add(Color::rgb(0.0, 0.2, 0.6)),
            transform: Transform::from_translation(position),
            ..default()
        })
        .insert(mResorce{
            timer: Timer::from_seconds(Duration::from_secs((10.0 * event.quontity).floor() as u64), false),
            amount: event.quontity,
            resorce_type: event.resorce_type
        });
    }
}

#[derive(Deref)]
struct ResorceDespawnEvent(Entity);

fn resorce_despawner(
    mut commands: Commands,
    mut events: EventReader<ResorceDespawnEvent>
){
    for event in events{
        let entitiy: Entity = event;
        commands.entity(entity).despawn_recursive();
    }
}

fn decay(
    mut query: Query<&mut mResorce, Entity>,
    time: Res<Time>,
    mut events: EventWriter<ResorceDespawnEvent>,
){
    for (resorce, entitiy) in query{
        resorce.timer.tick(time.delta());
        if resorce.timer.finish(){
            events.send(ResorceDespawnEvent(entitiy));
        }
    }
}