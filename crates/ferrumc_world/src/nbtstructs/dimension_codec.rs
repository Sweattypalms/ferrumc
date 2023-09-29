use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: DimensionType,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome_registry: BiomeRegistry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionType {
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: Vec<DimensionTypeValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionTypeValue {
    pub name: String,
    pub id: i32,
    pub element: DimensionTypeElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionTypeElement {
    pub piglin_safe: bool,
    pub natural: bool,
    pub ambient_light: f32,
    pub infiniburn: String,
    pub respawn_anchor_works: bool,
    pub has_skylight: bool,
    pub bed_works: bool,
    pub effects: String,
    pub has_raids: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f32,
    pub ultrawarm: bool,
    pub has_ceiling: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeRegistry {
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: Vec<BiomeRegistryValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeRegistryValue {
    pub name: String,
    pub id: i32,
    pub element: BiomeRegistryElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeRegistryElement {
    pub precipitation: String,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: String,
    pub effects: BiomeRegistryEffects,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeRegistryEffects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
}