use derive_builder::Builder;
use derive_getters::Getters;

#[derive(PartialEq, Eq, Debug, Getters)]
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
            parameters: parameters.map(|parameters| parameters.into()),
            command,
            source,
            tags,
        }
    }
}

#[derive(Builder, Clone, PartialEq, Eq, Debug, Getters)]
#[builder(setter(into))]
pub struct Tags {
    badges: Badge,
    color: String,
    display_name: String,
    emote_only: bool,
    emotes: Vec<Emote>,
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
    target_user_id: String,
    message_id: String,
    ban_duration: usize,
    login: String,
    target_message_id: String,
    emote_sets: Vec<usize>,
    followers_only: bool,
    r9k: bool,
    slow: usize,
    subs_only: bool,
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
            .target_user_id("")
            .message_id("")
            .reply_parent_msg_id("")
            .login("")
            .ban_duration(0usize)
            .target_message_id("")
            .emote_sets(vec![])
            .followers_only(false)
            .r9k(false)
            .slow(usize::MIN)
            .subs_only(false)
            .id("");
        builder
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Getters)]
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

#[derive(PartialEq, Eq, Debug, Clone, Getters)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CommandType {
    PrivMSG,
    Part,
    Join,
    Notice,
    ClearChat,
    HostTarget,
    ClearMessage,
    Ping,
    Cap,
    GlobalUserState,
    UserState,
    RoomState,
    Reconnect,
    Numeric(u16),
    Unknown(String),
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
            "ROOMSTATE" => Self::RoomState,
            "RECONNECT" => Self::Reconnect,
            "CLEARMSG" => Self::ClearMessage,
            "JOIN" => Self::Join,
            v if value.parse::<u16>().is_ok() => Self::Numeric(v.parse().unwrap()),
            cmd => Self::Unknown(cmd.into()),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Getters)]
pub struct Badge {
    admin: Option<String>,
    bits: Option<String>,
    broadcaster: Option<String>,
    moderator: Option<String>,
    subscriber: Option<String>,
    staff: Option<String>,
    turbo: Option<String>,
}

impl Badge {
    pub fn set_admin(&mut self, admin: String) {
        self.admin = Some(admin);
    }

    pub fn set_bits(&mut self, bits: String) {
        self.bits = Some(bits);
    }

    pub fn set_broadcaster(&mut self, broadcaster: String) {
        self.broadcaster = Some(broadcaster);
    }

    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = Some(moderator);
    }

    pub fn set_subscriber(&mut self, subscriber: String) {
        self.subscriber = Some(subscriber);
    }

    pub fn set_staff(&mut self, staff: String) {
        self.staff = Some(staff);
    }

    pub fn set_turbo(&mut self, turbo: String) {
        self.turbo = Some(turbo);
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

    pub fn host(&self) -> String {
        self.host.clone()
    }
}
