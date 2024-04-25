use std::io;

use anyhow::{Context, Result};
use clap::Parser;

use rgrep::find_matches;

/// 匹配命令行参数中的模式和文件路径
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(||format!("不能读取文件{}", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut io::stdout());
    Ok(())
}