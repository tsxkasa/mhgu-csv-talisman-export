use crate::data::{constants::Constants, game_constants::GameConstants, player::Player};
use crate::utils::talisman::Talisman;
use csv::Writer;
use std::path::PathBuf;

fn load_talisman_from_csv(player: &Player) -> Vec<usize> {
    let mut talisman_list: Vec<usize> = Vec::default();
    for i in 0..Constants::TOTAL_EQUIP_SLOTS {
        let high = player.equipment_info[(i * 36) + 1] as u16;
        let low = player.equipment_info[i * 36] as u16;
        let type_level_bit = (high << 8) | low;
        let equip_type = (type_level_bit & 0x1F) as u8;
        if equip_type == 6 {
            talisman_list.push(i);
        }
    }
    talisman_list
}

fn write_talisman_to_csv(wtr: &mut Writer<std::fs::File>, t: &Talisman) -> csv::Result<()> {
    let skill_1 = GameConstants::SKILL_NAMES[t.skill_1];
    let skill_2 = GameConstants::SKILL_NAMES[t.skill_2];
    if t.skill_2 == 0 {
        return wtr.write_record([
            "",
            &t.slots.to_string(),
            skill_1,
            &t.skill_1_level.to_string(),
            "",
            "",
        ]);
    }
    wtr.write_record([
        "",
        &t.slots.to_string(),
        skill_1,
        &t.skill_1_level.to_string(),
        skill_2,
        &t.skill_2_level.to_string(),
    ])
}

pub fn export_talisman_to_csv(player: &Player, path: &PathBuf) -> csv::Result<()> {
    let talisman_list = load_talisman_from_csv(player);
    let mut wtr = Writer::from_path(path)?;
    for i in talisman_list {
        let talisman = Talisman {
            slots: player.equipment_info[(i * 36) + 16],
            skill_1: player.equipment_info[(i * 36) + 12] as usize,
            skill_1_level: player.equipment_info[(i * 36) + 14] as i8,
            skill_2: player.equipment_info[(i * 36) + 13] as usize,
            skill_2_level: player.equipment_info[(i * 36) + 15] as i8,
        };
        write_talisman_to_csv(&mut wtr, &talisman)?;
    }

    wtr.flush()?;
    Ok(())
}
