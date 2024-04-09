use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn filedetails(
    ctx: Context<'_>,
    #[description = "File to examine"] file: poise::serenity_prelude::Attachment
) -> Result<(), Error> {
    ctx.say(format!("File name: **{}**. File size: **{}** bytes",
        file.filename,
        file.size
    )).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn totalsize(
    ctx: Context<'_>,
    #[description = "File(s) to examine"] files: Vec<poise::serenity_prelude::Attachment>,
) -> Result<(), Error> {
    let total = files.iter().map(|f| f.size as u64).sum::<u64>();

    ctx.say(format!("Total file size: `{}B`. Average size: `{}B`",
        total,
        total.checked_div(files.len() as _).unwrap_or(0)
    )).await?;

    Ok(())
}
