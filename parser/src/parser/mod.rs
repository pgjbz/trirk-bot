use self::twitch::TwitchMessage;

pub mod twitch;

#[non_exhaustive]
pub struct TrirkParser;

impl TrirkParser {
    #[inline(always)]
    pub fn new() -> Self {
        Self
    }

    pub fn parse<T: Into<String>>(&self, msg: T) -> TwitchMessage {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::{
        twitch::{Badges, Command, CommandType, Emote, Source, TagsBuilder},
        *,
    };

    #[test]
    fn should_parse_message_with_tags() {
        let msg: String = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let source = Source::new("petsgomoo", "petsgomoo@petsgomoo.tmi.twitch.tv");
        let command = Command::new(CommandType::PrivMSG, "#petsgomoo");
        let mut badges = Badges::default();
        badges.staff = Some("1".into());
        badges.broadcaster = Some("1".into());
        badges.turbo = Some("1".into());
        let tags = TagsBuilder::default()
            .badges(badges)
            .color("#FF0000")
            .display_name("PetsgomOO")
            .emote_only(true)
            .emotes(vec![Emote::new("33", 0, 7)])
            .id("c285c9ed-8b1b-4702-ae1c-c64d76cc74ef")
            .r#mod(false)
            .room_id("81046256")
            .subscriber(false)
            .turbo(false)
            .tmi_sent_ts(1550868292494usize)
            .user_id("81046256")
            .user_type("staff")
            .build().unwrap();
        let parameters = "DansGame";
        let expected_message = TwitchMessage::new(parameters, command, source, Some(tags));
    }

    #[test]
    fn should_parse_message_without_tags() {
        let msg: String =
            ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let source = Source::new("lovingt3s", "lovingt3s.tmi.twitch.tv");
        let command = Command::new(CommandType::PrivMSG, "lovingt3s");
        let parameters = "!dilly";
        let expected_message = TwitchMessage::new(parameters, command, source, None);
        assert_eq!(expected_message, twitch_message);
    }
}
