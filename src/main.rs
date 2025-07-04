// main.rs
mod wordle_game;
use wordle_game::wordle_game_tty;
use wordle_game::wordle_game_not_tty;
use std::io;

// 根据是否是终端创建合适的游戏实例
pub fn create_game(config: GameConfig,is_tty: bool) -> Box<dyn WordleGame> {
    if is_tty {
        Box::new(WordleGameTty(config))
    } else {
        Box::new(WordleGameNotTty(config))
    }
}

/// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 端检测
    let is_tty = atty::is(atty::Stream::Stdout);
    // 数据
    let config = GameConfig {
        ..GameConfig::default()
    };
    // 创建游戏主体
    let mut wordle_game_main = create_game(config,is_tty);
    
    //运行
    if let Err(e) = main_game_main.run(){
        eprintln!("Errorcode:{}",e);
        std::process::exit(1);
    }

    Ok(())
}