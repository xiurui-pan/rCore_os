#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]

#[macro_use]
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

const SBI_SHUTDOWN: usize = 8;

pub fn shutdown() -> ! {
    sbi::sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

#[no_mangle]
pub fn rust_main() -> ! {
    shutdown()
}