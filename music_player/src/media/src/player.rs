
/// 媒体文件类别枚举
pub enum Source {
    /// meu8 网络流媒体文件
    M3u8,
    /// 本地文件
    Local(String)
}

/// 媒体载体类
pub struct Media {
    /// 媒体源
    pub src: Source
}