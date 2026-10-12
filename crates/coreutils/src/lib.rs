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

pub fn print_u64(mut value: u64) {
  let mut digits = [0u8; 20];
  let mut position = digits.len();

  if value == 0 {
    position -= 1;
    digits[position] = b'0';
  }

  while value > 0 {
    position -= 1;
    digits[position] = b'0' + (value % 10) as u8;
    value /= 10;
  }

  syscall::write(1, &digits[position..]);
}

pub fn is_dir(path: &[u8]) -> bool {
  let mut buf = [0u8; 144];
  if syscall::stat(path, &mut buf) < 0 {
    return false;
  }

  let mode = u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]);
  mode & syscall::S_IFMT == syscall::S_IFDIR
}

pub fn resolve_target<'a>(
  source: &[u8],
  destination: &'a [u8],
  buffer: &'a mut [u8],
) -> Option<&'a [u8]> {
  let has_slash = destination.len() >= 2 && destination[destination.len() - 2] == b'/';

  if !has_slash && !is_dir(destination) {
    return Some(destination);
  }

  let source_name = &source[..source.len() - 1];
  let mut name_start = 0;

  for index in 0..source_name.len() {
    if source_name[index] == b'/' {
      name_start = index + 1;
    }
  }

  let basename = &source_name[name_start..];
  let prefix = &destination[..destination.len() - 1];
  let slash_len = (!has_slash) as usize;
  let total = prefix.len() + slash_len + basename.len() + 1;

  if basename.is_empty() || total > buffer.len() {
    return None;
  }

  buffer[..prefix.len()].copy_from_slice(prefix);
  if slash_len == 1 {
    buffer[prefix.len()] = b'/';
  }

  let name_offset = prefix.len() + slash_len;
  buffer[name_offset..name_offset + basename.len()].copy_from_slice(basename);
  buffer[total - 1] = 0;

  Some(&buffer[..total])
}