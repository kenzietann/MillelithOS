#![no_std]

pub mod syscall;

use core::panic::PanicInfo;

unsafe extern "C" {
  fn main_entry(argc: usize, argv: *const *const u8) -> usize;
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
  core::arch::naked_asm!(
    "mov rdi, rsp",
    "and rsp, -16",
    "call {entry}",
    entry = sym start_rust,
  )
}

extern "C" fn start_rust(stack_pointer: *const usize) -> ! {
  let argc = unsafe { *stack_pointer };
  let argv = unsafe {stack_pointer.add(1) as *const *const u8 };
  let status = unsafe { main_entry(argc, argv) };

  syscall::exit(status)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  syscall::exit(101)
}

pub fn c_str_len(string: *const u8) -> usize {
  let mut length = 0;
  while unsafe { *string.add(length) } != 0 {
    length += 1;
  }
  length
}

pub fn arg(argv: *const *const u8, index: usize) -> &'static [u8] {
  unsafe {
    let string = *argv.add(index);
    core::slice::from_raw_parts(string, c_str_len(string) + 1)
  }
}

pub fn print(text: &str) {
  syscall::write(1, text.as_bytes());
}

pub fn eprint(text: &str) {
  syscall::write(2, text.as_bytes());
}