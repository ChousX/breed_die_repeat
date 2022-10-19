use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use noise::{NoiseFn, OpenSimplex, Seedable};
use std::{collections::HashMap, mem::MaybeUninit};

use super::{
    table::*,
    vertex::{Vertex, VertexBank},
};
type Normal = [f32; 3];
type Uv = Vec2;
type Index = usize;
///(x,y,z)
type Size = (usize, usize, usize);

///(x,y,z)
type Pos = [f32; 3];

type Space = [f32; CHUNK_SIZE_TOTALE];

const CHUNK_SIZE: Size = (10, 10, 10);
const CHUNK_SIZE_TOTALE: Index = CHUNK_SIZE.0 * CHUNK_SIZE.1 * CHUNK_SIZE.2;
///         +Y
///      -Z  |
///       \  |
///        \ |(0,0,0)
///-X_______\|/________+X
///          \
///          |\
///          | \+Z
///         -Y
const CHUNK_ZERO_ZERO: Pos = [0.0, 0.0, 0.0];
///(min, max)
const SHEAR_RANGE: (f32, f32) = (-1.0, 1.0);
const SHEAR_POINT: f32 = SHEAR_RANGE.0 + SHEAR_RANGE.1;

const CUBE_COUNT: Index = (CHUNK_SIZE.0 - 1) * (CHUNK_SIZE.1 - 1) * (CHUNK_SIZE.2 - 1);

type Cube = [f32; 8];

const ISO_DISTANCE: f32 = 1.0;

pub struct Chunk {
    space: Space,
    pub changed: bool,
}

impl Chunk {
    pub fn new(space: Space) -> Self {
        Self {
            space,
            changed: true,
        }
    }

    pub fn cube(&self, x: Index, y: Index, z: Index) -> Cube {
        Self::p_cube(&self.space, (x, y, z))
    }

    pub fn cubes(&self) -> Cubes {
        Cubes::from(self)
    }

    pub fn blank() -> Self {
        let space: Space = [f32::MIN; CHUNK_SIZE_TOTALE];
        Self::new(space)
    }

