use crate::data::constants::Constants;

type Byte = u8;

pub struct Player {
    pub save_offset: usize,

    //General Info
    pub name: &'static str,
    pub play_time: i32,
    pub funds: i32,
    pub hunter_rank: i32,

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
    pub item_id: [Byte; Constants::TOTAL_ITEM_SLOTS],
    pub item_count: [Byte; Constants::TOTAL_ITEM_SLOTS],

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
    pub manual_shoutouts: [Byte; Constants::TOTAL_MANUAL_SHOUTOUTS],
}
