use super::CHUNK_SIZE;

mod chunk;
pub use chunk::*;

pub fn chunks_for_radius(radius: i32, x: f32, z: f32) -> Vec<(i32, i32, f32)> {
    let mut chunks = Vec::new();
    let center_chunk_x = (x / CHUNK_SIZE as f32).floor() as i32;
    let center_chunk_z = (z / CHUNK_SIZE as f32).floor() as i32;

    for dx in -radius..=radius {
        for dz in -radius..=radius {
            let chunk_x = center_chunk_x + dx;
            let chunk_z = center_chunk_z + dz;
            let distance =
                ((chunk_x - center_chunk_x).pow(2) + (chunk_z - center_chunk_z).pow(2)) as f32;
            chunks.push((chunk_x, chunk_z, distance));
        }
    }

    chunks
}

pub fn chunk_to_global_position(x: i32, z: i32) -> (f32, f32) {
    let x = (x * CHUNK_SIZE as i32) as f32 + CHUNK_SIZE as f32 / 2.0;
    let z = (z * CHUNK_SIZE as i32) as f32 + CHUNK_SIZE as f32 / 2.0;
    (x, z)
}
