

pub mod machine;
pub mod asm;
pub mod expr;
pub mod stmt;
pub mod ty;
pub mod evaluate;
pub mod evaluate_unary;
pub mod evaluate_binary;
pub mod decl;
pub mod program;
pub mod scope;
pub mod symbol_table;
pub mod tplg_sort;
pub mod execute;
//pub mod codify;
pub mod dictionary;




pub const WORD_SIZE: usize = 8;

pub fn
get_word_aligned(off: usize)-> usize
{
  (off+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
}

pub fn
get_aligned(al: usize, off: usize)-> usize
{
  if al != 0{(off+(al-1))/al*al} else{off}
}


pub fn
to_u64_from_bool(b: bool)-> u64
{
  if b{1} else{0}
}





