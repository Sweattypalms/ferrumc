pub trait NBTSerializable {
    fn to_nbt(&self) -> Vec<u8>;
}


pub enum Biome {
    Void = 0,
    Plains = 1,
    SunflowerPlains = 2,
    SnowyPlains = 3,
    IceSpikes = 4,
    Desert = 5,
    Swamp = 6,
    MangroveSwamp = 7,
    Forest = 8,
    FlowerForest = 9,
    BirchForest = 10,
    DarkForest = 11,
    OldGrowthBirchForest = 12,
    OldGrowthPineTaiga = 13,
    OldGrowthSpruceTaiga = 14,
    Taiga = 15,
    SnowyTaiga = 16,
    Savanna = 17,
    SavannaPlateau = 18,
    WindsweptHills = 19,
    WindsweptGravellyHills = 20,
    WindsweptForest = 21,
    WindsweptSavanna = 22,
    Jungle = 23,
    SparseJungle = 24,
    BambooJungle = 25,
    Badlands = 26,
    ErodedBadlands = 27,
    WoodedBadlands = 28,
    Meadow = 29,
    CherryGrove = 30,
    Grove = 31,
    SnowySlopes = 32,
    FrozenPeaks = 33,
    JaggedPeaks = 34,
    StonyPeaks = 35,
    River = 36,
    FrozenRiver = 37,
    Beach = 38,
    SnowyBeach = 39,
    StonyShore = 40,
    WarmOcean = 41,
    LukewarmOcean = 42,
    DeepLukewarmOcean = 43,
    Ocean = 44,
    DeepOcean = 45,
    ColdOcean = 46,
    DeepColdOcean = 47,
    FrozenOcean = 48,
    DeepFrozenOcean = 49,
    MushroomFields = 50,
    DripstoneCaves = 51,
    LushCaves = 52,
    DeepDark = 53,
    NetherWastes = 54,
    WarpedForest = 55,
    CrimsonForest = 56,
    SoulSandValley = 57,
    BasaltDeltas = 58,
    TheEnd = 59,
    EndHighlands = 60,
    EndMidlands = 61,
    SmallEndIslands = 62,
    EndBarrens = 63,
}

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Air,
    Dirt,
    Grass,
    Stone,
    Sand,
    // TODO: Add more blocks
}

/// ### A sector of blocks in a chunk <br>
/// This is a set of blocks that are all the same type. <br>
/// It will be more efficient to store these in a chunk than individual blocks. <br>
/// If a chunk has a lot of different block types, it will have a lot of sectors but this is still
/// more efficient than storing each block individually.
pub struct BlockSector {
    // The start and end of the sector are the offsets from the chunk's origin
    // Chunk origin is south-west corner of the chunk at y=0
    start: (u16, u16, u16),
    end: (u16, u16, u16),
    block_type: Block,
}

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub blocks: Vec<BlockSector>,
    pub biome: Biome,
}