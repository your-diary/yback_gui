use std::env;

use env_logger;

use yback::{
    backup::{BackupConfig, Backups},
    config,
    gui::MyEguiApp,
};

fn main() {
//     env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let is_dryrun_mode = yback::parse_argv();
    let backup_config_list: Vec<BackupConfig> = config::read_config_file(is_dryrun_mode).unwrap();

    let app = MyEguiApp::new(Backups::new(backup_config_list).unwrap());
    eframe::run_native(Box::new(app), yback::gui::create_native_options());
}
