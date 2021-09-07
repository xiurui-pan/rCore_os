use crate::loader::get_user_stack_sp;

const FD_STDOUT: usize = 1;
const USER_STACK_SIZE: usize = 0x1000;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            extern "C" {
                fn sbss();
                fn ebss();
                //fn te
            }
            let user_stack_top = get_user_stack_sp() as usize;
            let user_stack_bottom = user_stack_top - USER_STACK_SIZE;
            let mut in_range_flag = 1;
            let buf_u = buf as usize;
            // info!("user_stack_top: {:#x}", user_stack_top);
            // debug!("buf_u: {:#x}, len: {:#x}", buf_u, len);
            if buf_u < user_stack_bottom || buf_u+len >= user_stack_top {
                in_range_flag = 0;
            }
            if buf_u >= 0x80400000 {
                in_range_flag = 1;
            }
            if in_range_flag == 0 {
                error!("You are writing an invalid address in the stack!");
                return -1;
            }

            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => { 
            error!("Unsupported fd in sys_write!");
            -1
        }
    }
}