use bitvec::prelude::*;
use rustc_hash::FxHashMap;

use bevy::prelude::*;
use bimap::BiMap;
use itertools::*;
use ndshape::{ConstShape, ConstShape3usize};
use serde::{Deserialize, Serialize};
use serde_big_array::Array;
use strum::EnumString;

use crate::storage::{
    blocks::descriptor::BlockDescriptor,
    crafting::descriptor::RecipeDescriptor,
    geometry::{descriptor::BlockGeo, load::block_geo},
    items::descriptor::ItemDescriptor,
};

pub const HORIZONTAL_DISTANCE: usize = 15;
pub const VERTICAL_DISTANCE: usize = 9;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_ARR: u32 = CHUNK_SIZE as u32 - 1;
pub const TOTAL_CHUNK_SIZE: usize = (CHUNK_SIZE) * (CHUNK_SIZE) * (CHUNK_SIZE);

type ChunkShape = ConstShape3usize<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

#[derive(Resource, Clone, Default, Deref, DerefMut)]
pub struct RecipeTable(pub FxHashMap<String, RecipeDescriptor>);

#[derive(Resource, Clone, Default, Deref, DerefMut)]
pub struct BlockTable(pub FxHashMap<String, BlockDescriptor>);

#[derive(Resource, Clone, Default, Deref, DerefMut)]
pub struct ItemTable(pub FxHashMap<String, ItemDescriptor>);

#[derive(EnumString, Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub enum VoxelVisibility {
    #[default]
    Empty,
    Opaque,
    Transparent,
}

#[derive(EnumString, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum Direction {
    #[default]
    North,
    West,
    East,
    South,
}

impl Direction {
    pub fn get_as_string(&self) -> String {
        match self {
            Direction::North => "north".to_string(),
            Direction::West => "west".to_string(),
            Direction::East => "east".to_string(),
            Direction::South => "south".to_string(),
        }
    }
}

