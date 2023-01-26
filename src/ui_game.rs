use eframe::{egui};
use super::ui_state::*;
use super::define::*;

#[derive(Debug)]
pub struct Game{
    input: String,
    count: f32,
    old_count: f32,
    is_input_request: bool,
}

impl Default for Game{
    fn default() -> Self {
        Self {
            input: String::from(""),
            count: common::COUNTDEFAULT,
            old_count: common::COUNTDEFAULT,
            is_input_request: true,
        }
    }
}

impl MyUi for Game{
    fn setup(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame, _gs: &mut GlobalStruct) {
        frame.set_window_title(&format!("{}{}", common::TOOLNAME, " [ゲーム]"));
    }
    fn update(&mut self, ctx: &egui::Context, gs: &mut GlobalStruct) {
        let Self{input, count, old_count,is_input_request} = self;
        if gs.state != State::Game{return;}
        gs.kanjis.update();
        if gs.kanjis.get_kanji_count() == 0{
            eframe::egui::TopBottomPanel::top("top").show(ctx, |ui|{
                ui.vertical_centered_justified(|ui| {
                    ui.label("ロード中...");
                });
            });
            ctx.request_repaint();
            return;
        }
        let mut ks = Vec::new();
        for k in &gs.kanjis.kanji{ ks.push(k.kanji.to_owned()); }
        eframe::egui::SidePanel::left("left").default_width(150.0).show(ctx,|ui|{
            egui::ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.add_space(6.0);
                    ui.label(&format!("{}","答え"));
                });
                ui.separator();
                if !gs.result.is_empty(){
                    let kanji = gs.result[gs.result.len()-1].character.to_string();
                    let v_kunyomi = gs.result[gs.result.len()-1].yomi.kun_yomi.clone();
                    let kunyomi = v_kunyomi.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                    let v_onyomi = gs.result[gs.result.len()-1].yomi.on_yomi.clone();
                    let onyomi = v_onyomi.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                    let v_english_meaning = gs.result[gs.result.len()-1].yomi.english_meaning.clone();
                    let english_meaning = v_english_meaning.into_iter().map(|x| x).collect::<Vec<String>>().join(",");
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(&kanji);
                    });
                    ui.horizontal(|ui|{
                        ui.label("訓読み:");
                        ui.label(&kunyomi);
                    });
                    ui.horizontal(|ui|{
                        ui.label("音読み:");
                        ui.label(&onyomi);
                    });
                    ui.horizontal(|ui|{
                        ui.label("英語:");
                        ui.label(&english_meaning);
                    });
                }
            });
        });
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(&format!("{}{}{}","第",gs.current_number,"問"));
            });
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                ui.heading(&gs.kanjis.kanji[0].kanji.character);
            });
            ui.horizontal(|ui| {
                let tmp = &input.to_owned();
                let te = match gs.game_mode{
                    GameMode::Japanese => {egui::TextEdit::singleline(input).hint_text("ひらがなで入力してください")},
                    _ => {egui::TextEdit::singleline(input).hint_text("アルファベットで入力してください")}
                };
                
                let res = ui.add(te);
                if *is_input_request{
                    res.request_focus();
                    *is_input_request = false;
                }
                if tmp != input{ gs.audio.play("Windows Menu Command"); }
                if res.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    *is_input_request = true;
                    let b = gs.kanjis.kanji[0].check_answer(&input, &gs.game_mode);
                    if b{
                        if gs.current_number != 20{
                            gs.current_number += 1;
                            gs.result.push(gs.kanjis.kanji[0].kanji.clone());
                            gs.kanjis.delete();
                            gs.audio.play("Speech Sleep");
                        }else{
                            gs.current_number = 1;
                            gs.result.push(gs.kanjis.kanji[0].kanji.clone());
                            gs.kanjis.delete();
                            *count = common::COUNTDEFAULT;
                            gs.state = State::Ending;
                            gs.audio.play("Alarm03");
                        }
                    }else{
                        gs.result.push(gs.kanjis.kanji[0].kanji.clone());
                        gs.kanjis.delete();
                        gs.state = State::GameOver;
                        gs.audio.play("Windows Critical Stop");
                    }
                    *count = common::COUNTDEFAULT;
                    input.clear();
                }
            });

            *count = *count - ctx.input().unstable_dt;
            if *count < 0.000{
                *count = common::COUNTDEFAULT;
                gs.result.push(gs.kanjis.kanji[0].kanji.clone());
                gs.kanjis.delete();
                gs.state = State::GameOver;
                gs.audio.play("Windows Critical Stop");
                *is_input_request = true;
            }
            ui.vertical_centered_justified(|ui|{
                ui.heading(&(*count as u32).to_string());
            });
            if *count < 9.0 && *old_count as usize != *count as usize{//カウント1秒ごとに音を鳴らす
                gs.audio.play("Windows Feed Discovered");
            }
            *old_count = *count;
            ui.ctx().request_repaint();
        });
    }
}
