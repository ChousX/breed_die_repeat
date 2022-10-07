use bevy::prelude::*;
use crate::recorce::ResorceSpawnEvent;

pub struct GameGovenerPlugin;
impl Plugin for GameGovenerPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(test);
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
        quontity: 1.,
        ..default()
    });
}