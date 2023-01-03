use dotenv::dotenv;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult, Args};
use alphavantage::Client;

#[command]
async fn stock(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    dotenv().ok();
    let token = std::env::var("API_ALPHA_VANTAGE").expect("nao achei");
    let alpha_client = Client::new(&token);

    let stock_id = args.single::<String>()?;

    let time_series = alpha_client.get_time_series_weekly(&stock_id).await.unwrap();
    let entry = time_series.entries.last().unwrap().to_owned();
    let enviar = format!(" Data: {},\nAbertura: {},\nFechamento: {},\nMaior Valor: {},\nMenor Valor: {},\nVolume: {} 
    ", entry.date, entry.open, entry.close, entry.high, entry.low, entry.volume);
    msg.channel_id.say(&ctx.http, enviar).await?;
    
    Ok(())
}