use crate::recorce::{ResorceSpawnEvent, ResorceType};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct SlimePlugin;
impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(metabolism)
            .add_system(death)
            .add_system(metabolism);
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
}

#[derive(Component, Default, Inspectable)]
pub struct Slime;

/// Mass max >= min * 2 else it will not be able to bud
#[derive(Component, Default, Inspectable)]
pub struct Mass {
    pub current: f32,
    pub max: f32,
    pub min: f32,
}

impl Mass {
    pub fn new(current: f32, max: f32, min: f32) -> Self {
        Self { current, max, min }
    }
    //so if it goes below min then its starving
    pub fn loss(&mut self, amount: f32) -> bool {
        self.current -= amount;
        self.current <= self.min
    }

    // if it goes over max then split
    pub fn gain(&mut self, amount: f32) -> bool {
        self.current += amount;
        self.current >= self.max
    }

    //true means dead
    pub fn zero_or_less(&self) -> bool {
        self.current <= 0.0
    }

    pub fn split_count(&self) -> u32 {
        (self.current / self.min).floor() as u32
    }
}

#[derive(Component, Default, Deref, Inspectable)]
pub struct Metabolism(f32);

fn metabolism(
    mut query: Query<(&Metabolism, &mut Mass)>,
    time: Res<Time>,
) {
    for (metabolism, mut mass) in query.iter_mut() {
        if mass.loss(metabolism.0 * time.delta_seconds()) {
            
        }
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

#[derive(Component, Default, Inspectable)]
pub struct Enderance(f32);

#[derive(Component, Default, Inspectable)]
pub struct Speed(f32);