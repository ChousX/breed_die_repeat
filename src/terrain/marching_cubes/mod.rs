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

