pub const USER_STACK_SIZE: usize = 0x1000 * 2;
pub const KERNEL_STACK_SIZE: usize = 0x1000 * 2;
pub const MAX_APP_NUM: usize = 10;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
pub const CLOCK_FREQ: usize = 12500000;
pub const BIG_STRIDE: isize = 255;
pub const MAX_RUNNING_TIME: usize = 5;