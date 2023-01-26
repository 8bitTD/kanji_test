use eframe::{egui};
use super::ui_state::*;
use super::define::*;
#[derive(PartialEq, Eq)]
enum EndingState{
    Move,
    Stop,
}
pub struct Ending{
    ending_state: EndingState,
    ty: f32,
    stop_timer: f32,
}

impl Default for Ending{
    fn default() -> Self {
        Self {
            ending_state: EndingState::Move,
            ty: 170.0,
            stop_timer: 0.0,
        }
    }
}

impl MyUi for Ending{
    fn setup(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame, _gs: &mut GlobalStruct) {
        frame.set_window_title(&format!("{}{}", common::TOOLNAME, " [エンディング]"));
    }
    fn update(&mut self, ctx: &egui::Context, gs: &mut GlobalStruct) {
        let Self{ ending_state, ty, stop_timer} = self;
        if gs.state != State::Ending{return;}
        gs.is_clear = true;
        if *ty > 50.0{
            *ty -= ctx.input().unstable_dt * 20.0;
        }else{
            if *ending_state == EndingState::Move{
                gs.audio.play("Windows Shutdown");
            }
            *ending_state = EndingState::Stop;
        }
        if *ending_state == EndingState::Stop{
            *stop_timer += ctx.input().unstable_dt;
            if *stop_timer > 3.0{
                gs.state = State::GameOver;
                *ending_state = EndingState::Move;
                *ty = 170.0;
                *stop_timer = 0.0;
            }
        }
        egui::CentralPanel::default().show(ctx,|ui|{
            let widget_rect = egui::Rect::from_min_size(egui::Pos2::new(-55.0, *ty), egui::Vec2::new(500.0, 100.0));
            ui.put(widget_rect, egui::Label::new("Thank you for playing!"));
            ui.ctx().request_repaint();
        });
    }
}
