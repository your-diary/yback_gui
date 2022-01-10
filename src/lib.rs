pub mod backup;
pub mod config;
pub mod gui;

use std::env;
use std::process;

use config::IsDryrunMode;

pub fn print_usage() -> () {
    println!(
        "\
Usage
  yback [<option(s)>]

Options
  -n/--dry-run #Passes `--dry-run` option to `rsync`.
  -h/--help    #Show this help."
    );
}

pub fn parse_argv() -> IsDryrunMode {
    let mut is_dryrun_mode = IsDryrunMode::NO;
    env::args().skip(1).for_each(|s| {
        let s = s.as_str();
        match s {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-n" | "--dry-run" => is_dryrun_mode = IsDryrunMode::YES,
            _ => {
                println!("Unknown option [ {} ] is specified.", s);
                process::exit(1);
            }
        }
    });
    is_dryrun_mode
}
