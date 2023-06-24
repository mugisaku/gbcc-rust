

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};

use std::cell::Cell;

use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  Library
};


use super::{
  Type,
  r#struct::Struct,
  r#struct::Member,
  r#union::Union,
  r#enum::Enum,
  function_signature::FunctionSignature,
};


pub fn
read_type(dir: &Directory, lib: &mut Library)-> Result<Type,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(subdir) = cur.get_directory_with_name("primitive")
    {
      return read_primitive(&subdir);
    }

  else
    if let Some(subdir) = cur.get_directory_with_name("tuple")
    {
      return read_tuple(&subdir,lib);
    }

  else
    if let Some(subdir) = cur.get_directory_with_name("function_pointer")
    {
      return read_function_pointer(&subdir,lib);
    }

  else
    if let Some(s) = cur.get_identifier()
    {
//      return Ok(Type::Indefinite(s.clone()));
    }


  Err(())
}


pub fn
read_member(dir: &Directory, lib: &mut Library)-> Result<Member,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  s = id.clone();

      cur.advance(2);

        if let Some(subdir) = cur.get_directory_with_name("type")
        {
            if let Ok(t) = read_type(&subdir,lib)
            {
              return Ok(Member{name: s, type_index: lib.push_type(t)});
            }
        }
    }


  Err(())
}


pub fn
read_member_list(dir: &Directory, lib: &mut Library)-> Result<Struct,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut st = Struct::new();

    while let Some(subdir) = cur.seek_directory_with_name("member")
    {
        if let Ok(m) = read_member(&subdir,lib)
        {
          st.push(m);

          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


  Ok(st)
}

 
pub fn
read_type_list(dir: &Directory, lib: &mut Library)-> Result<Vec<Type>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Type> = Vec::new();

    while let Some(subdir) = cur.seek_directory_with_name("type")
    {
        if let Ok(t) = read_type(&subdir,lib)
        {
          ls.push(t);

          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


  Ok(ls)
}

 
pub fn
read_tuple(dir: &Directory, lib: &mut Library)-> Result<Type,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(subdir) = cur.get_directory_with_name("type_list")
    {
        if let Ok(ls) = read_type_list(&subdir,lib)
        {
          return Ok(Type::Tuple(lib.push_type_list(ls)));
        }
    }


  Err(())
}


pub fn
read_function_pointer(dir: &Directory, lib: &mut Library)-> Result<Type,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

  let  mut fnsig = FunctionSignature::new();

    if let Some(subdir) = cur.get_directory_with_name("type_note_list")
    {
        if let Ok(ls) = read_type_list(&subdir,lib)
        {
          fnsig.parameter_list = lib.push_type_list(ls);

          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


    if let Some(subdir) = cur.seek_directory_with_name("type")
    {
        if let Ok(t) = read_type(&subdir,lib)
        {
          fnsig.return_type_index = lib.push_type(t);
        }

      else
        {
          return Err(());
        }
    }


  Ok(Type::FunctionPointer(fnsig))
}


pub fn
read_primitive(dir: &Directory)-> Result<Type,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(s) = cur.get_keyword()
    {
           if s == "bool" {return Ok(Type::Bool);}
      else if s == "void" {return Ok(Type::Void);}
      else if s == "i8"   {return Ok(Type::I8);}
      else if s == "i16"  {return Ok(Type::I16);}
      else if s == "i32"  {return Ok(Type::I32);}
      else if s == "i64"  {return Ok(Type::I64);}
      else if s == "isize"{return Ok(Type::ISize);}
      else if s == "u8"   {return Ok(Type::U8);}
      else if s == "u16"  {return Ok(Type::U16);}
      else if s == "u32"  {return Ok(Type::U32);}
      else if s == "u64"  {return Ok(Type::U64);}
      else if s == "usize"{return Ok(Type::USize);}
      else if s == "f32"  {return Ok(Type::F32);}
      else if s == "f64"  {return Ok(Type::F64);}
    }


  Err(())
}




