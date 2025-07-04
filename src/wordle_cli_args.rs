// wordle_cli_args.rs
use clap::Parser;

// 命令行参数解析
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// 加载游戏状态
    #[arg(short = 'S', long = "state"), default_value = "state.json"]
    state_path: String,

    // 困难模式
    #[arg(short = 'D', long = "difficult")]
    difficult_mode: bool,

    // 随机模式
    #[arg(short = 'r', long = "random")]
    file_path: String,

    // 指定答案模式
    #[arg(short = 'w', long = "word")]
    file_path: String,

    // 输入候选词库
    #[arg(short = 'f', long = "final-set")]
    file_path: String,

    // 输入可用词库
    #[arg(short = 'a', long = "acceptable-set")]
    file_path: String,

    // 输入开始局数
    #[arg(short = 'd', long = "day"), default_value = 1]
    file_path: u64,//注意上限

    // 输入种子
    #[arg(short = 's', long = "seed"), default_value = 1]
    file_path: u64,//注意上限
}

impl Args{
    fn get_args() -> Vector<bool>{
        {}
    }
}