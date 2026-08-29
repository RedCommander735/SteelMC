use crate::behavior::blocks::SkullBlock;
use crate::behavior::blocks::decoration::skull::abstract_skull_block::{
    AbstractSkullBlock, SkullBlockType,
};
use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
};
use crate::entity::ai::path::PathComputationType;
use crate::world::{LevelReader, World};
use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::blocks::BlockRef;
use steel_utils::{BlockPos, BlockStateId};

#[block_behavior]
pub struct PlayerHeadBlock {
    base: SkullBlock,
}

impl PlayerHeadBlock {
    pub const fn new(block: BlockRef) -> Self {
        Self {
            base: SkullBlock::new(block, SkullBlockType::Player),
        }
    }
}

impl BlockBehavior for PlayerHeadBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.base.get_state_for_placement(context)
    }

    fn handle_neighbor_changed(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        source_block: BlockRef,
        moved_by_piston: bool,
    ) {
        self.base
            .handle_neighbor_changed(state, world, pos, source_block, moved_by_piston);
    }

    fn is_pathfindable(&self, state: BlockStateId, computation_type: PathComputationType) -> bool {
        self.base.is_pathfindable(state, computation_type)
    }

    fn new_block_entity(
        &self,
        level: Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> BlockEntityCreation {
        self.base.new_block_entity(level, pos, state)
    }
}

impl AbstractSkullBlock for PlayerHeadBlock {
    fn get_type(&self) -> SkullBlockType {
        self.base.get_type()
    }

    fn state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.base.state_for_placement(context)
    }
}
