#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, close, open, read, write};
use coreutils::{arg, eprint, print};

const PAGE_LINES: usize = 22;

fn wait_for_key() -> bool {
  print("--More--");

  let mut input = [0u8; 64];
  let count = read(0, &mut input);

  if count <= 0 {
    return false;
  }

  !(input[0] == b'q' || input[0] == b'Q')
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 2 {
    eprint("usage: more FILE\n");
    return 1;
  }

  let path = arg(argv, 1);
  let file_fd = open(path, OPEN_READ_ONLY);

  if file_fd < 0 {
    eprint("more: cannot open file\n");
    return 1;
  }

  let mut chunk = [0u8; 4096];
  let mut lines = 0;

  loop {
    let count = read(file_fd as usize, &mut chunk);

    if count < 0 {
      eprint("more: read failed\n");
      close(file_fd as usize);
      return 1;
    }

    if count == 0 {
      break;
    }

    let count = count as usize;
    let mut start = 0;

    for index in 0..count {
      if chunk[index] != b'\n' {
        continue;
      }

      lines += 1;

      if lines == PAGE_LINES {
        write(1, &chunk[start..=index]);
        start = index + 1;
        lines = 0;

        if !wait_for_key() {
          close(file_fd as usize);
          return 0;
        }
      }
    }

    write(1, &chunk[start..count]);
  }

  close(file_fd as usize);
  return 0;
}