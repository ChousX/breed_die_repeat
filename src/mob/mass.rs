use bevy::prelude::*;
/// Mass max >= min * 2 else it will not be able to bud
#[derive(Component, Default)]
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
