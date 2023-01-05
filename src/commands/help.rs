use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult};

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help:&str = "Para utilizar qualquer comando do bot basta colocar o caracter '!' antes do comando.\n
    Os comandos disponíveis para esse bot são:\n
    help -> Retorna todas as funções disponíveis para o bot e um exemplo de como usá-las;\n
    About -> Retorna informações sobre o Bot.;\n
    Crypto (moeda) -> Retorna informações sobre alguma cryptomoeda. Exemplo: !crypto BTC;\n
    stock (ação) -> Retorna informações sobre alguma ação. Exemplo: !stock IBM;\n
    cep (cep) -> Retorna informações sobre algum CEP em específico. Exemplo: !cep 96203-900;\n
    todo list -> Retorna todas as tarefas na ToDo List do usuário. Exemplo: !todo list;\n
    todo add (tarefa) -> Adiciona uma tarefa na ToDo List do usuário. Exemplo: !todo add Varrer a casa;\n
    todo remove (tarefa) -> remove uma tarefa da ToDo List do usuário. Exemplo: !todo remove Varrer a casa;\n
    ";
    msg.channel_id.say(&ctx.http, help).await?;
    return Ok(())
}