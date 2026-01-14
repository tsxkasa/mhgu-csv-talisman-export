type Byte = u8;

use crate::data::{constants::Constants, offsets::Offsets, player::Player};

pub fn getSaveInfo(save: &[Byte], slot: i32, player: &mut Player) {
    let mut char_name_byte: [Byte; 32] = [0; 32];
    let mut play_time_byte: [Byte; 4] = [0; 4];
    let mut funds_byte: [Byte; 4] = [0; 4];
    let mut rank_byte: [Byte; 2] = [0; 2];
    let mut hr_points_byte: [Byte; 4] = [0; 4];
    let mut aca_points_byte: [Byte; 4] = [0; 4];
    let mut village_points_byte: [Byte; 4] = [0; 4];
    let mut item_bytes: [Byte; Constants::SIZEOF_ITEMBOX] = [0; Constants::SIZEOF_ITEMBOX];

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

    char_name_byte.copy_from_slice(
        &save[current_offset + Offsets::NAME_OFFSET
            ..current_offset + Offsets::NAME_OFFSET + Constants::SIZEOF_NAME],
    );
}
