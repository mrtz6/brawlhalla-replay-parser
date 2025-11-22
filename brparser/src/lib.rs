use bitreader::BitReader;
use flate2::read::ZlibDecoder;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug, Serialize)]
pub struct Input {
    pub time_stamp: u32,
    pub input_state: u32,
}

#[derive(Debug, Serialize)]
pub struct Death {
    pub entity_id: u32,
    pub time_stamp: u32,
}

#[derive(Debug, Default, Serialize)]
pub struct GameSettings {
    pub flags: u32,
    pub max_players: u32,
    pub duration: u32,
    pub round_duration: u32,
    pub starting_lives: u32,
    pub scoring_type_id: u32,
    pub score_to_win: u32,
    pub game_speed: u32,
    pub damage_multiplier: u32,
    pub level_set_id: u32,
    pub item_spawn_ruleset_id: u32,
    pub weapon_spawn_rate_id: u32,
    pub gadget_spawn_rate_id: u32,
    pub custom_gadgets_field: u32,
    pub variation: u32,
}

#[derive(Debug, Serialize)]
pub struct Hero {
    pub hero_id: u32,
    pub costume_id: u32,
    pub stance_index: u32,
    pub weapon_skin_2: u16,
    pub weapon_skin_1: u16,
}

#[derive(Debug, Serialize)]
pub struct PlayerType {
    pub color_scheme_id: u32,
    pub spawn_bot_id: u32,
    pub companion_id: u32,
    pub emitter_id: u32,
    pub player_theme_id: u32,
    pub trail_effect_id: u32,
    pub taunts: [u32; 8],
    pub win_taunt_id: u16,
    pub lose_taunt_id: u16,
    pub taunt_database: Vec<u32>,
    pub avatar_id: u16,
    pub team: u32,
    pub connection_time: u32,
    pub heroes: Vec<Hero>,
    pub is_bot: bool,
    pub handicaps_enabled: bool,
    pub handicap_stock_count: Option<u32>,
    pub handicap_damage_multiplier: Option<u32>,
    pub handicap_damage_taken_multiplier: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct Entity {
    pub entity_id: u32,
    pub name: String,
    pub player_type: PlayerType,
}

#[derive(Debug, Default, Serialize)]
pub struct Replay {
    pub version: u32,
    pub random_seed: u32,
    pub playlist_id: u32,
    pub playlist_name: String,
    pub online_game: bool,
    pub game_settings: GameSettings,
    pub level_id: u32,
    pub hero_count: u16,
    pub entities: Vec<Entity>,
    pub checksum: u32,
    pub inputs: HashMap<u32, Vec<Input>>,
    pub deaths: Vec<Death>,
    pub length: u32,
    pub results: HashMap<u32, u16>,
    pub end_of_match_fan_fare_id: u32,
}

pub struct ReplayParser;

impl ReplayParser {
    const XOR_KEY: [u8; 64] = [
        0x6B, 0x10, 0xDE, 0x3C, 0x44, 0x4B, 0xD1, 0x46, 0xA0, 0x10, 0x52, 0xC1, 0xB2, 0x31, 0xD3,
        0x6A, 0xFB, 0xAC, 0x11, 0xDE, 0x06, 0x68, 0x08, 0x78, 0x8C, 0xD5, 0xB3, 0xF9, 0x6A, 0x40,
        0xD6, 0x13, 0x0C, 0xAE, 0x9D, 0xC5, 0xD4, 0x6B, 0x54, 0x72, 0xFC, 0x57, 0x5D, 0x1A, 0x06,
        0x73, 0xC2, 0x51, 0x4B, 0xB0, 0xC9, 0x8C, 0x78, 0x04, 0x11, 0x7A, 0xEF, 0x74, 0x3E, 0x46,
        0x39, 0xA0, 0xC7, 0xA6,
    ];

    fn read_int(reader: &mut BitReader) -> u32 {
        reader.read_u32(32).unwrap()
    }

