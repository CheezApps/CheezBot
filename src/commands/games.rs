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
#[prefix = "game"]
#[commands(mafia)]
struct Games;

#[command]
fn mafia(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("Channel id: {}", msg.channel_id.0);

    msg.reply(&ctx.http, "Pong!")?;


    Ok(())
}
