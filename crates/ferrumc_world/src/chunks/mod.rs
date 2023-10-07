// Packet structure
// Packet ID 	State 	Bound To 	Field Name 	Field Type 	Notes
// 0x20 	Play 	Client 	Chunk X 	Int 	Chunk coordinate (block coordinate divided by 16, rounded down)
// Chunk Z 	Int 	Chunk coordinate (block coordinate divided by 16, rounded down)
// Heightmaps 	NBT 	See heightmaps structure below.
// Size 	VarInt 	Size of Data in bytes; in some cases this is larger than it needs to be (e.g. MC-131684, MC-247438) in which case extra bytes should be skipped before reading fields after Data
// Data 	Byte array 	See data structure below
// Additional Data 	Various 	See protocol docs


// Data structure
//
// The data section of the packet contains most of the useful data for the chunk.
// Field Name 	Field Type 	Notes
// Data 	Array of Chunk Section 	The number of elements in the array is calculated based on the world's height. Sections are sent bottom-to-top.
// Chunk Section structure
// Chunk Section
//
// A Chunk Section is defined in terms of other data types. A Chunk Section consists of the following fields:
// Field Name 	Field Type 	Notes
// Block count 	Short 	Number of non-air blocks present in the chunk section. "Non-air" is defined as any fluid and block other than air, cave air, and void air. The client will keep count of the blocks as they are broken and placed, and, if the block count reaches 0, the whole chunk section is not rendered, even if it still has blocks.
// Block states 	Paletted Container 	Consists of 4096 entries, representing all the blocks in the chunk section
// Biomes 	Paletted Container 	Consists of 64 entries, representing 4x4x4 biome regions in the chunk section
// Paletted Container structure
//
// A Paletted Container is a palette-based storage of entries. Paletted Containers have an associated global palette (either block states or biomes as of now), where values are mapped from. A Paletted Container consists of the following fields:
// Field Name 	Field Type 	Notes
// Bits Per Entry 	Unsigned Byte 	Determines how many bits are used to encode entries. Note that not all numbers are valid here.
// Palette 	Varies 	See below for the format.
// Data Array Length 	VarInt 	Number of longs in the following array (ignored by minecraft)
// Data Array 	Array of Long 	Compacted list of indices pointing to entry IDs in the Palette. When Bits Per Entry is 0, this array is empty (see Single valued palette)
//
// Data Array is given for each entry with increasing x coordinates, within rows of increasing z coordinates, within layers of increasing y coordinates.
// Palettes
//
// The bits per entry value determines what format is used for the palette. In most cases, invalid values will be interpreted as a different value when parsed by the notchian client, meaning that chunk data will be parsed incorrectly if you use an invalid bits per entry. Servers must make sure that the bits per entry value is correct. There are currently three types of palettes:
// Single valued
//
// This format is used when bits per entry is equal to 0, and signifies that the palette contains a single value. When this palette is used, the Data Array sent/received is empty, since entries can be inferred from the palette's single value.
//
// The format is as follows:
// Field Name 	Field Type 	Notes
// Value 	VarInt 	ID of the corresponding entry in its global palette
// Indirect
//
// There are three variants of this:
//
//     For block states with bits per entry <= 4, 4 bits are used to represent a block.
//     For block states and bits per entry between 5 and 8, the given value is used.
//     For biomes the given value is always used, and will be <= 3
//
// This is an actual palette which lists the entries used. Values in the chunk section's data array are indices into the palette, which in turn gives a proper entry.
//
// The format is as follows:
// Field Name 	Field Type 	Notes
// Palette Length 	VarInt 	Number of elements in the following array.
// Palette 	Array of VarInt 	Mapping of entry IDs in the global palette to indices of this array
// Direct
//
// This format is used for bits per entry values greater than or equal to a threshold (9 for block states, 4 for biomes). The number of bits used to represent an entry is the base 2 logarithm of the number of entries in the global palette, rounded up. For the current vanilla release, this is 15 bits per entry for block states, and 6 bits per entry for biomes.
//
// The "palette" uses the following format:
// Field Name 	Field Type 	Notes
// no fields
//
// If Minecraft Forge is installed and a sufficiently large number of blocks are added, the bits per block value for the global palette will be increased to compensate for the increased ID count. This increase can go up to 16 bits per block (for a total of 4096 block IDs; when combined with the 16 damage values, there are 65536 total states). You can get the number of blocks with the "Number of ids" field found in the RegistryData packet in the Forge Handshake.
// Compacted data array
//
// The data array stores several entries within a single long, and sometimes overlaps one entry between multiple longs. For a bits per block value of 15, the data is stored such that bits 1 through 15 are the first entry, 16 through 30 are the second, and so on. Note that bit 1 is the least significant bit in this case, not the most significant bit. The same behavior applies when a value stretches between two longs: for instance, block 5 would be bits 57 through 64 of the first long and then bits 1 through 6 of the second long.
//
// The Data Array, although varying in length, will never be padded due to the number of blocks being evenly divisible by 64, which is the number of bits in a long.
//
// However, the compacted array format has been adjusted between MC 1.15 and MC 1.16 so that individual entries no longer span across multiple longs.
// Example (Old)
//
// Format used up to Minecraft 1.15.2
//
// 5 bits per block, containing the following references to blocks in a palette (not shown): 122344566480743131516914101202114 (although note that 4 could instead be any other value ending in those bits)
//
// 7020863148418841 0111000000100000100001100011000101001000010000011000100001000001
// 8B1018A7260F68C8 1000101100010000000110001010011100100110000011110110100011001000
// Example (New)
//
// Format used since Minecraft 1.16.0
//
// 5 bits per block, containing the following references to blocks in a palette (not shown): 122344566480743131516914101202
//
// 0020863148418841 0000000000100000100001100011000101001000010000011000100001000001
// 01018A7260F68C87 0000000100000001100010100111001001100000111101101000110010000111
// Huh.png 	The following information needs to be added to this page:
// Numeric IDs are outdated in this example, though the format is still correct
//
// A second older example: 13 bits per block, using the global palette.
//
// The following two longs would represent...
//
// 01001880C0060020 = 0000000100000000000110001000000011000000000001100000000000100000
// 0200D0068004C020 = 0000001000000000110100000000011010000000000001001100000000100000
//
// 9 blocks, with the start of a 10th (that would be finished in the next long).
//
//     Grass, 2:0 (0x020)
//     Dirt, 3:0 (0x030)
//     Dirt, 3:0 (0x030)
//     Coarse dirt, 3:1 (0x031)
//     Stone, 1:0 (0x010)
//     Stone, 1:0 (0x010)
//     Diorite, 1:3 (0x013)
//     Gravel, 13:0 (0x0D0)
//     Gravel, 13:0 (0x0D0)
//     Stone, 1:0 (or potentially emerald ore, 129:0) (0x010 or 0x810)

