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
    loop {
        let mut irc_connection = irc.clone().open_connection().await?;
        //TODO: handle user notice with extra tag msg-id: resub, maybe msg-id sub
        'message: loop {
            match irc_connection.read_next().await {
                Ok(msg) => match msg.command().command() {
                    CommandType::UserNotice if msg.tags().is_some() => {
                        let tags = msg.tags().as_ref().unwrap();
                        let extra_tags = tags.extra_tags();
                        if let Some(sub) = extra_tags.get("msg-id") {
                            if sub.contains("sub") {
                                let _ = irc_connection.privmsg(&format!(
                                    "{nickname} fez a boa PogChamp",
                                    nickname = tags.display_name()
                                ));
                            }
                        }
                    }
                    CommandType::ClearChat => {
                 
                        if let Some(tags) = msg.tags() {
                            if *tags.ban_duration() > 0 {
                                let _ = irc_connection
                                    .privmsg(&format!(
                                        "{nickname} foi de base por {duration}s",
                                        nickname = msg.parameters().as_ref().map_or(IRINEU.into(), |p| p.replace('\n', "")),
                                        duration = tags.ban_duration()
                                    ))
                                    .await
                                    .map_err(|err| {
                                        eprintln!("ERROR: could not send message privmsg: {err}")
                                    });
                            }
                        }
                    }
                    CommandType::Join => println!(
                        "{} entrou na brincadeira",
                        msg.source().as_ref().map_or(IRINEU.into(), |s| s.nick())
                    ),
                    CommandType::Part(_) => println!(
                        "{} saiu da brincadeira",
                        msg.source().as_ref().map_or(IRINEU.into(), |s| s.nick())
                    ),
                    CommandType::PrivMSG => print!(
                        "{}: {}",
                        msg.source().as_ref().map_or(IRINEU.into(), |s| s.nick()),
                        msg.parameters().as_ref().map_or("", |p| p)
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
                    CommandType::UserState => {
                        dbg!(msg);
                    }

                    CommandType::UserNotice => println!("user notice: {msg:?}"),
                    _ => println!("'{msg:?}'not implemented yet"),
                },
                Err(e) => {
                    eprintln!("{e}");
                    if &e.to_string()[..] == "Parse - empty irc message" {
                        break 'message;
                    }
                }
            }
        }
    }
}
