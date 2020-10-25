use crate::data::types::DataType;

pub fn get_index(val: DataType, pool: &mut Vec<DataType>) -> Option<usize> {
  if !pool.contains(&val) {
    pool.push(val.clone());
  }
  pool.iter().position(|itm| itm == &val)
}