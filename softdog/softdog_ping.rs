#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct watchdog_info {
    pub options: libc::c_int,
    pub firmware_version: libc::c_int,
    pub identity: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct watchdog_device {
    pub info: *const watchdog_info,
    pub ops: *const watchdog_ops,
    pub timeout: libc::c_uint,
    pub min_timeout: libc::c_uint,
    pub max_timeout: libc::c_uint,
    pub parent: *mut libc::c_void,
    pub pretimeout: Option::<unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct watchdog_ops {
    pub start: Option::<unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int>,
    pub stop: Option::<unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int>,
    pub ping: Option::<unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hrtimer {
    pub dummy: libc::c_int,
    pub function: Option::<unsafe extern "C" fn(*mut hrtimer) -> libc::c_int>,
}
#[no_mangle]
pub static mut timer: hrtimer = hrtimer {
    dummy: 0,
    function: None,
};
#[no_mangle]
pub static mut soft_margin: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub static mut softdog_info: watchdog_info = unsafe {
    {
        let mut init = watchdog_info {
            options: 0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int,
            firmware_version: 0 as libc::c_int,
            identity: *::core::mem::transmute::<
                &[u8; 32],
                &mut [libc::c_char; 32],
            >(b"Software Watchdog\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        };
        init
    }
};
#[no_mangle]
pub static mut softdog_ops: watchdog_ops = unsafe {
    {
        let mut init = watchdog_ops {
            start: Some(
                softdog_start
                    as unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int,
            ),
            stop: Some(
                softdog_stop as unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int,
            ),
            ping: Some(
                softdog_ping as unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut softdog_dev: watchdog_device = unsafe {
    {
        let mut init = watchdog_device {
            info: &softdog_info as *const watchdog_info as *mut watchdog_info,
            ops: &softdog_ops as *const watchdog_ops as *mut watchdog_ops,
            timeout: 60 as libc::c_int as libc::c_uint,
            min_timeout: 1 as libc::c_int as libc::c_uint,
            max_timeout: 65535 as libc::c_int as libc::c_uint,
            parent: 0 as *const libc::c_void as *mut libc::c_void,
            pretimeout: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn hrtimer_active(mut t: *mut hrtimer) -> bool {
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cancel(mut t: *mut hrtimer) {
    printf(b"Called hrtimer_cancel()\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn hrtimer_start(
    mut t: *mut hrtimer,
    mut ns: libc::c_longlong,
    mut mode: libc::c_int,
) {
    printf(
        b"Called hrtimer_start(): time = %lld ns, mode = %d\n\0" as *const u8
            as *const libc::c_char,
        ns,
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ktime_set(
    mut sec: libc::c_int,
    mut nsec: libc::c_int,
) -> libc::c_longlong {
    return sec as libc::c_longlong * 1000000000 as libc::c_longlong
        + nsec as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn module_put(mut m: *mut libc::c_void) {
    printf(b"Called module_put()\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn watchdog_register_device(
    mut wdev: *mut watchdog_device,
) -> libc::c_int {
    printf(b"Registered watchdog device\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn watchdog_init_timeout(
    mut wdev: *mut watchdog_device,
    mut timeout: libc::c_int,
    mut dev: *mut libc::c_void,
) {
    printf(
        b"watchdog_init_timeout: timeout = %d\n\0" as *const u8 as *const libc::c_char,
        timeout,
    );
    (*wdev).timeout = timeout as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn watchdog_set_nowayout(
    mut wdev: *mut watchdog_device,
    mut nowayout: libc::c_int,
) {
    printf(
        b"watchdog_set_nowayout: nowayout = %d\n\0" as *const u8 as *const libc::c_char,
        nowayout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn watchdog_stop_on_reboot(mut wdev: *mut watchdog_device) {
    printf(b"watchdog_stop_on_reboot\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn watchdog_unregister_device(mut wdev: *mut watchdog_device) {
    printf(b"Unregistered watchdog device\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn hrtimer_init(
    mut t: *mut hrtimer,
    mut clockid: libc::c_int,
    mut mode: libc::c_int,
) {
    printf(
        b"hrtimer_init: clock = %d, mode = %d\n\0" as *const u8 as *const libc::c_char,
        clockid,
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn softdog_pretimeout(
    mut wdev: *mut watchdog_device,
) -> libc::c_int {
    printf(b"softdog_pretimeout called\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn softdog_init() -> libc::c_int {
    watchdog_init_timeout(&mut softdog_dev, soft_margin, 0 as *mut libc::c_void);
    watchdog_set_nowayout(&mut softdog_dev, 1 as libc::c_int);
    watchdog_stop_on_reboot(&mut softdog_dev);
    hrtimer_init(&mut timer, 1 as libc::c_int, 0 as libc::c_int);
    timer
        .function = Some(
        softdog_fire as unsafe extern "C" fn(*mut hrtimer) -> libc::c_int,
    );
    softdog_dev
        .pretimeout = Some(
        softdog_pretimeout as unsafe extern "C" fn(*mut watchdog_device) -> libc::c_int,
    );
    softdog_dev.timeout = soft_margin as libc::c_uint;
    return watchdog_register_device(&mut softdog_dev);
}
#[no_mangle]
pub unsafe extern "C" fn softdog_start(mut wdev: *mut watchdog_device) -> libc::c_int {
    printf(b"Called softdog_start()\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn softdog_stop(mut wdev: *mut watchdog_device) -> libc::c_int {
    hrtimer_cancel(&mut timer);
    module_put(0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn softdog_ping(mut wdev: *mut watchdog_device) -> libc::c_int {
    if hrtimer_active(&mut timer) {
        hrtimer_cancel(&mut timer);
        hrtimer_start(
            &mut timer,
            ktime_set(soft_margin, 0 as libc::c_int),
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn softdog_fire(mut t: *mut hrtimer) -> libc::c_int {
    printf(b"CRIT: Watchdog will fire\n\0" as *const u8 as *const libc::c_char);
    module_put(0 as *mut libc::c_void);
    printf(
        b"PANIC: %s\n\0" as *const u8 as *const libc::c_char,
        b"Watchdog timer expired\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn softdog_exit() {
    watchdog_unregister_device(&mut softdog_dev);
}
unsafe fn main_0() -> libc::c_int {
    softdog_init();
    ((*softdog_dev.ops).start).expect("non-null function pointer")(&mut softdog_dev);
    ((*softdog_dev.ops).ping).expect("non-null function pointer")(&mut softdog_dev);
    ((*softdog_dev.ops).stop).expect("non-null function pointer")(&mut softdog_dev);
    softdog_fire(&mut timer);
    softdog_exit();
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
