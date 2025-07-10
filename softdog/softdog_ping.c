#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// --- Macros ---
#define panic(msg) do { printf("PANIC: %s\n", msg); exit(1); } while (0)
#define pr_crit(msg, ...) printf("CRIT: " msg "\n", ##__VA_ARGS__)
#define THIS_MODULE NULL
#define WDIOF_SETTIMEOUT      0x0001
#define WDIOF_KEEPALIVEPING   0x0002
#define WDIOF_MAGICCLOSE      0x0004
#define HRTIMER_MODE_REL      0
#define HRTIMER_NORESTART     0

// --- Structs ---
struct watchdog_info {
    int options;
    int firmware_version;
    char identity[32];
};

struct watchdog_device;
struct watchdog_ops {
    int (*start)(struct watchdog_device *);
    int (*stop)(struct watchdog_device *);
    int (*ping)(struct watchdog_device *);
};

struct watchdog_device {
    const struct watchdog_info *info;
    const struct watchdog_ops *ops;
    unsigned int timeout;
    unsigned int min_timeout;
    unsigned int max_timeout;
    void *parent;
    int (*pretimeout)(struct watchdog_device *);
};

struct hrtimer {
    int dummy;
    int (*function)(struct hrtimer *);
};

// --- Globals ---
struct hrtimer timer;
int soft_margin = 10;

// --- Function declarations ---
int softdog_start(struct watchdog_device *);
int softdog_stop(struct watchdog_device *);
int softdog_ping(struct watchdog_device *);
int softdog_fire(struct hrtimer *);
int softdog_pretimeout(struct watchdog_device *wdev);
int watchdog_register_device(struct watchdog_device *wdev);
void watchdog_init_timeout(struct watchdog_device *wdev, int timeout, void *dev);
void watchdog_set_nowayout(struct watchdog_device *wdev, int nowayout);
void watchdog_stop_on_reboot(struct watchdog_device *wdev);
void hrtimer_init(struct hrtimer *t, int clockid, int mode);
void softdog_exit(void);

// --- Static structs ---
struct watchdog_info softdog_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .firmware_version = 0,
    .identity = "Software Watchdog"
};

struct watchdog_ops softdog_ops = {
    .start = softdog_start,
    .stop = softdog_stop,
    .ping = softdog_ping,
};

struct watchdog_device softdog_dev = {
    .info = &softdog_info,
    .ops = &softdog_ops,
    .timeout = 60,
    .min_timeout = 1,
    .max_timeout = 65535,
    .parent = NULL
};

// --- Mocked kernel functions ---
bool hrtimer_active(struct hrtimer *t) { return true; }
void hrtimer_cancel(struct hrtimer *t) { printf("Called hrtimer_cancel()\n"); }
void hrtimer_start(struct hrtimer *t, long long ns, int mode) {
    printf("Called hrtimer_start(): time = %lld ns, mode = %d\n", ns, mode);
}
long long ktime_set(int sec, int nsec) {
    return ((long long)sec) * 1000000000LL + nsec;
}
void module_put(void *m) { printf("Called module_put()\n"); }

// --- Logic functions ---
int watchdog_register_device(struct watchdog_device *wdev) {
    printf("Registered watchdog device\n");
    return 0;
}

void watchdog_init_timeout(struct watchdog_device *wdev, int timeout, void *dev) {
    printf("watchdog_init_timeout: timeout = %d\n", timeout);
    wdev->timeout = timeout;
}

void watchdog_set_nowayout(struct watchdog_device *wdev, int nowayout) {
    printf("watchdog_set_nowayout: nowayout = %d\n", nowayout);
}

void watchdog_stop_on_reboot(struct watchdog_device *wdev) {
    printf("watchdog_stop_on_reboot\n");
}

void watchdog_unregister_device(struct watchdog_device *wdev) {
    printf("Unregistered watchdog device\n");
}


void hrtimer_init(struct hrtimer *t, int clockid, int mode) {
    printf("hrtimer_init: clock = %d, mode = %d\n", clockid, mode);
    // In real kernel: set up hrtimer base
}

int softdog_pretimeout(struct watchdog_device *wdev) {
    printf("softdog_pretimeout called\n");
    return 0;
}


int softdog_init(void) {
    watchdog_init_timeout(&softdog_dev, soft_margin, NULL);
    watchdog_set_nowayout(&softdog_dev, 1);  // hardcoded nowayout
    watchdog_stop_on_reboot(&softdog_dev);

    hrtimer_init(&timer, 1 /* CLOCK_MONOTONIC */, HRTIMER_MODE_REL);
    timer.function = softdog_fire;

    softdog_dev.pretimeout = softdog_pretimeout;
    softdog_dev.timeout = soft_margin;

    return watchdog_register_device(&softdog_dev);
}



int softdog_start(struct watchdog_device *wdev) {
    printf("Called softdog_start()\n");
    return 0;
}

int softdog_stop(struct watchdog_device *wdev) {
    hrtimer_cancel(&timer);
    module_put(THIS_MODULE);
    return 0;
}

int softdog_ping(struct watchdog_device *wdev) {
    if (hrtimer_active(&timer)) {
        hrtimer_cancel(&timer);
        hrtimer_start(&timer, ktime_set(soft_margin, 0), HRTIMER_MODE_REL);
    }
    return 0;
}

int softdog_fire(struct hrtimer *t) {
    pr_crit("Watchdog will fire");
    module_put(THIS_MODULE);
    panic("Watchdog timer expired");
    return HRTIMER_NORESTART;
}

void softdog_exit(void) {
    watchdog_unregister_device(&softdog_dev);
}


// --- Main ---
int main() {
    softdog_init();
    softdog_dev.ops->start(&softdog_dev);
    softdog_dev.ops->ping(&softdog_dev);
    softdog_dev.ops->stop(&softdog_dev);
    softdog_fire(&timer);
    softdog_exit();
    return 0;
}
