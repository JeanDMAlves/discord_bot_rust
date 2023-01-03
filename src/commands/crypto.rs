use alpha_vantage::ApiClient;
use alpha_vantage::crypto::Crypto;
use dotenv::dotenv;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult, Args};

#[command]
async fn crypto(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    dotenv().ok();
    
    let token: String = std::env::var("API_ALPHA_VANTAGE").expect("nao achei");
    let crypto_id: String = args.single::<String>()?;    
    let api_key: ApiClient = alpha_vantage::set_api(&token, reqwest::Client::new());

    let results: Crypto = api_key
        .crypto(alpha_vantage::crypto::CryptoFunction::Daily, &crypto_id, "BRL")
        .json()
        .await
        .unwrap();
    
    let last_refreshed: &str = results.last_refreshed();

    for entry in results.data(){
        if &last_refreshed[0..=9] == entry.time(){
            let enviar: String = format!(
                "Data: {},\nAbertura: {},\nFechamento: {},\nMaior Valor: {},\nMenor Valor: {},\nVolume: {}",
                entry.time(), entry.market_open(), entry.market_close(), entry.market_high(), entry.market_low(), entry.volume());
            msg.channel_id.say(&ctx.http, enviar).await?;
            break;
        }
    } 
    Ok(())
}