use std::time::Duration;

/// 配置文件,配置了 刷新率 连续帧间隔时间 主页信息
pub struct Config{
    /// 刷新率
    pub refresh_rate: Duration,
    /// 连续帧间隔时间
    pub tick_gap: Duration,
    /// 主页信息
    pub home_page: &'static str
}

impl Config {
    /// 生成配置
    pub fn new() -> Self {
        Self {
            refresh_rate: Duration::from_millis(15),
            tick_gap: Duration::from_millis(100),
            home_page: "https://github.com/Leyusf"
        }
    }
}