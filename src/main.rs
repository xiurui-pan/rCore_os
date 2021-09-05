#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![allow(dead_code)]
#![allow(deprecated)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod trap;
mod syscall;
mod batch;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    // extern "C" {
    //     fn stext();
    //     fn etext();
    //     fn srodata();
    //     fn erodata();
    //     fn sdata();
    //     fn edata();
    //     fn sbss();
    //     fn ebss();
    // }
    // println!("[kernel] .text [{:#x}, {:#x})", stext as usize, etext as usize);
    // println!("[kernel] .rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    // println!("[kernel] .data [{:#x}, {:#x})", sdata as usize, edata as usize);
    // println!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    println!("[kernel] Hello world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}