use serenity::{
    async_trait,
    client::EventHandler,
    model::channel::Message,
    model::prelude::Ready,
    prelude::{Context, GatewayIntents},
};
use utils::{create_shorten, get_env};

mod utils;

struct Handler;

#[tokio::main]
async fn main() {
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::Client::builder(get_env("DISCORD_TOKEN"), intents)
        .event_handler(Handler)
        .await
        .expect("Failed to generate client");

    if let Err(why) = client.start().await {
        print!("An error occurred while running the client: {why:?}")
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready! {}({})", ready.user.tag(), ready.user.id)
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.is_private() {
            return;
        }

        if !(msg.content.starts_with("!link")) {
            return;
        }

        let context: Vec<&str> = msg.content.split(' ').collect();
        let mut link = context[1].to_string();

        if !(link.starts_with("https://") || link.starts_with("http://")) {
            return;
        }

        link = create_shorten(link.to_string()).await;

        if let Err(why) = msg.reply(&ctx.http, link).await {
            println!("Error sending message: {:?}", why);
        }

        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error deleting message: {:?}", why);
        }
    }
}
