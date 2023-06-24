

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




pub struct
Field
{
  pub(crate) name: String,

  pub(crate) offset: usize,

  pub(crate) type_index: TypeIndex,

}


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
make_from_type(t: &Type, lib: &Library)-> Result<TypeInfo,()>
{
  let  mut ti = TypeInfo::new();

    if ti.update_by_type(t,lib).is_ok(){Ok(ti)}
  else{Err(())}
}


pub fn
make_from_type_index(tx: TypeIndex, lib: &Library)-> Result<TypeInfo,()>
{
  let  mut ti = TypeInfo::new();

    if ti.update_by_type_index(tx,lib).is_ok(){Ok(ti)}
  else{Err(())}
}


pub fn
update_by_type(&mut self, t: &Type, lib: &Library)-> Result<(),()>
{
  self.field_list.clear();

    match t
    {
  Type::Undefined=>
        {
        },
  Type::FromExpression(ei)=>
        {
            if let Some(e) = lib.get_expression(*ei)
            {
                if let Ok(eti) = e.get_type_index(lib)
                {
                  return Ok(());
                }
            }


          return Err(());
        },
  Type::Void=>
        {
          self.id.push_str("voi");
        },
  Type::Bool=>
        {
          self.id.push_str("bol");
          self.size  = 1;
          self.align = 1;
        },
  Type::Char=>
        {
          self.id.push_str("chr");
          self.size  = 1;
          self.align = 1;
        },
  Type::I8=>
        {
          self.id.push_str("i8");
          self.size  = 1;
          self.align = 1;
        },
  Type::I16=>
        {
          self.id.push_str("i16");
          self.size  = 2;
          self.align = 2;
        },
  Type::I32=>
        {
          self.id.push_str("i32");
          self.size  = 4;
          self.align = 4;
        },
  Type::I64=>
        {
          self.id.push_str("i64");
          self.size  = 8;
          self.align = 8;
        },
  Type::ISize=>
        {
          self.id.push_str("isz");
          self.size  = WORD_SIZE;
          self.align = WORD_SIZE;
        },
  Type::U8=>
        {
          self.id.push_str("u8");
          self.size  = 1;
          self.align = 1;
        },
  Type::U16=>
        {
          self.id.push_str("u16");
          self.size  = 2;
          self.align = 2;
        },
  Type::U32=>
        {
          self.id.push_str("u32");
          self.size  = 4;
          self.align = 4;
        },
  Type::U64=>
        {
          self.id.push_str("u64");
          self.size  = 8;
          self.align = 8;
        },
  Type::USize=>
        {
          self.id.push_str("usz");
          self.size  = WORD_SIZE;
          self.align = WORD_SIZE;
        },
  Type::F32=>
        {
          self.id.push_str("f32");
          self.size  = 4;
          self.align = 4;
        },
  Type::F64=>
        {
          self.id.push_str("f64");
          self.size  = 8;
          self.align = 8;
        },
  Type::FunctionPointer(sig)=>
        {
          self.id.push_str("fnp");

            for tx in &sig.parameter_list
            {
                if self.update_by_type_index(*tx,lib).is_ok()
                {
                  continue;
                }


              return Err(());
            }


          self.id.push_str("->");

          return self.update_by_type_index(sig.return_type_index,lib);
        },
  Type::Tuple(tx_ls)=>
        {
          self.id.push_str("tpl");

          let  mut offset: usize = 0;
          let  mut  align: usize = 0;

            for tx in tx_ls
            {
                if let Ok(ti) = Self::make_from_type_index(*tx,lib)
                {
                  align = std::cmp::max(align,ti.align);

                  let  next_offset = get_aligned_size(offset+ti.size);

                  self.field_list.push(Field{name: String::new(), offset, type_index: *tx});

                  offset = next_offset;
                }

              else
                {
                  return Err(());
                }
            }


          self.size  = offset;
          self.align =  align;
        },
  Type::Pointer(target_tx)=>
        {
          self.id.push_str("ptr");

            if self.update_by_type_index(*target_tx,lib).is_err()
            {
              return Err(());
            }


          self.size  = WORD_SIZE;
          self.align = WORD_SIZE;
        },
  Type::Reference(target_tx)=>
        {
          self.id.push_str("ref");

            if self.update_by_type_index(*target_tx,lib).is_err()
            {
              return Err(());
            }


          self.size  = WORD_SIZE;
          self.align = WORD_SIZE;
        },
  Type::Array(target_tx,e)=>
        {
          self.id.push_str("arr");

            if self.update_by_type_index(*target_tx,lib).is_err()
            {
              return Err(());
            }
        },
  Type::Symbol(name)=>
        {
          self.id.push_str(name);
        },
    }


  Ok(())
}


pub fn
update_by_type_index(&mut self, ti: TypeIndex, lib: &Library)-> Result<(),()>
{
    if let Some(t) = lib.get_type(ti)
    {
      return self.update_by_type(t,lib);
    }


  Err(())
}


pub fn
get_field(&self, name: &str)-> Option<&Field>
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





