use std::cmp::PartialEq;

#[derive(PartialEq, Clone)]
pub enum DataType {
  Nil, // xD0
  I8(i8), // xD1
  I16(i16), // xD2
  I32(i32), // xD3
  I64(i64), // xD4
  U8(u8), // xD5
  U16(u16), // xD6
  U32(u32), // xD7
  U64(u64), // xD8
  F32(f32), // xD9
  F64(f64), // xDA
  Boolean(bool), // xDB
  Chr(char), // xDC
  Str(String), // xDD
  // Fun(Fn::Fun), // xDF
}

impl DataType {
  pub fn get_typ(val: &u8) -> DataType {
    // DEBUG: println!("value: {}", *val);
    match *val {
      0xD0 => DataType::Nil,
      0xD1 => DataType::I8(0i8),
      0xD2 => DataType::I16(0i16),
      0xD3 => DataType::I32(0i32),
      0xD4 => DataType::I64(0i64),
      0xD5 => DataType::U8(0u8),
      0xD6 => DataType::U16(0u16),
      0xD7 => DataType::U32(0u32),
      0xD8 => DataType::U64(0u64),
      0xD9 => DataType::F32(0f32),
      0xDA => DataType::F64(0f64),
      0xDB => DataType::Boolean(false),
      0xDC => DataType::Chr('\0'),
      0xDD => DataType::Str(String::new()),
      _ => panic!("Invalid type: 0x{:x}", *val)
    }
  }
}