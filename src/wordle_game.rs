mod wordle_cli_args;
mod wordle_config;
mod wordle_console;


use std::collections::HashSet;
use std::collections::Vector;
use std::io;
use std::fs;// 等等

// 游戏行为 trait
pub trait WordleGame {
    // 合法输入检测
    fn CheckWord(&self,word_input: &String) -> bool{
        // 是否有
        if !self.gameInfo.acceptable_set.contains(word_input) {
            return false;
        }
        
        // 困难模式
        if self.gameConfig.difficult {
            return difficultInfo.difficultCheck(word_input);
        }
        else {
            true
        }
    }
    fn show(&self);
    fn wordle_one_end(&self,times: &u8);
    fn save(&self) {

    }

    // 运行
    pub fn run(&mut self){
        let args = Args::parse();
        let load_config_path = args.state_path;
        gameConfig.loadFrom(load_config_path);
        let load_archieve_path = args.state.clone();
        if let rando = args.random;
        while(gameContinue){
            let self.gameInfo.tryTimes = 0;
            while(times < 7){
                let input_word = String::None();
                io::stdin().read_line(&mut input_word)?;
                if !gameInfo.acceptableSet.contains(&input_word) {
                    if difficult and !CheckWord(word_input,difficultInfo) {

                    }
                    else{
                        self.save();
                        self.show();
                        self.gameInfo.tryTimes++;
                    }
                }
                else{
                    self.word_not_in_list(word_input);
                }
            }
            self.wordle_one_end(times);
        }
        self.wordle_end();
        Ok(0)
    }
}

// 运行中的信息
pub struct WordleGameInfo{
    pub acceptableSet: HashSet<String>,
    pub finalSet: Vector<String>,
    pub correct_answer: String,
    pub tryTimes: u8,
}
impl WordleGameInfo{
    // 加载词库（允许的范围）
    fn load_word_list(path: &str) -> io::Result<HashSet<String>> {
        let content = fs::read_to_string(path)?;
        Ok(content
            .lines()
            .map(|s| s.trim().to_uppercase()) // 统一转换为大写
            .collect())
    }
}

// TTY 交互模式
pub struct WordleGameTty {
    gameConfig: GameConfig,
    gameInfo: WordleGameInfo,
    difficultInfo: DifficultInfo,
}
impl WordleGame for WordleGameTty {
    fn wordle_one_end(&self){
        if self.gameInfo.tryTimes == 7 {
            println!("FAILED {}",self.gameInfo.correct_answer);
        }
        else {
            println!("CORRECT {}",self.gameInfo.tryTimes);
        }
    }
    fn show(&self){
        
    }

}

// 非 TTY 非交互模式
pub struct WordleGameNotTty {
    pub gameConfig: GameConfig,
    pub gameInfo: WordleGameInfo,
    pub difficultInfo: DifficultInfo,
}
impl WordleGame for WordleGameNotTty {
    fn wordle_one_end(&self){
    }// 无需输出
    fn show(&self){
        
    }
}

struct DifficultInfo {

}

/// 未完成
impl DifficultInfo {
    pub fn difficultCheck(word_input: &String) -> bool{
        true
    }
}