

pub mod r#enum;
pub mod r#struct;
pub mod r#union;
pub mod function_signature;
pub mod read_type;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
  SpaceIndex,
  Space,
  Library
};

use std::cell::Cell;
use super::get_aligned_size;
use super::expression::Expression;
use self::function_signature::FunctionSignature;
use self::r#struct::Struct;
use self::r#union::Union;
use self::r#enum::Enum;

use crate::token::{
  Token,
};

use crate::syntax::{
  Directory,
  Cursor,
};


pub const WORD_SIZE: usize = 8;




#[derive(PartialEq)]
pub enum
Type
{
  Undefined,

  Void,

  Bool,Char,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,

  F32, F64,

  FunctionPointer(FunctionSignature),

  Pointer(TypeIndex),
  Reference(TypeIndex),

  Array(TypeIndex,ExpressionIndex),

  Tuple(Vec<TypeIndex>),

  Symbol(String),

  FromExpression(ExpressionIndex),

}


impl
Type
{


pub fn
make_from_string(s: &str, lib: &mut Library)-> Result<Type,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = self::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"type_note",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(t_dir) = cur.get_directory()
        {
//                  t_dir.print(0);

          return self::read_type::read_type(&t_dir,lib);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
print(&self, lib: &Library)
{
    match self
    {
  Type::Undefined=>{print!("undef");},
  Type::FromExpression(_)=>{print!("from expr");},
  Type::Void=>{print!("void");},
  Type::Bool=>{print!("bool");},
  Type::Char=>{print!("char");},
  Type::I8=>{print!("i8");},
  Type::I16=>{print!("i16");},
  Type::I32=>{print!("i32");},
  Type::I64=>{print!("i64");},
  Type::ISize=>{print!("isize");},
  Type::U8=>{print!("u8");},
  Type::U16=>{print!("u16");},
  Type::U32=>{print!("u32");},
  Type::U64=>{print!("u64");},
  Type::USize=>{print!("usize");},
  Type::F32=>{print!("f32");},
  Type::F64=>{print!("f64");},
  Type::FunctionPointer(sig)=>
        {
          print!("fn");
          sig.print(lib);
        },
  Type::Pointer(ti)=>
        {
          print!("*");
          lib.print_type(*ti);
        },
  Type::Reference(ti)=>
        {
          print!("&");
          lib.print_type(*ti);
        },
  Type::Tuple(ti_ls)=>
        {
          print!("(");

            for ti in ti_ls
            {
              lib.print_type(*ti);

              print!(", ");
            }


          print!(")");
        },
  Type::Array(ti,ei)=>
        {
          print!("[");

          lib.print_expression(*ei);

          print!("]");

          lib.print_type(*ti);
        },
  Type::Symbol(name)=>
        {
          print!("sym {}",name);
        },
    }
}


}




#[derive(Clone)]
pub struct
Field
{
  pub(crate) name: String,

  pub(crate) offset: usize,

  pub(crate) type_index: TypeIndex,

}


#[derive(Clone)]
pub struct
TypeInfo
{
  pub(crate) id: String,

  pub(crate)  size: usize,
  pub(crate) align: usize,

  pub(crate) field_list: Vec<Field>,

}


impl
TypeInfo
{


pub fn
new()-> TypeInfo
{
  TypeInfo{id: String::new(), size: 0, align: 0, field_list: Vec::new()}
}


pub fn
find_field(&self, name: &str)-> Option<&Field>
{
    for f in &self.field_list
    {
        if f.name == name
        {
          return Some(f);
        }
    }


  None
}


}





