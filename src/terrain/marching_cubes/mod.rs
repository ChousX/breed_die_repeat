//         +Y
//      -Z  |
//       \  |
//        \ |(0,0,0)
//-X_______\|/________+X
//          \
//          |\
//          | \+Z
//         -Y
use bevy::{prelude::*, transform};
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use std::collections::VecDeque;

mod vertex;
use vertex::*;

mod chunk;
pub use chunk::*;

mod table;
#[derive(Component)]
pub struct ChunkLoader(u16);

pub struct ChunkManager{
    chunks: VecDeque<VecDeque<VecDeque<Option<(Entity, (i32, i32, i32))>>>>
}

impl ChunkManager{
    pub fn exists(&self, pos: (i32, i32, i32)) -> bool{
        for z in 0..self.chunks.len(){
            for y in 0..self.chunks[z].len(){
                for x in 0..self.chunks[z][y].len(){
                    if let Some((_, (cx, cy, cz)) = self.chunks[z][y][x]{
                        if pos.0 == cx && pos.1 == cy && pos.2 == cz{
                            return true;
                        } else {
                            
                        }
                    }
                }
            }
        }        
    }
}

