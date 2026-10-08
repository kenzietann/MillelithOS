// Linux x86_64 system call numbers
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_EXIT: usize = 60;
pub const OPEN_READ_ONLY: usize = 0;

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