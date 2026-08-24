use crate::block_entity::{BlockEntity, BlockEntityBase};
use crate::world::World;
use simdnbt::borrow::{BaseNbtCompound as BorrowedNbtCompound, NbtCompound as NbtCompoundView};
use simdnbt::owned::NbtCompound;
use simdnbt::{FromNbtTag, ToNbtTag};
use std::sync::Weak;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty};
use steel_registry::{ResolvableProfile, vanilla_block_entity_types};
use steel_utils::locks::SyncMutex;
use steel_utils::{BlockPos, BlockStateId, DowncastType, DowncastTypeKey, Identifier};
use text_components::TextComponent;

const POWERED: &BoolProperty = &BlockStateProperties::POWERED;

const PROFILE_NBT_KEY: &str = "profile";
const NOTE_BLOCK_SOUND_NBT_KEY: &str = "note_block_sound";
const CUSTOM_NAME_NBT_KEY: &str = "custom_name";

/// Skull block entity.
///
/// Stores player profile, note block sound and custom name
pub struct SkullBlockEntity {
    base: BlockEntityBase,
    state: SyncMutex<SkullState>,
}

struct SkullState {
    owner: Option<ResolvableProfile>,
    note_block_sound: Option<Identifier>,
    custom_name: Option<TextComponent>,
    is_animating: bool,
    animation_tick_count: usize,
}

// SAFETY: This key is owned by Steel and uniquely identifies `EndPortalBlockEntity`.
unsafe impl DowncastType for SkullBlockEntity {
    const TYPE_KEY: DowncastTypeKey = DowncastTypeKey::new("steel:block_entity/skull");
}

impl SkullBlockEntity {
    /// Creates a Skull block entity.
    #[must_use]
    pub fn new(level: Weak<World>, pos: BlockPos, state: BlockStateId) -> Self {
        Self {
            base: BlockEntityBase::new(&vanilla_block_entity_types::SKULL, level, pos, state),
            state: SyncMutex::new(SkullState {
                owner: None,
                note_block_sound: None,
                custom_name: None,
                is_animating: false,
                animation_tick_count: 0,
            }),
        }
    }

    pub fn animation(_level: World, _pos: BlockPos, state: BlockStateId, entity: SkullBlockEntity) {
        if state.try_get_value(POWERED).is_some() && state.get_value(POWERED) {
            let mut entity_state = entity.state.lock();
            entity_state.is_animating = true;
            entity_state.animation_tick_count += 1;
        } else {
            let mut entity_state = entity.state.lock();
            entity_state.is_animating = false;
        }
    }

    pub fn get_animation(&self, a: f32) -> f32 {
        let entity_state = self.state.lock();
        if entity_state.is_animating {
            entity_state.animation_tick_count as f32 + a
        } else {
            entity_state.animation_tick_count as f32
        }
    }

    pub fn get_owner_profile(&self) -> Option<ResolvableProfile> {
        self.state.lock().owner.clone()
    }

    pub fn get_note_block_sound(&self) -> Option<Identifier> {
        self.state.lock().note_block_sound.clone()
    }
}

impl BlockEntity for SkullBlockEntity {
    fn base(&self) -> &BlockEntityBase {
        &self.base
    }

    fn load_additional(&self, nbt: &BorrowedNbtCompound<'_>) {
        let nbt: NbtCompoundView<'_, '_> = nbt.into();
        let mut state = self.state.lock();

        if let Ok(profile) = ResolvableProfile::from_optional_nbt_tag(nbt.get(PROFILE_NBT_KEY)) {
            state.owner = profile
        };
        if let Ok(sound) = Identifier::from_optional_nbt_tag(nbt.get(NOTE_BLOCK_SOUND_NBT_KEY)) {
            state.note_block_sound = sound
        };
        if let Ok(name) = TextComponent::from_optional_nbt_tag(nbt.get(CUSTOM_NAME_NBT_KEY)) {
            state.custom_name = name
        };
    }

    fn save_additional(&self, nbt: &mut NbtCompound) {
        let state = self.state.lock();

        nbt.insert(PROFILE_NBT_KEY, state.owner.clone().to_nbt_tag());

        nbt.insert(
            NOTE_BLOCK_SOUND_NBT_KEY,
            state.note_block_sound.clone().to_nbt_tag(),
        );

        nbt.insert(CUSTOM_NAME_NBT_KEY, state.custom_name.clone().to_nbt_tag());
    }

    fn get_update_tag(&self) -> Option<NbtCompound> {
        Some(self.save_custom_only())
    }
}
