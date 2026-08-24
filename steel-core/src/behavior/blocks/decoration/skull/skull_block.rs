use std::sync::Weak;
use steel_registry::blocks::BlockRef;
use steel_utils::{BlockPos, BlockStateId};
use crate::behavior::{BlockBehavior, BlockEntityCreation};
use crate::world::World;

pub struct SkullBlock {
    block: BlockRef
}

impl BlockBehavior for SkullBlock {
    fn new_block_entity(&self, level: Weak<World>, pos: BlockPos, state: BlockStateId) -> BlockEntityCreation {
        todo!()
    }
}