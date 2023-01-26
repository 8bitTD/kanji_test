#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod define;
mod ui;
mod ui_state;
mod ui_title;
mod ui_game;
mod ui_gameover;
mod ui_ending;
mod kanji_info;
mod kanji;

fn main()  {
    set_exec();
}

fn set_exec(){
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(385.0, 200.0)),
        initial_window_pos: Some(eframe::egui::pos2(800.0,300.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        define::common::TOOLNAME,
        options,
        Box::new(|cc| Box::new(ui::MyApp::new(cc))),
    );
}