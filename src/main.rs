use serenity::{client::{bridge::gateway::GatewayIntents, Client, Context, EventHandler}, model::{prelude::Ready, channel::Message}, async_trait};
use dotenv;

pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
	async fn ready(&self, _context: Context, ready: Ready) {
		println!("Readied as {}#{} ({})", ready.user.name, ready.user.discriminator, ready.user.id);
	}

    async fn message(&self, context: Context, message: Message) {
		if message.author.bot {
			return;
		}

		if message.content.to_lowercase().trim_end_matches(|x| !char::is_alphabetic(x)).ends_with("indeed") {
			let channel = match context.cache.guild_channel(message.channel_id).await {
				Some(channel) => channel,
				None => return,
			};

			if let Ok(permissions) = channel.permissions_for_user(&context.cache, context.cache.current_user_id().await).await {
				if permissions.send_messages() {
					if let Err(_) = message.reply(&context.http, ".com").await {
						if let Err(err) = message.channel_id.send_message(&context.http, |msg| msg.content(".com")).await {
							eprintln!("Failed to send the funny, {}", err);
						}
					}
				}
			}
		}
	}
}

/* https://discord.com/oauth2/authorize?client_id=961049670921117717&permissions=8&scope=bot */

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let bot_secret: String = dotenv::var("BOT_SECRET").expect("Set BOT_SECRET in .env");

	let mut client: Client = Client::builder(bot_secret).intents(GatewayIntents::all()).event_handler(BotEventHandler).await.expect("Couldn't create client");

	if let Err(err) = client.start().await {
		eprintln!("Client error'd: {:?}", err);
	}
}
