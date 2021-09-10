use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

pub const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const USEC_PER_SEC: usize = 1000000;

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / USEC_PER_SEC)
}

pub fn gettime(ts: *mut usize) -> isize {
    let usec = get_time_us();
    unsafe {
        *ts = usec / USEC_PER_SEC;
        *ts.offset(1) = usec % USEC_PER_SEC;
        0
    }
}

pub fn set_next_trigger() {
    set_timer(time::read() + CLOCK_FREQ / TICKS_PER_SEC);
}
