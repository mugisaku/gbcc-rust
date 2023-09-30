

pub mod r#enum;
pub mod r#struct;
pub mod read_type;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  Library
};

use std::cell::Cell;
use super::get_aligned_size;
use super::expression::Expression;
use self::r#struct::Struct;
use self::r#enum::Enum;

use crate::token::{
  Token,
};

use crate::syntax::{
  Directory,
  Cursor,
};


pub const WORD_SIZE: usize = 8;




#[derive(Clone)]
pub enum
Type
{
  Void,

  Bool,Char,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,

  F32, F64,

  FunctionPointer(Vec<Type>,Box<Type>),

  Pointer(Box<Type>),
  Reference(Box<Type>),

  Array(Box<Type>,String),

  Tuple(Vec<Type>),

  Symbol(String),

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


fn
make_id_from_list(ls: &Vec<Type>)-> String
{
  let  mut id = String::from("(");

    for ty in ls
    {
      id.push_str(ty.get_id().as_str());

      id.push(',');
    }


  id.push(')');

  id
}


pub fn
is_bool(&self)-> bool
{
    match self
    {
  Type::Bool=>{ true}
  _         =>{false}
    }
}


pub fn
is_char(&self)-> bool
{
    match self
    {
  Type::Char=>{ true}
  _         =>{false}
    }
}


pub fn
is_signed_integer(&self)-> bool
{
    match self
    {
  Type::I8|Type::I16|Type::I32|Type::I64|Type::ISize=>{ true}
  _                                                 =>{false}
    }
}


pub fn
is_i8(&self)-> bool
{
    match self
    {
  Type::I8=>{ true}
  _       =>{false}
    }
}


pub fn
is_i16(&self)-> bool
{
    match self
    {
  Type::I16=>{ true}
  _        =>{false}
    }
}


pub fn
is_i32(&self)-> bool
{
    match self
    {
  Type::I32=>{ true}
  _        =>{false}
    }
}


pub fn
is_i64(&self)-> bool
{
    match self
    {
  Type::I64=>{ true}
  _        =>{false}
    }
}


pub fn
is_isize(&self)-> bool
{
    match self
    {
  Type::ISize=>{ true}
  _          =>{false}
    }
}


pub fn
is_unsigned_integer(&self)-> bool
{
    match self
    {
  Type::U8|Type::U16|Type::U32|Type::U64|Type::USize=>{ true}
  _                                                 =>{false}
    }
}


pub fn
is_u8(&self)-> bool
{
    match self
    {
  Type::U8=>{ true}
  _       =>{false}
    }
}


pub fn
is_u16(&self)-> bool
{
    match self
    {
  Type::U16=>{ true}
  _        =>{false}
    }
}


pub fn
is_u32(&self)-> bool
{
    match self
    {
  Type::U32=>{ true}
  _        =>{false}
    }
}


pub fn
is_u64(&self)-> bool
{
    match self
    {
  Type::U64=>{ true}
  _        =>{false}
    }
}


pub fn
is_usize(&self)-> bool
{
    match self
    {
  Type::USize=>{ true}
  _          =>{false}
    }
}


pub fn
is_integer(&self)-> bool
{
    match self
    {
   Type::I8|Type::I16|Type::I32|Type::I64|Type::ISize
  |Type::U8|Type::U16|Type::U32|Type::U64|Type::USize=>{ true}
  _                                                  =>{false}
    }
}


pub fn
is_floating(&self)-> bool
{
    match self
    {
  Type::F32|Type::F64=>{ true}
  _                  =>{false}
    }
}


pub fn
is_f32(&self)-> bool
{
    match self
    {
  Type::F32=>{ true}
  _        =>{false}
    }
}


pub fn
is_f64(&self)-> bool
{
    match self
    {
  Type::F64=>{ true}
  _        =>{false}
    }
}


pub fn
get_id(&self)-> String
{
    match self
    {
  Type::Void =>{String::from("void")},
  Type::Bool =>{String::from("bool")},
  Type::Char =>{String::from("char")},
  Type::I8   =>{String::from("i8")},
  Type::I16  =>{String::from("i16")},
  Type::I32  =>{String::from("i32")},
  Type::I64  =>{String::from("i64")},
  Type::ISize=>{String::from("isize")},
  Type::U8   =>{String::from("u8")},
  Type::U16  =>{String::from("u16")},
  Type::U32  =>{String::from("u32")},
  Type::U64  =>{String::from("u64")},
  Type::USize=>{String::from("usize")},
  Type::F32  =>{String::from("f32")},
  Type::F64  =>{String::from("f64")},
  Type::FunctionPointer(para_ls,ret_ty)=>
        {
          let  mut id = String::from("fp");

          id.push_str(Self::make_id_from_list(para_ls).as_str());
          id.push_str("->");
          id.push_str(ret_ty.get_id().as_str());

          id
        },
  Type::Pointer(target)=>
        {
          let  mut id = target.get_id();

          id.push('*');

          id
        },
  Type::Reference(target)=>
        {
          let  mut id = target.get_id();

          id.push('&');

          id
        },
  Type::Tuple(ty_ls)=>
        {
          Self::make_id_from_list(ty_ls)
        },
  Type::Array(ty,s)=>
        {
          String::from("")
        },
  Type::Symbol(name)=>
        {
          String::from("")
        },
    }
}


pub fn
print(&self)
{
    match self
    {
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
  Type::FunctionPointer(para_ls,ret_ty)=>
        {
          print!("fn(");

            for para in para_ls
            {
              para.print();

              print!(",");
            }


          print!(")-> ");

          ret_ty.print();
        },
  Type::Pointer(ty)=>
        {
          print!("*");

          ty.print();
        },
  Type::Reference(ty)=>
        {
          print!("&");

          ty.print();
        },
  Type::Tuple(ty_ls)=>
        {
          print!("(");

            for ty in ty_ls
            {
              ty.print();

              print!(", ");
            }


          print!(")");
        },
  Type::Array(ty,s)=>
        {
          print!("[{}]",s);

          ty.print();
        },
  Type::Symbol(name)=>
        {
          print!("{}",name);
        },
    }
}


}




