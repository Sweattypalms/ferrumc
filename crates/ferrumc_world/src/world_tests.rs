use crate::nbtstructs;
use crate::nbtstructs::{Chunk, SeriableRegion};
use bytes::{Buf, Bytes};
use fastanvil::Region;
use fastnbt::DeOpts;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::io::BufReader;

macro_rules! human_readable_filesize {
    ($bytes:expr) => {{
        const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
        let mut bytes = $bytes as f64;
        let mut index = 0;
        while bytes >= 1024.0 && index < UNITS.len() - 1 {
            bytes /= 1024.0;
            index += 1;
        }

        format!("{:.2} {}", bytes as f64, UNITS[index])
    }};
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct GenericData {
    #[serde(flatten, rename = "Data")]
    data: HashMap<String, Value>,
}
#[test]
fn test_world() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\level.dat").unwrap();
    let bufreader = BufReader::new(file);
    let reader = flate2::read::MultiGzDecoder::new(bufreader);
    let _: nbtstructs::WorldData = fastnbt::from_reader(reader).unwrap();
}

#[test]
fn load_region() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\region\\r.0.0.mca")
        .unwrap();
    let mut region = Region::from_stream(file).unwrap();
    for chunk in region.iter().flatten() {
        println!("Chunk {}, {} loaded", chunk.x, chunk.z);
    }
}

#[test]
pub fn read_chunks() {
    let start = std::time::Instant::now();
    let mut last = std::time::Instant::now();
    let worlddir = std::env::var("WORLD_DIR").unwrap();
    let file = std::fs::File::open(format!("{}\\region\\r.0.0.mca", worlddir)).unwrap();
    let buf = BufReader::new(file);
    println!("File load time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let mut region = Region::from_stream(buf).unwrap();
    let mut chunks: Vec<Chunk> = Vec::new();
    for chunk in region.iter().flatten() {
        chunks.push(fastnbt::from_bytes(&chunk.data).unwrap());
    }
    println!("Decode time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let serial_region = SeriableRegion { chunks };
    let mut serial_buffer = flexbuffers::FlexbufferSerializer::new();
    serial_region.serialize(&mut serial_buffer).unwrap();
    println!("Serialize time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let compressed = lz4_flex::compress(&serial_buffer.view());
    println!(
        "Compressed size: {}",
        human_readable_filesize!(compressed.len())
    );
    println!(
        "Uncompressed size: {}",
        human_readable_filesize!(serial_buffer.take_buffer().len())
    );
    println!("Compress time: {:?}", last.elapsed());
    println!("Time taken: {:?}", start.elapsed());
}

#[test]
fn write_chunks() {
    let start = std::time::Instant::now();
    let mut last = std::time::Instant::now();
    let worlddir = std::env::var("WORLD_DIR").unwrap();
    let file = std::fs::File::open(format!("{}\\region\\r.0.0.mca", worlddir)).unwrap();
    let buf = BufReader::new(file);
    println!("File load time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let mut region = Region::from_stream(buf).unwrap();
    let mut chunks: Vec<Chunk> = Vec::new();
    for chunk in region.iter().flatten() {
        chunks.push(fastnbt::from_reader((&*chunk.data).reader()).unwrap());
    }
    println!("Decode time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let serial_region = SeriableRegion { chunks };
    let mut serial_buffer = flexbuffers::FlexbufferSerializer::new();
    serial_region.serialize(&mut serial_buffer).unwrap();
    println!("Serialize time: {:?}", last.elapsed());
    let bytes = fastnbt::to_bytes(&serial_region).unwrap();
}