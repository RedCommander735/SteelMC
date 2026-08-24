use crate::behavior::blocks::decoration::skull::abstract_skull_block::{
    AbstractSkullBlock, SkullBlockType,
};
use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
};
use crate::world::{LevelReader, World};
use std::sync::Weak;
use steel_macros::block_behavior;
use steel_protocol::packets::login::SHello;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty, IntProperty};
use steel_registry::blocks::shapes::VoxelShape;
use steel_registry::blocks::{BlockRef, ShapeFn};
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::entity_data::EntityData::Rotations;
use steel_utils::{BlockLocalAabb, BlockPos, BlockStateId};
use steel_utils::angle::convert_to_rotation_segment;

const ROTATION_16: &IntProperty = &BlockStateProperties::ROTATION_16;
const ROTATIONS: u8 = ROTATION_16.max + 1;

const SKULL_COLUMN_BOXES: &[BlockLocalAabb] =
    &[BlockLocalAabb::new(0.25, 0.0, 0.25, 0.75, 0.5, 0.75)];

const PIGLIN_SKULL_COLUMN_BOXES: &[BlockLocalAabb] = &[BlockLocalAabb::new(
    0.1875, 0.0, 0.1875, 0.8125, 0.5, 0.8125,
)];

const SHAPE: VoxelShape = VoxelShape::from_boxes(SKULL_COLUMN_BOXES);
const PIGLIN_SHAPE: VoxelShape = VoxelShape::from_boxes(PIGLIN_SKULL_COLUMN_BOXES);

#[block_behavior]
pub struct SkullBlock {
    block: BlockRef,
    #[json_arg(r#enum = "SkullBlockType", json = "type")]
    skull_type: SkullBlockType,
}

impl SkullBlock {
    const fn new(block: BlockRef, skull_type: SkullBlockType) -> Self {
        Self { block, skull_type }
    }

    fn rotate(block_state: BlockStateId, rotation: u8) -> BlockStateId {
        block_state.set_value(ROTATION_16, rotation)
    }
}

impl AbstractSkullBlock for SkullBlock {
    fn get_type(&self) -> SkullBlockType {
        self.skull_type
    }

    fn state_for_placement(&self, context: &BlockPlaceContext<'_>) -> BlockStateId {
        let rotation = convert_to_rotation_segment(context.rotation());
        self.block.default_state().set_value(ROTATION_16, rotation)
    }

    fn collision_shape(
        &self,
        state: BlockStateId,
        world: &dyn LevelReader,
        pos: BlockPos,
        context: BlockCollisionContext,
    ) -> VoxelShape {
        if self.get_type() == SkullBlockType::Piglin {
            PIGLIN_SHAPE
        } else {
            SHAPE
        }
    }
}
