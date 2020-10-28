use crate::data::types::DataType;

static mut POOL: Vec<DataType> = Vec::new();

pub fn get_val<'a>(indx: usize) -> &'a DataType {
  unsafe {
    &POOL[indx]
  }
}

pub fn add_val(val: DataType) -> usize {
  unsafe {
    /*
    if !POOL.contains(&val) {
      POOL.push(val);
      POOL.len() - 1
    }
    POOL.iter().position(|itm| itm == &val).unwrap()
    */
    POOL.push(val);
    POOL.len() - 1
  }
}