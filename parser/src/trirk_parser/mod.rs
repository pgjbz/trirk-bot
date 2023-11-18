use std::collections::HashMap;

use self::{
    error::UnparsableError,
    twitch::{Badge, Command, CommandType, Emote, Source, Tags, TwitchMessage},
};

pub mod error;
pub mod twitch;

#[non_exhaustive]
#[derive(Default)]
pub struct TrirkParser;

impl TrirkParser {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }

    pub fn parse<T: Into<String>>(&self, msg: T) -> Result<TwitchMessage, UnparsableError> {
        let msg: String = msg.into();
        if msg.is_empty() {
            return Err(UnparsableError::new("empty irc message"));
        }
        let mut idx = 0;
        let tags: Option<Tags> = if msg.starts_with('@') {
            let Some(space_idx) = msg.find(' ') else {
                Err(UnparsableError::new("message does not contains any space"))?
            };
            idx = space_idx + 1;
            let sub_str = &msg[0..space_idx];
            Some(self.parse_tags(sub_str))
        } else {
            None
        };

        match (msg.get(idx..idx + 1), &msg[..]) {
            (Some(":"), _) => {
                idx += 1;
                let sub_msg = &msg[idx..];
                let Some(space_idx) = sub_msg.find(' ') else {
                    Err(UnparsableError::new("message does not contains any space"))?
                };
                let source: Source = self.parse_source(&sub_msg[..space_idx]);
                idx += space_idx + 1;
                let (command, add_idx) = self.parse_command(&msg[idx..], &source);
                idx += add_idx + 1;
                let parameter = self.parse_parameter(&msg[idx..]);
                Ok(TwitchMessage::new(parameter, command, Some(source), tags))
            }
            (_, "PING") => Ok(TwitchMessage::new::<&str>(
                None,
                Command::new(CommandType::Ping, ""),
                None,
                tags,
            )),
            (_, msg) if msg.contains("JOIN") => self.parse_join(msg),
            (current_char, msg) => Err(UnparsableError::new(format!(
                "ERROR: could not parse message '{msg}' {complement}",
                complement = current_char
                    .map_or("".into(), |current| format!("with start char '{current}'"))
            ))),
        }
    }

    fn parse_tags(&self, input: &str) -> Tags {
        let splited = input.split(';');
        let mut tags = Tags::builder();
        let mut extra_tags = HashMap::new();
        for value in splited {
            let mut key_value = value.split('=');
            let Some(key) = key_value.next() else {
                continue;
            };
            let Some(value) = key_value.next() else {
                continue;
            };
            match key {
                "@badges" | "badges" => {
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
                    tags.turbo(value == "1");
                }
                "tmi-sent-ts" => {
                    let Ok(value) = value.parse::<usize>() else {
                        continue;
                    };
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
                "@room-id" => {
                    tags.room_id(value);
                }
                "@ban-duration" => {
                    tags.ban_duration(value.parse::<usize>().unwrap_or(0));
                }
                "@login" => {
                    tags.login(value);
                }
                "target-msg-id" => {
                    tags.target_message_id(value);
                }
                "emote-sets" => {
                    let emote_sets = self.parse_emote_sets(value);
                    tags.emote_sets(emote_sets);
                }
                "@emote-only" => {
                    tags.emote_only(value == "1");
                }
                "followers-only" => {
                    tags.followers_only(value == "1");
                }
                "r9k" => {
                    tags.r9k(value == "1");
                }
                "slow" | "@slow" => {
                    tags.slow(value.parse::<usize>().unwrap_or(0));
                }
                "subs-only" => {
                    tags.subs_only(value == "1");
                }
                unk => {
                    extra_tags.insert(unk.into(), value.into());
                }
            }
        }
        tags.extra_tags(extra_tags);
        tags.build().unwrap()
    }

    fn parse_badges(&self, value: &str) -> Badge {
        let badge_pair = value.split(',');
        let mut badge = Badge::default();
        for badge_key_value in badge_pair {
            let mut key_value = badge_key_value.split('/');
            let Some(key) = key_value.next() else {
                continue;
            };
            let Some(value) = key_value.next() else {
                continue;
            };
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
            let Some(code) = code_value.next() else {
                continue;
            };
            let Some(position) = code_value.next() else {
                continue;
            };
            let mut start_end = position.split('-');
            let Some(start) = start_end.next() else {
                continue;
            };
            let Some(end) = start_end.next() else {
                continue;
            };
            let Ok(start) = start.parse::<usize>() else {
                continue;
            };
            let Ok(end) = end.parse::<usize>() else {
                continue;
            };
            let emote = Emote::new(code, start, end);
            emotes.push(emote);
        }
        emotes
    }

    fn parse_source(&self, value: &str) -> Source {
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
        let Some(space_idx) = value.find(' ') else {
            return (Command::new(CommandType::from(value), source.nick()), 0);
        };
        let value = &value[..space_idx];
        let command = Command::new(CommandType::from(value), source.nick());
        (command, space_idx)
    }

    fn parse_parameter(&self, value: &str) -> Option<String> {
        let idx = value.find(':')?;
        Some(value[idx + 1..].into())
    }

    fn parse_emote_sets(&self, value: &str) -> Vec<usize> {
        value
            .split(',')
            .map(|v| v.parse::<usize>().unwrap_or(0))
            .collect()
    }

    fn parse_join(&self, msg: &str) -> Result<TwitchMessage, UnparsableError> {
        let Some(nick) = msg.split('!').next() else {
            Err(UnparsableError::new(format!(
                "ERROR: could not parse '{}', not have '!'",
                msg
            )))?
        };
        let mut idx = (nick.len() * 2) + 2;
        let sub_msg = &msg[idx..];

        let Some(host) = sub_msg.split(' ').next() else {
            Err(UnparsableError::new(format!(
                "ERROR: could not parse '{}', not have host",
                msg
            )))?
        };
        idx += host.len() + 7;
        let channel = &msg[idx..];
        Ok(TwitchMessage::new::<String>(
            None,
            Command::new(CommandType::Join, channel),
            Some(Source::new(nick.replace(':', ""), host.into())),
            None,
        ))
    }
}

