use std::io::Read;
use colored::Colorize;
use std::path;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use std::io::BufReader;
use std::fs::File;

// 使用StructOPT生成一个读取命令行参数的Opt工具
#[derive(Debug, StructOpt)]
#[structopt(
    name = "xd",
    about = "一个模仿Linux xxd以16进制命令查看文件的工具",
    author = "Hugo"
)]
pub struct Opt{
    #[structopt(long, short, help="切换颜色")]
    color_on: bool,
    #[structopt(short, long, help = "停止的长度设置")]
    length: Option<usize>,
    #[structopt(name = "文件名字", parse(from_os_str))]
    input_file: path::PathBuf
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }

    pub fn get_file_name(&self) -> &path::Path {
        &self.input_file
    }

    pub fn get_length(&self) -> Option<usize> {
        self.length
    }

    pub fn get_color_option(&self) -> bool {
        self.color_on
    }
}

pub fn open_reader(filename: &path::Path) -> Result<BufReader<File>, ExitFailure>{
    Ok(BufReader::new(File::open(filename)?))
}

pub fn print_hex(reader: impl Read, length: Option<usize>, color_on: bool) -> Result<(), ExitFailure> {
    let mut reader:Box<dyn Read> = match length {
        Some(length) => Box::new(reader.take(length as u64)),
        None => Box::new(reader)
    };

    let mut buf = [0u8; 16];
    let mut printed = 0;
    loop{
        // 计算单词的打印长度
        let length = reader.read(&mut buf)?;
        if length == 0{
            break;
        }
        // 添加颜色, 打印新的一行
        match color_on {
            true => print!("{}", format!("{:08x}    ", printed).cyan()),
            false => print!("{:08x}    ", printed)
        };
        printed += length;

        // 打印2位的数字
        for i in 0..16{
            // 打印出空格
            if i == 8{
                print!(" ")
            }
            match i < length {
                true => match color_on {
                    true => print!("{}", format!(" {:02x}", buf[i]).green()),
                    false => print!(" {:02x}", buf[i])
                },
                false => print!("   ")
            };
        }

        // 打印行的结尾
        print!("    ");
        for &i in &buf[0..length] {
            let ascii = if i >= 0x20 && i <= 0x7e {i as char} else { '.' };
            match color_on {
                true => print!("{}", format!("{}", ascii).blue()),
                false => print!("{}", ascii)
            };
        }
        println!();
    }


    // 多打印一行
    match color_on {
        true => println!("{}", format!("{:08x}", printed).cyan()),
        false => println!("{:08x}", printed)
    };
    // 总结最后的总字数
    println!("Total length: {} bytes",printed);
    Ok(())
}