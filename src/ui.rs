use eframe::{egui, App};
use egui::{FontFamily, FontId, TextStyle};
use super::ui_state::*;

pub struct Gui{
    uis: Uis,
    global: GlobalStruct,
}
impl Default for Gui{
    fn default() -> Self{
        Self {
            uis: Uis::default(),
            global: GlobalStruct::default(),
        }
    }
}

pub struct MyApp{
    gui: Gui,
}
impl Default for MyApp{
    fn default() -> Self{
        Self{
            gui: Gui::default(),
        } 
    }
}

impl MyApp{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self{
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(40.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/Meiryo.ttc")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let app = MyApp::default();
        app
    }
}

impl App for MyApp{
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>){}
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        let Self{gui} = self;
        if gui.global.state != gui.global.old_state{//状態移行時にsetup関数を実行
            gui.uis.setup_ui(ctx, frame,&mut gui.global); 
            gui.global.old_state = gui.global.state;
        }
        gui.uis.update_ui(ctx, &mut gui.global);
    }
}