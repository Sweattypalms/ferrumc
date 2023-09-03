use std::collections::HashMap;
use std::sync::{Arc, Once};

use async_trait::async_trait;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::network::packet::inbound::packet_play_in_handshake::PacketPlayInHandshake;
use crate::network::packet::inbound::packet_play_in_ping::PacketPlayInPing;

mod inbound;
mod outbound;

#[async_trait]
pub trait Packet: Send + Sync + 'static {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized;
    fn get_id(&self) -> u32;
    fn get_name(&self) -> String;
    async fn handle(&self, stream: &mut TcpStream);

    fn construct_boxed(data: Vec<u8>) -> Box<dyn Packet> where Self: Sized,
    {
        Box::new(Self::deserialize(data).unwrap())
    }
}

type PacketConstructor = fn(Vec<u8>) -> Box<dyn Packet>;

pub struct PacketRegistry {
    pub inbound: HashMap<u32, PacketConstructor>,
    pub outbound: HashMap<u32, PacketConstructor>,
}

lazy_static! {
    pub static ref REGISTRY: Arc<PacketRegistry> = {
        let mut registry = PacketRegistry::new();
        registry.initialize();
        Arc::new(registry)
    };
}
impl PacketRegistry {

    pub fn new() -> Self {
        PacketRegistry {
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.inbound.insert(
            0x00,
            PacketPlayInHandshake::construct_boxed,
        );

        self.inbound.insert(
            0x01,
            PacketPlayInPing::construct_boxed,
        );
    }

    pub fn deserialize_inbound(&self, bytes: Vec<u8>) -> Option<Box<dyn Packet>> {
        let id = bytes[1] as u32;
        if let Some(constructor) = self.inbound.get(&id) {
            return Some(constructor(bytes));
        }
        None
    }
}