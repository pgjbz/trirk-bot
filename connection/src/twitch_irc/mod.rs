use parser::{TwitchMessage, parser::TrirkParser};
use std::io::Result;
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::TcpStream,
};

const IRC_HOST: &'static str = "irc.chat.twitch.tv";
const IRC_PORT: usize = 6667;

pub struct TwitchIrc<T> {
    channel: String,
    oauth: String,
    nickname: String,
    connection: T,
}

impl TwitchIrc<()> {
    #[inline(always)]
    pub fn new<S: Into<String>>(channel: S, oauth: S, nickname: S) -> Self {
        Self {
            channel: channel.into(),
            oauth: oauth.into(),
            nickname: nickname.into(),
            connection: (),
        }
    }

    pub async fn open_connection(self) -> Result<TwitchIrc<TcpStream>> {
        println!("opening connection for channel '{}', with nickname '{}'", self.channel, self.nickname);
        let mut connection = TcpStream::connect(format!("{}:{}", IRC_HOST, IRC_PORT)).await?;
        
        connection
        .write_all(format!("PASS {}\r\nNICK {}\r\nJOIN #{}\r\n", self.oauth, self.nickname, self.channel).as_bytes())
        .await?;
        connection
        .write_all(b"CAP REQ :twitch.tv/commands\r\n")
        .await?;
        connection
        .write_all(b"CAP REQ :twitch.tv/membership\r\n")
        .await?;
        connection.write_all(b"CAP REQ :twitch.tv/tags\r\n").await?;
        connection.flush().await?;
        let irc = TwitchIrc::<TcpStream> {
            channel: self.channel,
            oauth: self.oauth,
            nickname: self.nickname,
            connection,
        };
        Ok(irc)
    }


}

impl TwitchIrc<TcpStream> {
    pub async fn send_command(&mut self, message: &str) -> Result<()> {
        self.connection.writable().await?;
        self.connection.write_all(message.as_bytes()).await?;
        Ok(())
    }

    pub async fn read_next(&mut self) -> Result<TwitchMessage> {
        let mut buffer = vec![0; 1024];
        self.connection.readable().await?;
        match self.connection.read(&mut buffer).await {
            Ok(size) => {
                buffer.truncate(size);
                let message = String::from_utf8_lossy(&buffer);
                let trirk_parser = TrirkParser::new();
                let twitch_message = trirk_parser.parse(message);
                Ok(twitch_message)
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn foo() {
        let irc = TwitchIrc::new("", "", "");
        irc.open_connection();
    }
}