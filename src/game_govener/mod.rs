use bevy::prelude::*;
use crate::recorce::ResorceSpawnEvent;
use rand::prelude::*;

pub struct GameGovenerPlugin;
impl Plugin for GameGovenerPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ResorceSpawnTimer::default())
            .add_system(spawn_random_recorse);
    }
}

fn test(
    mut output: EventWriter<ResorceSpawnEvent>,
){
    output.send(ResorceSpawnEvent{
        position: (1., 1.),
        ..default()
    });
    output.send(ResorceSpawnEvent{
        position: (2., 1.),
        ..default()
    });
    output.send(ResorceSpawnEvent{
        position: (0., 1.),
        amount: 1.,
        ..default()
    });
}

#[derive(DerefMut, Deref)]
struct ResorceSpawnTimer(Timer);
fn spawn_random_recorse(
    mut timer: ResMut<ResorceSpawnTimer>,
    mut output: EventWriter<ResorceSpawnEvent>,
    time: Res<Time>
){
    timer.tick(time.delta());
    let mut rng = thread_rng();
    if timer.just_finished(){
        //span a recorce
        output.send(ResorceSpawnEvent{
            amount: rng.gen_range((1.0)..(20.0)),
            resorce_type: crate::recorce::ResorceType::Plant,
            position: (
                rng.gen_range((-50.0)..(50.0)),
                rng.gen_range((-50.0)..(50.0)),
                )

        })
    }
}
impl Default for ResorceSpawnTimer{
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, true))
    }
}