#![feature(llvm_asm)]

use core::cell::RefCell;
use lazy_static::*;
use crate::trap::TrapContext;

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNAL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernalStack {
    data: [u8; KERNAL_STACK_SIZE]
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE]
}

static KERNAL_STACK: KernalStack = KernalStack { data:[0; KERNAL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data:[0; USER_STACK_SIZE]};

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
impl KernalStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNAL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe { 
            *cx_ptr = cx; 
            cx_ptr.as_mut().unwrap()
        }
    }
}

struct AppManager {
    inner: RefCell<AppManagerInner>,
}

struct AppManagerInner {
    num_app: usize, 
    current_app: usize, 
    app_start: [usize; MAX_APP_NUM + 1],
}
unsafe impl Sync for AppManager {}

impl AppManagerInner {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!("[kernel] app_{} [{:#x}, {:#x}]", i, self.app_start[i], self.app_start[i+1]);
        }
    }

    pub fn get_current_app(&self) -> usize { self.current_app }

    pub fn move_to_next_app(&mut self) { self.current_app += 1;}

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        //clear icache
        llvm_asm!("fence.i" :::: "volatile");
        //clear app data
        (APP_BASE_ADDRESS..APP_BASE_ADDRESS+APP_SIZE_LIMIT).for_each(|addr|{
            (addr as *mut u8).write_volatile(0);
        });
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id]
        );
        let app_dst = core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            app_src.len()
        );
        // debug!("app_dst_1: {:?}", app_dst);
        app_dst.copy_from_slice(app_src);
        // debug!("app_dst_2: {:?}", app_dst);
    }
}

lazy_static! {
    static ref APP_MANAGER: AppManager = AppManager {
        inner: RefCell::new({
            extern "C" { fn _num_app(); }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = unsafe { num_app_ptr.read_volatile() };
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] = unsafe {
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
            };
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManagerInner {
                num_app,
                current_app: 0,
                app_start,
            }
        }),
    };
}

fn print_app_info() {
    APP_MANAGER.inner.borrow().print_app_info();
}

pub fn init() {
    print_app_info();
}

pub fn run_next_app() -> ! {
    let current_app = APP_MANAGER.inner.borrow().current_app;
    unsafe {
        APP_MANAGER.inner.borrow().load_app(current_app);
    }
    APP_MANAGER.inner.borrow_mut().move_to_next_app();
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNAL_STACK.push_context(
            TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp())
        )as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}