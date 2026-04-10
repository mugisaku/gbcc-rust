

pub mod machine;
pub mod asm;
pub mod expr;
pub mod stmt;
pub mod ty;
pub mod evaluate_const;
pub mod evaluate;
pub mod decl;
pub mod program;
pub mod scope;
pub mod symbol_table;
pub mod tplg_sort;
//pub mod codify;
pub mod dictionary;




pub const WORD_SIZE: usize = 8;

pub struct Align(usize);
impl Align{pub fn  get(&self, off: usize)-> usize{if self.0 != 0{(off+(self.0-1))/self.0*self.0} else{off}}}

pub fn
get_word_aligned(off: usize)-> usize
{
  Align(WORD_SIZE).get(off)
}


pub fn
to_u64_from_bool(b: bool)-> u64
{
  if b{1} else{0}
}