#[derive(Clone)]
pub struct
Field<'a>
{
  pub(crate) name: String,

  pub(crate) offset: usize,

  pub(crate) r#type: &'a Type,

}


#[derive(Clone)]
pub struct
TypeInfo
{
  pub(crate)  size: usize,
  pub(crate) align: usize,

}


impl
TypeInfo
{


pub fn
new(sz: usize)-> TypeInfo
{
  TypeInfo{size: sz, align: sz}
}


fn
from_list(ls: &Vec<Type>)-> TypeInfo
{
  let  mut ti = Self::new(0);

    for ty in ls
    {
      let  co_ti = Self::from(ty);

      ti.size  = get_aligned_size(ti.size+co_ti.size);
      ti.align = std::cmp::max(ti.align,co_ti.align);
    }


  ti
}


pub fn
from(ty: &Type)-> TypeInfo
{
    match ty
    {
  Type::Void =>{Self::new(0)},
  Type::Bool =>{Self::new(1)},
  Type::Char =>{Self::new(1)},
  Type::I8   =>{Self::new(1)},
  Type::I16  =>{Self::new(2)},
  Type::I32  =>{Self::new(4)},
  Type::I64  =>{Self::new(8)},
  Type::ISize=>{Self::new(WORD_SIZE)},
  Type::U8   =>{Self::new(1)},
  Type::U16  =>{Self::new(2)},
  Type::U32  =>{Self::new(4)},
  Type::U64  =>{Self::new(8)},
  Type::USize=>{Self::new(WORD_SIZE)},
  Type::F32  =>{Self::new(4)},
  Type::F64  =>{Self::new(8)},
  Type::FunctionPointer(_,_)=>{Self::new(WORD_SIZE)},
  Type::Pointer(_)=>{Self::new(WORD_SIZE)},
  Type::Reference(_)=>{Self::new(WORD_SIZE)},
  Type::Tuple(ty_ls)=>{Self::from_list(ty_ls)},
  Type::Array(ty,s)=>
        {
          Self::new(0)
        },
  Type::Symbol(name)=>
        {
          Self::new(0)
        },
    }
}


/*
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
*/


}





