use derive_builder::Builder;

#[derive(PartialEq, Eq, Debug)]
pub struct TwitchMessage {
    parameters: Option<String>,
    command: Command,
    source: Option<Source>,
    tags: Option<Tags>,
}

impl TwitchMessage {
    #[inline(always)]
    pub fn new<T: Into<String>>(
        parameters: Option<T>,
        command: Command,
        source: Option<Source>,
        tags: Option<Tags>,
    ) -> Self {
        Self {
            parameters: if parameters.is_some() {
                Some(parameters.unwrap().into())
            } else {
                None
            },
            command,
            source,
            tags,
        }
    }
}

#[derive(Builder, Clone, PartialEq, Eq, Debug)]
#[builder(setter(into))]
pub struct Tags {
    badges: Badge,
    color: String,
    display_name: String,
    emote_only: bool,
    emotes: Option<Vec<Emote>>,
    id: String,
    r#mod: bool,
    room_id: String,
    subscriber: bool,
    turbo: bool,
    tmi_sent_ts: usize,
    user_id: String,
    user_type: String,
    vip: bool,
    reply_parent_msg_id: String,
}

impl Tags {
    pub fn builder() -> TagsBuilder {
        let mut builder = TagsBuilder::default();
        builder
            .vip(false)
            .badges(Badge::default())
            .color("")
            .display_name("")
            .emote_only(false)
            .emotes(vec![])
            .r#mod(false)
            .room_id("")
            .subscriber(false)
            .turbo(false)
            .tmi_sent_ts(usize::MIN)
            .user_id("")
            .user_type("")
            .reply_parent_msg_id("");
        builder
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Emote {
    emote_code: String,
    start_position: usize,
    end_position: usize,
}

impl Emote {
    #[inline(always)]
    pub fn new<T: Into<String>>(emote_code: T, start_position: usize, end_position: usize) -> Self {
        Self {
            emote_code: emote_code.into(),
            start_position,
            end_position,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Command {
    command: CommandType,
    channel: String,
}

impl Command {
    #[inline(always)]
    pub fn new<T: Into<String>>(command: CommandType, channel: T) -> Self {
        Self {
            command,
            channel: channel.into(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CommandType {
    PrivMSG,
    Part,
    Notice,
    ClearChat,
    HostTarget,
    Ping,
    Cap,
    GlobalUserState,
    UserState,
    RoomState,
    Reconnect,
    Numeric,
    Unknown,
}

impl From<&str> for CommandType {
    fn from(value: &str) -> Self {
        match value {
            "PRIVMSG" => Self::PrivMSG,
            "PART" => Self::Part,
            "NOTICE" => Self::Notice,
            "CLEARCHAT" => Self::ClearChat,
            "HOSTTARGEtT" => Self::HostTarget,
            "PING" => Self::Ping,
            "CAP" => Self::Cap,
            "GLOBALUSERSTATE" => Self::GlobalUserState,
            "USERSTATE" => Self::UserState,
            "ROOMSTAT" => Self::RoomState,
            "RECONNECT" => Self::Reconnect,
            _ if value.parse::<usize>().is_ok() => Self::Numeric,
            _ => Self::Unknown,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Badge {
    pub admin: Option<String>,
    pub bits: Option<String>,
    pub broadcaster: Option<String>,
    pub moderator: Option<String>,
    pub subscriber: Option<String>,
    pub staff: Option<String>,
    pub turbo: Option<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Source {
    nick: String,
    host: String,
}

impl Source {
    #[inline(always)]
    pub fn new<T: Into<String>>(nick: T, host: T) -> Self {
        Self {
            nick: nick.into(),
            host: host.into(),
        }
    }

    pub fn nick(&self) -> String {
        self.nick.clone()
    }
}
