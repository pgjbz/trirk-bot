use derive_builder::Builder;

#[derive(PartialEq, Eq, Debug)]
pub struct TwitchMessage {
    parameters: String,
    command: Command,
    source: Source,
    tags: Option<Tags>,
}

impl TwitchMessage {
    #[inline(always)]
    pub fn new<T: Into<String>>(
        parameters: T,
        command: Command,
        source: Source,
        tags: Option<Tags>,
    ) -> Self {
        Self {
            parameters: parameters.into(),
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
}
