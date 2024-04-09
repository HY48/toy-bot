// Shoutouts to Kanglioo for 

use super::{commands, utils};
use crate::{serenity, Error, Data};

use poise::{Framework, FrameworkOptions};

const MESSAGE_PREFIX: &str = "!";

pub async fn init_bot(token: &str) -> Result<(), Error> {

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::ping(),
                commands::filedetails(),
                commands::totalsize(),
                commands::getcolorscheme()
            ],
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some(String::from(MESSAGE_PREFIX)), 
                edit_tracker: Some(std::sync::Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600)))),
                ..Default::default()
            },
            on_error: |e| {
                Box::pin(utils::on_error(e))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::MESSAGE_CONTENT;
    
    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();

    Ok(())
}

