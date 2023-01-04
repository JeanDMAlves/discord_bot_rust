use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult, Args};
use std::fmt::Write;

#[command]
async fn todo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id: i64 = msg.author.id.0 as i64;
    let command: String = args.single::<String>()?;
    let prefix: String = "!todo ".to_owned() + &command.to_owned() + " ";
    let task_description: Option<&str> =  msg.content.strip_prefix(&prefix);

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    if command == "add"{
        let teste = sqlx::query!(
            "INSERT INTO todo (task, user_id) VALUES (?, ?)",
            task_description, 
            user_id
        ).execute(&database).await.unwrap();
        println!("{:?}", teste);
        msg.channel_id.say(&ctx, "Item adicionado com sucesso!").await.unwrap();
    } 
    else if command == "remove"{
        let resultado = sqlx::query!(
            "DELETE FROM todo WHERE user_id = ? AND task = ?",
            user_id, task_description
        ).execute(&database).await.unwrap();
        if resultado.rows_affected() > 0 {
            msg.channel_id.say(&ctx, "Item removido com sucesso!").await.unwrap();
        } else{
            msg.channel_id.say(&ctx, "Não consegui remover o item!").await.unwrap();
        };     
    } 
    else if command == "list"{
        let todos = sqlx::query!("SELECT task FROM todo WHERE user_id = ?", user_id)
            .fetch_all(&database)
            .await
            .unwrap();

        let mut response = format!("Você tem {} tarefas pendentes:\n", todos.len());

        for (i, todo) in todos.iter().enumerate(){
            writeln!(response, "{}. {}", i + 1, todo.task).unwrap();
        };

        msg.channel_id.say(&ctx, response).await.unwrap();
    } 
    else {
        msg.channel_id.say(&ctx, "Comando desconhecido").await.unwrap();
    };

    Ok(())
}