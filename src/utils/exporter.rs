use crate::data::{constants::Constants, game_constants::GameConstants, player::Player};
use csv::Writer;
use std::path::PathBuf;

fn load_talisman(player: &Player) -> Vec<usize> {
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

struct Talisman {
    slots: u8,
    skill1_name: String,
    skill1_level: u8,
    skill2_name: String,
    skill2_level: u8,
}

fn write_talisman(wtr: &mut Writer<std::fs::File>, t: &Talisman) -> csv::Result<()> {
    if t.skill2_name == "(None)" {
        return wtr.write_record([
            "",
            &t.slots.to_string(),
            &t.skill1_name,
            &t.skill1_level.to_string(),
            "",
            "",
        ]);
    }
    wtr.write_record([
        "",
        &t.slots.to_string(),
        &t.skill1_name,
        &t.skill1_level.to_string(),
        &t.skill2_name,
        &t.skill2_level.to_string(),
    ])
}

pub fn export_talisman(player: &Player, path: &PathBuf) -> csv::Result<()> {
    let talisman_list = load_talisman(player);
    let mut wtr = Writer::from_path(path)?;
    for i in talisman_list {
        let talisman = Talisman {
            slots: player.equipment_info[(i * 36) + 16],
            skill1_name: GameConstants::SKILL_NAMES[player.equipment_info[(i * 36) + 12] as usize]
                .to_string(),
            skill1_level: player.equipment_info[(i * 36) + 14],
            skill2_name: GameConstants::SKILL_NAMES[player.equipment_info[(i * 36) + 13] as usize]
                .to_string(),
            skill2_level: player.equipment_info[(i * 36) + 15],
        };
        write_talisman(&mut wtr, &talisman)?;
    }

    wtr.flush()?;
    Ok(())
}
