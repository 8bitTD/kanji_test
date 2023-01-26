use rand::Rng;
use super::define::*;
use super::ui_state::*;

#[derive(Debug, Clone)]
pub struct Yomi{
    pub on_yomi: Vec<String>,
    pub kun_yomi: Vec<String>,
    pub english_meaning: Vec<String>,
}
impl Default for Yomi{
    fn default() -> Self{
        Self { 
            on_yomi: Vec::new(),
            kun_yomi: Vec::new(),
            english_meaning: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KanjiBase{
    pub character: String,
    pub yomi: Yomi,
}

impl Default for KanjiBase{
    fn default() -> Self{
        Self { 
            character: String::from(""), 
            yomi: Yomi::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GetKanjiState{
    Idle,
    GetKanji,
    GetYomi,
}

#[derive(Debug)]
pub struct KanjiSet{
    pub kanji: KanjiBase,
    pub state: GetKanjiState,
    rx_kanji: Option<std::sync::mpsc::Receiver<String>>,
    rx_yomi: Option<std::sync::mpsc::Receiver<Yomi>>,
}


impl Default for KanjiSet{
    fn default() -> Self{
        Self { 
            kanji: KanjiBase::default(),
            state: GetKanjiState::GetKanji,
            rx_kanji: None,
            rx_yomi: None,
        }
    }
}

impl KanjiSet{
    pub fn new(&mut self){
        self.state = GetKanjiState::GetKanji;
        self.get_kanji();
    }

    pub fn update(&mut self){
        if self.state == GetKanjiState::GetKanji{
            let rs = self.rx_kanji.as_ref().unwrap().try_recv();
            if rs.is_ok(){
                let res = rs.unwrap();
                self.kanji.character = res;
                self.state = GetKanjiState::GetYomi;
                self.get_yomi();
            }
        }else if self.state == GetKanjiState::GetYomi{
            let rs = self.rx_yomi.as_ref().unwrap().try_recv();
            if rs.is_ok(){
                let res = rs.unwrap();
                self.kanji.yomi = res;
                self.state = GetKanjiState::Idle;
            }
        }else{

        }
    }

    pub fn check_answer(&self, ans:&str,game_mode: &GameMode) -> bool{
        if game_mode == &GameMode::Japanese{
            for a in &self.kanji.yomi.on_yomi{
                if a == ans{
                    return true;
                }
            }
            for a in &self.kanji.yomi.kun_yomi{
                if a == ans{
                    return true;
                }
            }
            return false;
        }else{
            for a in &self.kanji.yomi.english_meaning{
                if a == ans{
                    return true;
                }
            }
            return false;
        }
    }

    pub fn get_yomi(&mut self){
        let (tx, rx) = std::sync::mpsc::channel();
        self.rx_yomi = Some(rx);
        let kanji = self.kanji.character.to_owned();
        std::thread::spawn(move || {
            let url = format!("{}{}",common::URLKANJI,kanji);
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-RapidAPI-Key",reqwest::header::HeaderValue::from_static("1ad590418dmsh2e0eac274260890p1c2c70jsn71d4168193d2"));
            headers.insert("X-RapidAPI-Host",reqwest::header::HeaderValue::from_static("kanjialive-api.p.rapidapi.com"));
            let client = reqwest::blocking::Client::new();
            let res = client.get(&url).headers(headers).send().unwrap();
            let contents:String = String::from_utf8(res.bytes().unwrap().to_vec()).unwrap();
            let data1:serde_json::Value = serde_json::from_str(&contents).unwrap();
            let data2 = data1.as_object().unwrap();
            let data3 = data2["kanji"].as_object().unwrap();
            let data4 = data3["kunyomi"].as_object().unwrap();
            let kun = data4["hiragana"].as_str().unwrap();
            let res_kun  = if kun == "n/a"{
                vec![]
            }else{
                let res = match kun.contains("、"){
                    true => {
                        let res :Vec<&str> = kun.split("、").collect();
                        res
                    },
                    _ => {
                        vec![kun]
                    }
                };
                res
            };
            let data5 = data3["onyomi"].as_object().unwrap();
            let on = data5["katakana"].as_str().unwrap();
            let con_on = kanaria::string::UCSStr::from_str(on).hiragana().to_string();
            let con_on = con_on.as_str();
            let res_on  = if con_on == "n/a"{
                vec![]
            }else{
                let res = match con_on.contains("、"){
                    true => {
                        let res :Vec<&str> = con_on.split("、").collect();
                        res
                    },
                    _ => {
                        vec![con_on]
                    }
                };
                res
            };
            let mut onyomi = Vec::new();
            let mut kunyomi = Vec::new();
            for k in res_kun{kunyomi.push(k.to_owned());}
            for o in res_on{onyomi.push(o.to_owned());}

            let data5 = data3["meaning"].as_object().unwrap();
            let e_res = data5["english"].as_str().unwrap();
            let mut english_meaning = Vec::new();
            let res = match e_res.contains(","){
                true => {
                    let res :Vec<&str> = e_res.split(",").collect();
                    res
                },
                _ => {
                    vec![e_res]
                }
            };
            for e in res{english_meaning.push(e.trim().to_owned());}

            let mut yomi = Yomi::default();
            yomi.on_yomi = onyomi;
            yomi.kun_yomi = kunyomi;
            yomi.english_meaning = english_meaning;
            tx.send(yomi).unwrap();
        });
    }

    pub fn get_kanji(&mut self){
        let (tx, rx) = std::sync::mpsc::channel();
        self.rx_kanji = Some(rx);
        std::thread::spawn(move || {
            let url = common::URLADVANCED;
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-RapidAPI-Key",reqwest::header::HeaderValue::from_static(common::APIKEY));
            headers.insert("X-RapidAPI-Host",reqwest::header::HeaderValue::from_static(common::APIHOST));
            let client = reqwest::blocking::Client::new();
            let mut rng = rand::thread_rng();
            let rv = rng.gen_range(1..6);
            let res = client.get(url).headers(headers).query(&[("grade",rv.to_string())]).send().unwrap();
            let contents:String = String::from_utf8(res.bytes().unwrap().to_vec()).unwrap();
            let data1:serde_json::Value = serde_json::from_str(&contents).unwrap();
            let data2 = data1.as_array().unwrap();
            let mut kanjis = Vec::new();
            for d in data2{
                let res = d.as_object().unwrap();
                let k = res["kanji"].as_object().unwrap()["character"].as_str().unwrap();
                kanjis.push(String::from(k));
            }
            let mut rng = rand::thread_rng();
            let rv = rng.gen_range(0..kanjis.len());
            tx.send(kanjis[rv].to_owned()).unwrap();
        });
    }
}
