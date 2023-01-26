use super::kanji_info::*;
use super::define::*;

#[derive(Debug)]
pub struct Kanji{
    pub kanji: Vec<KanjiSet>
}

impl Default for Kanji{
    fn default() -> Self{
        Self{kanji: Vec::new()}
    }
}

impl Kanji{

    pub fn get_kanji_count(&self) -> usize{
        let mut count = 0;
        for k in &self.kanji{
            if k.state != GetKanjiState::Idle{continue;}
            count += 1;
        }
        count
    }
    pub fn update(&mut self){
        for k in &mut self.kanji{
            k.update();
        }
        if self.kanji.len() != common::KANJINUM{
            let sa =  common::KANJINUM - self.kanji.len();
            for _ in 0..sa{
                self.add_kanji();
            }
        }
    }
    pub fn delete(&mut self){
        if self.kanji.len() > 0{
            self.kanji.remove(0);
        }
        
    }
    pub fn add_kanji(&mut self){
        let mut kanji = KanjiSet::default();
        kanji.new();
        self.kanji.push(kanji);
    }
}