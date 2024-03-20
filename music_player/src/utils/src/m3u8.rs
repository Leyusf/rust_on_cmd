use std::fs::read_dir;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use failure::format_err;
use m3u8_rs::Playlist;

use crate::net;


/// 下载 m3u8 类型的播放列表
pub fn download_m3u8_playlist(url: String) -> Result<Playlist, failure::Error> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || net::download(url.as_str(), &sender));
    let response = receiver.recv_timeout(Duration::from_secs(5));
    if let Ok(data) = response {
        let playlist = m3u8_rs::parse_playlist(data.as_bytes());
        match playlist {
            Ok(list) => Ok(list.1),
            Err(_) => Err(format_err!("解析播放列表失败")),
        }
    } else {
        println!("{:?}", response);
        Err(format_err!("下载超时 5 秒"))
    }
}

/// 清空缓存
pub fn empty_cache() {
    let mut dir = dirs::cache_dir().unwrap();
    dir.push("MusicPlayer");
    if dir.exists() && dir.is_dir() {
        let it = read_dir(dir.clone());
        if let Ok(dirs) = it {
            for dir in dirs {
                if let Ok(d) = dir {
                    std::fs::remove_file(d.path()).unwrap();
                }
            }
        }
    }
}