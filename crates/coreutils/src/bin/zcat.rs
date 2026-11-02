#![no_std]
#![no_main]

use coreutils::syscall::write;
use coreutils::{arg, eprint, load_file};

const MAX_BYTES: usize = 256 * 1024;

fn is_gzip(data: &[u8]) -> bool {
  data.len() >= 10 && data[0] == 0x1f && data[1] == 0x8b && data[2] == 8
}

fn skip_c_string(data: &[u8], start: usize) -> Option<usize> {
  let end = data.get(start..)?.iter().position( |&byte| byte == 0)?;
  Some(start + end + 1)
}

fn deflate_start(data: &[u8]) -> Option<usize> {
  let flags = data[3];
  let mut pos = 10;

  if flags & 0x04 != 0 {
    let extra = data.get(pos..pos + 2)?;
    pos += 2 + u16::from_le_bytes([extra[0], extra[1]]) as usize;
  }

  if flags & 0x08 != 0 {
    pos = skip_c_string(data, pos)?;
  }

  if flags & 0x10 != 0 {
    pos = skip_c_string(data, pos)?;
  }

  if flags & 0x02 != 0 {
    pos += 2;
  }

  if pos + 8 > data.len() {
    return None;
  }

  Some(pos)
}

const WINDOW: usize = 32 * 1024;

struct BitReader<'a> {
  data: &'a [u8],
  pos: usize,
  bit_buf: u32,
  bit_count: u32,
}

impl<'a> BitReader<'a> {
  fn new(data: &'a [u8], pos: usize) -> BitReader<'a> {
    BitReader { data, pos, bit_buf: 0, bit_count: 0 }
  }

  fn bits(&mut self, count: u32) -> Option<u32> {
    while self.bit_count < count {
      let byte = *self.data.get(self.pos)?;
      self.pos += 1;
      self.bit_buf |= (byte as u32) << self.bit_count;
      self.bit_count += 8;
    }

    let value = self.bit_buf & ((1 << count) - 1);
    self.bit_buf >>= count;
    self.bit_count -= count;
    Some(value)
  }

  fn align(&mut self) {
    self.bit_buf = 0;
    self.bit_count = 0;
  }
}

fn crc32_update(crc: u32, byte: u8) -> u32 {
  let mut crc = crc ^ byte as u32;

  for _ in 0..8 {
    crc = if crc & 1 != 0 { (crc >> 1) ^ 0xedb8_8320 } else { crc >> 1 };
  }

  crc
}

struct Output {
  window: [u8; WINDOW],
  total: usize,
  member_start: usize,
  crc: u32,
}

impl Output {
  fn new() -> Output {
    Output { window: [0; WINDOW], total: 0, member_start: 0, crc: 0xffff_ffff }
  }

  fn put(&mut self, byte: u8) {
    self.window[self.total % WINDOW] = byte;
    self.total += 1;
    self.crc = crc32_update(self.crc, byte);

    if self.total % WINDOW == 0 {
      write(1, &self.window);
    }
  }

  fn copy(&mut self, distance: usize, length: usize) -> bool {
    if distance == 0 || distance > self.total - self.member_start || distance > WINDOW {
      return false;
    }

    for _ in 0..length {
      let byte = self.window[(self.total - distance) % WINDOW];
      self.put(byte);
    }

    true
  }

  fn start_member(&mut self) {
    self.member_start = self.total;
    self.crc = 0xffff_ffff;
  }

  fn member_len(&self) -> u32 {
    (self.total - self.member_start) as u32
  }

  fn member_crc(&self) -> u32 {
    !self.crc
  }

