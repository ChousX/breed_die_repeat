use bevy::{prelude::Vec3, utils::hashbrown::HashMap};
use ordered_float::*;

///This is mostly here so I can hash things for the vb
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Eq)]
pub struct Vertex([OrderedFloat<f32>; 3]);

///A data object for making vertex, and index list
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_bank_basic() {
        let mut vb = VertexBank::default();
        let pos = [
            [0.0, 0.0, 0.0], //0
            [1.0, 0.0, 0.0], //1
            [0.0, 0.0, 1.0], //2
            [0.0, 1.0, 0.0], //3
        ];
        let indexs = [
            vb.id(pos[0].into()),
            vb.id(pos[1].into()),
            vb.id(pos[2].into()),
            vb.id(pos[3].into()),
        ];

        for (i, id) in indexs.into_iter().enumerate() {
            assert_eq!(i as u32, id);
        }

        for (i, v) in vb.drain().iter().enumerate() {
            assert_eq!(pos[i], *v);
        }
    }

    #[test]
    fn verex_bank_id() {
        let mut vb = VertexBank::default();
        let pos = [
            [0.0, 0.0, 0.0], //0
            [1.0, 0.0, 0.0], //1
            [0.0, 0.0, 1.0], //2
            [0.0, 1.0, 0.0], //3
        ];

        let indexs = [
            vb.id(pos[0].into()), //0
            vb.id(pos[1].into()), //1
            vb.id(pos[1].into()), //2
            vb.id(pos[2].into()), //3
            vb.id(pos[0].into()), //4
            vb.id(pos[3].into()), //5
            vb.id(pos[1].into()), //6
            vb.id(pos[0].into()), //7
        ];

        assert_eq!(indexs[0], 0);
        assert_eq!(indexs[1], 1);
        assert_eq!(indexs[2], 1);
        assert_eq!(indexs[3], 2);
        assert_eq!(indexs[4], 0);
        assert_eq!(indexs[5], 3);
        assert_eq!(indexs[6], 1);
        assert_eq!(indexs[7], 0);
    }
}
