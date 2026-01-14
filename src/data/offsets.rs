pub struct Offsets;

impl Offsets {
    //Header Data
    pub const FIRST_CHAR_SLOT_USED: usize = 0x04; //Size 1
    pub const SECOND_CHAR_SLOT_USED: usize = 0x05; //Size 1
    pub const THIRD_CHAR_SLOT_USED: usize = 0x06; //Size 1
    pub const FIRST_CHARACTER_OFFSET: usize = 0x10; //Size 4
    pub const SECOND_CHARACTER_OFFSET: usize = 0x14; //Size 4
    pub const THIRD_CHARACTER_OFFSET: usize = 0x18; //Size 4

    //Character Offsets [CHARACTER BASE +  CHARACTER OFFSET]
    pub const NAME_OFFSET: usize = 0x23B7D; // size 4
    pub const PLAY_TIME_OFFSET: usize = 0x20; //Size 4m this only shows on the save screen
    pub const PLAY_TIME_OFFSET2: usize = 0x2248B; //Size 4
    pub const FUNDS_OFFSET: usize = 0x24; //Size 4, this only shows on the save screen
    pub const FUNDS_OFFSET2: usize = 0x280F; // size 4
    pub const HUNTER_RANK_OFFSET: usize = 0x28; //Size 2 
    pub const CHARACTER_VOICE_OFFSET: usize = 0x23B48; //Size 1
    pub const CHARACTER_EYE_COLOR_OFFSET: usize = 0x23B49; //Size 1
    pub const CHARACTER_CLOTHING_OFFSET: usize = 0x23B4A; //Size 1
    pub const CHARACTER_GENDER_OFFSET: usize = 0x23B4B; // Size 1
    pub const CHARACTER_HUNTINGSTYLE_OFFSET: usize = 0x23B4C; // Size 1
    pub const CHARACTER_HAIRSTYLE_OFFSET: usize = 0x23B4D; //Size 1
    pub const CHARACTER_FACE_OFFSET: usize = 0x23B4E; //Size 1
    pub const CHARACTER_FEATURES_OFFSET: usize = 0x23B4F; //Size 1
    pub const CHARACTER_SKIN_COLOR_OFFSET: usize = 0x23B67; //Size 4 
    pub const CHARACTER_HAIR_COLOR_OFFSET: usize = 0x23B6B; //Size 4
    pub const CHARACTER_FEATURES_COLOR_OFFSET: usize = 0x23B6F; //Size 4
    pub const CHARACTER_CLOTHING_COLOR_OFFSET: usize = 0x23B73; //Size 4

    // Only shown on save screen ?
    //pub const NAME_OFFSET: usize = 0x0; //Size 32
    //pub const HUNTER_ART_1_OFFSET: usize = 0x2C; //Size 2
    //pub const HUNTER_ART_2_OFFSET: usize = 0x2E; //Size 2
    //pub const HUNTER_ART_3_OFFSET: usize = 0x30; //Size 2
    //pub const EQUIPPED_WEAPON_OFFSET: usize = 0x010C; //Size 48
    //pub const EQUIPPED_HEAD_OFFSET: usize = 0x013C; //Size 48
    //pub const EQUIPPED_CHEST_OFFSET: usize = 0x016C; //Size 48
    //pub const EQUIPPED_ARMS_OFFSET: usize = 0x019c; //Size 48
    //pub const EQUIPPED_WAIST_OFFSET: usize = 0x01CC; //Size 48
    //pub const EQUIPPED_LEG_OFFSET: usize = 0x01FC; //Size 48
    //pub const EQUIPPED_TALISMAN_OFFSET: usize = 0x022C; //Size 48
    //pub const WEAPON_TYPE_OFFSET: usize = 0x025C; //Size 1
    //pub const CHARACTER_VOICE_OFFSET: usize = 0x241; //Size 1
    //pub const CHARACTER_EYE_COLOR_OFFSET: usize = 0x242; //Size 1
    //pub const CHARACTER_CLOTHING_OFFSET: usize = 0x243; //Size 1
    //pub const CHARACTER_GENDER_OFFSET: usize = 0x244;  //Size 1
    //pub const CHARACTER_HUNTING_STYLE_OFFSET: usize = 0x245; //Size 1
    //pub const CHARACTER_HAIRSTYLE_OFFSET: usize = 0x246; //Size 1
    //pub const CHARACTER_FACE_OFFSET: usize = 0x247; //Size 1
    //pub const CHARACTER_FEATURES_OFFSET: usize = 0x248; //Size 1
    //pub const CHARACTER_SKIN_COLOR_OFFSET: usize = 0x260; //Size 4
    //pub const CHARACTER_HAIR_COLOR_OFFSET: usize = 0x264; //Size 4
    //pub const CHARACTER_FEATURES_COLOR_OFFSET: usize = 0x268; //Size 4
    //pub const CHARACTER_CLOTHING_COLOR_OFFSET: usize = 0x26C; //Size 4
    //pub const CHEST_ARMOR_PIGMENT_OFFSET: usize = 0x0268; //Size 4
    //pub const ARMS_ARMOR_PIGMENT_OFFSET: usize = 0x026C; //Size 4
    //pub const WAIST_ARMOR_PIGMENT_OFFSET: usize = 0x0270;  //Size 4
    //pub const LEG_ARMOR_PIGMENT_OFFSET: usize = 0x0274; //Size 4
    //pub const HEAD_ARMOR_PIGMENT_OFFSET: usize = 0x0278; //Size 4

    // Palico
    pub const PALICO_OFFSET: usize = 0x23BB6; //Size 27216 (84 of them each 324 bytes long)

