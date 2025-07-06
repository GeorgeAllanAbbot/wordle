use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};

// 配置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    #[serde(default = "default_random")]
    random: bool, // 随机模式
    
    #[serde(default = "default_difficult")]
    difficult: bool, // 困难模式
    
    #[serde(default)]
    states: bool, // 统计
    
    #[serde(default = "default_day")]
    day: i64, // 日期
    
    #[serde(default = "default_seed")]
    seed: i64, // 种子
    
    #[serde(default = "default_final_set")]
    final_set: String, // 答案库
    
    #[serde(default = "default_acceptable_set")]
    acceptable_set: String, // 允许的猜测范围
    
    #[serde(default)]
    state: String, // 历史记录
    
    #[serde(default)]
    word: String, // 谜底
}

// 默认值函数
fn default_random() -> bool { false }
fn default_difficult() -> bool { false }
fn default_seed() -> i64 { 1 }
fn default_day() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}
fn default_final_set() -> String { "possible_words.txt".into() }
fn default_acceptable_set() -> String { "allowed_words.txt".into() }

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            random: default_random(),
            difficult: default_difficult(),
            stats: false,
            day: default_day(),
            seed: default_seed(),
            final_set: default_final_set(),
            acceptable_set: default_acceptable_set(),
            state: String::new(),
            word: String::new(),
        }
    }
}

impl GameConfig {
    /// 从JSON文件加载配置
    pub fn load_from(load_config_path: &str) -> Result<Self> {
        // 打开文件
        let file = File::open(load_config_path)
            .context(format!("Failed to open config file: {}", load_config_path))?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .context("Failed to parse config JSON")?;
        
        Ok(config)
    }
}