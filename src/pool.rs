use crate::data::types::DataType;

pub static mut POOL: Vec<DataType> = Vec::new();

pub fn get_val(indx: Option<usize>) -> DataType {
  unsafe {
    POOL[indx.unwrap()].clone()
  }
}

pub fn get_index(val: DataType) -> Option<usize> {
  unsafe {
    if !POOL.contains(&val) {
      POOL.push(val.clone());
    }
    POOL.iter().position(|itm| itm == &val)
  }
}