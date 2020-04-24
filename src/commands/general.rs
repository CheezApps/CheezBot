use serenity::prelude::Context;
use serenity::model::channel::Message;

use serenity::framework::standard::{
    CommandResult,
    macros::{
        group,
        command,
    },
};

#[group]
#[commands(ping)]
struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong!")?;


    Ok(())
}
