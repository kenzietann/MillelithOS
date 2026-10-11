#![no_std]
#![no_main]

use coreutils::syscall::{AT_FDCWD, close, open_dir_at, unlink_at, getdents64};
use coreutils::{arg, eprint};

const MAX_DEPTH: usize = 32;

fn remove_entry_at(parent_fd: usize, path: &[u8], recursive: bool, depth: usize) -> isize {
  let result = unlink_at(parent_fd, path, false);

  if result == 0 || !recursive {
    return result;
  }

  if depth >= MAX_DEPTH {
    eprint("rm: directory tree too deep\n");
    return -1;
  }

  let directory_fd = open_dir_at(parent_fd, path);
  if directory_fd < 0 {
    return directory_fd;
  }

  let fd = directory_fd as usize;
  let children_result = remove_children(fd, depth + 1);
  let close_result = close(fd);

  if children_result < 0 {
    return children_result;
  }

  if close_result < 0 {
    return close_result;
  }

  return unlink_at(parent_fd, path, true);
}

fn remove_children(directory_fd: usize, depth: usize) -> isize {
  let mut buffer = [0u8; 4096];

  loop {
    let bytes_read = getdents64(directory_fd, &mut buffer);

    if bytes_read <= 0 {
      return bytes_read;
    }

    let used = bytes_read as usize;
    if used > buffer.len() {
      return -1;
    }

    let mut offset = 0;

    while offset < used {
      if used - offset < 19 {
        return -1;
      }

      let record_len = u16::from_le_bytes([buffer[offset + 16], buffer[offset + 17]]) as usize;

      if record_len < 20 || record_len > used - offset {
        return -1;
      }

      let name_field = &buffer[offset + 19..offset + record_len];
      let mut name_len = 0;

      while name_len < name_field.len() && name_field[name_len] != 0 {
        name_len += 1;
      }

      if name_len == 0 || name_len == name_field.len() {
        return -1;
      }

      let is_dot = name_len == 1 && name_field[0] == b'.';
      let is_dot_dot = name_len == 2 && name_field[0] == b'.' && name_field[1] == b'.';

      if !is_dot && !is_dot_dot {
        let child_name = &name_field[..name_len + 1];
        let child_result = remove_entry_at(directory_fd, child_name, true, depth);

        if child_result < 0 {
          return child_result;
        }
      }

      offset += record_len;
    }
  }
}

fn protected_target(path: &[u8]) -> bool {
  let mut end = path.len() - 1;

  while end > 0 && path[end - 1] == b'/' {
    end -= 1;
  }

  if end == 0 {
    return true;
  }

  let mut start = end;
  while start > 0 && path[start - 1] != b'/' {
    start -= 1;
  }

  let last = &path[start..end];
  return last == &b"."[..] || last == &b".."[..];
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  let mut recursive = false;
  let mut force = false;
  let mut first_path = 1;

  while first_path < argc {
    let option = arg(argv, first_path);

    if option == &b"-r\0"[..] {
      recursive = true;
    } else if option == &b"-f\0"[..] {
      force = true;
    } else if option == &b"-rf\0"[..] || option == &b"-fr\0"[..] {
      recursive = true;
      force = true;
    } else if option == &b"--\0"[..] {
      first_path += 1;
      break;
    } else if option.first() == Some(&b'-') {
      eprint("rm: unknown option\n");
      return 1;
    } else {
      break;
    }

    first_path += 1;
  }

  if first_path == argc {
    if force {
      return 0;
    }
    eprint("usage: rm [-r] [-f] FILE...\n");
    return 1;
  }

  let mut status = 0;

  for index in first_path..argc {
    let path = arg(argv, index);

    if recursive && protected_target(path) {
      eprint("rm: refusing protected path\n");
      status = 1;
      continue;
    }

    let result = remove_entry_at(AT_FDCWD, path, recursive, 0);

    if result < 0 && !(force && result == -2){
      eprint("rm: cannot remove file\n");
      status = 1;
    }
  }

  return status;
}