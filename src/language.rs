

pub mod constant;
pub mod expression;
pub mod statement;
pub mod opcode;
pub mod compile;
pub mod memory;
pub mod dictionary;
pub mod read;
pub mod element;
pub mod library;
pub mod type_kind;


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