pub type VarInt = i32;
pub type VarLong = i64;
pub type UnsignedByte = u8;
pub type Short = i16;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub heightmaps: Heightmaps,
    pub size: VarInt,
    pub data: Vec<ChunkSection>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub block_count: Short,
    pub block_states: PalettedContainer,
    pub biomes: PalettedContainer,
}

#[derive(Debug, Clone)]
pub struct PalettedContainer {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette,
    pub data_array_length: VarInt,
    pub data_array: Vec<VarLong>,
}

#[derive(Debug, Clone)]
pub enum Palette {
    SingleValued(VarInt),
    Indirect(Vec<VarInt>),
    Direct,
}

#[derive(Debug, Clone)]
pub struct Heightmaps {
    pub motion_blocking: LongArray,
    // pub world_surface: LongArray, // Not required for now
}

#[derive(Debug, Clone)]
pub struct LongArray {
    pub length: VarInt,
    pub data: Vec<VarLong>,
}

// pub fn generate_flat_grass_world() -> Chunk {
//     // Chunk coordinates
//     let x = 0;
//     let z = 0;
//
//     // Heightmaps
//     let motion_blocking = LongArray {
//         length: 1, // Simplified for our example
//         data: vec![64], // Assuming our grass world is at Y=64
//     };
//     let heightmaps = Heightmaps { motion_blocking };
//
//     // Chunk sections
//     let mut data = Vec::new();
//
//     // For simplicity, let's create two sections: one for stone and one for grass
//     let stone_section = ChunkSection {
//         block_count: 16 * 16 * 16, // Full section of stone
//         block_states: PalettedContainer {
//             bits_per_entry: 4, // Simplified for our example
//             palette: Palette::Indirect(vec![1]), // 1 represents stone in our example
//             data_array_length: 16 * 16 * 16,
//             data_array: vec![0; 16 * 16 * 16], // All stone
//         },
//         biomes: PalettedContainer {
//             bits_per_entry: 4, // Simplified for our example
//             palette: Palette::Indirect(vec![1]), // 1 represents plains biome in our example
//             data_array_length: 4 * 4 * 4,
//             data_array: vec![0; 4 * 4 * 4], // All plains
//         },
//     };
//
//     let grass_section = ChunkSection {
//         block_count: 16 * 16, // Only the top layer is grass
//         block_states: PalettedContainer {
//             bits_per_entry: 4, // Simplified for our example
//             palette: Palette::Indirect(vec![2]), // 2 represents grass in our example
//             data_array_length: 16 * 16 * 16,
//             data_array: vec![0; 16 * 16 * 15 + 16 * 16], // Bottom 15 layers are stone, top layer is grass
//         },
//         biomes: stone_section.biomes.clone(), // Same biomes as stone section
//     };
//
//     data.push(stone_section);
//     data.push(grass_section);
//
//     // Chunk size (simplified for our example)
//     let size = 2 * (std::mem::size_of::<ChunkSection>() as VarInt);
//
//     Chunk { x, z, heightmaps, size, data }
// }
//
// impl Chunk {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//
//         // Chunk X
//         bytes.extend_from_slice(&self.x.to_be_bytes());
//
//         // Chunk Z
//         bytes.extend_from_slice(&self.z.to_be_bytes());
//
//         // Heightmaps
//         bytes.extend_from_slice(&self.heightmaps.to_bytes());
//
//         // Size
//         bytes.extend_from_slice(&self.size.to_be_bytes());
//
//         // Data
//         bytes.extend_from_slice(&self.data.to_bytes());
//
//         bytes
//     }
// }
//
// impl Heightmaps {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//
//         // Motion blocking
//         bytes.extend_from_slice(&self.motion_blocking.to_bytes());
//
//         bytes
//     }
// }
//
// impl LongArray {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//
//         // Length
//         bytes.extend_from_slice(&self.length.to_bytes());
//
//         // Data
//         bytes.extend_from_slice(&self.data.to_bytes());
//
//         bytes
//     }
// }
//
// impl ChunkSection {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//
//         // Block count
//         bytes.extend_from_slice(&self.block_count.to_bytes());
//
//         // Block states
//         bytes.extend_from_slice(&self.block_states.to_bytes());
//
//         // Biomes
//         bytes.extend_from_slice(&self.biomes.to_bytes());
//
//         bytes
//     }
// }
//
// impl Vec<ChunkSection> {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//
//         for section in self {
//             bytes.extend_from_slice(&section.to_bytes());
//         }
//
//         bytes
//     }
// }