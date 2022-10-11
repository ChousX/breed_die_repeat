use crate::recorce::{ResorceSpawnEvent, ResorceType, mResorce};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct SlimePlugin;
impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SlimeMoveEvent>()
            .add_event::<PerseptionEvent>()
            .add_system(metabolism)
            .add_system(death)
            .add_system(metabolism)
            .add_system(slime_move)
            ;
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
    pub vition: Vition
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

pub struct SlimeMoveEvent(Entity, Vec3);

pub enum MovePlan{
    Avoid(Entity),
    Ingaje(Entity),
    MoveToSpawt(f32, f32),
    MovetoEntity(Entity) 
}

#[derive(Component, Default)]
pub struct ThinkingBits{
    move_plan: Vec<MovePlan>
}

#[derive(Component)]
pub struct Vition(f32);
impl Default for Vition{
    fn default() -> Self {
        Self(10.0)
    }
}

fn sight(
    mut vition: Query<(&Vition, &Transform, Entity)>,
    mut all: Query<(&Transform, Entity)>,
    mut output: EventWriter<PerseptionEvent>
){
    for (Vition(range), transform, entity) in vition.iter(){
        let (x, z) = (transform.translation.x, transform.translation.z);
        let grater: (f32, f32) = (x + range,  z + range);
        let lesser: (f32, f32) = (x - range,  z - range);
        //this is really bad no good no good must fix 
        //fix chunk entitys and only have to look at most 4 chunks
        //chunks should be of a size to reduse the change of 4 way chunk but also should be of a practigle size
        //the golden ration may work(may also be solved)
        let mut seen = Vec::new();
        let _ = all.iter().map(|(tran, e)|{
            let (tx, tz) = (tran.translation.x, tran.translation.z);
            //this is a box witch is fine but not a cirle witch would be more intutive
            if entity != e && tx <= grater.0 && tx >= lesser.0 && tz <= grater.1 && tz >= lesser.1{
                let tag_type = if let Ok(_) =all.get_component::<mResorce>(e){
                    TagType::Food
                } else {
                    TagType::Danger
                };
                seen.push((e, ));
            }
        });
        output.send(PerseptionEvent::TrueSight(seen, entity))
    }
}

pub enum TagType{
    Food,
    Danger,
}

pub enum PerseptionEvent{
    //add 
    TrueSight(Vec<(Entity, TagType)>, Entity),

}

fn perseption_event_handler(
    mut events: EventReader<PerseptionEvent>,
    mut output: EventWriter<SlimeMoveEvent>,
    mut speeds: Query<&Speed>,
    
){
    //so I would like to add in some entity matching but this is a todo
    for PerseptionEvent::TrueSight(things, entity) in events.iter(){
        let speed = if let Ok(Speed(speed)) = speeds.get_component::<Speed>(*entity){
            *speed
        } else {
            0.0
        };
       
        for (e, TagType::Food) in things.iter(){
            
            break;
        } 
    }
}

fn slime_move(
    mut query: Query<(&mut Transform), With<Slime>>,
    mut event: EventReader<SlimeMoveEvent>
){
    for SlimeMoveEvent(entity, amount) in event.iter(){
        if let Ok(mut transform) =query.get_component_mut::<Transform>(*entity){
            transform.translation += *amount;
        }
    }
}