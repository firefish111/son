use std::collections::HashMap;
use super::types::DataType;
use super::string::get_str;
use crate::pool;

pub struct Fun {
  start: usize,
  length: usize,
  sig: HashMap<String, DataType>,
  ret: DataType
}

impl Fun {
  pub fn new(dat: &bytes::Bytes, indx: &mut usize) -> Fun {
    assert_eq!(&dat[*indx], &0xDFu8);
    // DEBUG: println!("{:?}", dat);
    let start = *indx; // includes func headers, TODO.
    let sig = HashMap::new();
    let ret = DataType::get_typ(&dat[*indx+1]);

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
  pub fn exec(&self, code: &bytes::Bytes) {
    let mut x = self.start + 3;
    if let DataType::Str(elf) = pool::get_val(get_str(&code, &mut x)) {
      print!("{}", elf);
    }
  }
}