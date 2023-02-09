use crate::prelude::*;
use program::DataType;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Schema {
    ordered_types : Vec<DataType>,
}