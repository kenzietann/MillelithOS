#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, SYS_OPEN, close, copy_file_range, open, syscall3};
use coreutils::{arg, eprint};

const OPEN_WRITE_ONLY: usize = 1;
const OPEN_CREATE: usize = 0o100;
const OPEN_EXCLUSIVE: usize = 0o200;

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 3 {
    eprint("usage: cp SOURCE DEST\n");
    return 1;
  }

  let source_path = arg(argv, 1);
  let target_path = arg(argv, 2);

  let source_fd = open(source_path, OPEN_READ_ONLY);
  if source_fd < 0 {
    eprint("cp: cannot open source\n");
    return 1;
  }

  let target_fd = unsafe {
    syscall3(
      SYS_OPEN,
      target_path.as_ptr() as usize,
      OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_EXCLUSIVE,
      0o644
    )
  };

  if target_fd < 0 {
    close(source_fd as usize);
    eprint("cp: cannot create destination\n");
    return 1;
  }

  loop {
    let copied = copy_file_range(source_fd as usize, target_fd as usize, 64 * 1024);

    if copied < 0 {
      eprint("cp: copy failed\n");
      close(target_fd as usize);
      close(source_fd as usize);
      return 1;
    }

    if copied == 0 {
      break;
    }
  }

  let target_close = close(target_fd as usize);
  let source_close = close(source_fd as usize);

  if target_close < 0 || source_close < 0 {
    eprint("cp: close failed\n");
    return 1;
  }

  return 0;
}