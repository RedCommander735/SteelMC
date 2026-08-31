//! Vanilla data command.

use steel_utils::{Identifier};
use crate::command::brigadier::CommandNodeBuilder;
use crate::command::execution::SteelCommandRuntime;
use super::super::{
    execution::{
        CommandSource
    },
    registration::CommandRegistration,
};

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("clear"), |_| command())
}

fn command() -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    todo!()
}