#[derive(EnumString, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub enum GrowthState {
    #[default]
    Planted,
    Sapling,
    Young,
    Ripe,
    Spoiled,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Container {
    pub items: Vec<String>, // Hashmap would be better and may do more into implementing hashmyself at some point but this approach works for now
    pub max_size: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RenderedBlockData {
    pub identifier: String,
    pub direction: Option<Direction>,
    pub top: Option<bool>,
    pub geo: BlockGeo,
    pub visibility: VoxelVisibility,
    pub has_direction: bool,
    pub exclusive_direction: bool,
    // pub textures: [Handle<Image>; 6],
    pub tex_variance: [bool; 6],
    pub blocks: [bool; 6],
}

pub fn name_to_identifier(namespace: String, name: String) -> String {
    let mut temp_name = namespace;
    temp_name.push(':');
    temp_name.push_str(&name);
    temp_name
}

pub fn identifier_to_name(identifier: String) -> Option<(String, String)> {
    if let Some((namespace, name)) = identifier.splitn(2, ':').tuples().next() {
        return Some((namespace.to_string(), name.to_string()));
    }
    None
}

pub fn identifier_to_just_name(identifier: String) -> Option<String> {
    if let Some((_, name)) = identifier.splitn(2, ':').tuples().next() {
        return Some(name.to_string());
    }
    None
}

pub fn trim_geo_identifier(identifier: String) -> String {
    if let Some((prefix, _)) = identifier.split_once('.') {
        prefix.to_string()
    } else {
        identifier
    }
}

impl Default for RenderedBlockData {
    fn default() -> Self {
        RenderedBlockData {
            identifier: "vinox:air".to_string(),
            visibility: VoxelVisibility::Empty,
            blocks: [false, false, false, false, false, false],
            tex_variance: [false, false, false, false, false, false],
            has_direction: false,
            exclusive_direction: false,
            direction: None,
            top: None,
            geo: block_geo().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct BlockData {
    pub namespace: String,
    pub name: String,
    pub direction: Option<Direction>,
    pub container: Option<Container>,
    pub growth_state: Option<GrowthState>,
    pub last_tick: Option<u64>,
    pub arbitary_data: Option<String>,
    pub top: Option<bool>,
}

impl BlockData {
    pub fn is_empty(&self, block_table: &BlockTable) -> bool {
        block_table
            .get(&name_to_identifier(
                self.namespace.clone(),
                self.name.clone(),
            ))
            .unwrap()
            .visibility
            .unwrap_or_default()
            == VoxelVisibility::Empty
    }
}

impl Default for BlockData {
    fn default() -> Self {
        BlockData {
            namespace: "vinox".to_string(),
            name: "air".to_string(),
            direction: None,
            container: None,
            growth_state: None,
            last_tick: None,
            arbitary_data: None,
            top: None,
        }
    }
}

impl BlockData {
    pub fn new(namespace: String, name: String) -> Self {
        BlockData {
            namespace,
            name,
            ..Default::default()
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct RawChunk {
//     pub palette: BiMap<u16, BlockData>,
//     pub voxels: Box<Array<u16, TOTAL_CHUNK_SIZE>>,
// }

// #[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub enum VoxelType {
//     Empty(u16),
//     Opaque(u16),
//     Transparent(u16),
// }

// impl Default for VoxelType {
//     fn default() -> VoxelType {
//         Self::Empty(0)
//     }
// }

// impl Voxel for VoxelType {
//     fn visibility(&self) -> VoxelVisibility {
//         match self {
//             Self::Empty(_) => VoxelVisibility::Empty,
//             Self::Opaque(_) => VoxelVisibility::Opaque,
//             Self::Transparent(_) => VoxelVisibility::Transparent,
//         }
//     }
// }

// pub trait Voxel: Eq {
//     fn visibility(&self) -> VoxelVisibility;
// }

// macro_rules! as_variant {
//     ($value:expr, $variant:path) => {
//         match $value {
//             $variant(x) => Some(x),
//             _ => None,
//         }
//     };
// }

// impl VoxelType {
//     pub fn value(self) -> u16 {
//         match self {
//             Self::Empty(_) => as_variant!(self, VoxelType::Empty).unwrap_or(0),
//             Self::Opaque(_) => as_variant!(self, VoxelType::Opaque).unwrap_or(0),
//             Self::Transparent(_) => as_variant!(self, VoxelType::Transparent).unwrap_or(0),
//         }
//     }
// }

// pub trait Chunk {
//     type Output;

//     const X: usize;
//     const Y: usize;
//     const Z: usize;

//     fn size() -> usize {
//         Self::X * Self::Y * Self::Z
//     }

//     fn linearize(pos: UVec3) -> usize {
//         let x = pos.x as usize;
//         let y = pos.y as usize;
//         let z = pos.z as usize;
//         x + (y * Self::X) + (z * Self::X * Self::Y)
//     }

//     fn delinearize(mut index: usize) -> (u32, u32, u32) {
//         let z = index / (Self::X * Self::Y);
//         index -= z * (Self::X * Self::Y);

//         let y = index / Self::X;
//         index -= y * Self::X;

//         let x = index;

//         (x as u32, y as u32, z as u32)
//     }

//     fn get(&self, x: u32, y: u32, z: u32, block_table: &BlockTable) -> Self::Output;
//     fn get_descriptor(&self, x: u32, y: u32, z: u32, block_table: &BlockTable) -> BlockDescriptor;
//     fn get_data(&self, x: u32, y: u32, z: u32) -> RenderedBlockData;
// }

// impl Default for RawChunk {
//     fn default() -> RawChunk {
//         let mut raw_chunk = RawChunk {
//             palette: BiMap::new(),
//             voxels: Box::default(),
//         };
//         raw_chunk.palette.insert(
//             0,
//             BlockData {
//                 namespace: "vinox".to_string(),
//                 name: "air".to_string(),
//                 ..Default::default()
//             },
//         );
//         raw_chunk
//     }
// }

// impl Chunk for RawChunk {
//     type Output = VoxelType;

//     const X: usize = CHUNK_SIZE as usize;
//     const Y: usize = CHUNK_SIZE as usize;
//     const Z: usize = CHUNK_SIZE as usize;

//     fn get(&self, x: u32, y: u32, z: u32, block_table: &BlockTable) -> Self::Output {
//         self.get_voxel(RawChunk::linearize(UVec3::new(x, y, z)), block_table)
//     }
//     fn get_data(&self, x: u32, y: u32, z: u32) -> RenderedBlockData {
//         self.get_rend(x, y, z)
//     }

//     fn get_descriptor(&self, x: u32, y: u32, z: u32, block_table: &BlockTable) -> BlockDescriptor {
//         self.get_data(RawChunk::linearize(UVec3::new(x, y, z)), block_table)
//     }
// }

// impl RawChunk {
//     // Very important to use this for creation because of air
//     pub fn new() -> RawChunk {
//         let mut raw_chunk = RawChunk {
//             palette: BiMap::new(),
//             voxels: Box::default(),
//         };
//         raw_chunk.palette.insert(
//             0,
//             BlockData {
//                 namespace: "vinox".to_string(),
//                 name: "air".to_string(),
//                 ..Default::default()
//             },
//         );
//         raw_chunk
//     }

//     pub fn get_voxel(&self, index: usize, block_table: &BlockTable) -> VoxelType {
//         let block_state = self
//             .get_state_for_index(self.voxels[index] as usize)
//             .unwrap();
//         let block_id = self.get_index_for_state(&block_state).unwrap();
//         let mut block_name = block_state.namespace.clone();
//         block_name.push(':');
//         block_name.push_str(block_state.name.as_str());
//         if let Some(voxel) = block_table.get(&block_name) {
//             let voxel_visibility = voxel.visibility;
//             if let Some(voxel_visibility) = voxel_visibility {
//                 match voxel_visibility {
//                     VoxelVisibility::Empty => VoxelType::Empty(block_id),
//                     VoxelVisibility::Opaque => VoxelType::Opaque(block_id),
//                     VoxelVisibility::Transparent => VoxelType::Transparent(block_id),
//                 }
//             } else {
//                 VoxelType::Empty(0)
//             }
//         } else {
//             println!("No name: {block_name:?}");
//             VoxelType::Empty(0)
//         }
//     }

//     pub fn get_data(&self, index: usize, block_table: &BlockTable) -> BlockDescriptor {
//         let block_state = self
//             .get_state_for_index(self.voxels[index] as usize)
//             .unwrap();
//         let mut block_name = block_state.namespace.clone();
//         block_name.push(':');
//         block_name.push_str(block_state.name.as_str());
//         block_table.get(&block_name).unwrap().clone()
//     }

//     pub fn get_rend(&self, x: u32, y: u32, z: u32) -> RenderedBlockData {
//         let index = RawChunk::linearize(UVec3::new(x, y, z));
//         let block_state = self
//             .get_state_for_index(self.voxels[index] as usize)
//             .unwrap();
//         RenderedBlockData::new(
//             block_state.namespace,
//             block_state.name,
//             block_state.direction,
//             block_state.top,
//         )
//     }

//     pub fn get_index_for_state(&self, block_data: &BlockData) -> Option<u16> {
//         self.palette.get_by_right(block_data).copied()
//     }

//     pub fn get_state_for_index(&self, index: usize) -> Option<BlockData> {
//         self.palette.get_by_left(&(index as u16)).cloned()
//     }

//     // This is most likely a VERY awful way to handle this however for now I just want a working solution ill
//     // rewrite this if it causes major performance issues
//     pub fn update_chunk_pal(&mut self, old_pal: &BiMap<u16, BlockData>) {
//         for i in 0..self.voxels.len() {
//             if let Some(block_data) = old_pal.get_by_left(&self.voxels[i]) {
//                 if let Some(new_index) = self.get_index_for_state(block_data) {
//                     self.voxels[i] = new_index;
//                 } else {
//                     self.voxels[i] = 0;
//                 }
//             }
//         }
//     }
//     fn max_block_id(&self) -> u16 {
//         let mut counter = 0;
//         for id in self.palette.left_values().sorted() {
//             if *id != 0 && counter < id - 1 {
//                 return *id;
//             }
//             counter = *id;
//         }
//         counter + 1
//     }
//     pub fn add_block_state(&mut self, block_data: &BlockData) {
//         let old_pal = self.palette.clone();
//         if let Some(_id) = self.get_index_for_state(block_data) {
//         } else {
//             self.palette
//                 .insert(self.max_block_id(), block_data.to_owned());
//             self.update_chunk_pal(&old_pal);
//         }
//     }
//     pub fn remove_block_state(&mut self, block_data: &BlockData) {
//         if block_data.eq(&BlockData {
//             namespace: "vinox".to_string(),
//             name: "air".to_string(),
//             ..Default::default()
//         }) {
//             return;
//         }
//         let old_pal = self.palette.clone();
//         if let Some(id) = self.get_index_for_state(block_data) {
//             self.palette.remove_by_left(&id);
//             self.update_chunk_pal(&old_pal);
//         } else {
//             warn!("Block data: {}, doesn't exist!", block_data.name);
//         }
//     }
//     // This actual chunks data starts at 1,1,1 and ends at chunk_size
//     pub fn set_block(&mut self, pos: UVec3, block_data: &BlockData) {
//         self.add_block_state(block_data);
//         let index = RawChunk::linearize(pos);
//         if let Some(block_type) = self.get_index_for_state(block_data) {
//             if block_type == 0 {
//                 self.voxels[index] = 0;
//             } else {
//                 self.voxels[index] = block_type; // Set based off of transluency
//             }
//         } else {
//             warn!("Voxel doesn't exist");
//         }
//     }

//     pub fn get_block(&self, pos: UVec3) -> Option<BlockData> {
//         let index = RawChunk::linearize(pos);
//         self.get_state_for_index(self.voxels[index] as usize)
//     }

//     pub fn get_identifier(&self, pos: UVec3) -> String {
//         let index = RawChunk::linearize(pos);
//         if let Some(block) = self.get_state_for_index(self.voxels[index] as usize) {
//             let mut identifier = block.namespace.clone();
//             identifier.push(':');
//             identifier.push_str(&block.name);
//             identifier
//         } else {
//             "vinox:air".to_string()
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use bevy::prelude::UVec3;

//     use super::{BlockData, RawChunk};

//     #[test]
//     fn palette_works() {
//         let mut raw_chunk = RawChunk::default();
//         raw_chunk.add_block_state(&BlockData::new("vinox".to_string(), "dirt".to_string()));
//         let grass = BlockData::new("vinox".to_string(), "grass".to_string());
//         raw_chunk.add_block_state(&grass);
//         raw_chunk.set_block(UVec3::new(1, 1, 1), &grass);
//         assert_eq!(
//             raw_chunk.get_block(UVec3::new(1, 1, 1)),
//             Some(grass.clone())
//         );
//         raw_chunk.remove_block_state(&BlockData::new("vinox".to_string(), "dirt".to_string()));
//         assert_eq!(raw_chunk.get_block(UVec3::new(1, 1, 1)), Some(grass));
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Storage {
    Single(SingleStorage),
    Multi(MultiStorage),
}

/// Compressed storage for volumes with a single voxel type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleStorage {
    size: usize,
    voxel: BlockData,
}

/// Palette compressed storage for volumes with multiple voxel types
/// Based on https://voxel.wiki/wiki/palette-compression/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiStorage {
    /// Size of chunk storage, in voxels
    size: usize,
    data: BitBuffer,
    palette: Vec<PaletteEntry>,
    /// Palette capacity given size of indices
    /// Not necessarily equal to palette vector capacity
    palette_capacity: usize,
    /// Bit length of indices into the palette
    indices_length: usize,
}

impl MultiStorage {
    fn new(size: usize, initial_voxel: BlockData) -> Self {
        // Indices_length of 2 since this is only used for multiple voxel types
        let indices_length = 2;
        let initial_capacity = 2_usize.pow(indices_length as u32);
        let mut palette = Vec::with_capacity(initial_capacity);
        palette.push(PaletteEntry {
            voxel_type: initial_voxel,
            ref_count: size,
        });

        Self {
            size,
            data: BitBuffer::new(size * indices_length),
            palette,
            palette_capacity: initial_capacity,
            indices_length,
        }
    }

    fn grow_palette(&mut self) {
        let mut indices: Vec<usize> = Vec::with_capacity(self.size);
        for i in 0..self.size {
            indices.push(self.data.get(i * self.indices_length, self.indices_length));
        }

        self.indices_length <<= 1;
        let new_capacity = 2usize.pow(self.indices_length as u32);
        self.palette.reserve(new_capacity - self.palette_capacity);
        self.palette_capacity = new_capacity;

        self.data = BitBuffer::new(self.size * self.indices_length);

        for (i, idx) in indices.into_iter().enumerate() {
            self.data
                .set(i * self.indices_length, self.indices_length, idx);
        }
    }
}

impl Storage {
    pub fn new(size: usize) -> Self {
        Self::Single(SingleStorage {
            size,
            voxel: BlockData::default(),
        })
    }

    fn toggle_storage_type(&mut self) {
        *self = match self {
            Storage::Single(storage) => {
                Storage::Multi(MultiStorage::new(storage.size, storage.voxel.clone()))
            }
            Storage::Multi(storage) => {
                assert!(storage.palette.len() == 1);
                Storage::Single(SingleStorage {
                    size: storage.size,
                    voxel: storage.palette[0].voxel_type.clone(),
                })
            }
        };
    }

    pub fn set(&mut self, target_idx: usize, voxel: BlockData) {
        match self {
            Storage::Single(storage) => {
                if storage.voxel != voxel {
                    self.toggle_storage_type();
                    self.set(target_idx, voxel);
                }
            }
            Storage::Multi(storage) => {
                let palette_target_idx: usize = storage
                    .data
                    .get(target_idx * storage.indices_length, storage.indices_length);
                if let Some(target) = storage.palette.get_mut(palette_target_idx) {
                    target.ref_count -= 1;
                }

                // Look for voxel palette entry
                let palette_entry_voxel =
                    storage.palette.iter().enumerate().find_map(|(idx, entry)| {
                        if entry.voxel_type == voxel {
                            Some(idx)
                        } else {
                            None
                        }
                    });

                // Voxel type already in palette
                if let Some(idx) = palette_entry_voxel {
                    storage.data.set(
                        target_idx * storage.indices_length,
                        storage.indices_length,
                        idx,
                    );
                    storage
                        .palette
                        .get_mut(idx)
                        .expect("Failed to get palette entry of target voxel")
                        .ref_count += 1;

                    return;
                }

                // Overwrite target palette entry
                if let Some(target) = storage.palette.get_mut(palette_target_idx) {
                    if target.ref_count == 0 {
                        target.voxel_type = voxel;
                        target.ref_count = 1;

                        return;
                    }
                }

                // Create new palette entry
                //bevy::prelude::info!("Creating new voxel entry for {:?}", voxel);
                let new_entry_idx = if let Some((i, entry)) = storage
                    .palette
                    .iter_mut()
                    .enumerate()
                    .find(|(_i, entry)| entry.ref_count == 0)
                {
                    // Recycle a ref_count 0 entry if any exists
                    entry.voxel_type = voxel;
                    entry.ref_count = 1;

                    i
                } else {
                    // Create a new entry from scratch
                    if storage.palette.len() == storage.palette_capacity {
                        storage.grow_palette();
                    }

                    storage.palette.push(PaletteEntry {
                        voxel_type: voxel,
                        ref_count: 1,
                    });

                    storage.palette.len() - 1
                };
                storage.data.set(
                    target_idx * storage.indices_length,
                    storage.indices_length,
                    new_entry_idx,
                );
            }
        }
    }

    pub fn get(&self, idx: usize) -> BlockData {
        match self {
            Storage::Single(storage) => storage.voxel.clone(),
            Storage::Multi(storage) => {
                let palette_idx: usize = storage
                    .data
                    .get(idx * storage.indices_length, storage.indices_length);

                storage
                    .palette
                    .get(palette_idx)
                    .expect("Failed to get palette entry in voxel get")
                    .voxel_type
                    .clone()
            }
        }
    }

    pub fn trim(&mut self) {
        match self {
            Storage::Single(_) => (),
            Storage::Multi(storage) => {
                if storage.palette.len() == 1 {
                    self.toggle_storage_type();
                }
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PaletteEntry {
    voxel_type: BlockData,
    ref_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BitBuffer {
    bytes: BitVec<u8, Lsb0>,
}

impl BitBuffer {
    /// Create a new BitBuffer
    /// size is specified in bits, not bytes
    fn new(size: usize) -> Self {
        Self {
            bytes: BitVec::repeat(false, size),
        }
    }

    /// Set arbitraty bits in BitBuffer.
    /// idx, bit_length and bits are specified in bits, not bytes
    fn set(&mut self, idx: usize, bit_length: usize, bits: usize) {
        self.bytes[idx..idx + bit_length].store_le::<usize>(bits);
    }

    /// Get arbitraty bits in BitBuffer.
    /// idx, bit_length are specified in bits, not bytes
    fn get(&self, idx: usize, bit_length: usize) -> usize {
        self.bytes[idx..idx + bit_length].load_le::<usize>()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawChunk {
    voxels: Storage,
}

#[derive(Component, Clone, Debug)]
pub struct ChunkData {
    voxels: Storage,
    change_count: u16,
    dirty: bool,
}

impl Default for ChunkData {
    fn default() -> Self {
        Self {
            voxels: Storage::new(ChunkShape::USIZE),
            change_count: 0,
            dirty: true,
        }
    }
}

#[allow(dead_code)]
impl ChunkData {
    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockData {
        self.voxels.get(Self::linearize(x, y, z))
    }
    pub fn get_identifier(&self, x: usize, y: usize, z: usize) -> String {
        let voxel = self.voxels.get(Self::linearize(x, y, z));
        name_to_identifier(voxel.namespace, voxel.name)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: BlockData) {
        self.voxels.set(Self::linearize(x, y, z), voxel);
        self.change_count += 1;
        self.set_dirty(true);

        if self.change_count > 500 {
            self.voxels.trim();
            self.change_count = 0;
        }
    }

    pub fn is_uniform(&self) -> bool {
        match self.voxels {
            Storage::Single(_) => true,
            Storage::Multi(_) => false,
        }
    }

    pub fn is_empty(&self, block_table: &BlockTable) -> bool {
        self.is_uniform() && self.get(0, 0, 0).is_empty(block_table)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn trim(&mut self) {
        self.voxels.trim();
    }

    pub const fn size() -> usize {
        ChunkShape::USIZE
    }

    pub const fn edge() -> usize {
        CHUNK_SIZE
    }

    #[inline]
    pub fn linearize(x: usize, y: usize, z: usize) -> usize {
        ChunkShape::linearize([x, y, z])
    }

    #[inline]
    pub fn delinearize(idx: usize) -> (usize, usize, usize) {
        let res = ChunkShape::delinearize(idx);
        (res[0], res[1], res[2])
    }

    pub fn from_raw(raw_chunk: RawChunk) -> Self {
        Self {
            voxels: raw_chunk.voxels,
            change_count: 0,
            dirty: false,
        }
    }

    pub fn to_raw(&self) -> RawChunk {
        RawChunk {
            voxels: self.voxels.clone(),
        }
    }
}
