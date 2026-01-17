type Byte = u8;

use crate::data::{constants::Constants, offsets::Offsets, player::Player};

pub fn populate_player_save(save: &[Byte], slot: i32, player: &mut Player) {
    match slot {
        1 => {
            let first_slot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x13], save[0x12], save[0x11], save[0x10]
            );
            player.save_offset =
                usize::from_str_radix(first_slot.as_str(), 16).expect("Unknown err");
        }
        2 => {
            let second_slot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x17], save[0x16], save[0x15], save[0x14]
            );
            player.save_offset =
                usize::from_str_radix(second_slot.as_str(), 16).expect("Unknown err");
        }
        3 => {
            let third_slot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x1B], save[0x1A], save[0x19], save[0x18]
            );
            player.save_offset =
                usize::from_str_radix(third_slot.as_str(), 16).expect("Unknown err");
        }
        _ => {}
    }

    let current_offset = player.save_offset;

    player.name = String::from_utf8(
        save[current_offset + Offsets::NAME_OFFSET
            ..current_offset + Offsets::NAME_OFFSET + Constants::SIZEOF_NAME]
            .to_vec(),
    )
    .unwrap();

    player.play_time = i32::from_le_bytes(
        save[current_offset + Offsets::PLAY_TIME_OFFSET
            ..current_offset + Offsets::PLAY_TIME_OFFSET + 4]
            .try_into()
            .unwrap(),
    );

    player.funds = i32::from_le_bytes(
        save[current_offset + Offsets::FUNDS_OFFSET..current_offset + Offsets::FUNDS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );

    player.hunter_rank = i16::from_le_bytes(
        save[current_offset + Offsets::HUNTER_RANK_OFFSET
            ..current_offset + Offsets::HUNTER_RANK_OFFSET + 2]
            .try_into()
            .unwrap(),
    );

    player.hrpoints = i32::from_le_bytes(
        save[current_offset + Offsets::HR_POINTS_OFFSET
            ..current_offset + Offsets::HR_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );

    player.academy_points = i32::from_le_bytes(
        save[current_offset + Offsets::ACADEMY_POINTS_OFFSET
            ..current_offset + Offsets::ACADEMY_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );

    player.bherna_points = i32::from_le_bytes(
        save[current_offset + Offsets::BHERNA_POINTS_OFFSET
            ..current_offset + Offsets::BHERNA_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );
    player.kokoto_points = i32::from_le_bytes(
        save[current_offset + Offsets::KOKOTO_POINTS_OFFSET
            ..current_offset + Offsets::KOKOTO_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );
    player.pokke_points = i32::from_le_bytes(
        save[current_offset + Offsets::POKKE_POINTS_OFFSET
            ..current_offset + Offsets::POKKE_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );
    player.yukumo_points = i32::from_le_bytes(
        save[current_offset + Offsets::YUKUMO_POINTS_OFFSET
            ..current_offset + Offsets::YUKUMO_POINTS_OFFSET + 4]
            .try_into()
            .unwrap(),
    );

    player.voice = save[current_offset + Offsets::CHARACTER_VOICE_OFFSET];
    player.eye_color = save[current_offset + Offsets::GUILDCARD_EYE_COLOR_OFFSET];
    player.clothing = save[current_offset + Offsets::CHARACTER_CLOTHING_OFFSET];
    player.gender = save[current_offset + Offsets::CHARACTER_GENDER_OFFSET];
    player.hair_style = save[current_offset + Offsets::CHARACTER_HAIRSTYLE_OFFSET];
    player.face = save[current_offset + Offsets::CHARACTER_FACE_OFFSET];
    player.features = save[current_offset + Offsets::CHARACTER_FEATURES_OFFSET];

    player.skin_color_rgba = save[current_offset + Offsets::CHARACTER_SKIN_COLOR_OFFSET
        ..current_offset + Offsets::CHARACTER_SKIN_COLOR_OFFSET + 4]
        .try_into()
        .unwrap();
    player.hair_color_rgba = save[current_offset + Offsets::CHARACTER_HAIR_COLOR_OFFSET
        ..current_offset + Offsets::CHARACTER_HAIR_COLOR_OFFSET + 4]
        .try_into()
        .unwrap();
    player.features_color_rgba = save[current_offset + Offsets::CHARACTER_FEATURES_COLOR_OFFSET
        ..current_offset + Offsets::CHARACTER_FEATURES_COLOR_OFFSET + 4]
        .try_into()
        .unwrap();
    player.clothing_color_rgba = save[current_offset + Offsets::CHARACTER_CLOTHING_COLOR_OFFSET
        ..current_offset + Offsets::CHARACTER_CLOTHING_COLOR_OFFSET + 4]
        .try_into()
        .unwrap();

    let mut item_bytes = save[current_offset + Offsets::ITEM_BOX_OFFSET
        ..current_offset + Offsets::ITEM_BOX_OFFSET + Constants::SIZEOF_ITEMBOX]
        .to_vec();
    item_bytes.reverse();
    let mut result = String::with_capacity(item_bytes.len() * 8);
    for b in item_bytes {
        use std::fmt::Write;
        write!(result, "{:08b}", b).unwrap();
    }
    let mut result = &result[4..result.len() - 4];
    for i in (0..2999).rev() {
        player.item_count[i] = i32::from_str_radix(&result[..7], 2).unwrap().to_string();
        player.item_id[i] = i32::from_str_radix(&result[7..12], 2).unwrap().to_string();
        result = &result[19..result.len() - 19];
    }

    player.equipment_info = save[current_offset + Offsets::EQUIPMENT_BOX_OFFSET
        ..current_offset + Offsets::EQUIPMENT_BOX_OFFSET + Constants::SIZEOF_EQUIPBOX]
        .try_into()
        .unwrap();
    player.equipment_palico = save[current_offset + Offsets::PALICO_EQUIPMENT_OFFSET
        ..current_offset + Offsets::PALICO_EQUIPMENT_OFFSET + Constants::SIZEOF_PALICOEQUIPBOX]
        .try_into()
        .unwrap();

    player.monster_kills = save[current_offset + Offsets::MONSTERHUNT_OFFSETS
        ..current_offset + Offsets::MONSTERHUNT_OFFSETS + Constants::SIZEOF_MONSTERHUNTS]
        .try_into()
        .unwrap();
    player.monster_captures = save[current_offset + Offsets::MONSTERCAPTURE_OFFSETS
        ..current_offset + Offsets::MONSTERCAPTURE_OFFSETS + Constants::SIZEOF_MONSTERCAPTURES]
        .try_into()
        .unwrap();
    player.monster_sizes = save[current_offset + Offsets::MONSTERSIZE_OFFSETS
        ..current_offset + Offsets::MONSTERSIZE_OFFSETS + Constants::SIZEOF_MONSTERSIZES]
        .try_into()
        .unwrap();

    player.palico_data = save[current_offset + Offsets::PALICO_OFFSET
        ..current_offset + Offsets::PALICO_OFFSET + Constants::SIZEOF_PALICOES]
        .try_into()
        .unwrap();

    player.guild_card_data = save[current_offset + Offsets::GUILCARD_OFFSET
        ..current_offset + Offsets::GUILCARD_OFFSET + Constants::SIZEOF_GUILDCARD]
        .try_into()
        .unwrap();
    player.arena_data = save[current_offset + Offsets::GUILDCARD_ARENA_LOG_OFFSET
        ..current_offset + Offsets::GUILDCARD_ARENA_LOG_OFFSET + Constants::SIZEOF_ARENALOG]
        .try_into()
        .unwrap();

    player.manual_shoutouts = save[current_offset + Offsets::MANUAL_SHOUTOUT_OFFSETS
        ..current_offset + Offsets::MANUAL_SHOUTOUT_OFFSETS + Constants::SIZEOF_MANUAL_SHOUTOUTS]
        .try_into()
        .unwrap();
    player.automatic_shoutouts = save[current_offset + Offsets::AUTOMATIC_SHOUTOUT_OFFSETS
        ..current_offset
            + Offsets::AUTOMATIC_SHOUTOUT_OFFSETS
            + Constants::SIZEOF_AUTOMATIC_SHOUTOUTS]
        .try_into()
        .unwrap();
}
