use bevy::prelude::*;
pub struct MobMoveEvent {
    direction: Vec3,
    id: Entity,
}

impl MobMoveEvent {
    pub fn new(direction: Vec3, speed: f32, id: Entity) -> Self {
        let direction = direction * speed;
        Self { direction, id }
    }
}

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
    pub max: f32,
    pub momentum: Vec3,
}

impl Speed {
    pub fn new(speed: f32, max: f32) -> Self {
        Self {
            speed,
            max,
            momentum: Vec3::ZERO,
        }
    }

    pub fn add(&mut self, direction: Vec3) {
        self.momentum += direction * self.speed;
        self.momentum.y = 0.0;
        //x
        if self.momentum.x.abs() > self.max {
            self.momentum.x = self.max
        } else if self.momentum.x.abs() > self.max {
            self.momentum.x = -self.max
        }
        //y
        // if self.momentum.y.abs() > self.max {
        //     self.momentum.y = self.max
        // } else if self.momentum.y.abs() > self.max {
        //     self.momentum.y = -self.max
        // }
        //z
        if self.momentum.z > self.max {
            self.momentum.z = self.max
        } else if self.momentum.z.abs() > self.max {
            self.momentum.z = -self.max
        }
    }

    pub fn get(&self) -> Vec3 {
        self.momentum
    }

    pub fn stop(&mut self) {
        self.momentum = Vec3::ZERO;
    }

    pub fn friction(&mut self, value: f32) {
        let (x, _y, z) = (self.momentum.x, self.momentum.y, self.momentum.z);
        //x
        if x.abs() - value < 0.0 {
            self.momentum.x = 0.0;
        } else {
            self.momentum.x -= value;
        }
        // //y
        // if y.abs() - value < 0.0 {
        //     self.momentum.y = 0.0;
        // } else {
        //     self.momentum.y -= value;
        // }
        //z
        if z.abs() - value < 0.0 {
            self.momentum.z = 0.0;
        } else {
            self.momentum.z -= value;
        }
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self::new(2.0, 7.0)
    }
}

pub fn mob_move(
    mut events: EventReader<MobMoveEvent>,
    mut entities: Query<(&mut Transform, &mut Speed)>,
    time: Res<Time>,
) {
    for event in events.iter() {
        if let Ok((mut transform, mut speed)) = entities.get_mut(event.id) {
            speed.add(event.direction * time.delta_seconds());
            transform.translation += speed.get() * time.delta_seconds();
            speed.friction(1.25 * time.delta_seconds());
        }
    }
}
