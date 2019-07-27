extern crate clap;
extern crate sysinfo;
use clap::{Arg, App};
use sysinfo::{SystemExt};
use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Right, Left};
mod ascii_art;

static DEFAULT_CONFIG_PATHS: [&'static str; 2] = ["~/.sysinforc.yml", "~/.config/sysinfo/config.yml"];

fn main() {
    App::new("sysinfo")
        .version("0.1")
        .author("Anthony T. Lannutti <lannuttia@gmail.com>")
        .about("Displays system information")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
            .default_value(&format!("{:?}", DEFAULT_CONFIG_PATHS)))
        .get_matches();

    let system = sysinfo::System::new();

    let stats = vec![
        format!(
            "RAM usage: {:.2}/{:.2} GB",
            (system.get_used_memory() as f64)/1E6,
            (system.get_total_memory() as f64)/1E6
        ),
        format!(
            "Swap usage: {:.2}/{:.2} GB",
            (system.get_used_swap() as f64)/1E6,
            (system.get_total_swap() as f64)/1E6
        )
    ];

    let art = ascii_art::get_ascii_art();

    for it in art.iter().zip_longest(stats.iter()) {
        match it {
          Both(art, stats) => println!("{} {}", art, stats),
          Left(art) => println!("{}", art),
          Right(stats) => println!("{}", stats),
        }
    }
}
