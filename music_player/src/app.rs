/// 输入模式
pub enum InputMode {
    /// 正常模式
    Normal,
}

/// 页面
#[derive(Clone, Copy, PartialEq)]
pub enum Routes {
    /// 主页
    Main,
    /// 帮助页面
    Help
}

/// 播放的模式
#[derive(PartialEq)]
pub enum ActiveModules {
    /// 本地模式
    Fs,
    /// 广播模式
    RadioList,
}

/// 程序应用的主体
pub struct App {

}
