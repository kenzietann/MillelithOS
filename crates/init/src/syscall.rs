// Linux x86_64 system call numbers
pub const SYS_WRITE: usize = 1;
pub const SYS_PAUSE: usize = 34;
pub const SYS_MKDIR: usize = 83;
pub const SYS_MOUNT: usize = 165;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_DUP2: usize = 33;
pub const SYS_WAIT4: usize = 61;
pub const SYS_RT_SIGACTION: usize = 13;
pub const SYS_GETPID: usize = 39;
pub const SYS_FORK: usize = 57;
pub const SYS_EXIT: usize = 60;
pub const SYS_EXECVE: usize = 59;

// Standard POSIX signal numbers
pub const SIGINT: usize = 2;
pub const SIGTERM: usize = 15;
pub const SIGCHILD: usize = 17;

// Flags for rt_sigaction
pub const SA_RESTORER: usize = 0x0400_0000;
pub const SA_RESTART: usize = 0x1000_0000;

// Flag option to return immediately if no child has exited (WHOHANG)
pub const WAIT_FLAG_NO_HANG: usize = 1;

// File status flag for read-write (0-RDWR)
pub const OPEN_FLAG_READ_WRITE: usize = 2;

// POSIX signal action configuration struct
#[repr(C)]
pub struct SigAction {
  pub handler: usize,
  pub flags: usize,
  pub restorer: usize,
  pub mask: u64
}

// Low-evel signal trampoline that calls SYS_RT_SIGRETURN (syscall 15)
#[unsafe(naked)]
pub unsafe extern "C" fn signal_restorer() {
  core::arch::naked_asm!(
    "mov rax, 15",
    "syscall"
  );
}

// Invoke 0-argument Linux syscall
#[inline(always)]
pub unsafe fn syscall0(number: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "syscall",
      in("rax") number,
      lateout("rax") ret,
      lateout("rcx") _,
      lateout("r11") _,
      options(nostack)
    );
  }
  ret
}

#[inline(always)]
pub unsafe fn syscall1(number: usize, arg1: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "syscall",
      in("rax") number,
      in("rdi") arg1,
      lateout("rax") ret,
      lateout("rcx") _,
      lateout("r11") _,
      options(nostack)
    )
  }
  ret
}

// Invoke 2-argument Linux syscall
pub unsafe fn syscall2(number: usize, arg1: usize, arg2: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "syscall",
      in("rax") number,
      in("rdi") arg1,
      in("rsi") arg2,
      lateout("rax") ret,
      lateout("rcx") _,
      lateout("r11") _,
      options(nostack)
    );
  }
  ret
}

// Invoke 3-argument Linux syscall
pub unsafe fn syscall3(number: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "syscall",
      in("rax") number,
      in("rdi") arg1,
      in("rsi") arg2,
      in("rdx") arg3,
      lateout("rax") ret,
      lateout("rcx") _,
      lateout("r11") _,
      options(nostack)
    );
  }
  ret
}

// Invoke 4-argument linux syscall
#[inline(always)]
pub unsafe fn syscall4(
  number: usize,
  arg1: usize,
  arg2: usize,
  arg3: usize,
  arg4: usize
) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "syscall",
      in("rax") number,
      in("rdi") arg1,
      in("rsi") arg2,
      in("rdx") arg3,
      in("r10") arg4,
      lateout("rax") ret,
      lateout("rcx") _,
      lateout("r11") _,
      options(nostack)
    );
  }
  ret
}

// Invoke 5-argument Linux syscall
#[inline(always)]
pub unsafe fn syscall5(
    number: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") number,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
    }
    ret
}

// Write buffer to a file descriptor (stdout = 1, stderr = 2)
pub fn write(file_descriptor: usize, buffer: &[u8]) -> isize {
  unsafe {
    syscall3(
      SYS_WRITE,
      file_descriptor,
      buffer.as_ptr() as usize,
      buffer.len(),
    )
  }
}

// Create a directory with POSIX file permissions
pub fn mkdir(path: &[u8], permission_mode: usize) -> isize {
  unsafe {
    syscall2(
      SYS_MKDIR,
      path.as_ptr() as usize,
      permission_mode,
    )
  }
}

// Mount a filesystem to a target directory
pub fn mount(source: &[u8], target: &[u8], filesystem_type: &[u8], flags: usize) -> isize {
  unsafe {
    syscall5(
      SYS_MOUNT,
      source.as_ptr() as usize,
      target.as_ptr() as usize,
      filesystem_type.as_ptr() as usize,
      flags,
      0,
    )
  }
}

// Pause process execution until a signal is received
pub fn pause() -> isize {
  unsafe { syscall0(SYS_PAUSE) }
}

// Open a file or device node with specified access flags
pub fn open(path: &[u8], flags: usize) -> isize {
  unsafe {
    syscall3(
      SYS_OPEN,
      path.as_ptr() as usize,
      flags,
      0
    )
  }
}

// Close an active file descriptor
pub fn close(file_descriptor: usize) -> isize {
  unsafe { syscall1(SYS_CLOSE, file_descriptor) }
}

// Duplicate a file descriptor onto a specific target slot
pub fn dup2(old_file_descriptor: usize, new_file_descriptor: usize) -> isize {
  unsafe {
    syscall2(
      SYS_DUP2,
      old_file_descriptor,
      new_file_descriptor
    )
  }
}

// Wait for process status change without blocking
pub fn wait_non_blocking(status_ptr: *mut u32) -> isize {
  unsafe {
    syscall4(
      SYS_WAIT4,
      !0, // -1 as usize
      status_ptr as usize,
      WAIT_FLAG_NO_HANG,
      0
    )
  }
}

// Register a POSIX signal handler with the Linux Kernel
pub fn sigaction(signal_number: usize, action: &SigAction) -> isize {
  unsafe {
    syscall4(
      SYS_RT_SIGACTION,
      signal_number,
      action as *const SigAction as usize,
      0,
      core::mem::size_of::<u64>()
    )
  }
}

// Get the current process ID
pub fn getpid() -> usize {
  unsafe { syscall0(SYS_GETPID) as usize }
}

// Clone the current process into a parent and child
pub fn fork() -> isize {
  unsafe { syscall0(SYS_FORK) }
}

// Terminate the calling process with an exit status code
pub fn exit(status: usize) -> ! {
  unsafe {
    syscall1(SYS_EXIT, status);
  }

  loop {
    pause();
  }
}

// Replace current process image with a new executable program
pub fn execve(path: &[u8], arguments: &[*const u8], environment: &[*const u8]) -> isize {
  unsafe {
    syscall3(
      SYS_EXECVE,
      path.as_ptr() as usize,
      arguments.as_ptr() as usize,
      environment.as_ptr() as usize
    )
  }
}