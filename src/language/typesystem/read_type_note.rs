

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use super::{
  TypeNote,
  r#struct::Struct,
  r#struct::Member,
  r#union::Union,
  r#enum::Enum,
  function_signature::FunctionSignature,
};


pub fn
read_type_note(dir: &Directory)-> Result<TypeNote,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(subdir) = cur.get_directory_with_name("primitive")
    {
      return read_primitive(&subdir);
    }

  else
    if let Some(subdir) = cur.get_directory_with_name("tuple")
    {
      return read_tuple(&subdir);
    }

  else
    if let Some(subdir) = cur.get_directory_with_name("function_pointer")
    {
      return read_function_pointer(&subdir);
    }

  else
    if let Some(s) = cur.get_identifier()
    {
      return Ok(TypeNote::Identifier(s.clone()));
    }



  Err(())
}


pub fn
read_member(dir: &Directory)-> Result<Member,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  s = id.clone();

      cur.advance(2);

        if let Some(subdir) = cur.get_directory_with_name("type_note")
        {
            if let Ok(nt) = read_type_note(&subdir)
            {
              return Ok(Member{name: s, type_note: nt, offset: None});
            }
        }
    }


  Err(())
}


pub fn
read_member_list(dir: &Directory)-> Result<Struct,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut st = Struct::new();

    while let Some(subdir) = cur.seek_directory_with_name("member")
    {
        if let Ok(m) = read_member(&subdir)
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
read_type_note_list(dir: &Directory)-> Result<Vec<TypeNote>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<TypeNote> = Vec::new();

    while let Some(subdir) = cur.seek_directory_with_name("type_note")
    {
        if let Ok(nt) = read_type_note(&subdir)
        {
          ls.push(nt);

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
read_tuple(dir: &Directory)-> Result<TypeNote,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

  let  mut st = Struct::new();

    if let Some(subdir) = cur.get_directory_with_name("type_note_list")
    {
        if let Ok(ls) = read_type_note_list(&subdir)
        {
            for ty in ls
            {
              st.add("",ty);
            }
        }

      else
        {
          return Err(());
        }
    }


  Ok(TypeNote::Struct(Box::new(st)))
}


pub fn
read_function_pointer(dir: &Directory)-> Result<TypeNote,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

  let  mut fnsig = FunctionSignature::new();

    if let Some(subdir) = cur.get_directory_with_name("type_note_list")
    {
        if let Ok(ls) = read_type_note_list(&subdir)
        {
            for ty in ls
            {
              fnsig.parameter.add("",ty);
            }


          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


    if let Some(subdir) = cur.seek_directory_with_name("type_note")
    {
        if let Ok(ty) = read_type_note(&subdir)
        {
          fnsig.return_type_note = ty;
        }

      else
        {
          return Err(());
        }
    }


  Ok(TypeNote::FunctionPointer(Box::new(fnsig)))
}


pub fn
read_primitive(dir: &Directory)-> Result<TypeNote,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(s) = cur.get_keyword()
    {
           if s == "bool" {return Ok(TypeNote::Bool);}
      else if s == "void" {return Ok(TypeNote::Void);}
      else if s == "i8"   {return Ok(TypeNote::I8);}
      else if s == "i16"  {return Ok(TypeNote::I16);}
      else if s == "i32"  {return Ok(TypeNote::I32);}
      else if s == "i64"  {return Ok(TypeNote::I64);}
      else if s == "isize"{return Ok(TypeNote::ISize);}
      else if s == "u8"   {return Ok(TypeNote::U8);}
      else if s == "u16"  {return Ok(TypeNote::U16);}
      else if s == "u32"  {return Ok(TypeNote::U32);}
      else if s == "u64"  {return Ok(TypeNote::U64);}
      else if s == "usize"{return Ok(TypeNote::USize);}
      else if s == "f32"  {return Ok(TypeNote::F32);}
      else if s == "f64"  {return Ok(TypeNote::F64);}
    }


  Err(())
}




