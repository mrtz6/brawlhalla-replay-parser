# Brawlhalla Replay Parser
***
A simple-to-use CLI tool to decompress, decipher and parse Brawlhalla .replay files, written in Rust.

## Usage
`brawlhalla-replay-parser --replay <REPLAY_FILE> [--output <OUTPUT_FILE>]`

## Examples
`./brawlhalla-replay-parser --replay "~/BrawlhallaReplays/[9.08] VoidMinor (4).replay" --output void_minor_replay.json`

## Where are my replays?

### Windows
Brawlhalla's replay files are located in `<USER>/BrawlhallaReplays/`.

### Linux
Brawlhalla's replay files are located in its Proton prefix, which can be found in `<STEAM_INSTALLATION>/steam/steamapps/compatdata/291550/pfx/drive_c/users/steamuser/BrawlhallaReplays/`.