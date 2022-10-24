use bevy::prelude::*;

mod marching_cubes;
pub use marching_cubes::*;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChunkManager>()
            .add_system(system)
            ;
    }
}

#[derive(Bundle)]
pub struct TerrainBundle {}

#[derive(Component)]
pub struct Terrain;
