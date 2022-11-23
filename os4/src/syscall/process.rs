use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, get_current_task_status, get_syscall_times, get_current_run_time, task_mmap, task_munmap};
use crate::timer::get_time_us;
use crate::mm::{translate_by_token, VirtAddr};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// Lab2
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let ts_pa = translate_by_token(current_user_token(), VirtAddr::from(ts as usize)).unwrap().0 as *mut TimeVal;
    unsafe {
        *ts_pa = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// Lab2
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    task_mmap(start, len, port)
}

// Lab2
pub fn sys_munmap(start: usize, len: usize) -> isize {
    task_munmap(start, len)
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let ti_pa = translate_by_token(current_user_token(), VirtAddr::from(ti as usize)).unwrap().0 as *mut TaskInfo;
    unsafe {
        *ti_pa = TaskInfo {
            status: get_current_task_status(),
            syscall_times: get_syscall_times(),
            time: get_current_run_time(),
        }
    }
    0
}