This is a Twitch.tv chatbot written in Rust. It is meant for chat to control
some aspects of the stream with the use of Polls.

Currently there is 2 votables:

- VoteBot
  - After using `!reset_votes` the bot starts counting
  - Counts "Yes" or "1" and "No" or "2" in chat
  - `!results_votes` tallies the voting box and prints to chat
- LeagueBot
  - Detects when your League of Legends character has leveled up
  - Asks chat what ability it should level (`Q, W, E, R` not the passive lol)
  - Waits 10 seconds (approx)
  - Uses a InputBot to input the `Ctrl+Q` when your client is open and focused
    - `(it also does this when your client is not open so it'll input into whatever other app you have open, have fun)`

One last thing here, some commands only work with moderators but voting works
with any user. I'll put a detailed list of commands one day.

# How to use

First you need to make a copy of `config_default.toml` and name it
`config.toml` then edit the file accordingly to your needs. (I recomend using
an alt account for your `oauth` token for safety precautions)

Then you also need Rito's ssl certificate if you want the LeagueBot to detect
your level ups. LeagueBot connects to the backend of your client and needs
rito's super epic Self-Certificate for https conversations with the client.
All you need to do is go to Rito's Client API docs and look for a link or a
download regarding _root certificate_. I don't include this in the project
files because I am not sure if I can provide it legally.

[Here](https://developer.riotgames.com/docs/lol#league-client-api)'s a quick
link to their Client API docs. You'll have to find that _root certificate_
yourself but it should be named something like `riotgames.pem`. Once you have
that, drop it into `/path/to/hivemind/external/riotgames.pem`.

Now you can launch the bot! Yey.

# I have problems

Open an issue here on GitHub, I can't say I'll resolve it faster than Rito can
hehe.
