use eframe::{egui};
use super::ui_state::*;
use super::define::*;
#[derive(PartialEq, Eq)]
pub struct Title {}

impl Default for Title {
    fn default() -> Self {
        Self {}
    }
}

impl MyUi for Title{
    fn setup(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame, gs: &mut GlobalStruct) {
        frame.set_window_title(&format!("{}{}", common::TOOLNAME, " [タイトル]"));
        gs.audio.play("tada");
        gs.current_number = 1;
        gs.result.clear();
    }

    fn update(&mut self, ctx: &egui::Context, gs: &mut GlobalStruct) {
        if gs.state != State::Title{return;}
        gs.kanjis.update();
        egui::CentralPanel::default().show(ctx,|ui|{
            ui.add_space(15.0);
            ui.vertical_centered_justified(|ui| {
                ui.heading("漢字テスト");
                ui.label(" 20問連続で正解してください ");
            });
            ui.separator();
            ui.add_space(5.0);
            ui.vertical_centered_justified(|ui| {
                if ui.button("日本語モード(Enter)").clicked() || ctx.input().key_pressed(egui::Key::Enter){
                    gs.game_mode = GameMode::Japanese;
                    gs.state = State::Game;
                    gs.audio.play("Speech On");
                }
            });
            ui.add_space(5.0);
            if gs.is_clear{
                ui.vertical_centered_justified(|ui| {
                    if ui.button("英語モード(e)").clicked() || ctx.input().key_pressed(egui::Key::E){
                        gs.game_mode = GameMode::English;
                        gs.state = State::Game;
                        gs.audio.play("Speech On");
                    }
                });
            }
        });
     
    }
}

