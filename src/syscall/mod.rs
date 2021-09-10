mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YEILD: usize = 124;
const SYSCALL_GETTIME: usize = 169;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => fs::sys_write(args[0], args[1] as *const u8, args[2]), 
        SYSCALL_EXIT  => process::sys_exit(args[0] as i32),
        SYSCALL_YEILD => process::sys_yield(),
        SYSCALL_GETTIME => process::sys_gettime(args[0] as *mut usize),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),

    }
}

