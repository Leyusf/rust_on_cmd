use regex::Regex;

use search_files::{Opt, search_files};

/// 用一个简单的文件搜索器，允许用户搜索特定类型的文件或基于文件名的模式匹配。
/// 使用: search -r {搜索的根目录} -p {文件名}
/// 模式匹配遵从 正则匹配格式
fn main() -> Result<(), &'static str> {
    let opt = Opt::new();
    let mut files = Vec::new();
    let re:Result<Regex, regex::Error> = match opt.get_pattern_regex() {
        Ok(re) => Ok(re),
        Err(_) => return Err("请输入有效的正则匹配模式")
    };
    search_files(opt.get_root().unwrap(), &re.unwrap(), opt.get_max_deep(), 0, &mut files);
    for file in files{
        println!("{}", file.to_str().unwrap());
    }
    Ok(())
}
