use eframe::{
    egui::{widgets::ProgressBar, CentralPanel, CtxRef, ScrollArea, TextEdit, Ui, Vec2},
    epi::{App, Frame},
    NativeOptions,
};

use super::backup::{Backup, Backups};

pub fn create_native_options() -> NativeOptions {
    NativeOptions {
        initial_window_size: Some(Vec2::new(MyEguiApp::WIDTH as f32, MyEguiApp::HEIGHT as f32)),
        resizable: false,
        ..Default::default()
    }
}

pub struct MyEguiApp {
    backups: Backups,
}

impl MyEguiApp {
    const APP_NAME: &'static str = "hello";
    pub const WIDTH: u32 = 620;
    pub const HEIGHT: u32 = 800;

    pub fn new(backups: Backups) -> Self {
        MyEguiApp { backups }
    }
}

impl App for MyEguiApp {
    fn name(&self) -> &str {
        MyEguiApp::APP_NAME
    }
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        CentralPanel::default().show(ctx, draw(self));
    }
}

fn draw(app: &mut MyEguiApp) -> (impl FnMut(&mut Ui) -> () + '_) {
    |ui: &mut Ui| -> () {
        let completed: bool = !app.backups.update_progress();

        ui.spacing_mut().item_spacing = Vec2::new(10.0, 20.0);

        //log display
        ScrollArea::vertical()
            .stick_to_bottom()
            .max_height(200.)
            .show(ui, |ui: &mut Ui| {
                ui.add(
                    TextEdit::multiline(&mut app.backups.get_current_backup().output)
                        .code_editor()
                        .desired_width(MyEguiApp::WIDTH as f32)
                        .desired_rows(14)
                        .interactive(false),
                );
            });

        //progress bars
        app.backups.get_backup_list().iter().for_each(|b: &Backup| {
            ui.label(b.get_name());
            ui.add(
                ProgressBar::new(b.get_percentage() as f32 / 100.0)
                    .show_percentage()
                    .animate(true),
            );
        });

        if (completed) {
            ui.label("Completed!");
        }
    }
}
