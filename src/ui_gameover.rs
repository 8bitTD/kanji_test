use eframe::{egui};
use super::ui_state::*;
use super::define::*;
#[derive(PartialEq)]
pub struct GameOver{
    pub scroll: f32,
}

impl Default for GameOver{
    fn default() -> Self {
        Self {
            scroll: 0.0,
        }
    }
}

impl MyUi for GameOver{
    fn setup(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame, gs: &mut GlobalStruct) {
        frame.set_window_title(&format!("{}{}", common::TOOLNAME, " [ゲームオーバー]"));
        self.scroll = gs.result.len() as f32 * 160.0;
    }
    fn update(&mut self, ctx: &egui::Context, gs: &mut GlobalStruct) {
        let Self{scroll} = self;
        if gs.state != State::GameOver{return;}
        egui::CentralPanel::default().show(ctx,|ui|{
            let res = egui::ScrollArea::vertical().max_height(155.0).vertical_scroll_offset(*scroll).show(ui, |ui| {
                for (i, k) in gs.result.iter().enumerate(){
                    ui.group(|ui| {
                        ui.vertical_centered_justified(|ui|{
                            ui.add_sized(egui::Vec2::new(200.0, 20.0),egui::Label::new(&format!("{}{}{}","第",i+1,"問：答え")));
                            let kanji = k.character.to_string();
                            let v_kunyomi = k.yomi.kun_yomi.clone();
                            let kunyomi = v_kunyomi.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                            let v_onyomi = k.yomi.on_yomi.clone();
                            let onyomi = v_onyomi.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                            let v_english_meaning = k.yomi.english_meaning.clone();
                            let english_meaning = v_english_meaning.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                            ui.heading(&kanji);
                            ui.horizontal(|ui|{
                                ui.add_space(80.0);
                                ui.label("訓読み:");
                                ui.label(&kunyomi);
                            });
                            ui.horizontal(|ui|{
                                ui.add_space(80.0);
                                ui.label("音読み:");
                                ui.label(&onyomi);
                            });
                            ui.horizontal(|ui|{
                                ui.add_space(80.0);
                                ui.label("英語:");
                                ui.label(&english_meaning);
                            });
                        });
                    });
                
                }
            });
            *scroll = res.state.offset.y;
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                if ui.button("タイトル画面(Enter)").clicked() || ctx.input().key_pressed(egui::Key::Enter){
                    gs.state = State::Title;
                }
            });
            ui.ctx().request_repaint();
        });
    }
}
