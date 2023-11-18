use std::{env, error::Error};

use connection::{
    twitch::CommandType,
    twitch_irc::{config::TwitchConfig, TwitchIrc},
};
use dotenv::dotenv;

const IRINEU: &str = "irineu, você não sabe nem eu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let channel = env::var("TRIRK_CHANNEL").expect("please set twitch channel env var");
    let oauth = env::var("TRIRK_OAUTH").expect("please set twitch oauth env var");
    let nickname = env::var("TRIRK_NICKNAME").expect("please set twitch nickname env var");
    let configuration = TwitchConfig::new(nickname, oauth, channel);
    let irc = TwitchIrc::new(configuration);
    let mut irc_connection = irc.open_connection().await?;
    println!("listen socket");
    loop {
        match irc_connection.read_next().await {
            Ok(msg) => match msg.command().command() {
                CommandType::Join => println!(
                    "{} entrou na brincadeira",
                    msg.source().clone().map_or(IRINEU.into(), |s| s.nick())
                ),
                CommandType::Part => println!(
                    "{} saiu da brincadeira",
                    msg.source().clone().map_or(IRINEU.into(), |s| s.nick())
                ),
                CommandType::PrivMSG => print!(
                    "{}: {}",
                    msg.source().clone().map_or(IRINEU.into(), |s| s.nick()),
                    msg.parameters().clone().map_or("".into(), |p| p)
                ),
                CommandType::Numeric(n) => match *n {
                    1 => {
                        println!("bot run");
                        //let _ = irc_connection.privmsg("testando envio de msg do meu botzin em Rust ao entrar em um canal").await;
                    }
                    3 => println!("server bem novinho uhuu"),
                    _ => eprintln!("unknown numeric command '{}': {msg:?}", n),
                },
                CommandType::Ping => {
                    let _ = irc_connection
                        .pong()
                        .await
                        .map_err(|err| eprintln!("ERROR: could not send pong message: {err}"));
                }
                CommandType::UserState => println!(
                    "user state?... {}",
                    msg.tags()
                        .clone()
                        .map_or("".into(), |tag| tag.display_name().clone())
                ),
                _ => println!("'{msg:?}'not implemented yet"),
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}
