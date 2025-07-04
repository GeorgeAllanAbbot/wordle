// wordle_config.rs

// 配置
pub struct GameConfig {
    random: bool, // 随机模式
    difficult: bool, // 困难模式
    stats: bool, // 
    day: i64, // 日期
    seed: i64, // 种子
    final_set: String, // 答案库
    acceptable_set: String, // 允许的猜测范围
    state: String, // 目前的猜测
    word: String, // 谜底
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            random: true,
            difficult: false,
            stats: false,
            day: Instant::now();
            seed: 0,
            final_set: String::new(),
            acceptable_set: String::new(),
            state: String::new(),
            word: String::new(),
        }
    } 
}