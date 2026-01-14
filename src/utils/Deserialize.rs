#[warn(non_camel_case_types)]
type byte = u8;

use crate::data::{Constants::Constants, Offsets::Offsets, Player::Player};

pub fn getSaveInfo(save: &[byte], slot: i32, player: &mut Player) {
    let mut charNameByte: [byte; 32] = [0; 32];
    let mut playTimeByte: [byte; 4] = [0; 4];
    let mut fundsByte: [byte; 4] = [0; 4];
    let mut rankByte: [byte; 2] = [0; 2];
    let mut hrPointsByte: [byte; 4] = [0; 4];
    let mut acaPointsByte: [byte; 4] = [0; 4];
    let mut villagePointsByte: [byte; 4] = [0; 4];
    let mut itemBytes: [byte; Constants::SIZEOF_ITEMBOX] = [0; Constants::SIZEOF_ITEMBOX];

    match slot {
        1 => {
            let mut firstSlot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x13], save[0x12], save[0x11], save[0x10]
            );
            player.SaveOffset = usize::from_str_radix(firstSlot.as_str(), 16).expect("Unknown err");
        }
        2 => {
            let secondSlot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x17], save[0x16], save[0x15], save[0x14]
            );
            player.SaveOffset =
                usize::from_str_radix(secondSlot.as_str(), 16).expect("Unknown err");
        }
        3 => {
            let thirdSlot = format!(
                "{:02X}{:02X}{:02X}{:02X}",
                save[0x1B], save[0x1A], save[0x19], save[0x18]
            );
            player.SaveOffset = usize::from_str_radix(thirdSlot.as_str(), 16).expect("Unknown err");
        }
        _ => {}
    }

    let currentOffset = player.SaveOffset;

    charNameByte.copy_from_slice(
        &save[currentOffset + Offsets::NAME_OFFSET
            ..currentOffset + Offsets::NAME_OFFSET + Constants::SIZEOF_NAME],
    );
}