    fn read_short(reader: &mut BitReader) -> u16 {
        reader.read_u16(16).unwrap()
    }

    fn read_bool(reader: &mut BitReader) -> bool {
        reader.read_bool().unwrap()
    }

    fn read_string(reader: &mut BitReader) -> String {
        let string_length = reader.read_u16(16).unwrap();
        let mut string_bytes: Vec<u8> = vec![];
        for _ in 0..string_length {
            string_bytes.push(reader.read_u8(8).unwrap());
        }
        String::from_utf8(string_bytes).unwrap()
    }

    pub fn parse_from_file(path: &String) -> Result<Replay, Error> {
        let mut decoder = match File::open(path) {
            Ok(file) => ZlibDecoder::new(file),
            Err(e) => return Err(e),
        };

        let mut uncompressed_data: Vec<u8> = vec![];
        decoder.read_to_end(&mut uncompressed_data)?;

        for i in 0..uncompressed_data.len() {
            uncompressed_data[i] ^= Self::XOR_KEY[i % Self::XOR_KEY.len()]
        }

        let mut reader = BitReader::new(&uncompressed_data);

        let mut replay: Replay = Default::default();

        replay.version = reader.read_u32(32).unwrap();

        let mut end_of_replay = false;
        while !end_of_replay {
            let replay_state = reader.read_u32(4).unwrap();

            match replay_state {
                1 => {
                    replay.inputs = {
                        let mut inputs = HashMap::new();
                        while Self::read_bool(&mut reader) {
                            let entity_id = reader.read_u32(5).unwrap();
                            if !inputs.contains_key(&entity_id) {
                                inputs.insert(entity_id, vec![]);
                            }

                            let input_count = Self::read_int(&mut reader);
                            for _ in 0..input_count {
                                let time_stamp = Self::read_int(&mut reader);
                                let input_state = match Self::read_bool(&mut reader) {
                                    true => reader.read_u32(14).unwrap(),
                                    false => 0
                                };

                                inputs.get_mut(&entity_id).unwrap().push(Input { time_stamp, input_state });
                            }
                        }
                        inputs
                    };
                }
                2 => {
                    end_of_replay = true;
                }
                3 => {
                    replay.random_seed = Self::read_int(&mut reader);
                    replay.playlist_id = Self::read_int(&mut reader);

                    replay.playlist_name = Self::read_string(&mut reader);

                    replay.online_game = reader.read_bool().unwrap();
                }
                4 => {
                    replay.game_settings = GameSettings {
                        flags: Self::read_int(&mut reader),
                        max_players: Self::read_int(&mut reader),
                        duration: Self::read_int(&mut reader),
                        round_duration: Self::read_int(&mut reader),
                        starting_lives: Self::read_int(&mut reader),
                        scoring_type_id: Self::read_int(&mut reader),
                        score_to_win: Self::read_int(&mut reader),
                        game_speed: Self::read_int(&mut reader),
                        damage_multiplier: Self::read_int(&mut reader),
                        level_set_id: Self::read_int(&mut reader),
                        item_spawn_ruleset_id: Self::read_int(&mut reader),
                        weapon_spawn_rate_id: Self::read_int(&mut reader),
                        gadget_spawn_rate_id: Self::read_int(&mut reader),
                        custom_gadgets_field: Self::read_int(&mut reader),
                        variation: Self::read_int(&mut reader),
                    };

                    replay.level_id = Self::read_int(&mut reader);
                    replay.hero_count = Self::read_short(&mut reader);

                    replay.entities = {
                        let mut entities: Vec<Entity> = vec![];
                        while Self::read_bool(&mut reader) {
                            let entity_id = Self::read_int(&mut reader);
                            let name = Self::read_string(&mut reader);

                            let color_scheme_id = Self::read_int(&mut reader);
                            let spawn_bot_id = Self::read_int(&mut reader);
                            let companion_id = Self::read_int(&mut reader);
                            let emitter_id = Self::read_int(&mut reader);
                            let trail_effect_id = Self::read_int(&mut reader);
                            let player_theme_id = Self::read_int(&mut reader);

                            let taunts: [u32; 8] = std::array::from_fn(|_| Self::read_int(&mut reader));
                            let win_taunt_id = Self::read_short(&mut reader);
                            let lose_taunt_id = Self::read_short(&mut reader);

                            let mut taunt_database: Vec<u32> = vec![];
                            while Self::read_bool(&mut reader) {
                                taunt_database.push(Self::read_int(&mut reader));
                            }

                            let avatar_id = Self::read_short(&mut reader);
                            let team = Self::read_int(&mut reader);
                            let connection_time = Self::read_int(&mut reader);

                            let heroes: Vec<Hero> = {
                                let mut heroes: Vec<Hero> = vec![];
                                
                                for _ in 0..replay.hero_count {
                                    let hero_id = Self::read_int(&mut reader);
                                    let costume_id = Self::read_int(&mut reader);
                                    let stance_index = Self::read_int(&mut reader);
                                    let weapon_skin_2 = Self::read_short(&mut reader);
                                    let weapon_skin_1 = Self::read_short(&mut reader);

                                    heroes.push(Hero {
                                        hero_id,
                                        costume_id,
                                        stance_index,
                                        weapon_skin_2,
                                        weapon_skin_1,
                                    });
                                }
                                
                                heroes
                            };

                            let is_bot = Self::read_bool(&mut reader);
                            let handicaps_enabled = Self::read_bool(&mut reader);
                            let mut handicap_stock_count: Option<u32> = None;
                            let mut handicap_damage_multiplier: Option<u32> = None;
                            let mut handicap_damage_taken_multiplier: Option<u32> = None;

                            if handicaps_enabled {
                                handicap_damage_taken_multiplier = Option::from(Self::read_int(&mut reader));
                                handicap_stock_count = Option::from(Self::read_int(&mut reader));
                                handicap_damage_multiplier = Option::from(Self::read_int(&mut reader));
                            }

                            let player_type = PlayerType {
                                color_scheme_id,
                                spawn_bot_id,
                                companion_id,
                                emitter_id,
                                trail_effect_id,
                                player_theme_id,
                                taunts,
                                win_taunt_id,
                                lose_taunt_id,
                                taunt_database,
                                avatar_id,
                                team,
                                connection_time,
                                heroes,
                                is_bot,
                                handicaps_enabled,
                                handicap_stock_count,
                                handicap_damage_multiplier,
                                handicap_damage_taken_multiplier,
                            };

                            entities.push(Entity { entity_id, name, player_type });
                        }

                        entities
                    };

                    replay.checksum = Self::read_int(&mut reader);
                }
                5 => {
                    replay.deaths = {
                        let mut deaths: Vec<Death> = vec![];
                        while Self::read_bool(&mut reader) {
                            let entity_id = reader.read_u32(5).unwrap();
                            let time_stamp = Self::read_int(&mut reader);

                            deaths.push(Death { entity_id, time_stamp });
                        }

                        deaths.sort_by(|a, b| a.time_stamp.cmp(&b.time_stamp));

                        deaths
                    };
                }
                6 => {
                    replay.length = Self::read_int(&mut reader);

                    replay.results = {
                        let mut results = HashMap::new();

                        if Self::read_bool(&mut reader) {
                            while Self::read_bool(&mut reader) {
                                let entity_id = reader.read_u32(5).unwrap();
                                let result = Self::read_short(&mut reader);
                                results.insert(entity_id, result);
                            }
                        }

                        results
                    };

                    replay.end_of_match_fan_fare_id = Self::read_int(&mut reader);
                }
                _ => {}
            }
        }

        Ok(replay)
    }
}

impl Replay {}
