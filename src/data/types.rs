use std::{any::Any, cmp::PartialEq};

#[derive(PartialEq, Clone, Debug)]
pub enum DataType {
  Nil,           // xD0
  I8(i8),        // xD1
  I16(i16),      // xD2
  I32(i32),      // xD3
  I64(i64),      // xD4
  U8(u8),        // xD5
  U16(u16),      // xD6
  U32(u32),      // xD7
  U64(u64),      // xD8
  F32(f32),      // xD9
  F64(f64),      // xDA
  Boolean(bool), // xDB
  Chr(char),     // xDC
  Str(String),   // xDD
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
      _ => panic!("Invalid type: {:#x}", *val),
    }
  }
/*
  pub fn upd<'a>(&self, val: &'a dyn Any) -> &Self {
    let out = match self {
      DataType::Nil => &self,
      DataType::I8(_) => &DataType::I8(*val.downcast_ref::<i8>().unwrap()),
      DataType::I16(_) => &DataType::I16(*val.downcast_ref::<i16>().unwrap()),
      DataType::I32(_) => &DataType::I32(*val.downcast_ref::<i32>().unwrap()),
      DataType::I64(_) => &DataType::I64(*val.downcast_ref::<i64>().unwrap()),
      DataType::U8(_) => &DataType::U8(*val.downcast_ref::<u8>().unwrap()),
      DataType::U16(_) => &DataType::U16(*val.downcast_ref::<u16>().unwrap()),
      DataType::U32(_) => &DataType::U32(*val.downcast_ref::<u32>().unwrap()),
      DataType::U64(_) => &DataType::U64(*val.downcast_ref::<u64>().unwrap()),
      DataType::F32(_) => &DataType::F32(*val.downcast_ref::<f32>().unwrap()),
      DataType::F64(_) => &DataType::F64(*val.downcast_ref::<f64>().unwrap()),
      DataType::Boolean(_) => &DataType::Boolean(*val.downcast_ref::<bool>().unwrap()),
      DataType::Chr(_) => &DataType::Chr(*val.downcast_ref::<char>().unwrap()),
      DataType::Str(_) => &DataType::Str(val.downcast_ref::<String>().unwrap().clone()),
    };
    *out
  }
*/
}
