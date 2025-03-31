//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,current_user_token};
use crate::timer::get_time_ms;
use crate::mm::translated_type;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_ms();
    println!("time: {}", time);
    let ts = translated_type(current_user_token(), ts);
    if ts.is_none() {
        return -1;
    }
    let ts = ts.unwrap();
    (*ts).sec = time / 1000;
    (*ts).usec = time % 1000 * 1000;
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
use crate::task::read_syscall_count;
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request{
        0 => {
            let ptr = id as *const u8;
            unsafe{
                return ptr.as_ref().unwrap().clone() as u8 as isize;
            }
        },
        1 => {
            let ptr = id as *mut u8;
            let data = data as u8;
            unsafe{
                ptr.as_mut().unwrap().clone_from(&data);
            }
            0
        }
        2 => {
            let r = read_syscall_count(id);
            r as isize
        },
        _ => {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
