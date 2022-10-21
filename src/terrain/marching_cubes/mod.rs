//         +Y
//      +Z  |
//       \  |
//        \ |(0,0,0)
//-X_______\|/________+X
//          \
//          |\
//          | \-Z
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
    chunks: VecDeque<VecDeque<VecDeque<Option<(Entity, (i32, i32, i32))>>>>,
    last: Option<(usize, usize, usize)>,
    loaded: usize
}

impl ChunkManager{
    ///depricated still works though i think. Bump bump bummmmm
        fn _exists(&self, pos: (i32, i32, i32)) -> bool{
        for z in 0..self.chunks.len(){
            for y in 0..self.chunks[z].len(){
                for x in 0..self.chunks[z][y].len(){
                    if let Some((_, (cx, cy, cz))) = self.chunks[z][y][x]{
                        if pos.0 == cx && pos.1 == cy && pos.2 == cz{
                            return true;
                        } else {

                            let rx = cx - pos.0; 
                            let ry = cy - pos.1;
                            let rz = cz - pos.2; 

                            let x = rx + x as i32;
                            let y = ry + y as i32;
                            let z = rz + z as i32;

                            if z >= 0 && z < self.chunks.len() as i32{
                                if y >= 0 && y < self.chunks[z as usize].len() as i32{
                                    if x >= 0 && x < self.chunks[z as usize][y as usize].len() as i32{
                                        if let Some(_) = self.chunks[z as usize][y as usize][x as usize]{
                                            return true;
                                        } else {
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        false        
    }

    ///wether or not a chunk cell exists with in the manager
    pub fn exists(&self, pos: (i32, i32, i32)) -> bool{
        if let Some(_) = self.index(pos){
            true
        } else {
            false
        }
    }

    ///Get the Entity id if it exists
    pub fn get(&self, pos: (i32, i32, i32)) -> Option<(Entity, (i32, i32, i32))>{
        self.simple_get(self.index(pos)?)
    }

    ///Add an entity to the manager if one already exists do nothing and report the failer
    pub fn add(&mut self, entity: Entity, pos: (i32, i32, i32)) -> bool{
        if self.exists(pos){
            return false;
        }
        
        // Ok we need to actuly add it
        // 1) check if we can just insert it as an x or do we need add y's and z's


        true
    }

    ///Remove any unused z and y vecs
    pub fn cull(&mut self){
        todo!()
    }

    ///remove and return the Entity. if there is nothing there reter None aswell
    pub fn pop(&mut self, pos: (i32, i32, i32)) -> Option<Entity>{
        todo!()
    }

    ///give the number of chunks that are loaded 
    pub fn chunks_loaded(&self) -> usize{
        todo!()
    }
}

impl ChunkManager{
    /// If the chunk at the give pos is in the manager it will return its pos in the chunks
    fn index(&self, pos: (i32 , i32, i32)) -> Option<(usize, usize, usize)>{
        let last = self.last?;
        let (_, (x, y, z)) = self.simple_get(last)?;
        let x = pos.0 - x;
        let y = pos.1 - y;
        let z = pos.2 - z;

        let (lx, ly, lz) = last;

        let (x, y, z) = (lx as i32 + x, ly as i32 + y, lz as i32 + z);

        if x < 0 &&  y < 0 &&  z < 0{
            return None;
        }

        let z_len = self.chunks.len();
        if x < z_len as i32{
            let y_len = self.chunks[z_len].len();
            if y < y_len as i32{
                let x_len = self.chunks[z_len][y_len].len();
                if x < x_len as i32{
                    return Some((x as usize, y as usize, z as usize))
                }
            }
        }
        None
    }

    /// A helper fn for 
    #[inline]
    fn aux_index(&self, pos: (i32 , i32, i32)) -> Option<(i32, i32, i32)>{
        let last = self.last?;
        let (_, (x, y, z)) = self.simple_get(last)?;
        let x = pos.0 - x;
        let y = pos.1 - y;
        let z = pos.2 - z;

        let (lx, ly, lz) = last;

        let (x, y, z) = (lx as i32 + x, ly as i32 + y, lz as i32 + z);
        Some((x, y, z))
    }
    /// internal get operation
    fn simple_get(&self, pos: (usize, usize, usize)) -> Option<(Entity, (i32, i32, i32))>{
        self.chunks[pos.0][pos.1][pos.2]
    }
}

