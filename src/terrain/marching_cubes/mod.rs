//         +Y
//      -Z  |
//       \  |
//        \ |(0,0,0)
//-X_______\|/________+X
//          \
//          |\
//          | \+Z
//         -Y
use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use std::collections::VecDeque;

mod vertex;
use vertex::*;

mod chunk;
pub use chunk::*;

mod table;

pub fn chunk_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
){

}

pub fn chunk_culler(
    
){

}