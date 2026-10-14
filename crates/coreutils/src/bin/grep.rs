#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, close, open, read, write};
use coreutils::{arg, contains, eprint};

fn emit_if_match(line: &[u8], pattern: &[u8]) -> bool {
  if !contains(line, pattern) {
    return false;
  }

  write(1, line);
  write(1, b"\n");
  true
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 3 {
    eprint("usage: grep PATTERN FILE\n");
    return 2;
  }

  let pattern_arg = arg(argv, 1);
  let pattern = &pattern_arg[..pattern_arg.len() - 1];
  let path = arg(argv, 2);

  let file_fd = open(path, OPEN_READ_ONLY);
  if file_fd < 0 {
    eprint("grep: cannot open file\n");
    return 2;
  }

  let mut chunk = [0u8; 4096];
  let mut line = [0u8; 4096];
  let mut line_len = 0;
  let mut matched = false;

  loop {
    let count = read(file_fd as usize, &mut chunk);

    if count < 0 {
      eprint("grep: read failed\n");
      close(file_fd as usize);
      return 2;
    }

    if count == 0 {
      break;
    }

    for index in 0..count as usize {
      let byte = chunk[index];

      if byte == b'\n' {
        if emit_if_match(&line[..line_len], pattern) {
          matched = true;
        }
        line_len = 0;
      } else if line_len < line.len() {
        line[line_len] = byte;
        line_len += 1
      }
    }
  }

  if line_len > 0 && emit_if_match(&line[..line_len], pattern) {
    matched = true;
  }

  close(file_fd as usize);
  
  if matched { 0 } else { 1 }
}