    pub fn add_plain(&mut self, height: usize) {
        for z in 0..CHUNK_SIZE.2 {
            for x in 0..CHUNK_SIZE.0 {
                self.set(x, height, z, SHEAR_POINT)
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: f32) {
        let mut s_val = &mut self.space[to1D(x, y, z)];
        if *s_val != val {
            *s_val = val;
            self.changed = true;
        }
    }

    pub fn rng(seed: Option<u32>) -> Self {
        let mut noise = OpenSimplex::new(if let Some(seed) = seed { seed } else { 0 });

        let mut space: Space = {
            let mut space: [MaybeUninit<f32>; CHUNK_SIZE_TOTALE] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut index = 0;
            for z in 0..CHUNK_SIZE.2 {
                for y in 0..CHUNK_SIZE.1 {
                    for x in 0..CHUNK_SIZE.0 {
                        space[index] =
                            MaybeUninit::new(noise.get([x as f64, y as f64, z as f64]) as f32);
                        index += 1;
                    }
                }
            }

            unsafe { std::mem::transmute::<_, [f32; CHUNK_SIZE_TOTALE]>(space) }
        };

        Self::new(space)
    }
}

impl Chunk {
    /// ________________________________
    ///|\                               \              4________________5
    ///| \                               \             |\               |\
    ///|  \                               \            | \              | \
    ///|   \                               \           |  \             |  \          
    /// \   \                               \          |   7____________|__6\                 
    ///  \  |\---\---------------------------\        0|___|____________1   |     
    ///   \ | V___\___________________________\        \   |             \  |
    ///     | |    |                          |         \  |              \ |
    ///      \|    |                          |          \ |               \|
    ///       \____|__________________________|           \|3_______________2     
    fn p_cube(space: &[f32; CHUNK_SIZE_TOTALE], zz: (Index, Index, Index)) -> Cube {
        let (x, y, z) = zz;
        let (v0, v1, v2, v3, v4, v5, v6, v7) = (
            to1D(x, y, z + 1),
            to1D(x + 1, y, z + 1),
            to1D(x + 1, y, z),
            to1D(x, y, z),
            to1D(x, y + 1, z + 1),
            to1D(x + 1, y + 1, z + 1),
            to1D(x + 1, y + 1, z),
            to1D(x, y + 1, z),
        );
        [
            space[v0], space[v1], space[v2], space[v3], space[v4], space[v5], space[v6], space[v7],
        ]
    }
}

impl Chunk {
    pub fn march(&self) -> Mesh {
        let mut vb = VertexBank::default();
        let mut indeceis: Vec<u32> = Vec::new();
        let mut normal_list: Vec<([f32; 3], [usize; 3])> = Vec::new();
        let (mut x, mut y, mut z) = (0, 0, 0);
        for (i, cube) in self.cubes().enumerate() {
            let mut edges: [[f32; 3]; 12] = [[0.0000f32; 3]; 12];
            if x == CHUNK_SIZE.0 - 1 {
                x = 0;
                y += 1;
                if y == CHUNK_SIZE.1 - 1 {
                    y = 0;
                    z += 1;
                }
            }
            let cc = cube_case(&cube, SHEAR_POINT) as usize;

            const D: f32 = ISO_DISTANCE;
            let p: [[f32; 3]; 8] = {
                let (x, y, z) = (x as f32 * D, y as f32 * D, z as f32 * D);
                [
                    [x, y, z + D],         //0
                    [x + D, y, z + D],     //1
                    [x + D, y, z],         //2
                    [x + D, y, z],         //3
                    [x, y + D, z + D],     //4
                    [x + D, y + D, z + D], //5
                    [x + D, y + D, z],     //6
                    [x + D, y + D, z],     //7
                ]
            };
            x += 1;
            if EDGE_TABLE[cc] == 0 {}
            if EDGE_TABLE[cc] & 1 != 0 {
                edges[0] = vertex_interp(SHEAR_POINT, p[0], p[1], cube[0], cube[1]);
            }
            if EDGE_TABLE[cc] & 2 != 0 {
                edges[1] = vertex_interp(SHEAR_POINT, p[1], p[2], cube[1], cube[2]);
            }
            if EDGE_TABLE[cc] & 4 != 0 {
                edges[2] = vertex_interp(SHEAR_POINT, p[2], p[3], cube[2], cube[3]);
            }
            if EDGE_TABLE[cc] & 8 != 0 {
                edges[3] = vertex_interp(SHEAR_POINT, p[3], p[0], cube[3], cube[0]);
            }
            if EDGE_TABLE[cc] & 16 != 0 {
                edges[4] = vertex_interp(SHEAR_POINT, p[4], p[5], cube[4], cube[5]);
            }
            if EDGE_TABLE[cc] & 32 != 0 {
                edges[5] = vertex_interp(SHEAR_POINT, p[5], p[6], cube[5], cube[6]);
            }
            if EDGE_TABLE[cc] & 64 != 0 {
                edges[6] = vertex_interp(SHEAR_POINT, p[6], p[7], cube[6], cube[7]);
            }
            if EDGE_TABLE[cc] & 128 != 0 {
                edges[7] = vertex_interp(SHEAR_POINT, p[7], p[4], cube[7], cube[4]);
            }
            if EDGE_TABLE[cc] & 256 != 0 {
                edges[8] = vertex_interp(SHEAR_POINT, p[0], p[4], cube[0], cube[4]);
            }
            if EDGE_TABLE[cc] & 512 != 0 {
                edges[9] = vertex_interp(SHEAR_POINT, p[1], p[5], cube[1], cube[5])
            }
            if EDGE_TABLE[cc] & 1024 != 0 {
                edges[10] = vertex_interp(SHEAR_POINT, p[2], p[6], cube[2], cube[6])
            }
            if EDGE_TABLE[cc] & 2048 != 0 {
                edges[11] = vertex_interp(SHEAR_POINT, p[3], p[7], cube[3], cube[7]);
            }

            let mut i = 0;

            while TRI_TABLE[cc][i] != -1 {
                let p1 = edges[TRI_TABLE[cc][i] as usize].into();
                let p2 = edges[TRI_TABLE[cc][i + 1] as usize].into();
                let p3 = edges[TRI_TABLE[cc][i + 2] as usize].into();
                let i1 = vb.id(p1);
                let i2 = vb.id(p2);
                let i3 = vb.id(p3);
                indeceis.push(i1);
                indeceis.push(i2);
                indeceis.push(i3);
                normal_list.push((
                    surface_normal(p1, p2, p3),
                    [i1 as usize, i2 as usize, i3 as usize],
                ));
                i += 3;
            }
        }

        let positions = vb.drain();

        let mut normals = vec![Vec3::ZERO; positions.len()];
        for (normal, pos) in normal_list.into_iter() {
            let v = Vec3::from(normal);
            normals[pos[0]] += v;
            normals[pos[0]] /= 2.;
            normals[pos[1]] += v;
            normals[pos[1]] /= 2.;
            normals[pos[2]] += v;
            normals[pos[2]] /= 2.;
        }
        let normals: Vec<[f32; 3]> = normals.into_iter().map(|v| [v.x, v.y, v.z]).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indeceis)));
        mesh
    }
}

