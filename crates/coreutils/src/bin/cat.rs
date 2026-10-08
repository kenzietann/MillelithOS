#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, close, open, read, write};
use coreutils::{arg, eprint};

fn copy_to_stdout(file_descriptor: usize) -> bool {
  let mut buffer = [0u8; 4096];

  loop {
    let bytes_read = read(file_descriptor, &mut buffer);

    if bytes_read < 0 {
      return false;
    }

    if bytes_read == 0 {
      return true;
    }

    write(1, &buffer[..bytes_read as usize]);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc < 2 {
    return if copy_to_stdout(0) { 0 } else { 1 };
  }

  let mut status = 0;

  for index in 1..argc {
    let file_descriptor = open(arg(argv, index), OPEN_READ_ONLY);

    if file_descriptor < 0 {
      eprint("cat: cannot open file\n");
      status = 1;
      continue;
    }

    if !copy_to_stdout(file_descriptor as usize) {
      eprint("cat: read error\n");
      status = 1;
    }

    close(file_descriptor as usize);
  }

  status
}