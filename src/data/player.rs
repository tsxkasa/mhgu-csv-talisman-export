use crate::data::constants::Constants;

type Byte = u8;

#[derive(Clone)]
pub struct Player {
    pub save_offset: usize,

    //General Info
    pub name: String,
    pub play_time: i32,
    pub funds: i32,
    pub hunter_rank: i16,

    //Character Details
    pub voice: Byte,
    pub eye_color: Byte,
    pub clothing: Byte,
    pub gender: Byte,
    //pub HuntingStyle: byte,
    pub hair_style: Byte,
    pub face: Byte,
    pub features: Byte,

    //Character Colors
    pub skin_color_rgba: [Byte; 4],
    pub hair_color_rgba: [Byte; 4],
    pub features_color_rgba: [Byte; 4],
    pub clothing_color_rgba: [Byte; 4],

    //Points
    pub hrpoints: i32,
    pub academy_points: i32,
    pub bherna_points: i32,
    pub kokoto_points: i32,
    pub pokke_points: i32,
    pub yukumo_points: i32,

    // Monster Hunts / Sizes
    pub monster_kills: [Byte; Constants::SIZEOF_MONSTERHUNTS],
    pub monster_captures: [Byte; Constants::SIZEOF_MONSTERCAPTURES],
    pub monster_sizes: [Byte; Constants::SIZEOF_MONSTERSIZES],

    //Item Box
    pub item_id: [String; Constants::TOTAL_ITEM_SLOTS],
    pub item_count: [String; Constants::TOTAL_ITEM_SLOTS],

    //Equipment Box
    pub equipment_info: [Byte; Constants::SIZEOF_EQUIPBOX],

    // Palico Equipment Box
    pub equipment_palico: [Byte; Constants::SIZEOF_PALICOEQUIPBOX],

    // Palico
    pub palico_data: [Byte; Constants::SIZEOF_PALICOES],

    // Guild Card
    pub guild_card_data: [Byte; Constants::SIZEOF_GUILDCARD],
    pub arena_data: [Byte; Constants::SIZEOF_ARENALOG],

    // Shoutouts
    pub automatic_shoutouts: [Byte; Constants::SIZEOF_AUTOMATIC_SHOUTOUTS],
    pub manual_shoutouts: [Byte; Constants::SIZEOF_MANUAL_SHOUTOUTS],
}

impl Default for Player {
    fn default() -> Self {
        Player {
            save_offset: Default::default(),
            name: Default::default(),
            academy_points: Default::default(),
            arena_data: [0; Constants::SIZEOF_ARENALOG],
            automatic_shoutouts: [0; Constants::SIZEOF_AUTOMATIC_SHOUTOUTS],
            bherna_points: Default::default(),
            clothing: Default::default(),
            clothing_color_rgba: Default::default(),
            equipment_info: [0; Constants::SIZEOF_EQUIPBOX],
            equipment_palico: [0; Constants::SIZEOF_PALICOEQUIPBOX],
            eye_color: Default::default(),
            face: Default::default(),
            features: Default::default(),
            features_color_rgba: Default::default(),
            funds: Default::default(),
            gender: Default::default(),
            guild_card_data: [0; Constants::SIZEOF_GUILDCARD],
            hair_color_rgba: Default::default(),
            hair_style: Default::default(),
            hrpoints: Default::default(),
            yukumo_points: Default::default(),
            kokoto_points: Default::default(),
            pokke_points: Default::default(),
            play_time: Default::default(),
            palico_data: [0; Constants::SIZEOF_PALICOES],
            manual_shoutouts: [0; Constants::SIZEOF_MANUAL_SHOUTOUTS],
            item_id: std::array::from_fn(|_| String::new()),
            item_count: std::array::from_fn(|_| String::new()),
            skin_color_rgba: Default::default(),
            monster_kills: [0; Constants::SIZEOF_MONSTERHUNTS],
            monster_captures: [0; Constants::SIZEOF_MONSTERCAPTURES],
            monster_sizes: [0; Constants::SIZEOF_MONSTERSIZES],
            hunter_rank: Default::default(),
            voice: Default::default(),
        }
    }
}
