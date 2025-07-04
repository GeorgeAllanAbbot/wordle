use std::time::Instant;
use std::collections::HashSet;
use std::collections::Vector;
use std::fs::File;

// 游戏行为 trait
pub trait WordleGame {
    // 合法输入检测
    fn CheckWord(word_input: &String,difficult: &bool){
        // 困难模式
        if difficult {

        }
        
    }
    // 运行
    fn run(&mut self){
        let args = Args::parse();
        io::stdin().read_line(&mut input_word)?;
        while(){
            if(!gameInfo.acceptableSet.contains(&input_word))
            CheckWord();
        }
        wordle_end();
    }
}

pub struct WordleGameInfo{
    acceptableSet: HashSet<String>,
    finalSet: Vector<String>,
    tryTimes: u8,
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
}

impl WordleGame for WordleGameTty {
    fn show(){
        
    }

}

// 非 TTY 非交互模式
pub struct WordleGameNotTty {
    gameConfig: GameConfig,
    gameInfo: WordleGameInfo,
}

impl WordleGame for WordleGameNotTty {
}

    print!("{}", console::style("Your name: ").bold().red());
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());

    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);+
    }
    println!("");