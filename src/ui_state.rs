use eframe::{egui};
use egui::{Context};
use windows_audio::*;
use super::kanji_info::*;
use super::kanji::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GameMode{
    Japanese,
    English,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State{
    Title = 0,
    Game = 1,
    GameOver = 2,
    Ending = 3,
}

pub struct GlobalStruct{
    pub current_number: usize,
    pub state: State,
    pub old_state: State,
    pub result: Vec<KanjiBase>,
    pub audio: Audio,
    pub kanjis: Kanji,
    pub game_mode: GameMode,
    pub is_clear: bool
}
impl Default for GlobalStruct{
    fn default() -> Self{
        Self { 
            current_number: 1,
            state: State::Title,
            old_state: State::GameOver,
            result: Vec::new(),
            audio: Audio::new(), 
            kanjis: Kanji::default(), 
            game_mode: GameMode::Japanese,
            is_clear: false,
        }
    }
}

/// Something to view
pub trait MyUi {
    fn setup(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, gs: &mut GlobalStruct);
    fn update(&mut self, ctx: &egui::Context, gs: &mut GlobalStruct);
}


pub struct Uis{
    pub myuis:  Vec<Box<dyn MyUi>>,
}
impl Default for Uis{
    fn default() -> Self{
        Self::from_myuis(vec![
            Box::new(super::ui_title::Title::default()),
            Box::new(super::ui_game::Game::default()),
            Box::new(super::ui_gameover::GameOver::default()),
            Box::new(super::ui_ending::Ending::default()),
        ])
    }
}

impl Uis {
    pub fn from_myuis(myuis: Vec<Box<dyn MyUi>>) -> Self {
        Self { myuis }
    }
    pub fn setup_ui(&mut self, ctx: &Context, frame: &mut eframe::Frame, app: &mut GlobalStruct) {
        let Self { myuis } = self;
        let num = app.state as usize;
        myuis[num].setup(ctx, frame, app);
    }
    pub fn update_ui(&mut self, ctx: &Context, app: &mut GlobalStruct) {
        let Self { myuis } = self;
        let num = app.state as usize;
        myuis[num].update(ctx, app);
    }
}