use self::twitch::{Badge, Command, CommandType, Emote, Source, Tags, TwitchMessage};

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
        let mut idx = 0;
        let tags: Option<Tags> = if msg.starts_with('@') {
            let Some(space_idx) = msg.find(' ') else { panic!() };
            idx = space_idx + 1;
            let sub_str = &msg[0..space_idx];
            Some(self.parse_tags(sub_str))
        } else {
            None
        };
        let Some(current_char)= msg.get(idx..idx+1) else { panic!() };
        let source = if current_char == ":" {
            idx += 1;
            let sub_msg = &msg[idx..];
            let Some(space_idx) = sub_msg.find(' ') else { panic!() };
            let source: Source = self.parse_source(&sub_msg[..space_idx]);
            idx += space_idx + 1;
            source
        } else {
            return TwitchMessage::new::<&str>(
                None,
                Command::new(CommandType::Ping, ""),
                None,
                tags,
            );
        };

        let (command, add_idx) = self.parse_command(&msg[idx..], &source);
        idx += add_idx + 1;
        let parameter = self.parse_parameter(&msg[idx..]);
        TwitchMessage::new(parameter, command, Some(source), tags)
    }

    fn parse_tags(&self, input: &str) -> Tags {
        let splited = input.split(';');
        let mut tags = Tags::builder();
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
                    tags.emotes(emotes);
                }
                "id" => {
                    tags.id(value);
                }
                "mod" => {
                    tags.r#mod(value == "1");
                }
                "room-id" => {
                    tags.room_id(value);
                }
                "subscriber" => {
                    tags.subscriber(value == "1");
                }
                "turbo" => {
                    tags.turbo(value == "0");
                }
                "tmi-sent-ts" => {
                    let Ok(value) = value.parse::<usize>() else {continue;};
                    tags.tmi_sent_ts(value);
                }
                "user-id" => {
                    tags.user_id(value);
                }
                "user-type" => {
                    tags.user_type(value);
                }
                "vip" => {
                    tags.vip(value == "1");
                }
                "reply-parent-msg-id" => {
                    tags.reply_parent_msg_id(value);
                }
                "@msg-id" => {
                    tags.message_id(value);
                }
                "target-user-id" => {
                    tags.target_user_id(value);
                }
                _ => continue,
            }
        }
        tags.build().unwrap()
    }

    fn parse_badges(&self, value: &str) -> Badge {
        let badge_pair = value.split(',');
        let mut badge = Badge::default();
        for badge_key_value in badge_pair {
            let mut key_value = badge_key_value.split('/');
            let Some(key) = key_value.next() else { continue; };
            let Some(value) = key_value.next() else { continue; };
            match key {
                "admin" => badge.set_admin(value.to_owned()),
                "bits" => badge.set_bits(value.to_owned()),
                "broadcaster" => badge.set_broadcaster(value.to_owned()),
                "moderator" => badge.set_moderator(value.to_owned()),
                "subscriber" => badge.set_subscriber(value.to_owned()),
                "staff" => badge.set_staff(value.to_owned()),
                "turbo" => badge.set_turbo(value.to_owned()),
                _ => continue,
            }
        }
        badge
    }

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

    fn parse_source(&self, value: &str) -> Source {
        /*
            :lovingt3s!lovingt3s@
        */
        let bang_idx = value.find('!');
        let at_idx = value.find('@');
        match (bang_idx, at_idx) {
            (Some(bang_idx), Some(at_idx)) => {
                let nick = &value[0..bang_idx];
                let host = &value[at_idx + 1..];
                Source::new(nick, host)
            }
            _ => Source::new("", value),
        }
    }

    fn parse_command(&self, value: &str, source: &Source) -> (Command, usize) {
        let Some(space_idx) = value.find(' ') else { panic!() };
        let value = &value[..space_idx];
        let command = Command::new(CommandType::from(value), source.nick());
        (command, space_idx)
    }

    fn parse_parameter(&self, value: &str) -> Option<String> {
        let Some(idx) = value.find(':') else { return None };
        Some(value[idx + 1..].into())
    }
}

#[cfg(test)]
mod test {

    use super::{
        twitch::{Badge, Command, CommandType, Emote, Source},
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
        badges.set_staff("1".into());
        badges.set_broadcaster("1".into());
        badges.set_turbo("1".into());
        let tags = Tags::builder()
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
            .vip(false)
            .reply_parent_msg_id("")
            .build()
            .unwrap();
        let parameters = "DansGame";
        let expected_message =
            TwitchMessage::new(Some(parameters), command, Some(source), Some(tags));
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
        let expected_message = TwitchMessage::new(Some(parameters), command, Some(source), None);
        assert_eq!(expected_message, twitch_message);
    }

    #[test]
    fn should_parse_ping() {
        let msg: String = "PING".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        assert_eq!(CommandType::Ping, twitch_message.command().command())
    }

    #[test]
    fn should_parse_notice() {
        let msg: String = "@msg-id=delete_message_success :tmi.twitch.tv NOTICE #bar :The message from foo is now deleted.".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::Notice, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .message_id("delete_message_success")
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new(
            Some("The message from foo is now deleted."),
            command,
            Some(source),
            Some(tags),
        );
        assert_eq!(expected_message, twitch_message);
    }

    #[test]
    fn should_parse_notice_with_two_tags() {
        let msg: String = "@msg-id=whisper_restricted;target-user-id=12345678 :tmi.twitch.tv NOTICE #bar :Your settings prevent you from sending this whisper.".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::Notice, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .message_id("whisper_restricted")
            .target_user_id("12345678")
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new(
            Some("Your settings prevent you from sending this whisper."),
            command,
            Some(source),
            Some(tags),
        );
        assert_eq!(expected_message, twitch_message);
    }
}
