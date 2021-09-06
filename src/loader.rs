use crate::config::*;

pub fn load_apps() {
    extern "C" { fn num_app(); }
    let num_app_ptr = num_app as usize as const* usize;
    let num_app = unsafe { num_app_ptr.read_volatile(); }
    let app_start = unsafe { 
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
    }
    // clear i-cache
    unsafe { llvm_asm!("fence.i" :::: "volatile"); }
    // load apps
    for i in 0..num_app {
        let base_i = APP_BASE_ADDRESS + APP_SIZE_LIMIT * i;
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0)
        });
    }
}