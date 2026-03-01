use smallvec::SmallVec;

use crate::components::block::Block;

pub type RequirementNums = SmallVec<[u8; 7]>;

pub type RequirementNumsAllColors = SmallVec<[RequirementNums; 2]>;

pub type BoardContent = SmallVec<[SmallVec<[char; 7]>; 7]>;

pub type BlockContent = SmallVec<[SmallVec<[char; 7]>; 7]>;

pub type StringInputs = SmallVec<[String; 7]>;

pub type BlockVec = SmallVec<[Block; 8]>;
