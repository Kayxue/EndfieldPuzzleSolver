use smallvec::SmallVec;

use crate::components::block::Block;

pub type RequirementNums = SmallVec<[u8; 8]>;

pub type BoardContent = SmallVec<[SmallVec<[char; 8]>; 8]>;

pub type BlockContent = SmallVec<[SmallVec<[char; 8]>; 8]>;

pub type StringInputs = SmallVec<[String; 8]>;

pub type BlockVec = SmallVec<[Block; 8]>;
