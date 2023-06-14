

pub mod expression;
pub mod typesystem;
pub mod statement;
pub mod evaluate;
pub mod value;
pub mod library;
pub mod fixer;


use crate::language::{
  statement::Var,
  statement::Fn,
  statement::Definition,
  statement::Declaration,
  statement::Statement,
  statement::Block,
  statement::ConditionalBlock,
  statement::Program,
  expression::Expression,
  typesystem::r#struct::Struct,
  typesystem::r#union::Union,
  typesystem::r#enum::Enum,
  typesystem::function_signature::FunctionSignature,
  typesystem::Type,
  typesystem::WORD_SIZE,

};


pub fn
get_aligned_size(sz: usize)-> usize
{
  (sz+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
}




