use exitfailure::ExitFailure;
use xxd::{open_reader, Opt, print_hex};
fn main() -> Result<(), ExitFailure> {
    // 解析参数和文件名字
    let opt = Opt::new();
    let filename = opt.get_file_name();
    println!("File name: {:?}", filename);

    // 获取内容然后打印
    let reader = open_reader(filename)?;
    print_hex(reader, opt.get_length(), opt.get_color_option())?;
    Ok(())
}
