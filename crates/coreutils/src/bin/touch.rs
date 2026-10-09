#![no_std]
#![no_main]

use coreutils::syscall::{SYS_OPEN, close, syscall3, update_times_now};
use coreutils::{arg, eprint};

const OPEN_WRITE_ONLY: usize = 1;
const OPEN_CREATE: usize = 0o100;

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc < 2 {
    eprint("usage: touch FILE...\n");
    return 1;
  }

  let mut status = 0;

  for index in 1..argc {
    let path = arg(argv, index);

    const ERR_NO_ENTRY: isize = -2;

    let update_result = update_times_now(path);

    if update_result == 0{
      continue;
    }

    if update_result != ERR_NO_ENTRY {
      eprint("touch: cannot update file time\n");
      status = 1;
      continue;
    }

    let file_fd = unsafe {
      syscall3(
        SYS_OPEN,
        path.as_ptr() as usize,
        OPEN_WRITE_ONLY | OPEN_CREATE,
        0o644
      )
    };

    if file_fd < 0 {
      eprint("touch: cannot open file\n");
      status = 1;
      continue;
    }

    if close(file_fd as usize) < 0 {
      eprint("touch: cannot close file\n");
      status = 1;
    }
  };

  return status;
}