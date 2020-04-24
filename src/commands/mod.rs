use std::collections::HashSet;
use serenity::framework::standard::{
    Args,
    StandardFramework,
    CommandResult,
    CommandGroup,
    HelpOptions,
    help_commands,
    macros::help,
};
use serenity::model::{
    channel::Message,
    id::UserId,
};
use serenity::prelude::Context;

mod games;

#[help]
fn help_me(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

pub fn get_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|command| command
            .with_whitespace(true)
            .prefix("!cheez")
        )
        .group(&games::GAMES_GROUP)
        .help(&HELP_ME)
}
