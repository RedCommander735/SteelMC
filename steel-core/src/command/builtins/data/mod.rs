//! Vanilla data command.

mod block_accessor;
mod entity_accessor;
mod storage_accessor;

use super::super::{
    execution::{
        CommandSource, SteelArgumentType, SteelCommandContext, SteelCommandRuntime, argument,
        literal,
    },
    registration::CommandRegistration,
};
use crate::command::brigadier::{ArgumentType, CommandNodeBuilder, CommandSyntaxError};
use crate::entity::ai::path::Path;
use simdnbt::owned::{NbtList, NbtTag};
use steel_utils::nbt::NbtPath;
use steel_utils::{BlockPos, Identifier, translations};
use text_components::TextComponent;

type Builder = CommandNodeBuilder<CommandSource, SteelCommandRuntime>;
type Ctx = SteelCommandContext<CommandSource>;

pub(super) const PATH_ARG: &str = "path";
pub(super) const SCALE_ARG: &str = "scale";

const GET_ACCESSORS: [fn() -> Builder; 1] = [block_accessor::get];

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("data"), |_| command())
}

fn command() -> Builder {
    literal("data").then(
        GET_ACCESSORS
            .into_iter()
            .fold(literal("get"), |builder, get| builder.then(get())),
    )
}

pub(super) fn path_scale_args(
    get_data_from_path: fn(context: &Ctx) -> Result<i32, CommandSyntaxError>,
    get_numeric_value: fn(context: &Ctx) -> Result<i32, CommandSyntaxError>,
) -> Builder {
    argument(PATH_ARG, SteelArgumentType::nbt_path())
        .executes(get_data_from_path)
        .then(
            argument(SCALE_ARG, ArgumentType::double(f64::MIN, f64::MAX))
                .executes(get_numeric_value),
        )
}

pub(super) fn process_numeric_arg(
    tag: NbtTag,
    path: &NbtPath,
    scale: f64,
) -> Result<i32, CommandSyntaxError> {
    let val: f64 = match tag {
        NbtTag::Byte(b) => f64::from(b),
        NbtTag::Short(s) => f64::from(s),
        NbtTag::Int(i) => f64::from(i),
        NbtTag::Long(l) => l as f64,
        NbtTag::Float(f) => f64::from(f),
        NbtTag::Double(d) => d,
        _ => {
            return Err(CommandSyntaxError::dynamic(
                translations::COMMANDS_DATA_GET_INVALID
                    .message([TextComponent::plain(path.as_str().to_string())]),
            ));
        }
    };

    Ok((val * scale).floor() as i32)
}

fn process_path_arg(tag: NbtTag) -> i32 {
    match tag.clone() {
        NbtTag::Byte(int) => i32::from(int),
        NbtTag::Short(int) => i32::from(int),
        NbtTag::Int(int) => int,
        NbtTag::Long(int) => int as i32,
        NbtTag::Float(float) => float.floor() as i32,
        NbtTag::Double(float) => float.floor() as i32,
        NbtTag::List(list) => match &list {
            NbtList::Empty => 0,
            NbtList::Byte(v) => v.len() as i32,
            NbtList::Short(v) => v.len() as i32,
            NbtList::Int(v) => v.len() as i32,
            NbtList::Long(v) => v.len() as i32,
            NbtList::Float(v) => v.len() as i32,
            NbtList::Double(v) => v.len() as i32,
            NbtList::ByteArray(v) => v.len() as i32,
            NbtList::String(v) => v.len() as i32,
            NbtList::List(v) => v.len() as i32,
            NbtList::Compound(v) => v.len() as i32,
            NbtList::IntArray(v) => v.len() as i32,
            NbtList::LongArray(v) => v.len() as i32,
        },
        NbtTag::ByteArray(arr) => arr.len() as i32,
        NbtTag::IntArray(arr) => arr.len() as i32,
        NbtTag::LongArray(arr) => arr.len() as i32,
        NbtTag::Compound(comp) => comp.len() as i32,
        NbtTag::String(str) => str.len() as i32,
    }
}
