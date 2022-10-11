use crate::recorce::{mResorce, ResorceSpawnEvent, ResorceType};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
mod components;
use components::*;
mod perseption;
use perseption::*;

pub struct SlimePlugin;
impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SlimeMoveEvent>()
            .add_event::<PerseptionEvent>()
            .add_system(metabolism)
            .add_system(death)
            .add_system(metabolism)
            .add_system(sight)
            .add_system(perseption_event_handler)
            .add_system(slime_move);
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
    pub slime: Slime,
    pub thinking_bits: ThinkingBits,
    pub vition: Vition,
}

#[derive(Component, Default, Inspectable)]
pub struct Slime;


#[derive(Component, Default, Deref, Inspectable)]
pub struct Metabolism(f32);

fn metabolism(mut query: Query<(&Metabolism, &mut Mass)>, time: Res<Time>) {
    for (metabolism, mut mass) in query.iter_mut() {
        if mass.loss(metabolism.0 * time.delta_seconds()) {}
    }
}

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

pub struct SlimeMoveEvent(Entity, Vec3);

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

fn slime_move(
    mut query: Query<(&mut Transform), With<Slime>>,
    mut event: EventReader<SlimeMoveEvent>,
) {
    for SlimeMoveEvent(entity, amount) in event.iter() {
        if let Ok(mut transform) = query.get_component_mut::<Transform>(*entity) {
            transform.translation += *amount;
        }
    }
}

