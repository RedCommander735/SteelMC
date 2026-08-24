use crate::behavior::blocks::PoweredBlock;
use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
};
use crate::block_entity::BlockEntityTicker;
use crate::entity::ai::path::PathComputationType;
use crate::world::{LevelReader, SignalGetter, World};
use std::sync::{Arc, Weak};
use std::todo;
use steel_macros::block_behavior;
use steel_registry::block_entity_type::BlockEntityTypeRef;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{
    BlockStateProperties, BoolProperty, EnumProperty, RailShape,
};
use steel_registry::blocks::shapes::VoxelShape;
use steel_utils::types::UpdateFlags;
use steel_utils::{BlockPos, BlockStateId};

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
pub(super) trait AbstractSkullBlock: BlockBehavior {
    #[must_use]
    fn get_type(&self) -> SkullBlockType;
    #[must_use]
    fn state_for_placement(&self, context: &BlockPlaceContext<'_>) -> BlockStateId;
    #[must_use]
    fn collision_shape(
        &self,
        state: BlockStateId,
        world: &dyn LevelReader,
        pos: BlockPos,
        context: BlockCollisionContext,
    ) -> VoxelShape;
}

impl<T: AbstractSkullBlock> BlockBehavior for T {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        Some(self.state_for_placement(context).set_value(POWERED, context.world.has_neighbor_signal(context.place_pos())))
    }

    fn handle_neighbor_changed(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        _source_block: BlockRef,
        _moved_by_piston: bool,
    ) {
        let signal: bool = world.has_neighbor_signal(pos);
        if signal != state.get_value(POWERED) {
            world.set_block(
                pos,
                state.set_value(POWERED, signal),
                UpdateFlags::UPDATE_CLIENTS,
            );
        }
    }

    fn is_pathfindable(
        &self,
        _state: BlockStateId,
        _computation_type: PathComputationType,
    ) -> bool {
        false
    }

    fn get_collision_shape(
        &self,
        state: BlockStateId,
        world: &dyn LevelReader,
        pos: BlockPos,
        context: BlockCollisionContext,
    ) -> VoxelShape {
        self.collision_shape(state, world, pos, context)
    }

    fn new_block_entity(
        &self,
        level: Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> BlockEntityCreation {
        // TODO: Implement SkullBlockEntity
        todo!("Implement SkullBlockEntity")
    }
}
