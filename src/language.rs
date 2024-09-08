

pub mod expression;
pub mod typesystem;
pub mod statement;
pub mod operation;
pub mod compile;
pub mod declaration;
pub mod dynamic_space;
pub mod dynamic_machine;
pub mod dynamic_dictionary;
pub mod dynamic_read;


use crate::language::{
  typesystem::WORD_SIZE,

};


pub fn
get_default_aligned_size(sz: usize)-> usize
{
  (sz+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
}


pub fn
get_aligned_size(sz: usize, al: usize)-> usize
{
    if al != 0
    {
      (sz+(al-1))/al*al
    }

  else
    {
      sz
    }
}




