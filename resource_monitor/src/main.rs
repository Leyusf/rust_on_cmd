////一个统计系统CPU使用情况，内存使用情况的rust程序
use sysinfo::System;
use resource_monitor::{show_rsources, Opt};
use std::{thread::sleep, time::Duration};
fn main(){
    let opt = Opt::new();
    let mut sys = System::new_all();
    loop {
        sys.refresh_all();
        show_rsources(&sys, opt.get_cpu_option(), opt.get_memory_option(), opt.get_disk_option());
        sleep(Duration::from_millis(1000));
        clearscreen::clear().expect("failed to clear screen");
    }
}
