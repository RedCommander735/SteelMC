use std::sync::Weak;
use steel_macros::block_behavior;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty, EnumProperty, RailShape};
use steel_utils::{BlockPos, BlockStateId};
use crate::behavior::{BlockBehavior, BlockEntityCreation, BlockPlaceContext};
use crate::behavior::blocks::PoweredBlock;
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkullBlockType {
    Skeleton,
    WitherSkeleton,
    Player,
    Zombie,
    Creeper,
    Piglin,
    Dragon,
}

/// Shared server behavior inherited from vanilla's `AbstractSkullBlock`.
pub(super) trait AbstractSkullBlock {
    const POWERED: &BoolProperty = &BlockStateProperties::POWERED;

    /// Should be const
    #[must_use]
    fn new(block: BlockRef, skull_type: SkullBlockType) -> impl AbstractSkullBlock;
}

pub(super) trait SharedSkullBlockBehavior: BlockBehavior {
    fn skull_get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId>
}

impl<T: SharedSkullBlockBehavior> BlockBehavior for T {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.skull_get_state_for_placement(context)
    }
    fn new_block_entity(&self, level: Weak<World>, pos: BlockPos, state: BlockStateId) -> BlockEntityCreation {
        // TODO: Implement SkullBlockEntity
        todo!("Implement SkullBlockEntity")
    }
}
