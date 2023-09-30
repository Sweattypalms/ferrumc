pub mod blocks;
pub mod nbtstructs;

use fastanvil::{complete::Chunk, CurrentJavaChunk, Region};
use std::io::BufReader;

pub async fn get_chunk(x: usize, y: usize) -> Result<Chunk, Box<dyn std::error::Error>> {
    let worlddir = std::env::var("WORLD_DIR").unwrap();
    let file = std::fs::File::open(format!("{}\\region\\r.0.0.mca", worlddir)).unwrap();
    let buf = BufReader::new(file);
    let mut region = Region::from_stream(buf).unwrap();
    let chunk_binary = region.read_chunk(x, y).unwrap();
    if chunk_binary.is_some() {
        let chunk: CurrentJavaChunk = fastnbt::from_bytes(&chunk_binary.unwrap()).unwrap();
        return Ok(Chunk::from(chunk));
    }
    Err("Chunk not found".into())
}