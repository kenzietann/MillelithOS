// Linux x86_64 system call numbers
pub const SYS_WRITE: usize = 1;
pub const SYS_PAUSE: usize = 34;
pub const SYS_MKDIR: usize = 83;
pub const SYS_MOUNT: usize = 165;

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