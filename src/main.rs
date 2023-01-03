// use alphavantage::Client as alpha_client;
use dotenv::dotenv;
mod commands;

use crate::commands::{
    about::*,
    ping::*,
    stock::*,
    crypto::*,
};

use serenity::{
    async_trait,
    model::{
        gateway::Ready},
    prelude::*,
    Client,
    framework::standard::{
        macros::group,
        StandardFramework
    }
};

struct Handler;
#[group]
#[commands(about, ping, stock, crypto)]
struct General;

#[async_trait]
impl EventHandler for Handler{
    async fn ready(&self, _: Context, ready: Ready){
        println!("{} conectado", ready.user.name)
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("nao achei");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    
    if let Err(why) = client.expect("Algum erro aconteceu").start().await {    
        println!("Erro: {:?}", why);
    }
}
