use std::{
    io::{Result, Write, Read},
    marker::PhantomData,
    ops::{Deref, DerefMut}, net::TcpStream,
};

use parser::{trirk_parser::TrirkParser, TwitchMessage};

use crate::error::TrirkError;

use self::config::TwitchConfig;

pub mod config;

const IRC_HOST: &str = "irc.chat.twitch.tv";
const IRC_PORT: usize = 6667;
const CAP_REQ: &str = "CAP REQ :twitch.tv/";

#[derive(Clone)]
pub struct ClosedConnection;
pub struct OpenedConnection(TcpStream);

impl Deref for OpenedConnection {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OpenedConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct TwitchIrc<T = ClosedConnection> {
    configuration: TwitchConfig,
    connection: T,
    _marker: PhantomData<T>,
}

impl TwitchIrc<ClosedConnection> {
    #[inline(always)]
    pub fn new(config: TwitchConfig) -> Self {
        Self {
            configuration: config,
            connection: ClosedConnection,
            _marker: PhantomData,
        }
    }

    pub async fn open_connection(self) -> Result<TwitchIrc<OpenedConnection>> {
        println!(
            "opening connection for channel '{}', with nickname '{}'",
            self.configuration.channel, self.configuration.nickname
        );
        let mut connection = TcpStream::connect(format!("{}:{}", IRC_HOST, IRC_PORT))?;

        connection
            .write_all(
                format!(
                    "PASS {pass}\r\nNICK {user}\r\nJOIN #{join}\r\n{CAP_REQ}{command}\r\n{CAP_REQ}{membership}\r\n{CAP_REQ}{tags}\r\n",
                    pass = self.configuration.oauth,
                    user = self.configuration.nickname,
                    join = self.configuration.channel,
                    command = "commands",
                    membership = "membership",
                    tags = "tags"
                )
                .as_bytes(),
            )?;
        connection.flush()?;
        let irc = TwitchIrc::<OpenedConnection> {
            configuration: self.configuration,
            connection: OpenedConnection(connection),
            _marker: PhantomData,
        };
        Ok(irc)
    }
}

const PARSER: TrirkParser = TrirkParser::new();

impl TwitchIrc<OpenedConnection> {
    pub fn send_bytes(&mut self, message: &[u8]) -> Result<()> {
        self.connection.write_all(message)?;
        Ok(())
    }

    pub fn privmsg(&mut self, message: &str) -> Result<()> {
        self.send_bytes(
            format!("PRIVMSG #{} :{}\r\n", self.configuration.channel, message).as_bytes(),
        )
    }

    pub fn read_next(&mut self) -> std::result::Result<TwitchMessage, TrirkError> {
        let mut buffer = vec![0; 1024];
        match self.connection.read(&mut buffer) {
            Ok(size) => {
                buffer.truncate(size);
                let message = String::from_utf8(buffer)?;
                let twitch_message = PARSER.parse(message)?;
                Ok(twitch_message)
            }
            Err(e) => Err(e)?,
        }
    }

    pub fn pong(&mut self) -> Result<()> {
        self.send_bytes(b"PONG\r\n")
    }
}

#[cfg(test)]
mod tests {}
