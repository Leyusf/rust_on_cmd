
use sysinfo::System;
use resource_monitor::{show_resources, Opt};
use std::{thread::sleep, time::Duration};
////一个统计系统CPU，内存，磁盘使用情况的rust程序
fn main(){
    let opt = Opt::new();
    let mut sys = System::new_all();
    loop {
        sys.refresh_all();
        show_resources(&sys, opt.get_cpu_option(), opt.get_memory_option(), opt.get_disk_option());
        sleep(Duration::from_millis(1000));
        clearscreen::clear().expect("failed to clear screen");
    }
}
