use poise::serenity_prelude as serenity;

mod commands;
mod utils;
mod framework;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main()-> Result<(), Error>{
    env_logger::init();
    // Parse token from 
    let token = discord_token()?;
    println!("{}", &token);
    framework::init_bot(&token).await?;
    
    Ok(())
}


pub fn discord_token() -> Result<String, Error>{
    match dotenv::var("DISCORD_TOKEN") {
        Ok(token) => return Ok(token),
        Err(e) => return Err(e.into()),
    }
}