use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;

pub struct mResorcePlugin;
impl Plugin for mResorcePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResorceSpawnEvent>()
            .add_event::<ResorceDespawnEvent>()
            .add_system(resorce_spawner)
            .add_system(resorce_despawner)
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

impl Default for ResorceType {
    fn default() -> Self {
        Self::Slime
    }
}
impl ResorceType{
    pub fn color(&self) -> Color{
        match *self{
            ResorceType::Plant => Color::rgb(0.2, 0.6, 0.2),
            ResorceType::Slime => Color::rgb(0.3, 0.1, 0.6),
        }
    }
}
pub struct ResorceSpawnEvent {
    pub amount: f32,
    pub resorce_type: ResorceType,
    pub position: Vec3,
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

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.01 })),
                material: materials.add(event.resorce_type.color().into()),
                transform: Transform::from_translation(event.position),
                ..default()
            })
            .insert(mResorce {
                timer: Timer::from_seconds(RESORCE_LIFE_TIME_RATE * event.amount, false),
                amount: event.amount,
                resorce_type: event.resorce_type,
            });
    }
}

#[derive(Deref)]
struct ResorceDespawnEvent(Entity);

fn resorce_despawner(mut commands: Commands, mut events: EventReader<ResorceDespawnEvent>) {
    for entity in events.iter() {
        commands.entity(**entity).despawn_recursive();
    }
}

fn decay(
    mut query: Query<(&mut mResorce, Entity)>,
    time: Res<Time>,
    mut events: EventWriter<ResorceDespawnEvent>,
) {
    for (mut resorce, entitiy) in query.iter_mut() {
        resorce.timer.tick(time.delta());
        if resorce.timer.finished() {
            events.send(ResorceDespawnEvent(entitiy));
        }
    }
}
