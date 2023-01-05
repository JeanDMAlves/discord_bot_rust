use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult, Args};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CEP{
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade:String,
    pub uf: String,
    pub ibge: String,
    pub gia: String,
    pub ddd: String,
    pub siafi: String
}

#[command]
async fn cep(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let cep_value: String = args.single::<String>()?;
    let regex = Regex::new("[0-9]{5}-?[0-9]{3}");
    let resultado: bool = regex.expect("Deu ruim").is_match(&cep_value);

    if resultado{

        let url_base: String = format!("https://viacep.com.br/ws/{}/json/", cep_value);
        let body: CEP = reqwest::get(url_base).await?.json().await?;
        let response:String = format!(
            "Cep: {},\nlogradouro: {},\ncomplemento:{},\nbairro: {},\nlocalidade: {},\nuf: {},\nibge: {},\ngia: {},\nddd: {},\nsiafi: {}", 
            body.cep, body.logradouro, body.complemento, body.bairro, body.localidade, body.uf, body.ibge, body.gia, body.ddd, body.siafi,);
        
        msg.channel_id.say(&ctx.http, response).await?;
    } else{
        msg.channel_id.say(&ctx.http, "O Cep informado não é válido").await?;
    }
    Ok(())
}