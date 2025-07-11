use crate::wordle_cli_args::Args;
use crate::wordle_config::GameConfig;
use crate::wordle_terminal::WordleTerminal;
use crate::wordle_terminal::WordleTty;
use crate::wordle_terminal::WordleNotTty;
/// mod wordle_console;

use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::{self};

/// 困难模式信息存储
pub struct DifficultInfo {
    greens: [Option<char>; 5],
    must_use: Vec<char>,
}
impl Default for DifficultInfo {
    fn default() -> DifficultInfo {
        DifficultInfo {
            greens: [None; 5],
            must_use: Vec::<char>::new(),
        }
    }
}
/// 未完成
impl DifficultInfo {
    fn difficultCheck(&self,word_input: &String) -> bool{
        // 检查绿色位置
        for (i, c) in word_input.chars().enumerate() {
            if let Some(green_char) = self.greens[i] {
                if c != green_char {
                    return false;
                }
            }
        }
        // 检查必须包含的字母
        for &must_char in &self.must                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        _use {
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
impl Default for WordleGameInfo {
    fn default() -> WordleGameInfo{
        WordleGameInfo{
            acceptableSet: HashSet::<String>::new(),
            finalSet: Vec::<String>::new(),
            correctAnswer: String::new(),
            tryTimes: 0,
        }
    }
}
impl WordleGameInfo{
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
    // 加载词库（答案库）
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

pub struct WordleGame {
    terminal: WordleTerminal,
    gameConfig: GameConfig,
    gameInfo: WordleGameInfo,
    difficultInfo: DifficultInfo,
}

// 游戏
impl WordleGame {

    /// 根据是否是终端创建合适的游戏实例
    pub fn create_game() -> WordleGame {
        // 端检测
        let is_tty = atty::is(atty::Stream::Stdout);
        if is_tty {
            WordleGame {
                terminal: WordleTty::default(),
                gameConfig: GameConfig::default(),
                gameInfo: WordleGameInfo::default(),
                difficultInfo: DifficultInfo::default(),
            }
        } else {
            WordleGame {
                terminal: WordleNotTty::default(),
                gameConfig: GameConfig::default(),
                gameInfo: WordleGameInfo::default(),
                difficultInfo: DifficultInfo::default(),
            }
        }
    }

    // 合法输入检测
    fn CheckWord(&self,word_input: &String) -> bool{
        // 是否有
        if !self.gameInfo.acceptableSet.contains(word_input) {
            return false;
        }
        // 困难模式
        if self.gameConfig.difficult {
            return self.difficultInfo.difficultCheck(word_input);
        }
        else {
            true
        }
    }
    fn save(&self) {
        if
    }
    // 运行
    fn run(&mut self) -> anyhow::Result<()> {
        // wait
        let args = Args::parse();
        if let Some(load_config_path) = args.state_path {
            self.gameConfig = GameConfig::load_from(&load_config_path);
        }
        self.gameConfig.difficult = args.difficult_mode;
        self.gameConfig.random = args.random;
        if let Some(word) = args.set_answer {
            self.gameConfig.word = word;
        }
        if let Some(final_set) = args.set_final_set {
            self.gameConfig.final_set = final_set;
        }
        if let Some(acceptable_set) = args.set_acceptable_set {
            self.gameConfig.acceptable_set = acceptable_set;
        }
        self.gameConfig.day = args.set_day;
        self.gameConfig.seed = args.set_seed;

        loop {
            self.gameInfo.tryTimes = 0;
            while self.gameInfo.tryTimes < 7 {
                let mut input_word = String::new();
                io::stdin().read_line(&mut input_word);
                if self.gameInfo.acceptableSet.contains(&input_word) {
                    if self.gameConfig.difficult && !self.CheckWord(&input_word) {
                        continue;
                    }
                    else{
                        self.save();
                        self.show();
                        self.gameInfo.tryTimes += 1;
                    }
                }
                else{
                    self.word_not_in_list();
                }
            }
            self.wordle_one_end();
        }
        self.wordle_end();
        Ok(())
    }
}