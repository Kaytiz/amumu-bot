pub mod data;
pub mod prelude;

mod command;
mod event_handler;

use prelude::*;
use songbird::SerenityInit;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~!".into()),
                ..Default::default()
            },
            commands: vec![
                command::help(),
                command::owner::register(),
                command::owner::save(),
                command::owner::channel::channel(),
                command::user::user(),
                command::song::song(),
                command::dev::age(),
                command::dev::rand(),
                command::dev::modal(),
                command::dev::add(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::load_default().unwrap_or_default())
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    client
        .start()
        .await
        .unwrap_or_else(|err| panic!("Failed to start bot. {:?}", err));
}