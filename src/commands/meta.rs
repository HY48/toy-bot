use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = std::time::Instant::now();
    let msg = ctx.say("Calculating ping...").await?;
    let end_time = std::time::Instant::now();

    msg.edit(
        ctx,
        poise::CreateReply::default().content(format!("ping: {} ms", (end_time - start_time).as_millis())),
    ).await?;
    Ok(())
}


// TODO: Implement command to clear channel of messages