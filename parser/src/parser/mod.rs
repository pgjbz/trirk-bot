use self::twitch::{Badge, Emote, Tags, TagsBuilder, TwitchMessage};

pub mod twitch;

#[non_exhaustive]
pub struct TrirkParser;

impl TrirkParser {
    #[inline(always)]
    pub fn new() -> Self {
        Self
    }

    pub fn parse<T: Into<String>>(&self, msg: T) -> TwitchMessage {
        let msg: String = msg.into();
        if msg.starts_with('@') {
            let sub_str: String = msg.chars().take_while(|x| x != &' ').collect();
            let tags = self.parse_tags(sub_str);
        }
        todo!()
    }

    fn parse_tags(&self, input: String) -> Tags {
        let splited = input.split(';');
        let mut tags = TagsBuilder::default();
        for value in splited {
            let mut key_value = value.split('=');
            let Some(key) = key_value.next() else { continue; };
            let Some(value) = key_value.next() else { continue; };
            match key {
                "@badges" => {
                    let badge: Badge = self.parse_badges(value);
                    tags.badges(badge);
                }
                "color" => {
                    tags.color(value);
                }
                "display-name" => {
                    tags.display_name(value);
                }
                "emote-only" => {
                    tags.emote_only(value == "1");
                }
                "emotes" => {
                    let emotes: Vec<Emote> = self.parse_emotes(value);
                }
                /*

                id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;
                mod=0;
                room-id=81046256;
                subscriber=0;
                turbo=0;
                tmi-sent-ts=1550868292494;
                user-id=81046256;
                user-type=staff
                flags=0-7:A.6/P.6,25-36:A.1/I.2;
                */
                _ => continue,
            }
        }
        todo!()
    }

    fn parse_badges(&self, value: &str) -> Badge {
        let badge_pair = value.split(',');
        let mut badge = Badge::default();
        for badge_key_value in badge_pair {
            let mut key_value = badge_key_value.split('/');
            let Some(key) = key_value.next() else { continue; };
            let Some(value) = key_value.next() else { continue; };
            match key {
                "admin" => badge.admin = Some(value.to_owned()),
                "bits" => badge.bits = Some(value.to_owned()),
                "broadcaster" => badge.broadcaster = Some(value.to_owned()),
                "moderator" => badge.moderator = Some(value.to_owned()),
                "subscriber" => badge.subscriber = Some(value.to_owned()),
                "staff" => badge.staff = Some(value.to_owned()),
                "turbo" => badge.turbo = Some(value.to_owned()),
                _ => continue,
            }
        }
        badge
    }

    //emotes=33:0-7;
    fn parse_emotes(&self, value: &str) -> Vec<Emote> {
        let emotes_pair = value.split(',');
        let mut emotes: Vec<Emote> = Vec::new();
        for emote_key_value in emotes_pair {
            let mut code_value = emote_key_value.split(':');
            let Some(code) = code_value.next() else { continue; };
            let Some(position) = code_value.next() else { continue; };
            let mut start_end = position.split('-');
            let Some(start) = start_end.next() else { continue; };
            let Some(end) = start_end.next() else { continue; };
            let Ok(start) = start.parse::<usize>() else { continue; };
            let Ok(end) = end.parse::<usize>() else { continue; };
            let emote = Emote::new(code, start, end);
            emotes.push(emote);
        }
        emotes
    }
}

#[cfg(test)]
mod test {

    use super::{
        twitch::{Badge, Command, CommandType, Emote, Source, TagsBuilder},
        *,
    };

    #[test]
    fn should_parse_message_with_tags() {
        let msg: String = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let source = Source::new("petsgomoo", "petsgomoo@petsgomoo.tmi.twitch.tv");
        let command = Command::new(CommandType::PrivMSG, "#petsgomoo");
        let mut badges = Badge::default();
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
            .build()
            .unwrap();
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
