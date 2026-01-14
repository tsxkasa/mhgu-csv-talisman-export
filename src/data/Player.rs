use crate::data::Constants::Constants;

#[warn(non_camel_case_types)]
type byte = u8;

pub struct Player {
    pub SaveOffset: usize,

    //General Info
    pub Name: &'static str,
    pub PlayTime: i32,
    pub Funds: i32,
    pub HunterRank: i32,

    //Character Details
    pub Voice: byte,
    pub EyeColor: byte,
    pub Clothing: byte,
    pub Gender: byte,
    //pub HuntingStyle: byte,
    pub HairStyle: byte,
    pub Face: byte,
    pub Features: byte,

    //Character Colors
    pub SkinColorRGBA: [byte; 4],
    pub HairColorRGBA: [byte; 4],
    pub FeaturesColorRGBA: [byte; 4],
    pub ClothingColorRGBA: [byte; 4],

    //Points
    pub HRPoints: i32,
    pub AcademyPoints: i32,
    pub BhernaPoints: i32,
    pub KokotoPoints: i32,
    pub PokkePoints: i32,
    pub YukumoPoints: i32,

    // Monster Hunts / Sizes
    pub MonsterKills: [byte; Constants::SIZEOF_MONSTERHUNTS],
    pub MonsterCaptures: [byte; Constants::SIZEOF_MONSTERCAPTURES],
    pub MonsterSizes: [byte; Constants::SIZEOF_MONSTERSIZES],

    //Item Box
    pub ItemId: [byte; Constants::TOTAL_ITEM_SLOTS],
    pub ItemCount: [byte; Constants::TOTAL_ITEM_SLOTS],

    //Equipment Box
    pub EquipmentInfo: [byte; Constants::SIZEOF_EQUIPBOX],

    // Palico Equipment Box
    pub EquipmentPalico: [byte; Constants::SIZEOF_PALICOEQUIPBOX],

    // Palico
    pub PalicoData: [byte; Constants::SIZEOF_PALICOES],

    // Guild Card
    pub GuildCardData: [byte; Constants::SIZEOF_GUILDCARD],
    pub ArenaData: [byte; Constants::SIZEOF_ARENALOG],

    // Shoutouts
    pub AutomaticShoutouts: [byte; Constants::SIZEOF_AUTOMATIC_SHOUTOUTS],
    pub ManualShoutouts: [byte; Constants::TOTAL_MANUAL_SHOUTOUTS],
}
