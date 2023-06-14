

pub mod r#enum;
pub mod r#struct;
pub mod r#union;
pub mod function_signature;
pub mod read_type;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  Library
};
use std::cell::Cell;
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

use super::fixer::{
  Fixer,
  DeclarationIndex
};


pub const WORD_SIZE: usize = 8;




pub enum
Type
{
  Void,

  Bool,Char,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,

  F32, F64,

  FunctionPointer(Box<FunctionSignature>),

  Pointer(Box<Type>),
  Reference(Box<Type>),

  Array(Box<Type>,ExpressionIndex),

  Tuple(Vec<Type>),

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
  Type::Pointer(t)=>
        {
          print!("*");
          t.print(lib);
        },
  Type::Reference(t)=>
        {
          print!("&");
          t.print(lib);
        },
  Type::Tuple(ty_ls)=>
        {
          print!("(");

            for ty in ty_ls
            {
              ty.print(lib);

              print!(", ");
            }


          print!(")");
        },
  Type::Array(ty,ei)=>
        {
          print!("[");

            if let Some(e) = lib.get_expression(*ei)
            {
              e.print(lib);
            }


          print!("]");

          ty.print(lib);
        },
    }
}


}




pub struct
TypeInfo
{
  pub(crate) id: String,

  pub(crate)  size: usize,
  pub(crate) align: usize,

}


impl
TypeInfo
{


/*
pub fn
new(tn: &Type, e_opt: Option<&Expression>, f: &fixer)-> Type
{
  let  mut t = Type{kind: TypeKind::Void, id: String::new(), size: 0, align: 0};

    match tn
    {
  Type::Undefined=>{t.id.push_str("UD");},
  Type::Unspecified=>{},
  Type::Void=>
        {
          t.id.push_str("voi");
        },
  Type::Bool=>
        {
          t.kind = TypeKind::Bool;
          t.id.push_str("bol");
          t.size  = 1;
          t.align = 1;
        },
  Type::I8=>
        {
          t.kind = TypeKind::I8;
          t.id.push_str("i8");
          t.size  = 1;
          t.align = 1;
        },
  Type::I16=>
        {
          t.kind = TypeKind::I16;
          t.id.push_str("i16");
          t.size  = 2;
          t.align = 2;
        },
  Type::I32=>
        {
          t.kind = TypeKind::I32;
          t.id.push_str("i32");
          t.size  = 4;
          t.align = 4;
        },
  Type::I64=>
        {
          t.kind = TypeKind::I64;
          t.id.push_str("i64");
          t.size  = 8;
          t.align = 8;
        },
  Type::ISize=>
        {
          t.kind = TypeKind::ISize;
          t.id.push_str("isz");
          t.size  = WORD_SIZE;
          t.align = WORD_SIZE;
        },
  Type::U8=>
        {
          t.kind = TypeKind::U8;
          t.id.push_str("u8");
          t.size  = 1;
          t.align = 1;
        },
  Type::U16=>
        {
          t.kind = TypeKind::U16;
          t.id.push_str("u16");
          t.size  = 2;
          t.align = 2;
        },
  Type::U32=>
        {
          t.kind = TypeKind::U32;
          t.id.push_str("u32");
          t.size  = 4;
          t.align = 4;
        },
  Type::U64=>
        {
          t.kind = TypeKind::U64;
          t.id.push_str("u64");
          t.size  = 8;
          t.align = 8;
        },
  Type::USize=>
        {
          t.kind = TypeKind::USize;
          t.id.push_str("usz");
          t.size  = WORD_SIZE;
          t.align = WORD_SIZE;
        },
  Type::F32=>
        {
          t.kind = TypeKind::F32;
          t.id.push_str("f32");
          t.size  = 4;
          t.align = 4;
        },
  Type::F64=>
        {
          t.kind = TypeKind::F64;
          t.id.push_str("f64");
          t.size  = 8;
          t.align = 8;
        },
  Type::FunctionPointer(sig)=>
        {
          id.push_str("fnp");

            for t in &sig.parameter_list
            {
                if let Ok(new_id) = t.scan(String::new(),f)
                {
                  id.push_str(&new_id);
                }

              else
                {
                  return Err(());
                }
            }


          id.push_str("->");

            if let Ok(new_id) = sig.return_type_desk.scan(String::new(),f)
            {
              id.push_str(&new_id);

              return Ok(id);
            }
        },
  Type::Tuple(tn_ls)=>
        {
          t.id.push_str("tpl");

            for t in t_ls
            {
              t.id.push_str("ptr");

              let  target_t = Type::new(target_tn,e_opt,f);

              t.id.push_str(&target_t.id);

              t.size  = WORD_SIZE;
              t.align = WORD_SIZE;

              t.kind = TypeKind::Pointer(Box::new(target_t));
            }
        },
  Type::Pointer(target_tn)=>
        {
          t.id.push_str("ptr");

          let  target_t = Type::new(target_tn,e_opt,f);

          t.id.push_str(&target_t.id);

          t.size  = WORD_SIZE;
          t.align = WORD_SIZE;

          t.kind = TypeKind::Pointer(Box::new(target_t));
        },
  Type::Reference(target_tn)=>
        {
          t.id.push_str("ref");

          let  target_t = Type::new(target_tn,e_opt,f);

          t.id.push_str(&target_t.id);

          t.size  = WORD_SIZE;
          t.align = WORD_SIZE;

          t.kind = TypeKind::Reference(Box::new(target_t));
        },
  Type::Array(t,e)=>
        {
          t.id.push_str("arr");

            if let Ok(new_id) = t.scan(String::new(),f)
            {
              id.push_str(&new_id);

              return Ok(id);
            }
        },
  Type::Indefinite(s)=>
        {
          id.push_str("ID");

          id.push_str(s);

          return Ok(id);
        },
    }


  t
}
*/


}





