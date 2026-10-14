#![no_std]
#![no_main]

use coreutils::{arg, eprint, print, print_u64};
use coreutils::syscall::{get_termios, read, set_termios, OPEN_READ_ONLY, close, open, write};

const LFLAG_OFFSET: usize = 12;
const ICANON: u32 = 0o2;
const ECHO: u32 = 0o10;
const VTIME_OFFSET: usize = 22;
const VMIN_OFFSET: usize = 23;
const MAX_BYTES: usize = 256 * 1024;
const MAX_LINES: usize = 8192;
const PAGE_ROWS: usize = 22;
const SCREEN_COLS: usize = 80;

fn enter_raw_mode() -> Option<[u8; 36]> {
  let mut saved = [0u8; 36];

  if get_termios(0, &mut saved) < 0 {
    return None;
  }

  let mut raw = saved;
  let lflag = u32::from_le_bytes([
    raw[LFLAG_OFFSET],
    raw[LFLAG_OFFSET + 1],
    raw[LFLAG_OFFSET + 2],
    raw[LFLAG_OFFSET + 3],
  ]);
  
  let lflag = lflag & !(ICANON | ECHO);
  raw[LFLAG_OFFSET..LFLAG_OFFSET + 4].copy_from_slice(&lflag.to_le_bytes());
  raw[VMIN_OFFSET] = 1;
  raw[VTIME_OFFSET] = 0;

  if set_termios(0, &raw) < 0 {
    eprint("less: cannot enter raw mode\n");
    return None;
  }

  Some(saved)

}

fn load_file(path: &[u8], buffer: &mut [u8]) -> Option<usize> {
  let file_fd = open(path, OPEN_READ_ONLY);

  if file_fd < 0 {
    return None;
  }

  let mut length = 0;

  while length < buffer.len() {
    let count = read(file_fd as usize, &mut buffer[length..]);

    if count < 0 {
      close(file_fd as usize);
      return None;
    }

    if count == 0 {
      break;
    }

    length += count as usize;
  }

  close(file_fd as usize);
  Some(length)
}

fn index_lines(text: &[u8], starts: &mut [u32]) -> usize {
  let mut lines = 0;
  let mut line_start = 0;

  for index in 0..text.len() {
    if text[index] == b'\n' {
      if lines < starts.len() {
        starts[lines] = line_start as u32;
        lines += 1
      }
      line_start = index + 1;
    }
  }

  if line_start < text.len() && lines < starts.len() {
    starts[lines] = line_start as u32;
    lines += 1;
  }

  lines
}

fn draw_line(text: &[u8], start: usize) {
  let mut end = start;

  while end < text.len() && text[end] != b'\n' && end - start < SCREEN_COLS {
    end += 1;
  }

  write(1, &text[start..end]);
  write(1, b"\n");
}

fn draw_screen(text: &[u8], starts: &[u32], lines: usize, top: usize) {
  print("\x1b[2J\x1b[H");

  let bottom = if top + PAGE_ROWS < lines { top + PAGE_ROWS } else { lines };

  for line in top..bottom {
    draw_line(text, starts[line] as usize);
  }

  print(": lines ");
  print_u64((top + 1) as u64);
  print("-");
  print_u64(bottom as u64);
  print(" of ");
  print_u64(lines as u64);
  print("  (j/k line, space/b page, g/G ends, q quit)");
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 2 {
    eprint("usage: less FILE\n");
    return 1;
  }

  let path = arg(argv, 1);
  
  let mut buffer = [0u8; MAX_BYTES];
  let Some(length) = load_file(path, &mut buffer) else {
    eprint("less: cannot read file\n");
    return 1;
  };
  let text = &buffer[..length];

  let mut starts = [0u32; MAX_LINES];
  let lines = index_lines(text, &mut starts);

  let Some(saved) = enter_raw_mode() else {
    eprint("less: stdin is not a terminal\n");
    return 1;
  };

  let last_top = if lines > PAGE_ROWS { lines - PAGE_ROWS } else { 0 };
  let mut top = 0;
  let mut key = [0u8; 1];

  loop {
    draw_screen(text, &starts, lines, top);

    if read(0, &mut key) != 1 {
      break;
    }

    match key[0] {
      b'q' => break,
      b'j' | b'\n' | b'\r' => {
        if top < last_top {
          top += 1;
        }
      }
      b'k' => top = top.saturating_sub(1),
      b' ' | b'f' => {
        top = if top + PAGE_ROWS < last_top { top + PAGE_ROWS } else { last_top };
      }
      b'b' => top = top.saturating_sub(PAGE_ROWS),
      b'g' => top = 0,
      b'G' => top = last_top,
      _ => {}
    }
  }

  set_termios(0, &saved);
  print("\n");

  return 0;
}
