

pub mod r#enum;
pub mod r#struct;
pub mod r#union;
pub mod function_signature;
pub mod read_type_note;
pub mod dictionary;

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


pub fn
get_aligned_size(sz: usize)-> usize
{
  (sz+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
}


pub fn
get_max(a: usize, b: usize)-> usize
{
  if a <= b{b} else{a}
}




#[derive(Clone)]
pub enum
TypeNote
{
  Undefined,
  Unspecified,

  Void,

  Bool,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,

  F32, F64,

  FunctionPointer(Box<FunctionSignature>),

  Struct(Box<Struct>),
  Union(Box<Union>),
  Enum(Box<Enum>),

  UnknownLengthArray(Box<TypeNote>,Expression),

  Array(Box<TypeNote>,usize),

  Identifier(String),

}


impl
TypeNote
{


pub fn
make_from_string(s: &str)-> Result<TypeNote,()>
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

          return self::read_type_note::read_type_note(&t_dir);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
get_size(&self)-> Option<usize>
{
    match self
    {
  TypeNote::Bool=>{return Some(1);},
  TypeNote::I8=>{return Some(1);},
  TypeNote::I16=>{return Some(2);},
  TypeNote::I32=>{return Some(4);},
  TypeNote::I64=>{return Some(8);},
  TypeNote::ISize=>{return Some(WORD_SIZE);},
  TypeNote::U8=>{return Some(1);},
  TypeNote::U16=>{return Some(2);},
  TypeNote::U32=>{return Some(4);},
  TypeNote::U64=>{return Some(8);},
  TypeNote::USize=>{return Some(WORD_SIZE);},
  TypeNote::F32=>{return Some(4);},
  TypeNote::F64=>{return Some(8);},
  TypeNote::FunctionPointer(_)=>{return Some(WORD_SIZE);},
  TypeNote::Struct(st)=>{return *st.get_size();},
  TypeNote::Union(un)=>{return *un.get_size();},
  TypeNote::Enum(en)=>{return *en.get_size();},
  TypeNote::Array(ty,n)=>
        {
            if let Some(sz) = ty.get_size()
            {
              return Some(sz*n);
            }
        },
  _=>{},
    }


  None
}


pub fn
get_align(&self)-> Option<usize>
{
    match self
    {
  TypeNote::Struct(st)=>{*st.get_align()},
  TypeNote::Union(un)=>{*un.get_align()},
  TypeNote::Enum(en)=>{*en.get_align()},
  _=>{self.get_size()},
    }
}


pub fn
print(&self)
{
    match self
    {
  TypeNote::Undefined=>{print!("undefined");},
  TypeNote::Unspecified=>{print!("unspecified");},
  TypeNote::Void=>{print!("void");},
  TypeNote::Bool=>{print!("bool");},
  TypeNote::I8=>{print!("i8");},
  TypeNote::I16=>{print!("i16");},
  TypeNote::I32=>{print!("i32");},
  TypeNote::I64=>{print!("i64");},
  TypeNote::ISize=>{print!("isize");},
  TypeNote::U8=>{print!("u8");},
  TypeNote::U16=>{print!("u16");},
  TypeNote::U32=>{print!("u32");},
  TypeNote::U64=>{print!("u64");},
  TypeNote::USize=>{print!("usize");},
  TypeNote::F32=>{print!("f32");},
  TypeNote::F64=>{print!("f64");},
  TypeNote::FunctionPointer(sig)=>
        {
          print!("fn");
          sig.print();
        },
  TypeNote::Struct(st)=>{st.print()},
  TypeNote::Union(un)=>{un.print()},
  TypeNote::Enum(en)=>{en.print()},
  TypeNote::UnknownLengthArray(ty,e)=>
        {
          print!("[");

          e.print();

          print!("]");

          ty.print();
        },
  TypeNote::Array(ty,n)=>
        {
          print!("[{}]",*n);

          ty.print();
        },
  TypeNote::Identifier(s)=>
        {
          print!("?{}",s);
        },
    }
}


}




