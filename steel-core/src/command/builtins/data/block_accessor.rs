use super::super::super::execution::{
    CommandSource, SteelArgumentType, SteelCommandContext, SteelCommandRuntime, argument, literal,
};
use crate::command::brigadier::{CommandNodeBuilder, CommandSyntaxError};
use crate::command::builtins::data::{
    PATH_ARG, SCALE_ARG, path_scale_args, process_numeric_arg, process_path_arg,
};
use simdnbt::ToNbtTag;
use simdnbt::owned::NbtTag;
use steel_utils::nbt::NbtPath;
use steel_utils::text::command_nbt_component;
use steel_utils::{BlockPos, translations};
use text_components::TextComponent;

type Builder = CommandNodeBuilder<CommandSource, SteelCommandRuntime>;

const ACCESSOR_KEYWORD: &str = "block";
const POSITION_ARG: &str = "targetPos";

pub(super) fn get() -> Builder {
    literal(ACCESSOR_KEYWORD).then(
        argument(POSITION_ARG, SteelArgumentType::block_pos())
            .executes(get_data)
            .then(path_scale_args(get_tag_from_path, get_numeric_value)),
    )
}

fn get_data(context: &SteelCommandContext<CommandSource>) -> Result<i32, CommandSyntaxError> {
    let coordinates = context.coordinates(POSITION_ARG)?;
    let block_pos = coordinates.block_pos(context.source());

    if let Some(entity) = context.source().world().get_block_entity(block_pos) {
        let tag = entity.save_with_full_metadata().to_nbt_tag();

        context
            .source()
            .send_success(&get_print_success(&tag, &block_pos), false);
        return Ok(1);
    }

    Err(CommandSyntaxError::dynamic(TextComponent::from(
        &translations::COMMANDS_DATA_BLOCK_INVALID,
    )))
}

fn get_single_tag(
    context: &SteelCommandContext<CommandSource>,
) -> Result<(NbtTag, BlockPos, &NbtPath), CommandSyntaxError> {
    let coordinates = context.coordinates(POSITION_ARG)?;
    let block_pos = coordinates.block_pos(context.source());
    let path = context.nbt_path(PATH_ARG)?;

    let tag = if let Some(entity) = context.source().world().get_block_entity(block_pos) {
        let tag = entity.save_with_full_metadata().to_nbt_tag();

        let tags = path.get(&tag);
        if tags.is_empty() {
            return Err(CommandSyntaxError::dynamic(
                translations::COMMANDS_DATA_GET_UNKNOWN
                    .message([TextComponent::plain(path.as_str().to_string())])
                    .component(),
            ));
        }
        if tags.len() > 1 {
            return Err(CommandSyntaxError::dynamic(
                &translations::COMMANDS_DATA_GET_MULTIPLE,
            ));
        }

        tags[0].clone()
    } else {
        return Err(CommandSyntaxError::dynamic(TextComponent::from(
            &translations::COMMANDS_DATA_BLOCK_INVALID,
        )));
    };

    Ok((tag, block_pos, path))
}

fn get_tag_from_path(
    context: &SteelCommandContext<CommandSource>,
) -> Result<i32, CommandSyntaxError> {
    let (tag, block_pos, ..) = get_single_tag(context)?;

    context
        .source()
        .send_success(&get_print_success(&tag, &block_pos), false);

    Ok(process_path_arg(tag))
}

fn get_numeric_value(
    context: &SteelCommandContext<CommandSource>,
) -> Result<i32, CommandSyntaxError> {
    let (tag, block_pos, path) = get_single_tag(context)?;
    let scale = context.double(SCALE_ARG)?;

    let result = process_numeric_arg(tag, &path, scale)?;

    context.source().send_success(
        &get_print_success_scaled(path, &block_pos, scale, result),
        false,
    );

    Ok(result)
}

fn get_print_success(data: &NbtTag, pos: &BlockPos) -> TextComponent {
    translations::COMMANDS_DATA_BLOCK_QUERY
        .message([
            TextComponent::plain(pos.x().to_string()),
            TextComponent::plain(pos.y().to_string()),
            TextComponent::plain(pos.z().to_string()),
            command_nbt_component(data, false),
        ])
        .component()
}

fn get_print_success_scaled(path: &NbtPath, pos: &BlockPos, scale: f64, val: i32) -> TextComponent {
    translations::COMMANDS_DATA_BLOCK_GET
        .message([
            TextComponent::plain(path.as_str().to_string()),
            TextComponent::plain(pos.x().to_string()),
            TextComponent::plain(pos.y().to_string()),
            TextComponent::plain(pos.z().to_string()),
            TextComponent::plain(format!("{scale:.2}")),
            TextComponent::plain(val.to_string()),
        ])
        .component()
}

fn merge() -> Builder {
    literal(ACCESSOR_KEYWORD)
}

fn modify() -> Builder {
    todo!()
}

fn remove() -> Builder {
    todo!()
}
