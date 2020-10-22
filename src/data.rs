
pub fn get_typ(val: &u8) -> DataType {
  println!("value: {}", *val);
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
    0xDB => DataType::boolean(false),
    0xDC => DataType::chr(0 as char),
    0xDD => DataType::string(String::new()),
    _ => panic!("Invalid type: {}", *val)
  }
}

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
  boolean(bool), // xDB
  chr(char), // xDC
  string(String), // xDD
  //fun(Fn::Fun), // xDF
}

pub mod function {
  use std::collections::HashMap;
  use super::{DataType, get_typ};
  use crate::string::get_str;
  pub struct Fun {
    start: usize,
    length: usize,
    sig: HashMap<String, DataType>,
    ret: DataType
  }
  impl Fun {
    pub fn new(dat: &bytes::Bytes, indx: &mut usize) -> Fun {
      assert_eq!(&dat[*indx], &0xDFu8);
      println!("{:?}", dat);
      let start = *indx;
      let sig = HashMap::new();
      let ret = get_typ(&dat[*indx+1]);

      *indx += 2;
      /*
      let count = &dat[*indx];
      for _param in 0..*count {
        let typ = get_typ(&dat[*indx]);
        *indx += 1;
        sig.insert(
          get_str(&&dat, indx),
          typ
        );
      }*/
      let mut length = 0;
      while &dat[*indx..=*indx+1] != &[0x3bu8, 0x3bu8] {
        *indx += 1;
        length += 1;
      }
      Fun{start, length, sig, ret}
    }
    pub fn exec(&self, code: &bytes::Bytes, indx: &mut usize) {
      println!("{}", indx);
      print!("{}", crate::string::get_str(&code, indx));
    }
  }
}
