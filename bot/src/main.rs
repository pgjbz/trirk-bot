use std::{env, error::Error};

use connection::twitch_irc::TwitchIrc;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let channel = env::var("TRIRK_CHANNEL").expect("please set twitch channel env var");
    let oauth = env::var("TRIRK_OAUTH").expect("please set twitch oauth env var");
    let nickname = env::var("TRIRK_NICKNAME").expect("please set twitch nickname env var");
    let irc = TwitchIrc::new(channel, oauth, nickname);
    let mut irc_connection = irc.open_connection().await?;
    println!("listen socket");
    //irc_connection.privmsg("sauvi").await?;
    loop {
        match irc_connection.read_next().await {
            Ok(msg) => {
                dbg!(msg);
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
