
extern "C" {
    fn printf(msg: *const i8, ...) -> i32;
    fn exit(code: i32) -> !;
}

#[repr(C)]
pub struct watchdog_info {
    pub options: i32,
    pub firmware_version: i32,
    pub identity: [i8; 32],
}

#[repr(C)]
pub struct watchdog_device {
    pub info: *const watchdog_info,
    pub ops: *const watchdog_ops,
    pub timeout: u32,
    pub min_timeout: u32,
    pub max_timeout: u32,
    pub parent: *mut core::ffi::c_void,
    pub pretimeout: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
}

#[repr(C)]
pub struct watchdog_ops {
    pub start: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
    pub ping: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
}

// Global variables
static mut SOFT_MARGIN: i32 = 10;

#[repr(C)]
pub struct hrtimer {
    pub dummy: i32,
    pub function: Option<unsafe extern "C" fn(*mut hrtimer) -> i32>,
}

static mut TIMER: hrtimer = hrtimer {
    dummy: 0,
    function: None,
};

// Timer helper functions
#[no_mangle]
pub unsafe extern "C" fn ktime_set(sec: i32, nsec: i32) -> i64 {
    (sec as i64) * 1_000_000_000 + (nsec as i64)
}

#[no_mangle]
pub unsafe extern "C" fn hrtimer_active(_t: *mut hrtimer) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn hrtimer_cancel(_t: *mut hrtimer) {
    let msg = b"Called hrtimer_cancel()\n\0";
    printf(msg.as_ptr() as *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn hrtimer_start(_t: *mut hrtimer, time: i64, mode: i32) {
    let msg = b"Called hrtimer_start(): time = %lld, mode = %d\n\0";
    printf(msg.as_ptr() as *const i8, time, mode);
}

// Logic functions
#[no_mangle]
pub unsafe extern "C" fn softdog_ping(wdev: *mut watchdog_device) -> i32 {
    if hrtimer_active(&mut TIMER) {
        hrtimer_cancel(&mut TIMER);
        hrtimer_start(&mut TIMER, ktime_set(SOFT_MARGIN, 0), 0);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn softdog_stop(_wdev: *mut watchdog_device) -> i32 {
    hrtimer_cancel(&mut TIMER);
    let msg = b"Called module_put()\n\0";
    printf(msg.as_ptr() as *const i8);
    0
}

#[no_mangle]
pub unsafe extern "C" fn softdog_start(_wdev: *mut watchdog_device) -> i32 {
    let msg = b"Called softdog_start()\n\0";
    printf(msg.as_ptr() as *const i8);
    0
}

#[no_mangle]
pub unsafe extern "C" fn softdog_fire(_t: *mut hrtimer) -> i32 {
    let msg = b"CRIT: Watchdog will fire\n\0";
    printf(msg.as_ptr() as *const i8);
    let msg2 = b"Called module_put()\n\0";
    printf(msg2.as_ptr() as *const i8);
    let panic_msg = b"PANIC: Watchdog timer expired\n\0";
    printf(panic_msg.as_ptr() as *const i8);
    exit(1);
}
