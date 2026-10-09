#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, close, getdents64, open};
use coreutils::eprint;

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  let path: &[u8] = if argc > 1 {
    coreutils::arg(argv, 1)
  } else {
    b".\0"
  };

  let directory = open(path, OPEN_READ_ONLY);

  if directory < 0 {
    eprint("ls: cannot open directory\n");
    return 1;
  }

  let mut buffer = [0u8; 4096];

  loop {
    let bytes_read = getdents64(directory as usize, &mut buffer);
  
    if bytes_read < 0 {
      eprint("ls: cannot read directory\n");
      return 1;
    }
  
    if bytes_read == 0 {
      break;
    }
  
    let used= bytes_read as usize;
    let mut offset = 0;
  
    while offset < used {
      if used - offset < 19 {
        eprint("ls: incomplete directory entry\n");
        return 1;
      }
  
      let record_len = u16::from_le_bytes([buffer[offset + 16], buffer[offset + 17]]) as usize;
  
      if record_len < 20 || record_len > used - offset {
        eprint("ls: invalid directory entry length\n");
        return 1;
      }
  
      let name_field = &buffer[offset + 19..offset + record_len];
      let mut name_len = 0;
  
      while name_len < name_field.len() && name_field[name_len] != 0 {
        name_len += 1;
      }
  
      if name_len == name_field.len() {
        eprint("ls: filename has no terminator\n");
        return 1;
      }
  
      coreutils::syscall::write(1, &name_field[..name_len]);
      coreutils::syscall::write(1, b"\n");
  
      offset += record_len;
    }
  }
  close(directory as usize);
  return 0;
}