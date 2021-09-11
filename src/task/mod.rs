mod task;
mod switch;
mod context;

use lazy_static::*;
use crate::loader::{
    get_num_app,
    init_app_cx,
};
use crate::timer::TICKS_PER_SEC;
use crate::config::{
    MAX_APP_NUM,
    BIG_STRIDE,
    MAX_RUNNING_TIME,
};
use switch::__switch;
use task::*;
use core::cell::RefCell;
pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

unsafe impl Sync for TaskManager {}
impl TaskManager {
    fn mark_current_suspended(&self) { 
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
    fn mark_current_exited(&self) { 
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
    fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.tasks[0].task_status = TaskStatus::Running;
        let next_task_cx_ptr2 = inner.tasks[0].get_task_cx_ptr2();
        let stride = BIG_STRIDE / inner.tasks[0].priority;
        inner.tasks[0].next_pass += stride as usize;
        core::mem::drop(inner);
        info!("[kernel] Switch to app {}", 0);
        unsafe {
            __switch(&0, next_task_cx_ptr2);
        }
    }
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        let mut max_pass = usize::MAX;
        let mut max_id = None;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .for_each(|id| {
                // debug!("find id = {}, pass = {}, status = {:?}", id, inner.tasks[id].pass, inner.tasks[id].task_status);
                if inner.tasks[id].pass < max_pass && inner.tasks[id].task_status == TaskStatus::Ready {
                    max_pass = inner.tasks[id].pass;
                    max_id = Some(id);
                }
            });
        // debug!("[kernel] Find next task = {}", max_id.unwrap());
        max_id
            // .find(|id| {
            //     inner.tasks[*id].task_status == TaskStatus::Ready
            // })
    }
    fn run_next_task(&self) { 
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let stride = BIG_STRIDE / inner.tasks[next].priority;
            inner.tasks[next].next_pass += stride as usize;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            info!("[kernel] Switch to app {}", next);
            unsafe {
                __switch(
                    current_task_cx_ptr2,
                    next_task_cx_ptr2
                );
            }
        } else {
            panic!("All applications completed!");
        }
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [
            TaskControlBlock { 
                task_cx_ptr: 0, 
                task_status: TaskStatus::UnInit, 
                pass: 0,
                next_pass: 1,
                priority: 16,
            };
            MAX_APP_NUM
        ];
        for i in 0..num_app {
            tasks[i].task_cx_ptr = init_app_cx(i) as *const _ as usize;
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager { 
            num_app,
            inner: RefCell::new(TaskManagerInner { 
                tasks,
                current_task: 0,
            }),
        }
    };
}

pub fn schedule_tasks() {
    let times = pass_add_one();
    // debug!("Already running {} msecs", times * 10);
    if times >= TICKS_PER_SEC * MAX_RUNNING_TIME {
        exit_current_and_run_next();
    } else {
        //schedule_tasks();
        let current_task = get_current_task();
        let inner = TASK_MANAGER.inner.borrow(); 
        let next_pass = inner.tasks[current_task].next_pass;
        core::mem::drop(inner);
        if times >= next_pass {
            suspend_current_and_run_next(); 
        }
    }
}

pub fn pass_add_one() -> usize {
    let current_task = get_current_task();
    let mut inner = TASK_MANAGER.inner.borrow_mut(); 
    inner.tasks[current_task].pass += 1;
    inner.tasks[current_task].pass
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next(){
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_task();
}

pub fn exit_current_and_run_next(){
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}

pub fn get_current_task() -> usize {
    let current_task = TASK_MANAGER.inner.borrow().current_task;
    current_task
}

pub fn set_current_priority(prio: isize) -> isize {
    match prio {
        2..=BIG_STRIDE =>{
            let current_task = get_current_task();
            let mut inner = TASK_MANAGER.inner.borrow_mut(); 
            inner.tasks[current_task].priority = prio;
            prio
        }
        _ => -1,
    }
}