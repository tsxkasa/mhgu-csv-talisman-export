use crate::data::{game_constants::GameConstants, player::Player};
use clap::{Parser, command};
use std::path::PathBuf;
use std::process;

mod data;
mod utils;

type Byte = u8;

#[derive(Parser, Debug)]
#[command(name = "MHGU Charm Export", version)]
struct Args {
    file_path: PathBuf,

    #[arg(short, long)]
    out: Option<PathBuf>,

    #[arg(short, long, value_parser = clap::value_parser!(i32).range(1..=3))]
    slot: Option<i32>,
}

fn main() {
    let args = Args::parse();

    let path = match args.out {
        Some(p) => p,
        None => {
            let mut p = std::env::current_dir().expect("Error fetching current directory");
            p.push("talisman.csv");
            p
        }
    };
    let mut player = Player::default();
    let slot: i32 = args.slot.unwrap_or(1);
    let save_file_raw = std::fs::read(args.file_path).expect("Unable to read file.");
    let is_switch: bool = match save_file_raw.len() {
        4726152 => false,
        4726188 => true,
        5159100 => true,
        _ => process::exit(42),
    };
    let save_file = if is_switch {
        save_file_raw.iter().skip(36).copied().collect()
    } else {
        save_file_raw.clone()
    };

    if save_file[4] == 0 {
        process::exit(43);
    }
    if save_file[5] == 0 && slot == 2 {
        process::exit(43);
    }
    if save_file[6] == 0 && slot == 3 {
        process::exit(43);
    }
    if save_file[4] == 0 && save_file[5] == 0 && save_file[6] == 0 {
        process::exit(44);
    }

    utils::deserialize::populate_player_save(&save_file, slot, &mut player);
    utils::exporter::export_talisman(&player, &path).ok();
}
