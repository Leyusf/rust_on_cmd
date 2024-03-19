use std::{fmt::Display, fs::File, io::Read, path::PathBuf, time::Duration, vec};
use std::fmt::Formatter;

use regex::Regex;

/// 歌词
pub struct Lyrics {
    /// 所有单个句子的集合
    pub list: Vec<Lyric>
}

impl Display for Lyrics {
    /// 设定歌词显示的格式
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "歌词数量: {}", self.list.len())
    }
}

/// 歌词类(一个句子)
pub struct Lyric {
    /// 当前句子的时间点
    pub time: Duration,
    /// 当前句子
    pub content: String
}

impl Lyrics {

    /// 从文件读取歌词
    pub fn from_read(f: &mut File) -> Self {
        let mut buffer = vec![];
        let regex = Regex::new(r"\[(?P<min>\d+):(?P<sec>\d+).(?P<ms>\d+)](?P<content>[^\[\]]*)").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        let m = String::from_utf8(buffer).unwrap();
        let mut lyrics_vec = vec![];
        for cap in regex.captures_iter(m.as_str()){
            let min = cap["min"].parse::<u64>().unwrap();
            let sec = cap["sec"].parse::<u64>().unwrap();
            let ms = cap["ms"].parse::<u64>().unwrap();
            let dur = Duration::from_millis(ms + sec * 1000 + min * 60 * 1000);
            lyrics_vec.push(Lyric {time:dur, content:String::from(&cap["content"])})
        }

        Self {
            list: lyrics_vec
        }
    }

    /// 从音乐文件路径加载歌词
    pub fn from_music_path(s: &str) -> Self {
        let mut p = PathBuf::from(s);
        p.set_extension("lrc");
        let f = File::open(p);
        match f {
            Ok(mut f) => Lyrics::from_read(&mut f),
            Err(_) => Self{ list:vec![] }

        }
    }

    /// 统计歌词的文本长度(多少个句子)
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.list.len()
    }
}