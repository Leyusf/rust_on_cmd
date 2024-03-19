use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::Sender;

/// 下载超时错误
pub struct DownloadTimeoutError {
    msg: String
}

impl Display for DownloadTimeoutError {
    /// 下载超时则显示超时错误
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "下载超时: {}", self.msg.as_str())
    }
}

impl Debug for DownloadTimeoutError {
    /// 下载超时则显示超时错误
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "下载超时: {}", self.msg.as_str())
    }
}

impl failure::Fail for DownloadTimeoutError {}

/// 下载文字内容并发送到另一个线程
#[tokio::main]
pub async fn download(url: &str, sender: &Sender<String>) -> Result<(), failure::Error> {
    let response = reqwest::get(url).await?.text().await?;
    sender.send(response)?;
    Ok(())
}

/// 下载字节内容并发送到另一个线程
#[tokio::main]
pub async fn download_as_bytes(url: &str, sender: &Sender<bytes::Bytes>) -> Result<(), failure::Error> {
    let response = reqwest::get(url).await?.bytes().await?;
    sender.send(response)?;
    Ok(())
}