struct Cubes {
    data: [Cube; CUBE_COUNT],
    index: Index,
}

impl Iterator for Cubes {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < CUBE_COUNT {
            let id = self.index;
            self.index += 1;
            Some(self.data[id])
        } else {
            None
        }
    }
}

impl From<&Chunk> for Cubes {
    fn from(chunk: &Chunk) -> Self {
        let mut cubes: [MaybeUninit<Cube>; CUBE_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut index = 0;
        for z in 0..(CHUNK_SIZE.2 - 1) {
            for y in 0..(CHUNK_SIZE.1 - 1) {
                for x in 0..(CHUNK_SIZE.0 - 1) {
                    cubes[index] = MaybeUninit::new(chunk.cube(x, y, z));
                    index += 1;
                }
            }
        }
        let data = unsafe { std::mem::transmute::<_, [Cube; CUBE_COUNT]>(cubes) };
        Self { data, index: 0 }
    }
}

fn cube_case(cube: &Cube, shear: f32) -> u8 {
    let mut acum = 0u8;
    if cube[0] >= shear {
        acum |= 1
    }
    if cube[1] >= shear {
        acum |= 2
    }
    if cube[2] >= shear {
        acum |= 4
    }
    if cube[3] >= shear {
        acum |= 8
    }
    if cube[4] >= shear {
        acum |= 16
    }
    if cube[5] >= shear {
        acum |= 32
    }
    if cube[6] >= shear {
        acum |= 64
    }
    if cube[7] >= shear {
        acum |= 128
    }
    acum
}

#[inline]
fn vertex_interp(shear: f32, p1: Pos, p2: Pos, valp1: f32, valp2: f32) -> Pos {
    if (shear - valp1).abs() < 0.00001 {
        p1
    } else if (shear - valp2).abs() < 0.00001 {
        p2
    } else if (valp1 - valp2).abs() < 0.00001 {
        p1
    } else {
        let mu = (shear - valp1) / (valp1 - valp2);
        [
            p1[0] + mu * (p2[0] - p1[0]),
            p1[1] + mu * (p2[1] - p1[1]),
            p1[2] + mu * (p2[2] - p1[2]),
        ]
    }
}
/*
mpVector LinearInterp(mp4Vector p1, mp4Vector p2, float value)
{
    if (p2 < p1)
    {
        mp4Vector temp;
        temp = p1;
        p1 = p2;
        p2 = temp;
    }

    mpVector p;
    if(fabs(p1.val - p2.val) > 0.00001)
        p = (mpVector)p1 + ((mpVector)p2 - (mpVector)p1)/(p2.val - p1.val)*(value - p1.val);
    else
        p = (mpVector)p1;
    return p;
}

    bool operator<(const mp4Vector &right) const
    {
        if (x < right.x)
            return true;
        else if (x > right.x)
            return false;

        if (y < right.y)
            return true;
        else if (y > right.y)
            return false;

        if (z < right.z)
            return true;
        else if (z > right.z)
            return false;

        return false;
     } */

#[inline]
fn surface_normal(p1: Vertex, p2: Vertex, p3: Vertex) -> [f32; 3] {
    let p1: Vec3 = p1.into();
    let p2: Vec3 = p2.into();
    let p3: Vec3 = p3.into();

    let u = p2 - p1;
    let v = p3 - p1;
    [
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    ]
}

#[inline]
fn to1D(x: Index, y: Index, z: Index) -> Index {
    x + CHUNK_SIZE.1 * (y + CHUNK_SIZE.1 * z)
}
