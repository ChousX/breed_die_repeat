use bevy::{prelude::Vec3, utils::hashbrown::HashMap};
use ordered_float::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Eq)]
pub struct Vertex([OrderedFloat<f32>; 3]);

#[derive(Default)]
pub struct VertexBank {
    data: HashMap<Vertex, u32>,
    len: usize,
}

impl From<[f32; 3]> for Vertex {
    fn from(v: [f32; 3]) -> Self {
        Self([OrderedFloat(v[0]), OrderedFloat(v[1]), OrderedFloat(v[2])])
    }
}

impl Into<[f32; 3]> for Vertex {
    fn into(self) -> [f32; 3] {
        [self.0[0].into(), self.0[1].into(), self.0[2].into()]
    }
}

impl Into<Vec3> for Vertex {
    fn into(self) -> Vec3 {
        Vec3::new(self.0[0].into(), self.0[1].into(), self.0[2].into())
    }
}

impl VertexBank {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
            len: 0,
        }
    }
    ///inshures you have the id, and returns its index
    pub fn id(&mut self, id: Vertex) -> u32 {
        if let Some(output) = self.data.get(&id) {
            *output
        } else {
            let output = self.len as u32;
            self.data.insert(id, output);
            self.len += 1;
            output
        }
    }
    ///consumes self and returns a properly indexed vec
    pub fn drain(mut self) -> Vec<[f32; 3]> {
        let len = self.len;
        let mut output: Vec<[f32; 3]> = vec![[0., 0., 0.]; len];
        for (v, i) in self.data.drain() {
            output[i as usize] = v.into();
        }
        output
    }
}
