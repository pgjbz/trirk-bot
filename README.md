# Trirk Bot

Trirk bot is build on top of tokio.
Trirk read twitch chat and parse to object.

Environment variables:

```bash
TRIRK_NICKNAME=<your twitch nickname>
TRIRK_OAUTH=<your twitch oauth code>
TRIRK_CHANNEL=<channel to join>
```

[Generate your oauth code](https://twitchapps.com/tmi/)

Run bot:
```bash
cargo run --bin bot
```



Trirk is WIP