use std::fs::File;
use std::io::BufReader;
use std::ops::Add;
use std::time::{Duration, Instant};

use utils::lyrics::Lyrics;
use rodio::cpal;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use tui::widgets::ListState;

use media::Media;

/// 播放状态的枚举有
/// 等待
/// 正在播放
/// 播放完毕
/// 三种状态
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayStatus {
    /// 等待
    Waiting,
    /// 正在播放 (播放开始时间， 播放时长)
    Playing(Instant, Duration),
    /// 播放完毕 (播放时长)
    Stopped(Duration)
}

/// 音乐播放列表的子项
pub struct PlayListItem {
    pub name: String,
    pub duration: Duration,
    pub current_pos: Duration,
    pub status: PlayStatus,
    pub path: String,
    pub lyrics: Lyrics,
    pub lyrics_index: ListState
}

/// 音乐播放列表
pub struct PlayList {
    pub lists: Vec<PlayListItem>
}

/// 声明播放器特征
pub trait Player {

    /// 新建一个播放器
    fn new() -> Self;

    fn add_to_list(&mut self, media: Media, once: bool) -> bool;

    /// 播放
    fn play(&mut self) -> bool;

    /// 下一首
    fn next(&mut self) -> bool;

    /// 结束
    fn stop(&mut self) -> bool;

    /// 暂停
    fn pause(&mut self) -> bool;

    /// 继续播放
    fn resume(&mut self) -> bool;

    /// 获取播放进度
    fn get_progress(&self) -> (f32, f32);

    /// 是否正在播放
    fn is_playing(&self) -> bool;

    /// 提供一个接口，更新 player 状态
    fn tick(&mut self);

    /// 当前的歌词
    fn current_lyric(&self) -> &str;

    /// 是否有歌词
    fn has_lyrics(&self) -> bool;

    /// 当前音量
    fn volume(&self) -> f32;

    /// 设置音量
    fn set_volume(&mut self, new_volume: f32) -> bool;
}

/// 音乐播放器
pub struct MusicPlayer {
    /// 当前歌曲播放时长
    pub current_time: Duration,
    /// 当前歌曲总播放时长
    pub total_time: Duration,
    /// 播放列表
    pub play_list: PlayList,
    /// 输出流数据
    stream: OutputStream,
    /// 输出流Handle
    stream_handle: OutputStreamHandle,
    /// 数据发送的目的地
    sink: Sink,
    /// 当前的歌词
    current_lyric: Option<String>,
    /// 是否初始化
    initialized: bool
}

impl Player for MusicPlayer {
    /// 新建一个 Player
    fn new() -> Self {
        for dev in cpal::available_hosts() {
            println!("{:?}", dev);
        }
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            current_time: Duration::from_secs(0),
            total_time: Duration::from_secs(0),
            play_list: PlayList { lists: vec![] },
            stream,
            stream_handle,
            sink,
            current_lyric: None,
            initialized: false
        }
    }

    fn add_to_list(&mut self, media: Media, once: bool) -> bool {
        match media.src {
            media::Source::Local(path) => self.play_with_file(path, once),
            media::Source::M3u8(_path) => false
        }
    }

    /// 播放歌曲
    fn play(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                // 当前没有播放曲目，改变播放状态
                PlayStatus::Waiting => {
                    *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                }
                // 当前正在播放，则什么都不做
                PlayStatus::Playing(_, _) => {}
                // 改变播放状态
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        true
    }

    /// 播放下一曲
    fn next(&mut self) -> bool {
        if self.play_list.lists.len() >= 1 {
            // 播放列表有下一曲歌曲
            self.play_list.lists.remove(0);
            self.stop();
            if !self.play_list.lists.is_empty() {
                let top_music = self.play_list.lists.first().unwrap();
                let f = File::open(top_music.path.as_str()).unwrap();
                let buf_reader = BufReader::new(f);
                let (stream, stream_handle) = OutputStream::try_default().unwrap();
                self.stream = stream;
                self.stream_handle = stream_handle;
                let volume = self.volume();
                self.sink = Sink::try_new(&self.stream_handle).unwrap();
                self.set_volume(volume);
                self.sink.append(Decoder::new(buf_reader).unwrap());
                self.play();
            }
        }
        else {
            // 没有下一曲歌曲
            return false
        }
        true
    }

    /// 停止播放
    fn stop(&mut self)-> bool {
        self.sink.stop();
        true
    }

    /// 暂停播放
    fn pause(&mut self) -> bool {
        self.sink.pause();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &item.status;
            match status {
                PlayStatus::Waiting => {},
                PlayStatus::Playing(instant, duration) => {
                    *status = PlayStatus::Stopped(duration.add(instant.elapsed()));
                }
                PlayStatus::Stopped(_) => {}
            }
        }
        true
    }

    /// 恢复播放
    fn resume(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {}
                PlayStatus::Playing(_, _) => {}
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        return true;
    }

    fn get_progress(&self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn is_playing(&self) -> bool {
        self.initialized && !self.sink.is_paused() && !self.play_list.lists.is_empty()
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }

    fn set_volume(&mut self, new_volume: f32) -> bool {
        self.sink.set_volume(new_volume);
        true
    }

    fn has_lyrics(&self) -> bool {
        !self.play_list.lists.is_empty() && !self.play_list.lists.first().unwrap().lyrics.list.is_empty()
    }

    fn current_lyric(&self) -> &str {
        if let Some(lyric) = &self.current_lyric {
            lyric.as_str()
        }
        else {
            "暂无歌词"
        }
    }


    fn tick(&mut self) {
        let is_playing = self.is_playing();
        if let Some(song) = self.play_list.lists.first_mut() {
            let status = &song.status;
            match status {
                // 如果没有播放,且开始播放,则更新到播放状态
                PlayStatus::Waiting => {
                    if is_playing {
                        *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                    }
                }
                // 如果正在播放,则更新播放进度
                PlayStatus::Playing(instant, duration) => {
                    let now = instant.elapsed().add(duration.clone);
                    if now.ge(&song.duration) {
                        // 如果播放结束,则切到下一首歌
                        self.next()
                    } else {
                        // 如果还在播放,则更新播放进度
                        self.current_time = now;
                        self.total_time = song.duration.clone();
                        let select_index = song.lyrics_index.selected().unwrap();
                        // 更新显示的歌词
                        if selected_index + 1 < song.lyrics.list.len() {
                            let next_lyric = &song.lyrics.list[selected_index + 1];
                            if self.current_time > next_lyric.time {
                                song.lyrics_index.select(Some(selected_index + 1));
                            }
                        }
                    }
                }
                PlayStatus::Stopped(duration) => {
                    self.current_time = duration.clone();
                    self.total_time = song.duration.clone();
                }
            }
        } else {
            // 没有歌曲的时候停止播放
            if self.play_list.lists.is_empty() {
                self.stop();
            }
        }
    }
}