

pub mod expression;
pub mod statement;
pub mod type_info;
pub mod memory;
pub mod evaluator;
pub mod compile_for_expression;
pub mod compile_for_block;
pub mod space;
pub mod literal;
pub mod constant;
pub mod symbol;
//pub mod dynamic_machine;
pub mod dictionary;
pub mod read;


/*
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
*/




