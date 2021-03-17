use std::{env};
use serde::{Deserialize};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};



const HELP_MESSAGE: &str = "
Hello there, Human!
You have summoned me. Let's see about getting you what you need.
❓ Need technical help?
➡️ Post in the <#811714603766906934> channel and other humans will assist you.
❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>
❓ Something wrong?
➡️ You can flag an admin with @admin
I hope that resolves your issue!
— HelpBot 🤖
";

const HELP_COMMAND: &str = "_help";
const LATINIFY_COMMAND: &str = "_latinify";

struct Handler;

#[derive(Deserialize)]
struct Latinify {
    msg: String,
}

async fn latinify(msg: String) -> Result<String, Box<dyn std::error::Error>> {
    let request_url = format!("http://latinify-service:8000/{}/", msg);
    let resp = reqwest::get(&request_url)
        .await?;
    let latinify: Latinify = resp.json().await?;
    return Ok(latinify.msg)
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == LATINIFY_COMMAND {
            let latinify_res = latinify(msg.content).await.unwrap();

            
            if let Err(why) = msg.channel_id.say(&ctx.http, latinify_res).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}