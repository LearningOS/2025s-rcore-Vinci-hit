//! Process management syscalls

use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,current_user_token};
use crate::timer::get_time_ms;
use crate::mm::{translated_byte_buffer,translated_byte_buffer_with_user};
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
    let sec = (time / 1000).to_ne_bytes();
    let usec = (time % 1000 * 1000).to_ne_bytes();
    let mut sec_usec_iter = sec.iter().chain(usec.iter());
    let u8_ptr = translated_byte_buffer(current_user_token(), ts as *const u8, core::mem::size_of::<TimeVal>());
    for byte in u8_ptr{
        for b in byte{
            sec_usec_iter.next().map(|x| b.clone_from(x));
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
use crate::task::read_syscall_count;
pub fn sys_trace(trace_request: usize, id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request{
        0 => {
            let ptr = id as *const u8;
            let phy_ptr_u8 = translated_byte_buffer_with_user(current_user_token(), ptr, 1);
            if phy_ptr_u8.is_empty(){
                return -1;
            }
            unsafe{
                let byte_ptr = phy_ptr_u8[0][0] as *const u8;
                let data = *byte_ptr;
                data as isize
            }
        },
        1 => {
            // let ptr = id as *const u8;
            // let data = data as u8;
            // unsafe{
            //     let phy_ptr_u8 = translated_byte_buffer_with_user(current_user_token(), ptr, 1);
            //     if phy_ptr_u8.is_empty(){
            //         return -1;
            //     }
            //     (phy_ptr_u8[0][0] as *mut u8).as_mut().unwrap().clone_from(&data);
            // }
            -1
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
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // if start%4096 !=0 || prot & !0x7 != 0 || prot & 0x7 == 0{
    //     return -1;
    // }
    // let len = len.next_multiple_of(4096);
    // //保证不重复

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
