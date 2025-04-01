//! Process management syscalls

use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, map_vpn_to_ppn,unmap_vpn_to_ppn, suspend_current_and_run_next, user_vpn_to_ppn};
use crate::timer::get_time_ms;
use crate::mm::{translated_byte_buffer, MapPermission, VirtAddr};
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
    for bytes in u8_ptr{
        for b in bytes{
            sec_usec_iter.next().map(|x| b.clone_from(x));
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
use crate::task::read_syscall_count;
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    
    trace!("kernel: sys_trace");
    match trace_request{
        0 => {
            let ptr = VirtAddr::from(id);
            let phy_ptr_u8 = user_vpn_to_ppn(ptr.into());
            if let Some(x) = phy_ptr_u8{
                if !x.is_valid() || !x.user() || !x.readable(){
                    return -1;
                }
                let phy_ptr = x.ppn().get_bytes_array();
                phy_ptr[ptr.page_offset()] as isize
            }else{
                return -1
            }
        },
        1 => {
            let ptr = VirtAddr::from(id);
            let phy_ptr_u8 = user_vpn_to_ppn(ptr.into());
            if let Some(x) = phy_ptr_u8{
                if !x.is_valid() || !x.user() || !x.writable(){
                    return -1;
                }
                let phy_ptr = x.ppn().get_bytes_array();
                phy_ptr[ptr.page_offset()] = data as u8;
                0
            }else{
                return -1
            }
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
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start%4096 !=0 || prot & !0x7 != 0 || prot & 0x7 == 0{
        return -1;
    }
    let len = len.next_multiple_of(4096);
    let mut permission = MapPermission::U;
    if (prot & 1) == 1 {
        permission |= MapPermission::R;
    }
    if (prot & 2) == 2 {
        permission |= MapPermission::W;
    }
    if (prot & 4) == 4 {
        permission |= MapPermission::X;
    }
    if map_vpn_to_ppn(start.into(), (start + len).into(), permission){
        0
    }else{
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start%4096 !=0 {
        return -1;
    }
    if unmap_vpn_to_ppn(VirtAddr::from(start).ceil() ,VirtAddr::from(start + len).ceil()){
        0
    }else{
        -1
    }
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
