use bevy::{prelude::*};

#[derive(Bundle)]
pub struct SlimeBundle {
    mass: Mass,
    metabolism: Metabolism,
    enderance: Enderance,
    speed: Speed,
    slime: Slime,
}

#[derive(Component, Default)]
pub struct Slime {
    pub starving: bool,
}

/// Mass max >= min * 2 else it will not be able to bud
#[derive(Component, Default)]
pub struct Mass {
    current: f32,
    max: f32,
    min: f32,
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

#[derive(Component, Default, Deref)]
pub struct Metabolism(f32);

fn metabolism(mut query: Query<(&Metabolism, &mut Mass, &mut Slime)>, time: Res<Time>) {
    for (metabolism, mut mass, mut slime) in query.iter_mut() {
        if mass.loss(metabolism.0 * time.delta_seconds()) {
            slime.starving = true;
        }
    }
}

fn despawn(commands: &mut Commands, entitiy: Entity){
    commands.entity(entity).despawn_recursive();
}

fn death(
    mut commands: Commands,
    mut query: Query<(&Mass, Entity, &Transform)>,
    mut event: EventWriter<ResorceSpawnEvent>,
){
    for (mass, entity, transform) in query.iter(){
        if mass.zero_or_less(){
            despawn(&mut commands, entitiy);
            event.send(ResorceSpawnEvent{
                quontity: 10.0,
                resorce_type: ResorceType::Slime,
                position: transform.translation
            })
        }
    }
}



#[derive(Component, Default)]
pub struct Enderance(f32);

#[derive(Component, Default)]
pub struct Speed(f32);
