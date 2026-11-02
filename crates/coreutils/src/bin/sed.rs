#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, close, open, read, write};
use coreutils::{arg, eprint, find};

fn emit_substituted(line: &[u8], old: &[u8], new: &[u8], global: bool) {
  let mut rest = line;

  while let Some(at) = find(rest, old) {
    write(1, &rest[..at]);
    write(1, new);

    rest = &rest[at + old.len()..];

    if !global {
      break;
    }
    
  }
  write(1, rest);
  write(1, b"\n");
}

fn parse_script(script: &[u8]) -> Option<(&[u8], &[u8], bool)> {
  if script.len() < 2 || script[0] != b's' {
    return None;
  }

  let delimiter = script[1];
  let rest = &script[2..];

  let old_end = rest.iter().position(|&byte| byte == delimiter)?;
  let old = &rest[..old_end];
  let rest = &rest[old_end + 1..];

  let new_end = rest.iter().position(|&byte| byte == delimiter)?;
  let new = &rest[..new_end];
  let flags = &rest[new_end + 1..];

  if old.is_empty() {
    return None;
  }

  let global = if flags.is_empty() {
    false
  } else if flags == &b"g"[..] {
    true
  } else {
    return None;
  };

  Some((old, new, global))
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 3 {
    eprint("usage: sed s/OLD/NEW/[g] FILE \n");
    return 1;
  }

  let script_arg = arg(argv, 1);
  let script = &script_arg[..script_arg.len() - 1];

  let Some((old, new, global)) = parse_script(script) else {
    eprint("sed: invalid script\n");
    return 1;
  };

  let path = arg(argv, 2);
  let file_fd= open(path, OPEN_READ_ONLY);
  if file_fd < 0 {
    eprint("sed: cannot open file\n");
    return 1;
  }

  let mut chunk = [0u8; 4096];
  let mut line = [0u8; 4096];
  let mut line_len = 0;

  loop {
    let count = read(file_fd as usize, &mut chunk);

    if count < 0 {
      eprint("sed: read failed\n");
      close(file_fd as usize);
      return 1;
    }

    if count == 0 {
      break;
    }

    for index in 0..count as usize {
      let byte = chunk[index];

      if byte == b'\n' {
        emit_substituted(&line[..line_len], old, new, global);
        line_len = 0;
      } else if line_len < line.len() {
        line[line_len] = byte;
        line_len += 1;
      }
    }
  }

  if line_len > 0 {
    emit_substituted(&line[..line_len], old, new, global);
  }

  close(file_fd as usize);
  return 0;
}
