struct Fun {
  dat: &[u8],
  indx: usize,
  length: usize
}

impl Fun {
  fn set_fn(bytes: Vec<u8>, indx: &mut usize) -> Fun {
    // TODO 
  } 
}

enum DataType {
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  BOOL(bool),
  FUN(Fun)
}