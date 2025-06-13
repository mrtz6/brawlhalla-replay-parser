# Brawlhalla Replay Parser
A simple-to-use CLI tool to decompress, decipher and parse Brawlhalla .replay files, written in Rust.

## Building
```bash
git clone https://github.com/mrtz6/brawlhalla-replay-parser.git
cd brawlhalla-replay-parser
cargo build --release
```
Built executable is located in `/target/release/`.

## Usage
```bash
brawlhalla-replay-parser --replay <REPLAY_FILE> [--output <OUTPUT_FILE>]
```

### Example Usage
```bash
./brawlhalla-replay-parser --replay "~/BrawlhallaReplays/[9.08] VoidMinor (4).replay" --output void_minor_replay.json
```
## Replay File Locations

### Windows
```
C\Users\<USER>\BrawlhallaReplays\
```

### Linux (Proton via Steam)
```
<STEAM_INSTALLATION>/steam/steamapps/compatdata/291550/pfx/drive_c/users/steamuser/BrawlhallaReplays/
```
## Example Output
```json
{
  "version": 253,
  "random_seed": 3708841848,
  "playlist_id": 8,
  "playlist_name": "PlaylistType_2v2Unranked_DisplayName",
  "online_game": true,
  "game_settings": {
    ...
    "max_players": 4,
    "duration": 480,
    ...
  },
  "level_id": 151,
  "hero_count": 1,
  "entities": [
    {
      "entity_id": 1,
      "name": "mrtz",
      ...
    },
    ...
  ],
  ...
  "length": 126688,
  "results": {
    "2": 2,
    "1": 2,
    "3": 1,
    "4": 1
  },
  ...
}
```
