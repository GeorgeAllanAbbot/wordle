mod wordle_cli_args;
mod WordleConfig;
/// mod wordle_console;

use std::collections::HashSet;
use std::fs;
use std::io::{self};

pub struct DifficultInfo {
    greens: [Option<char>; 5],
    must_use: Vec<char>,
}

/// 未完成
impl DifficultInfo {
    fn new() -> DifficultInfo {
        DifficultInfo {
            greens: [None; 5],
            must_use: Vec::<char>::new(),
        }
    }
    pub fn difficultCheck(&self,word_input: &String) -> bool{
        // 检查绿色位置
        for (i, c) in word_input.chars().enumerate() {
            if let Some(green_char) = self.greens[i] {
                if c != green_char {
                    return false;
                }
            }
        }
        // 检查必须包含的字母
        for &must_char in &self.must_use {
            if !word_input.contains(must_char) {
                return false;
            }
        }
        true
    }  
}

// 运行中的信息
pub struct WordleGameInfo{
    pub acceptableSet: HashSet<String>,
    pub finalSet: Vec<String>,
    pub correctAnswer: String,
    pub tryTimes: u8,
}
impl WordleGameInfo{
    pub fn new() -> WordleGameInfo{
        WordleGameInfo{
            acceptableSet: HashSet::<String>::new(),
            finalSet: Vec::<String>::new(),
            correctAnswer: String::new(),
            tryTimes: 0,
        }
    }
    // 加载词库（允许的范围）
    fn load_allowed_word_list(path: &str) -> io::Result<HashSet<String>> {
        let content = fs::read_to_string(path)?;
        let mut word_set = HashSet::new();
        for line in content.lines() {
            let word = line.trim().to_uppercase();
            if word.is_empty() {
                continue;
            }
            word_set.insert(word);
        }
        Ok(word_set)
    }
    fn load_word_range(path: &str) -> io::Result<Vec<String>> {
        let content = fs::read_to_string(path)?;
        let mut word_set = Vec::<String>::new();
        for line in content.lines() {
            let word = line.trim().to_uppercase();
            if word.is_empty() {
                continue;
            }
            word_set.push(word);
        }
        Ok(word_set)
    }
}

// 游戏行为 trait
pub trait WordleGame {
    fn game_info(&self) -> &WordleGameInfo;
    fn game_config(&self) -> &WordleConfig::GameConfig;
    fn difficult_info(&self) -> &DifficultInfo;
    // 合法输入检测
    fn CheckWord(&self,word_input: &String) -> bool{
        // 是否有
        if !self.game_info().acceptableSet.contains(word_input) {
            return false;
        }
        // 困难模式
        if self.game_config().difficult {
            return self.difficult_info().difficultCheck(word_input);
        }
        else {
            true
        }
    }
    fn show(&self);
    fn wordle_end(&self);
    fn wordle_one_end(&self);
    fn word_not_in_list(&self);
    fn save(&self) {

    }

    // 运行
    fn run(&mut self){
        // wait
        let args = WordleCliArgs::parse();
        let load_config_path = args.state_path;
        self.game_config().loadFrom(load_config_path);
        let load_archieve_path = args.state.clone();
        let rando = args.random;

        loop {
            self.game_info().tryTimes = 0;
            while self.game_info().tryTimes < 7 {
                let mut input_word = String::new();
                io::stdin().read_line(&mut input_word);
                if self.game_info().acceptableSet.contains(&input_word) {
                    if self.game_config().difficult && !self.CheckWord(&input_word) {
                        continue;
                    }
                    else{
                        self.save();
                        self.show();
                        self.game_info().tryTimes += 1;
                    }
                }
                else{
                    self.word_not_in_list();
                }
            }
            self.wordle_one_end();
        }
        self.wordle_end();
    }
}

// TTY 交互模式
pub struct WordleGameTty {
    pub gameConfig: WordleConfig::GameConfig,
    pub gameInfo: WordleGameInfo,
    pub difficultInfo: DifficultInfo,
}
impl WordleGameTty {
    fn new() -> WordleGameTty {
        WordleGameTty {
            gameConfig: WordleConfig::GameConfig::new(),
            gameInfo: WordleGameInfo::new(),
            difficultInfo: DifficultInfo::new(),
        }
    } 
}
impl WordleGame for WordleGameTty {
    fn game_info(&self) -> &WordleGameInfo {
        &self.gameInfo
    }
    
    fn game_config(&self) -> &WordleConfig::GameConfig {
        &self.gameConfig
    }
    
    fn difficult_info(&self) -> &DifficultInfo {
        &self.difficultInfo
    }
    fn wordle_one_end(&self){
        if self.gameInfo.tryTimes == 7 {
            println!("FAILED {}",self.gameInfo.correctAnswer);
        }
        else {
            println!("CORRECT {}",self.gameInfo.tryTimes);
        }
    }
    fn show(&self){
        
    }
    fn wordle_end(&self){

    }
    fn word_not_in_list(&self){

    }
}

// 非 TTY 非交互模式
pub struct WordleGameNotTty {
    pub gameConfig: WordleConfig::GameConfig,
    pub gameInfo: WordleGameInfo,
    pub difficultInfo: DifficultInfo,
}
impl WordleGameNotTty {
    fn new() -> WordleGameNotTty {
        WordleGameNotTty {
            gameConfig: WordleConfig::GameConfig::new(),
            gameInfo: WordleGameInfo::new(),
            difficultInfo: DifficultInfo::new(),
        }
    } 
}
impl WordleGame for WordleGameNotTty {
    fn game_info(&self) -> &WordleGameInfo {
        &self.gameInfo
    }
    fn game_config(&self) -> &WordleConfig::GameConfig {
        &self.gameConfig
    }
    fn difficult_info(&self) -> &DifficultInfo {
        &self.difficultInfo
    }
    fn wordle_one_end(&self){
    }// 无需输出
    fn show(&self){
    }
    fn wordle_end(&self){

    }
    fn word_not_in_list(&self){
        
    }
}