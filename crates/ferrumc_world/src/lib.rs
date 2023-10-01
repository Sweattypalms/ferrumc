#![feature(int_roundings)]

pub mod blocks;
pub mod nbtstructs;

use log::error;

use fastanvil::Region;
use ferrumc_utils::err::FerrumcError;

use std::io::BufReader;

pub async fn get_chunk(x: usize, z: usize) -> Result<Vec<u8>, FerrumcError> {
    match std::env::var("WORLD_DIR") {
        Ok(worlddir) => {
            let file = std::fs::File::open(format!(
                "{}\\region\\r.{}.{}.mca",
                worlddir,
                (x as f64 / 32 as f64).round(),
                (z as f64 / 32 as f64).round()
            ));
            match file {
                Ok(file) => {
                    let buf = BufReader::new(file);
                    let mut region = Region::from_stream(buf).unwrap();
                    let chunk_binary = region.read_chunk(x, z);
                    match chunk_binary {
                        Ok(chunk) => Ok(chunk.unwrap()),
                        Err(e) => Err(FerrumcError::ChunkLoadError(e)),
                    }
                }
                Err(_) => {
                    error!("Region file {}.{}.mca not found", x, z);
                    Err(FerrumcError::RegionNotFound)
                }
            }
        }
        Err(_) => {
            error!("WORLD_DIR enviroment variable not set");
            return Err(FerrumcError::MissingEnvVar("WORLD_DIR".to_string()));
        }
    }
}