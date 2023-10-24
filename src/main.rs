mod odesli;

use dotenv::dotenv;
use linkify::LinkFinder;
use odesli::get_song_info;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::Client;
use std::env;

struct Handler;

const SUPPORTED_URLS: [&str; 3] = [
    "https://open.spotify.com",
    "https://music.apple.com",
    "https://spotify.link",
];

const DISPLAY_SERVICES: [&str; 4] = ["spotify", "appleMusic", "youtube", "soundcloud"];

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data: Ready) {
        println!("connected <3")
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        let mut finder = LinkFinder::new();
        finder.kinds(&[linkify::LinkKind::Url]);

        let mut all_urls = Vec::new();
        for link in finder.links(message.content.as_str()) {
            all_urls.push(link.as_str().to_owned());
        }

        // find all links in message
        let urls: Vec<&String> = all_urls
            .iter()
            .filter(|url| {
                SUPPORTED_URLS
                    .iter()
                    .any(|supported_url| url.starts_with(supported_url))
            })
            .collect();

        for url in urls {
            message.channel_id.start_typing(&ctx.http).unwrap();
            let song_info = match get_song_info(url).await {
                Ok(val) => val,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let spotify_platform = song_info.links_by_platform.get("spotify");
            if spotify_platform.is_none() {
                continue;
            }

            let spotify_data = song_info
                .entities_by_unique_id
                .get(&spotify_platform.unwrap().entity_unique_id)
                .unwrap();

            let mut content: String = String::new();

            // spotify will always have a title and artist, so
            // we can safely unwrap here
            content.push_str(
                format!(
                    "## {} - {} \n",
                    spotify_data.artist_name.as_ref().unwrap(),
                    spotify_data.title.as_ref().unwrap()
                )
                .as_str(),
            );

            for service in DISPLAY_SERVICES.iter() {
                let platform_info = match song_info.links_by_platform.get(&service.to_string()) {
                    None => {
                        continue;
                    }
                    Some(val) => val,
                };

                let separator = if DISPLAY_SERVICES.iter().position(|x| x == service).unwrap()
                    == DISPLAY_SERVICES.len() - 1
                {
                    ""
                } else {
                    "- "
                };

                content.push_str(
                    format!(
                        "[{}](<{}>) {}",
                        format!(
                            "{}{}",
                            service.chars().next().unwrap().to_uppercase(),
                            &service[1..]
                        ),
                        platform_info.url,
                        separator
                    )
                    .as_str(),
                );
            }

            match message.reply(&ctx.http, content).await {
                Ok(v) => v,
                Err(err) => {
                    println!("error: {}", err);
                    return;
                }
            };
        }

        return ();
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("invalid token");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        println!("error while running client: {:?}", why)
    }
}
