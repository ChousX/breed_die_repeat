use bevy::prelude::*;

mod marching_cubes;
pub use marching_cubes::*;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .add_system(chunk_spawn)
            .add_system(chunk_update)
            .add_system(chunk_despawn);
    }
}

fn chunk_spawn(
    mut cm: ResMut<ChunkManager>,
    mut commands: Commands,
    loaders: Query<(&Transform, &ChunkLoader)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (loader, range) in loaders.iter() {
        let pos = loader.translation;
        for pos in get_range(chunk_id((pos.x, pos.y, pos.z)), **range).into_iter(){
            if !cm.exists(pos){
                let (entity, pos) = spawn_chunk(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    pos,
                    Chunk::blank(),
                );
                if !cm.add(entity, pos){
                    panic!("chunk manager exists fn is not working")
                }
            }
            
        }
    }
}

fn chunk_update(mut cm: ResMut<ChunkManager>) {}

fn chunk_despawn(mut cm: ResMut<ChunkManager>) {}

fn get_range(pos: (i32, i32, i32), range: u16) -> Vec<(i32, i32, i32)>{
    let range = range as i32;
    let mut output = Vec::new();
    for z in (pos.2 - range)..(pos.2 + range){
        for x in (pos.0 - range)..(pos.0 + range){
            output.push((x, pos.1, z))
        }
    }
    output
}