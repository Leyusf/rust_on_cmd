use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use regex;
use regex::{Error, Regex};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Opt{
    #[arg(long, short, help="搜索的根目录", default_value=".")]
    root: PathBuf,
    #[arg(long, short, help="被搜索文件的名字/类型", default_value="*.*")]
    pattern: String,
    #[arg(long, short, help="最大搜索深度", default_value="5")]
    deep: u32
}

impl Opt{
    pub fn new() -> Self {
        Opt::parse()
    }

    pub fn get_root(&self) -> Result<&PathBuf, &'static str> {
        if self.root.is_dir() {
            Ok(&self.root)
        }
        else {
            Err("请输入一个有效的根目录")
        }
    }

    pub fn get_max_deep(&self) -> u32{
        self.deep
    }

    pub fn get_pattern_regex(&self) -> Result<Regex, Error> {
        let mut p = self.pattern.replace("*", "[A-Za-z0-9]|[\\|/]*");
        p = p.replace(".", "\\.");
        p = p.replace("\\", "\\\\");
        p = p.replace("/", "/");
        p = format!("^{}$", p);
        Regex::new(&p)
    }
}

pub fn search_files(root: &Path, regex: &Regex, max_deep:u32, cur_deep:u32, results: &mut Vec<PathBuf>) -> () {
    if cur_deep >= max_deep {
        ()
    }
    let entries = fs::read_dir(root).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if regex.is_match(&path.to_str().unwrap()){
                results.push(path);
            }
        }
        else {
            search_files(path.as_path(), regex, max_deep, cur_deep+1, results);
        }
    }
}