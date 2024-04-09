// use std::collections::HashMap;
use crate::{Context, Error};

// use rand::Rng;
// use serde_json::json;


#[poise::command(prefix_command, slash_command)]
pub async fn getcolorscheme(ctx: Context<'_>) -> Result<(), Error> {
    todo!()

    // let range = rand::distributions::Uniform::from(0..=255);
    // let values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(3).collect();
    // let res = reqwest::get(format!("https://www.thecolorapi.com/scheme?mode={}&count={}&rgb=({},{},{})", 
    //     "triad", 
    //     "3", 
    //     values[0], values[1], values[2]
    // ))
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;

    // Ok(())
}