use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::block_entity_type::BlockEntityTypeRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty, EnumProperty, RailShape};
use steel_utils::{BlockPos, BlockStateId};
use steel_utils::types::UpdateFlags;
use crate::behavior::{BlockBehavior, BlockEntityCreation, BlockPlaceContext};
use crate::behavior::blocks::PoweredBlock;
use crate::block_entity::BlockEntityTicker;
use crate::entity::ai::path::PathComputationType;
use crate::world::{SignalGetter, World};

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

pub(super) const POWERED: &BoolProperty = &BlockStateProperties::POWERED;

/// Shared server behavior inherited from vanilla's `AbstractSkullBlock`.
pub(super) trait AbstractSkullBlock {

    /// Should be const
    #[must_use]
    fn new(block: BlockRef, skull_type: SkullBlockType) -> impl AbstractSkullBlock;
    fn get_type() -> SkullBlockType;
}

pub(super) trait AbstractSkullBlockBehavior: BlockBehavior {
    #[must_use]
    fn state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId>;
}

impl<T: AbstractSkullBlockBehavior> BlockBehavior for T {
    #[must_use]
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.state_for_placement(context)
    }

    fn is_pathfindable(&self, _state: BlockStateId, _computation_type: PathComputationType) -> bool {
        false
    }

    fn new_block_entity(&self, level: Weak<World>, pos: BlockPos, state: BlockStateId) -> BlockEntityCreation {
        // TODO: Implement SkullBlockEntity
        todo!("Implement SkullBlockEntity")
    }

    fn handle_neighbor_changed(&self, state: BlockStateId, world: &Arc<World>, pos: BlockPos, _source_block: BlockRef, _moved_by_piston: bool) {
        let signal: bool = world.has_neighbor_signal(pos);
        if signal != state.get_value(POWERED) {
            world.set_block(pos, state.set_value(POWERED, signal), UpdateFlags::UPDATE_CLIENTS);
        }
    }
}
