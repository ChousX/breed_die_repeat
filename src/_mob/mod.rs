use crate::recorce::{mResorce, ResorceSpawnEvent, ResorceType};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
mod components;
use components::*;
mod perseption;
use perseption::*;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MobMoveEvent>()
            .add_event::<PerseptionEvent>()
            .add_system(metabolism)
            .add_system(death)
            .add_system(metabolism)
            .add_system(sight)
            .add_system(perseption_event_handler)
            .add_system(mob_move);
        //thinky thingy
        //event slime move reader
        //digestion
    }

    fn name(&self) -> &str {
        "SlimePlugin"
    }
}

#[derive(Bundle, Default)]
pub struct SlimeBundle {
    pub mass: Mass,
    pub metabolism: Metabolism,
    pub enderance: Enderance,
    pub speed: Speed,
    pub mob: Mob,
    pub thinking_bits: ThinkingBits,
    pub vition: Vition,
}

#[derive(Component, Default, Inspectable)]
pub struct Mob;




fn death(
    mut commands: Commands,
    query: Query<(&Mass, Entity, &Transform)>,
    mut event: EventWriter<ResorceSpawnEvent>,
) {
    for (mass, entity, transform) in query.iter() {
        if mass.zero_or_less() {
            commands.entity(entity).despawn_recursive();
            event.send(ResorceSpawnEvent {
                amount: mass.min,
                resorce_type: ResorceType::Slime,
                position: (transform.translation.x, transform.translation.z),
            })
        }
    }
}

pub struct MobMoveEvent(Entity, Vec3);

pub enum MovePlan {
    Avoid(Entity),
    Ingaje(Entity),
    MoveToSpawt(f32, f32),
    MovetoEntity(Entity),
}

#[derive(Component, Default)]
pub struct ThinkingBits {
    move_plan: Vec<MovePlan>,
}

fn mob_move(
    mut query: Query<(&mut Transform), With<Mob>>,
    mut event: EventReader<MobMoveEvent>,
) {
    for MobMoveEvent(entity, amount) in event.iter() {
        if let Ok(mut transform) = query.get_component_mut::<Transform>(*entity) {
            transform.translation += *amount;
        }
    }
}
