use crate::task::{
    exit_current_and_run_next,
    suspend_current_and_run_next,
    get_current_task
};
use crate::timer::get_time_ms;

pub fn sys_exit(xstate: i32) -> !{
    println!("[kernel] Application {} exited with code {}", get_current_task(), xstate);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_gettime() -> isize {
    get_time_ms() as isize
}