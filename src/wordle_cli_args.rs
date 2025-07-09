/// wordle_cli_args.rs
use clap::Parser;

/// 命令行参数解析
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 加载游戏状态
    #[arg(short = 'S', long = "state")]
    state_path: Option<String>,

    /// 困难模式
    #[arg(short = 'D', long = "difficult")]
    difficult_mode: bool,

    /// 随机模式
    #[arg(short = 'r', long = "random")]
    random: bool,

    /// 指定答案模式
    #[arg(short = 'w', long = "word")]
    set_answer: Option<String>,

    /// 输入候选词库
    #[arg(short = 'f', long = "final-set")]
    set_final_set: Option<String>,

    /// 输入可用词库
    #[arg(short = 'a', long = "acceptable-set")]
    set_acceptable_set: Option<String>,

    /// 输入开始局数 注意上限 默认值
    #[arg(short = 'd', long = "day", default_value = "1")]
    set_day: u64,

    /// 输入种子 注意上限 默认值
    #[arg(short = 's', long = "seed", default_value = "1")]
    set_seed: u64,
}