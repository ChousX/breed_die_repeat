use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
mod cpu;
pub use cpu::*;

mod table;
use table::*;

mod share;
use share::*;

///Normals are the vector that describe the derection a trinagle is facing
type Normal = [f32; 3];
type Uv = Vec2;
type Index = usize;
type Vertex = [f32; 3];
///(x,y,z)
type Size = (usize, usize, usize);

///(x,y,z)
type Pos = (f32, f32, f32);

type Space = [f32; CHUNK_SIZE_TOTALE];

const CHUNK_SIZE: Size = (10, 10, 10);
const CHUNK_SIZE_TOTALE: Index = CHUNK_SIZE.0 * CHUNK_SIZE.1 * CHUNK_SIZE.2;
///          Y
///      ?Z  |
///       \  |
///        \ |(0,0,0)
///-X_______\|/________X
///          \
///          |\
///          | \?Z
///         -Y
const CHUNK_ZERO_ZERO: Pos = (0.0, 0.0, 0.0);
///(min, max)
const SHEAR_RANGE: (f32, f32) = (-1.0, 1.0);
const SHEAR_POINT: f32 = SHEAR_RANGE.0 + SHEAR_RANGE.1;

const CUBE_COUNT: Index = (CHUNK_SIZE.0 - 1) * (CHUNK_SIZE.1 - 1) * (CHUNK_SIZE.2 - 1);

type Cube = [f32; 8];

pub struct Chunk {
    space: Space,
    changed: bool,
    mesh: Option<Handle<Mesh>>,
}

impl Chunk {
    pub fn new(space: Space) -> Self {
        Self {
            space,
            changed: true,
            mesh: None,
        }
    }

    pub fn cube(&self, x: Index, y: Index, z: Index) -> Cube {
        Self::p_cube(&self.space, (x, y, z))
    }

    pub fn cubes(&self) -> Cubes {
        Cubes::from(self)
    }

    pub fn set(&mut self, val: f32, pos: Pos) {
        let (x, y, z) = pos;
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
            to1D(x, y + 1, z ),
        );
        [
            space[v0], space[v1], space[v2], space[v3], space[v4], space[v5], space[v6], space[v7],
        ]
    }
}

impl Chunk{
    pub fn march(&self) -> Mesh{
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normales: Vec<Normal> = Vec::new();
        let _ = self.cubes().map(|cube| {
            let cc = cube_case(&cube, SHEAR_POINT);
            
        });

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normales);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

#[inline]
fn to1D(x: Index, y: Index, z: Index) -> Index {
    x + z * CHUNK_SIZE.0 * CHUNK_SIZE.1 + y * CHUNK_SIZE.0
}

///(index) -> (x, y, z)
#[inline]
fn to3d(mut index: Index) -> (Index, Index, Index) {
    let z = index / (CHUNK_SIZE.0 * CHUNK_SIZE.1);
    index -= z * CHUNK_SIZE.0 * CHUNK_SIZE.1;
    let y = index / CHUNK_SIZE.0;
    let x = index % CHUNK_SIZE.0;
    (x, y, z)
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
        use std::mem::MaybeUninit;

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
    if cube[0] < shear {
        acum |= 1
    }
    if cube[1] < shear {
        acum |= 2
    }
    if cube[2] < shear {
        acum |= 4
    }
    if cube[3] < shear {
        acum |= 8
    }
    if cube[4] < shear {
        acum |= 16
    }
    if cube[5] < shear {
        acum |= 32
    }
    if cube[6] < shear {
        acum |= 64
    }
    if cube[7] < shear {
        acum |= 128
    }
    acum
}

fn vertex_interp(shear: f32, p1: Pos, p2: Pos, valp1: f32, valp2: f32) -> Pos{
    if (shear - valp1).abs() < 0.00001{ 
        p1
    } else
    if (shear - valp2).abs() < 0.00001 {
        p2
    } else
    if (valp1-valp2).abs() < 0.00001{
        p1
    } else {
        let mu = (shear - valp1) / (valp1 - valp2);
        (
            p1.0 + mu * (p2.0 - p1.0),
            p1.1 + mu * (p2.1 - p1.1),
            p1.2 + mu * (p2.2 - p1.2)
        )
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

     pub struct Triangle{
        xyz: [[f32; 3]; 3],
     }

     pub struct GridCell{
        cube: [[f32; 3]; 8],
        val: [f32; 8],
     }

    pub fn polygonise(grid: GridCell, shear: f32, trinagle: &mut Triangle) -> i32{

    }