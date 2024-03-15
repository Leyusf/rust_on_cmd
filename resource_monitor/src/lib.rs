use clap::Parser;
use sysinfo::System;
use yansi::{Paint, Style, Color::*};
use sysinfo::Disks;

static CPU: Style = Blue.bold().on_bright_white();
static MEMORY: Style = Red.bold().on_bright_yellow();
static DISK: Style = BrightMagenta.bold().on_bright_cyan();

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opt{
    #[arg(long, short, help="显示cpu使用情况")]
    cpu_on: bool,
    #[arg(long, short, help="显示内存使用情况")]
    memory_on: bool,
    #[arg(long, short, help="显示硬盘使用情况")]
    disk_on: bool
}

impl Opt {
    pub fn new() -> Self{
        Opt::parse()
    }

    pub fn get_cpu_option(&self) -> bool {
        self.cpu_on
    }

    pub fn get_memory_option(&self) -> bool {
        self.memory_on
    }

    pub fn get_disk_option(&self) -> bool {
        self.disk_on
    }
}

pub fn show_rsources(sys: &System, cpu_on: bool, memory_on: bool, disk_on: bool) -> () {
    if cpu_on {
        println!("{}", format!("CPU 数量: {}", sys.cpus().len()));
        eprintln!("{}", format!("{:<8}  {:<8}   {:<5}", "CPU名字", "CPU使用率", "CPU频率").paint(CPU));
        for cpu in sys.cpus() {
            eprintln!("{}", format!("{:<10}  {:<10}%   {:<5}MHz", cpu.name(), cpu.cpu_usage(), cpu.frequency()).paint(CPU));
        }
    }
    if memory_on {
        println!("内存:");
        eprintln!("{}", format!("内存使用情况: {:.2}/{:.2} GB", bytes_to_gb(sys.used_memory()), bytes_to_gb(sys.total_memory())).paint(MEMORY));
    }
    if disk_on {
        println!("磁盘:");
        eprintln!("{}", format!("{} {} {}", "种类", "文件系统", "使用情况").paint(DISK));
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            eprintln!("{}", format!("{:?} {:?} {:.2}/{:.2}GB", disk.kind(), disk.file_system(), bytes_to_gb(disk.available_space()), bytes_to_gb(disk.total_space())).paint(DISK));
        }
    }
    ()
}

fn bytes_to_gb(byes: u64) -> f64{
    byes as f64 / (1024.0 * 1024.0 * 1024.0)
}