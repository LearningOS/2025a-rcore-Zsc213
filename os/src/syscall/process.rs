//! Process management syscalls
use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, get_trap_times, suspend_current_and_run_next,
    add_map};
use crate::mm::{accessible, translated_byte_buffer, MapPermission, VirtAddr};
use crate::timer::{get_time_us};
use crate::config::{PAGE_SIZE};

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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let timeval_buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, 2);
    unsafe {
        *(timeval_buffers[0][0] as *mut usize) = us / 1_000_000;
        if timeval_buffers.len() == 1 {
            *(timeval_buffers[0][0] as *mut TimeVal) = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        } else {
            *(timeval_buffers[0][0] as *mut usize) = us / 1_000_000;
            *(timeval_buffers[1][0] as *mut usize) = us % 1_000_000;
        }
        // *ts = TimeVal {
        //     sec: us / 1_000_000,
        //     usec: us % 1_000_000,
        // };
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            if accessible(current_user_token(), _id as *const u8, 0) {
                let id_buffer = translated_byte_buffer(current_user_token(), _id as *const u8, 1);
                unsafe { *(id_buffer[0][0] as *const u8) as isize }
            } else {
                -1
            }

        },
        1 => {
            if accessible(current_user_token(), _id as *const u8, 1) {
                let id_buffer = translated_byte_buffer(current_user_token(), _id as *const u8, 1);
                unsafe {
                    let p: *mut u8 = id_buffer[0][0] as *mut u8;
                    *p = _data as u8;
                }
                0
            } else {
                -1
            }
        }
        2 => (get_trap_times(_id)) as isize,
        _ => -1,
    }

}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 || _port >> 3 != 0 || _port == 0{
        // parameters non-standrad
        -1
    } else {
        let mut mp: MapPermission = MapPermission::U;
        if _port & 0 != 0 {mp |= MapPermission::R;}
        if _port & 1 != 0 {mp |= MapPermission::W;}
        if _port & 2 != 0 {mp |= MapPermission::X;}
        add_map(VirtAddr::from(_start), VirtAddr::from(_start + _len), mp);
        0
    }

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