    // Points
    pub const HR_POINTS_OFFSET: usize = 0x280B; //Size 4
    pub const ACADEMY_POINTS_OFFSET: usize = 0x2817; //Size 4
    pub const BHERNA_POINTS_OFFSET: usize = 0x281B; //Size 4
    pub const KOKOTO_POINTS_OFFSET: usize = 0x281F; //Size 4
    pub const POKKE_POINTS_OFFSET: usize = 0x2823; //Size 4
    pub const YUKUMO_POINTS_OFFSET: usize = 0x2827; //Size 4

    // Monster Hunts / Sizes
    pub const MONSTERHUNT_OFFSETS: usize = 0x5EA6; //Size 274, 137 Monsters (supposedly) 2 bytes each
    pub const MONSTERCAPTURE_OFFSETS: usize = 0x5FB8; //Size 274,137 Monsters (supposedly) 2 bytes each
    pub const MONSTERSIZE_OFFSETS: usize = 0x60CA; //Size 548, 4 bytes per monster

    // Items, Equips, Pouch
    pub const ITEM_BOX_OFFSET: usize = 0x0278; //Size 5463 (2300 of them each 19 bits long)
    pub const EQUIPMENT_BOX_OFFSET: usize = 0x62EE; //Size 72000 (2000 of them each 36 bytes long)
    pub const PALICO_EQUIPMENT_OFFSET: usize = 0x17C2E; //Size 36000 (1000 of them 36 bytes long)
    //pub const ITEM_SET_OFFSET: usize = 0x0EDE; //Size 1360 (8 of them each 170 bytes long)
    //pub const POUCH_OFFSET: usize = 0x142E; //Size 72 (32 Items each 18 bits long)

    // Player Guild Card
    pub const GUILCARD_OFFSET: usize = 0xC71BD;
    //pub const GUILDCARD_NAME_OFFSET: usize = 0xC71BD; //Size 12 ?
    //pub const GUILDCARD_WEAPONTYPE_OFFSET: usize = 0xC71D5; //Size 1
    pub const GUILDCARD_VOICE_OFFSET: usize = 0xC71D6; //Size 1
    pub const GUILDCARD_EYE_COLOR_OFFSET: usize = 0xC71D7; //Size 1
    pub const GUILDCARD_CLOTHING_OFFSET: usize = 0xC71D8; //Size 1
    pub const GUILDCARD_GENDER_OFFSET: usize = 0xC71D9; // Size 1
    pub const GUILDCARD_HUNTINGSTYLE_OFFSET: usize = 0xC71DA; // Size 1
    pub const GUILDCARD_HAIRSTYLE_OFFSET: usize = 0xC71DB; //Size 1
    pub const GUILDCARD_FACE_OFFSET: usize = 0xC71DC; //Size 1
    pub const GUILDCARD_FEATURES_OFFSET: usize = 0xC71DD; //Size 1
    pub const GUILDCARD_SKIN_COLOR_OFFSET: usize = 0xC71F5; //Size 4
    pub const GUILDCARD_HAIR_COLOR_OFFSET: usize = 0xC71F9; //Size 4
    pub const GUILDCARD_FEATURES_COLOR_OFFSET: usize = 0xC71FD; //Size 4
    pub const GUILDCARD_CLOTHING_COLOR_OFFSET: usize = 0xC7201; //Size 4
    //pub const GUILDCARD_LOWVIL_QUEST_OFFSET: usize = 0xC7A1B; //Size 2
    //pub const GUILDCARD_HIGHVIL_QUEST_OFFSET: usize = 0xC7A1D; //Size 2
    //pub const GUILDCARD_LOWHUB_QUEST_OFFSET: usize = 0xC7A1F; //Size 2
    //pub const GUILDCARD_HIGHHUB_QUEST_OFFSET: usize = 0xC7A21; //Size 2
    //pub const GUILDCARD_GRANK_QUEST_OFFSET: usize = 0xC7A23; //Size 2
    //pub const GUILDCARD_SP_QUEST_OFFSET: usize = 0xC7A25; //Size 2
    //pub const GUILDCARD_ARENA_QUEST_OFFSET: usize = 0xC7A27; //Size 2
    pub const GUILDCARD_ID_OFFSET: usize = 0xC7A6D; //Size 8
    //pub const GUILDCARD_VILLAGE_WEAPON_OFFSET: usize = 0xC7A77; //Size 30
    //pub const GUILDCARD_HUB_WEAPON_OFFSET: usize = 0xC7A95; //Size 30
    //pub const GUILDCARD_ARENA_WEAPON_OFFSET: usize = 0xC7AB3; //Size 30
    pub const GUILDCARD_ARENA_LOG_OFFSET: usize = 0xC83E1; //Size 324

    // Shoutouts
    pub const MANUAL_SHOUTOUT_OFFSETS: usize = 0x11D629; //Size 60
    pub const AUTOMATIC_SHOUTOUT_OFFSETS: usize = 0x11E169; //Size 60

    //pub const SHOP_OFFSETS: usize = 0x1D76;
    //pub const CRAFTABLE_WEAPONS_OFFSET: usize = 0x20BE;
    //pub const CRAFTABLE_ARMOR_SHOP_OFFSET: usize = 0x2316;
    //pub const CRAFTABLE_PALICO_GEAR_OFFSET: usize = 0x02ABE;

    //pub const FOOD_FLAG_OFFSETS: usize = 0x1A32; //Size 4
    //pub const AWARD_FLAG_OFFSETS: usize = 0x1B8A; //Size 13

    //pub const UNLOCKED_BOXES_OFFSET: usize = 0x1A22; //Size 8
}
