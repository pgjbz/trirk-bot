#[derive(Clone)]
pub struct TwitchConfig {
    pub(super) channel: String,
    pub(super) oauth: String,
    pub(super) nickname: String,
}

impl TwitchConfig {
    #[inline(always)]
    pub fn new<T: Into<String>>(nickname: T, oauth: T, channel: T) -> Self {
        Self {
            nickname: nickname.into(),
            oauth: oauth.into(),
            channel: channel.into(),
        }
    }
}
