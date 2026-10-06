#![no_std]
#![no_main]

mod syscall;

use core::panic::PanicInfo;
use syscall::*;

// Halt CPU in an infinite pause loop on unrecoverable userspace panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let error_message = b"[INIT PANIC] Unrecoverable error in PID 1!\n";
    write(2, error_message);

    loop {
        pause();
    }
}

// Helper to print UTF-8 strings to standard output (stdout = 1)
fn print(message: &str) {
    write(1, message.as_bytes());
}

// Userspace Ring 3 entry point called by the Linux kernel for PID 1
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print("\n=======================================================\n");
    print("  Welcome to Millelith OS (Phase 2 - Userspace Alive!) \n");
    print("  PID 1 Init successfully launched by Linux Kernel!    \n");
    print("=======================================================\n\n");

    // Create virtual filesystem mount point directories
    print("[*] Creating virtual filesystem mount points...\n");
    mkdir(b"/proc\0", 0o755);
    mkdir(b"/sys\0", 0o755);
    mkdir(b"/dev\0", 0o755);

    // Mount procfs for kernel diagnostics and process tracking
    print("[*] Mounting /proc (procfs)...\n");
    let proc_status = mount(b"proc\0", b"/proc\0", b"proc\0", 0);
    if proc_status == 0 {
        print("[OK] /proc mounted successfully!\n");
    } else {
        print("[WARN] Failed to mount /proc\n");
    }

    // Mount sysfs for device driver and hardware bus hierarchy
    print("[*] Mounting /sys (sysfs)...\n");
    let sys_status = mount(b"sysfs\0", b"/sys\0", b"sysfs\0", 0);
    if sys_status == 0 {
        print("[OK] /sys mounted successfully!\n");
    } else {
        print("[WARN] Failed to mount /sys\n");
    }

    // Mount devtmpfs for automatic device node population
    print("[*] Mounting /dev (devtmpfs)...\n");
    let dev_status = mount(b"devtmpfs\0", b"/dev\0", b"devtmpfs\0", 0);
    if dev_status == 0 {
        print("[OK] /dev mounted successfully!\n");
    } else {
        print("[WARN] Failed to mount /dev\n");
    }

    print("\n[OK] Core virtual filesystems initialized.\n");
    print("[*] PID 1 entering supervisory loop...\n");

    // Infinite supervisory loop to prevent PID 1 from exiting
    loop {
        pause();
    }
}