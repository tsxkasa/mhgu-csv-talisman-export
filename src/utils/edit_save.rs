use crate::data::{game_constants::GameConstants, player::Player};

use crate::utils::talisman::Talisman;

fn write_talisman_to_save(
    player: &mut Player,
    item_box_index: usize,
    t: &Talisman,
) -> Result<(), i32> {
    check_talisman_validity(
        player,
        item_box_index,
        t.slots,
        GameConstants::SKILL_NAMES[t.skill_1],
        t.skill_1_level,
        GameConstants::SKILL_NAMES[t.skill_2],
        t.skill_2_level,
    )?;
    player.equipment_info[(item_box_index * 36) + 16] = t.slots;
    player.equipment_info[(item_box_index * 36) + 12] = t.skill_1 as u8;
    player.equipment_info[(item_box_index * 36) + 14] = t.skill_1_level as u8;
    player.equipment_info[(item_box_index * 36) + 13] = t.skill_2 as u8;
    player.equipment_info[(item_box_index * 36) + 15] = t.skill_2_level as u8;
    player.equipment_info[(item_box_index * 36) + 17] = 0;
    player.equipment_info[(item_box_index * 36) + 18] = GameConstants::TALISMAN_RARITY
        [player.equipment_info[(item_box_index * 36) + 2] as usize]
        as u8;
    player.equipment_info[(item_box_index * 36) + 19] = 1;
    for i in 20..36 {
        player.equipment_info[(item_box_index * 36) + i] = 0;
    }
    Ok(())
}

/*
* 46 = unable to find skill/invalid skill
* 47 = skill 1 == skill 2
* 48 = invalid talisman rarity
* 49 = skill out of range
* 50 = incorrect slots
* */
fn check_talisman_validity(
    player: &Player,
    index: usize,
    slots: u8,
    skill_1_name: &str,
    skill_1_level: i8,
    skill_2_name: &str,
    skill_2_level: i8,
) -> Result<(), i32> {
    type SkillPack = (i8, i8, i8, i8);

    if !(0..=3).contains(&slots) {
        return Err(50);
    }

    if skill_1_name == skill_2_name && skill_1_name != GameConstants::SKILL_NAMES[0] {
        return Err(47);
    }

    let skill_1_selected_index = GameConstants::SKILL_NAMES
        .iter()
        .position(|&x| x == skill_1_name)
        .ok_or(46)?;
    let skill_2_selected_index = GameConstants::SKILL_NAMES
        .iter()
        .position(|&x| x == skill_2_name)
        .ok_or(46)?;

    let (skill_1, skill_2): (SkillPack, SkillPack);

    match GameConstants::TALISMAN_RARITY[player.equipment_info[(index * 36) + 2] as usize] {
        97 => {
            skill_1 = unpack_skill(GameConstants::TALISMAN_MYSTERY_SKILL[skill_1_selected_index]);
            skill_2 = unpack_skill(GameConstants::TALISMAN_MYSTERY_SKILL[skill_2_selected_index]);
        }
        98 => {
            skill_1 = unpack_skill(GameConstants::TALISMAN_SHINING_SKILL[skill_1_selected_index]);
            skill_2 = unpack_skill(GameConstants::TALISMAN_SHINING_SKILL[skill_2_selected_index]);
        }
        99 => {
            skill_1 = unpack_skill(GameConstants::TALISMAN_TIMEWORN_SKILL[skill_1_selected_index]);
            skill_2 = unpack_skill(GameConstants::TALISMAN_TIMEWORN_SKILL[skill_2_selected_index]);
        }
        100 => {
            skill_1 = unpack_skill(GameConstants::TALISMAN_ENDURING_SKILL[skill_1_selected_index]);
            skill_2 = unpack_skill(GameConstants::TALISMAN_ENDURING_SKILL[skill_2_selected_index]);
        }
        _ => return Err(48),
    }

    let (min1, max1, _, _) = skill_1;
    let (_, _, min2, max2) = skill_2;

    let skill_1_range = min1..=max1;
    let skill_2_range = min2..=max2;

    if !skill_1_range.contains(&skill_1_level) || !skill_2_range.contains(&skill_2_level) {
        return Err(49);
    }

    Ok(())
}

#[inline]
fn decode_5_bits(v: u32) -> i8 {
    let v = (v & 0x1F) as i8;
    (v << 3) >> 3
}

fn unpack_skill(p: u32) -> (i8, i8, i8, i8) {
    (
        decode_5_bits(p >> 15),
        decode_5_bits(p >> 10),
        decode_5_bits(p >> 5),
        decode_5_bits(p),
    )
}
