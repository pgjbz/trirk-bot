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
            parameters: parameters.map(|parameters| parameters.into()),
            command,
            source,
            tags,
        }
    }

    pub fn parameters(&self) -> Option<String> {
        self.parameters.clone()
    }
    pub fn command(&self) -> Command {
        self.command.clone()
    }
    pub fn source(&self) -> Option<Source> {
        self.source.clone()
    }
    pub fn tags(&self) -> Option<Tags> {
        self.tags.clone()
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
    target_user_id: String,
    message_id: String,
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
            .id("");
        builder
    }

    pub fn badges(&self) -> Badge {
        self.badges.clone()
    }

    pub fn color(&self) -> String {
        self.color.clone()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn emote_only(&self) -> bool {
        self.emote_only
    }

    pub fn emotes(&self) -> Option<Vec<Emote>> {
        self.emotes.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn r(&self) -> bool {
        self.r#mod
    }

    pub fn room_id(&self) -> String {
        self.room_id.clone()
    }

    pub fn subscriber(&self) -> bool {
        self.subscriber
    }

    pub fn turbo(&self) -> bool {
        self.turbo
    }

    pub fn tmi_sent_ts(&self) -> usize {
        self.tmi_sent_ts
    }

    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn user_type(&self) -> String {
        self.user_type.clone()
    }

    pub fn vip(&self) -> bool {
        self.vip
    }

    pub fn reply_parent_msg_id(&self) -> String {
        self.reply_parent_msg_id.clone()
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

    pub fn emote_code(&self) -> String {
        self.emote_code.clone()
    }

    pub fn start_position(&self) -> usize {
        self.start_position
    }

    pub fn end_position(&self) -> usize {
        self.end_position
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

    pub fn command(&self) -> CommandType {
        self.command.clone()
    }

    pub fn channel(&self) -> String {
        self.channel.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
    admin: Option<String>,
    bits: Option<String>,
    broadcaster: Option<String>,
    moderator: Option<String>,
    subscriber: Option<String>,
    staff: Option<String>,
    turbo: Option<String>,
}

impl Badge {
    pub fn admin(&self) -> Option<String> {
        self.admin.clone()
    }

    pub fn set_admin(&mut self, admin: String) {
        self.admin = Some(admin);
    }

    pub fn bits(&self) -> Option<String> {
        self.bits.clone()
    }

    pub fn set_bits(&mut self, bits: String) {
        self.bits = Some(bits);
    }

    pub fn broadcaster(&self) -> Option<String> {
        self.broadcaster.clone()
    }

    pub fn set_broadcaster(&mut self, broadcaster: String) {
        self.broadcaster = Some(broadcaster);
    }

    pub fn moderator(&self) -> Option<String> {
        self.moderator.clone()
    }

    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = Some(moderator);
    }

    pub fn subscriber(&self) -> Option<String> {
        self.subscriber.clone()
    }

    pub fn set_subscriber(&mut self, subscriber: String) {
        self.subscriber = Some(subscriber);
    }

    pub fn staff(&self) -> Option<String> {
        self.staff.clone()
    }

    pub fn set_staff(&mut self, staff: String) {
        self.staff = Some(staff);
    }

    pub fn turbo(&self) -> Option<String> {
        self.turbo.clone()
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
}
