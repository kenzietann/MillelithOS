// Linux x86_64 system call numbers
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_EXIT: usize = 60;
pub const OPEN_READ_ONLY: usize = 0;
pub const SYS_GETDENTS64: usize = 217;
pub const SYS_MKDIR: usize = 83;
pub const SYS_COPY_FILE_RANGE: usize = 326;
pub const SYS_UNLINK: usize = 87;
pub const SYS_UTIMENSAT: usize = 280;
pub const AT_FDCWD: usize = (-100isize) as usize;
pub const SYS_RENAME: usize = 82;

// Linux syscall invocation
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
    )
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

#[inline(always)]
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
    )
  }
  ret
}

#[inline(always)]
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
    )
  }
  ret
}

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
    )
  }
  ret
}

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
        )
    }
    ret
}

#[inline(always)]
pub unsafe fn syscall6(
    number: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize
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
            in("r9") arg6,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        )
    }
    ret
}

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

pub fn read(file_descriptor: usize, buffer: &mut [u8]) -> isize {
  unsafe {
    syscall3(
      SYS_READ,
      file_descriptor,
      buffer.as_mut_ptr() as usize,
      buffer.len()
    )
  }
}

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

pub fn close(file_descriptor: usize) -> isize {
  unsafe { syscall1(SYS_CLOSE, file_descriptor) }
}

pub fn exit(status: usize) -> ! {
  unsafe { syscall1(SYS_EXIT, status); }

  loop {}
}

pub fn getdents64(file_descriptor: usize, buffer: &mut[u8]) -> isize {
  unsafe {
    syscall3(
      SYS_GETDENTS64,
      file_descriptor,
      buffer.as_mut_ptr() as usize,
      buffer.len()
    )
  }
}

pub fn mkdir(path: &[u8], mode: usize) -> isize {
  unsafe {
    syscall2(
      SYS_MKDIR,
      path.as_ptr() as usize,
      mode
    )
  }
}

pub fn copy_file_range(fd_in: usize, fd_out: usize, count: usize) -> isize {
  unsafe {
    syscall6(
      SYS_COPY_FILE_RANGE,
      fd_in,
      0,
      fd_out,
      0,
      count,
      0
    )
  }
}

pub fn unlink(path: &[u8]) -> isize {
  unsafe {
    syscall1(SYS_UNLINK, path.as_ptr() as usize)
  }
}

pub fn update_times_now(path: &[u8]) -> isize {
  unsafe {
    syscall4(
      SYS_UTIMENSAT,
      AT_FDCWD,
      path.as_ptr() as usize,
      0,
      0
    )
  }
}

pub fn rename(old_path: &[u8], new_path: &[u8]) -> isize {
  unsafe {
    syscall2(
      SYS_RENAME, 
      old_path.as_ptr() as usize,
      new_path.as_ptr() as usize
    )
  }
}