  fn finish(&self) {
    write(1, &self.window[..self.total % WINDOW]);
  }
}

const LENGTH_BASE: [u16; 29] = [
  3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
  35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
  0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
  3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
  1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
  257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
  8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
  0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6,
  7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

struct Huffman {
  count: [u16; 16],
  symbol: [u16; 288],
}

impl Huffman {
  fn new(lengths: &[u8]) -> Huffman {
    let mut table = Huffman { count: [0; 16], symbol: [0; 288] };

    for &length in lengths {
      table.count[length as usize] += 1;
    }

    let mut offset = [0u16; 16];
    for length in 1..15 {
      offset[length + 1] = offset[length] + table.count[length];
    }

    for (symbol, &length) in lengths.iter().enumerate() {
      if length != 0 {
        table.symbol[offset[length as usize] as usize] = symbol as u16;
        offset[length as usize] += 1;
      }
    }

    table
  }

  fn decode(&self, reader: &mut BitReader) -> Option<u16> {
    let mut code: i32 = 0;
    let mut first: i32 = 0;
    let mut index: i32 = 0;

    for length in 1..16 {
      code |= reader.bits(1)? as i32;

      let count = self.count[length] as i32;
      if code - count < first {
        return Some(self.symbol[(index + (code - first)) as usize]);
      }

      index += count;
      first += count;
      first <<= 1;
      code <<= 1;
    }

    None
  }
}

fn codes_block(
  reader: &mut BitReader,
  out: &mut Output,
  literals: &Huffman,
  distances: &Huffman,
) -> Option<()> {
  loop {
    let symbol = literals.decode(reader)? as usize;

    if symbol < 256 {
      out.put(symbol as u8);
    } else if symbol == 256 {
      return Some(());
    } else {
      let index = symbol - 257;
      if index >= LENGTH_BASE.len() {
        return None;
      }
      let length = LENGTH_BASE[index] as usize
        + reader.bits(LENGTH_EXTRA[index] as u32)? as usize;

      let index = distances.decode(reader)? as usize;
      if index >= DIST_BASE.len() {
        return None;
      }
      let distance = DIST_BASE[index] as usize
        + reader.bits(DIST_EXTRA[index] as u32)? as usize;

      if !out.copy(distance, length) {
        return None;
      }
    }
  }
}

fn fixed_block(reader: &mut BitReader, out: &mut Output) -> Option<()> {
  let mut lengths = [0u8; 288];

  for symbol in 0..288 {
    lengths[symbol] = match symbol {
      0..=143 => 8,
      144..=255 => 9,
      256..=279 => 7,
      _ => 8,
    };
  }

  let literals = Huffman::new(&lengths);
  let distances = Huffman::new(&[5u8; 30]);
  codes_block(reader, out, &literals, &distances)
}

const CODE_LENGTH_ORDER: [usize; 19] = [
  16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

fn dynamic_block(reader: &mut BitReader, out: &mut Output) -> Option<()> {
  let literal_count = reader.bits(5)? as usize + 257;
  let distance_count = reader.bits(5)? as usize + 1;
  let code_length_count = reader.bits(4)? as usize + 4;

  if literal_count > 286 || distance_count > 30 {
    return None;
  }

  let mut code_lengths = [0u8; 19];
  for index in 0..code_length_count {
    code_lengths[CODE_LENGTH_ORDER[index]] = reader.bits(3)? as u8;
  }
  let code_length_table = Huffman::new(&code_lengths);

  let total = literal_count + distance_count;
  let mut lengths = [0u8; 320];
  let mut index = 0;

  while index < total {
    let symbol = code_length_table.decode(reader)?;

    if symbol < 16 {
      lengths[index] = symbol as u8;
      index += 1;
      continue;
    }

    let (value, repeat) = match symbol {
      16 => {
        if index == 0 {
          return None;
        }
        (lengths[index - 1], 3 + reader.bits(2)? as usize)
      }
      17 => (0, 3 + reader.bits(3)? as usize),
      _ => (0, 11 + reader.bits(7)? as usize),
    };

    if index + repeat > total {
      return None;
    }

    for _ in 0..repeat {
      lengths[index] = value;
      index += 1;
    }
  }

  if lengths[256] == 0 {
    return None;
  }

  let literals = Huffman::new(&lengths[..literal_count]);
  let distances = Huffman::new(&lengths[literal_count..total]);
  codes_block(reader, out, &literals, &distances)
}

fn stored_block(reader: &mut BitReader, out: &mut Output) -> Option<()> {
  reader.align();

  let len = reader.bits(16)?;
  let nlen = reader.bits(16)?;

  if (len ^ nlen) != 0xffff {
    return None;
  }

  for _ in 0..len {
    let byte = reader.bits(8)? as u8;
    out.put(byte);
  }

  Some(())
}

// Decodes every block of one DEFLATE stream and returns the offset just past it
fn inflate(data: &[u8], start: usize, out: &mut Output) -> Option<usize> {
  let mut reader = BitReader::new(data, start);

  loop {
    let last = reader.bits(1)?;
    let kind = reader.bits(2)?;

    match kind {
      0 => stored_block(&mut reader, out)?,
      1 => fixed_block(&mut reader, out)?,
      2 => dynamic_block(&mut reader, out)?,
      _ => return None,
    }

    if last == 1 {
      return Some(reader.pos);
    }
  }
}

// Decompresses every gzip member in the file, checking each trailer
fn decompress(data: &[u8], out: &mut Output) -> Result<(), &'static str> {
  let mut member = 0;

  loop {
    let rest = &data[member..];

    let Some(start) = deflate_start(rest) else {
      return Err("truncated gzip header");
    };

    out.start_member();
    let Some(end) = inflate(rest, start, out) else {
      return Err("corrupt data");
    };

    let Some(trailer) = rest.get(end..end + 8) else {
      return Err("truncated gzip trailer");
    };

    let crc = u32::from_le_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    if crc != out.member_crc() {
      return Err("crc error");
    }

    let size = u32::from_le_bytes([trailer[4], trailer[5], trailer[6], trailer[7]]);
    if size != out.member_len() {
      return Err("length error");
    }

    member += end + 8;
    if !is_gzip(&data[member..]) {
      return Ok(());
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 2 {
    eprint("usage: zcat FILE\n");
    return 1;
  }

  let mut buffer = [0u8; MAX_BYTES];
  let Some(length) = load_file(arg(argv, 1), &mut buffer) else {
    eprint("zcat: cannot read file\n");
    return 1;
  };
  if length == MAX_BYTES {
    eprint("zcat: file too large\n");
    return 1;
  }
  let data = &buffer[..length];

  if !is_gzip(data) {
    eprint("zcat: not in gzip format\n");
    return 1;
  }

  let mut out = Output::new();
  let result = decompress(data, &mut out);
  out.finish();

  if let Err(message) = result {
    eprint("zcat: ");
    eprint(message);
    eprint("\n");
    return 1;
  }

  return 0;
}