#[cfg(test)]
mod test {

    use super::{
        twitch::{Badge, Command, CommandType, Emote, Source},
        *,
    };

    #[test]
    fn should_parse_part() {
        let msg: String = ":kyoqz!kyoqz@kyoqz.tmi.twitch.tv PART #evazord".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);

        let source = Source::new("kyoqz", "kyoqz.tmi.twitch.tv");
        let command = Command::new(CommandType::Part, "kyoqz");

        let expected_message = TwitchMessage::new::<String>(None, command, Some(source), None);

        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_join() {
        let msg: String = "renildson!renildson@renildson.tmi.twitch.tv JOIN #evazord".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);

        let source = Source::new("renildson", "renildson.tmi.twitch.tv");
        let command = Command::new(CommandType::Join, "evazord");

        let expected_message = TwitchMessage::new::<String>(None, command, Some(source), None);
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_message_with_tags() {
        let msg: String = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let source = Source::new("petsgomoo", "petsgomoo.tmi.twitch.tv");
        let command = Command::new(CommandType::PrivMSG, "petsgomoo");
        let mut badges = Badge::default();
        badges.set_staff("1".into());
        badges.set_broadcaster("1".into());
        badges.set_turbo("1".into());
        let mut extra_tags = HashMap::new();
        extra_tags.insert("flags".into(), "0-7:A.6/P.6,25-36:A.1/I.2".into());
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
            .extra_tags(extra_tags)
            .build()
            .unwrap();
        let parameters = "DansGame";
        let expected_message =
            TwitchMessage::new(Some(parameters), command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
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
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_ping() {
        let msg: String = "PING".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        assert_eq!(
            &CommandType::Ping,
            twitch_message.unwrap().command().command()
        )
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
        assert_eq!(Ok(expected_message), twitch_message);
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
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_clearchat() {
        let msg: String = "@room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642715756806 :tmi.twitch.tv CLEARCHAT #dallas :ronni".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::ClearChat, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .room_id("12345678")
            .tmi_sent_ts(1642715756806usize)
            .target_user_id("87654321")
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new(Some("ronni"), command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_clearchat_with_ban_tag() {
        let msg: String = "@ban-duration=350;room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642719320727 :tmi.twitch.tv CLEARCHAT #dallas :ronni".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::ClearChat, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .room_id("12345678")
            .tmi_sent_ts(1642719320727usize)
            .target_user_id("87654321")
            .ban_duration(350usize)
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new(Some("ronni"), command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_clearmessage() {
        let msg: String = "@login=ronni;room-id=;target-msg-id=abc-123-def;tmi-sent-ts=1642720582342 :tmi.twitch.tv CLEARMSG #dallas :HeyGuys".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::ClearMessage, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .room_id("")
            .tmi_sent_ts(1642720582342usize)
            .target_message_id("abc-123-def")
            .login("ronni")
            .build()
            .unwrap();
        let expected_message =
            TwitchMessage::new(Some("HeyGuys"), command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_globaluserstate() {
        let msg: String = "@badge-info=subscriber/8;badges=subscriber/6;color=#0D4200;display-name=dallas;emote-sets=0,33,50,237,793,2126,3517,4578,5569,9400,10337,12239;turbo=0;user-id=12345678;user-type=admin :tmi.twitch.tv GLOBALUSERSTATE".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let source = Source::new("", "tmi.twitch.tv");
        let command = Command::new(CommandType::GlobalUserState, "");
        let mut badges = Badge::default();
        let mut extra_tags = HashMap::new();
        extra_tags.insert("@badge-info".into(), "subscriber/8".into());
        badges.set_subscriber("6".into());
        let tags = Tags::builder()
            .badges(badges)
            .color("#0D4200")
            .display_name("dallas")
            .user_id("12345678")
            .emote_sets(vec![
                0, 33, 50, 237, 793, 2126, 3517, 4578, 5569, 9400, 10337, 12239,
            ])
            .user_type("admin")
            .user_id("12345678")
            .extra_tags(extra_tags)
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new::<&str>(None, command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_roomstate() {
        let msg: String = "@emote-only=0;followers-only=0;r9k=0;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #dallas".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::RoomState, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder()
            .emote_only(false)
            .followers_only(false)
            .r9k(false)
            .slow(0usize)
            .subs_only(false)
            .build()
            .unwrap();
        let expected_message = TwitchMessage::new::<&str>(None, command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    fn should_parse_roomstate_slow() {
        let msg: String = "@slow=10 :tmi.twitch.tv ROOMSTATE #dallas".into();
        let parser: TrirkParser = TrirkParser::new();
        let twitch_message = parser.parse(msg);
        let command = Command::new(CommandType::RoomState, "");
        let source = Source::new("", "tmi.twitch.tv");
        let tags = Tags::builder().slow(10usize).build().unwrap();
        let expected_message = TwitchMessage::new::<&str>(None, command, Some(source), Some(tags));
        assert_eq!(Ok(expected_message), twitch_message);
    }

    #[test]
    #[should_panic]
    fn should_panic_with_empty_message() {
        let parser: TrirkParser = TrirkParser::new();
        parser.parse("").unwrap();
    }

    #[test]
    #[should_panic]
    fn should_panic_with_invalid_message() {
        let parser: TrirkParser = TrirkParser::new();
        parser.parse("xablau").unwrap();
    }
}
