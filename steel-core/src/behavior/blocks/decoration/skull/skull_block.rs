use crate::behavior::blocks::decoration::skull::abstract_skull_block::{
    AbstractSkullBlock, SkullBlockType,
};
use crate::behavior::{BlockBehavior, BlockEntityCreation, BlockPlaceContext};
use crate::entity::ai::path::PathComputationType;
use crate::world::World;
use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{BlockStateProperties, IntProperty};
use steel_utils::angle::convert_to_rotation_segment;
use steel_utils::{BlockPos, BlockStateId};

const ROTATION_16: &IntProperty = &BlockStateProperties::ROTATION_16;
const ROTATIONS: u8 = ROTATION_16.max + 1;

#[block_behavior]
pub struct SkullBlock {
    block: BlockRef,
    #[json_arg(r#enum = "SkullBlockType", json = "type")]
    skull_type: SkullBlockType,
}

impl SkullBlock {
    pub const fn new(block: BlockRef, skull_type: SkullBlockType) -> Self {
        Self { block, skull_type }
    }
}

impl BlockBehavior for SkullBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.default_state_for_placement(context)
    }

    fn handle_neighbor_changed(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        source_block: BlockRef,
        moved_by_piston: bool,
    ) {
        self.handle_skull_neighbor_changed(state, world, pos, source_block, moved_by_piston);
    }

    fn is_pathfindable(&self, state: BlockStateId, computation_type: PathComputationType) -> bool {
        self.is_skull_pathfindable(state, computation_type)
    }

    fn new_block_entity(
        &self,
        level: Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> BlockEntityCreation {
        self.new_skull_block_entity(level, pos, state)
    }
}

impl AbstractSkullBlock for SkullBlock {
    fn get_type(&self) -> SkullBlockType {
        self.skull_type
    }

    fn state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        let rotation = convert_to_rotation_segment(context.rotation());
        Some(self.block.default_state().set_value(ROTATION_16, rotation))
    }
}
