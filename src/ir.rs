

pub mod memory;
pub mod block;
pub mod block__add;
pub mod function;
pub mod executor;
pub mod test;

use crate::ir::{
  executor::Library,
};

use crate::language::{
  expression,
  statement,
  statement::Program,
  typesystem::TypeNote,
  typesystem::r#struct::Struct,
  typesystem::r#union::Union,
  typesystem::r#enum::Enum,
  typesystem::function_signature::FunctionSignature,
};


pub fn
compile(prog: &Program)-> Result<Library,()>
{
  Err(())
}




