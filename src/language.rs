

pub mod expression;
pub mod typesystem;
pub mod statement;
pub mod evaluate;
pub mod value;
pub mod library;
pub mod compile;
pub mod compile_expression;


use crate::language::{
  typesystem::WORD_SIZE,

};


pub fn
get_aligned_size(sz: usize)-> usize
{
  (sz+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
}




