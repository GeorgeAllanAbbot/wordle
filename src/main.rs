/// main.rs

/// 游戏模块
mod wordle_game;


/// 根据是否是终端创建合适的游戏实例
pub fn create_game(is_tty: bool) -> Box<dyn wordle_game::WordleGame> {
    if is_tty {
        Box::new(wordle_game::WordleGameTty())
    } else {
        Box::new(wordle_game::WordleGameNotTty())
    }
}

/// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 端检测
    let is_tty = atty::is(atty::Stream::Stdout);
    // 创建游戏主体
    let mut wordle_game_main = create_game(is_tty);

    //运行
    if let Err(e) = wordle_game_main.run(){
        eprintln!("Errorcode:{}",e);
        std::process::exit(1);
    }

    Ok(())
}