

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  Path,
  Expression,

};

use crate::language::declaration::{
  Component,
  Declaration,

};

use super::{
  TypeInfo,

};


pub fn
read_type(dir: &Directory)-> Result<TypeInfo,()>
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
    if let Some(subdir) = cur.get_directory_with_name("function_reference")
    {
      return read_function_reference(&subdir);
    }

  else
    if let Some(s) = cur.get_identifier()
    {
      let  mut path = Path::new();

      path.identifier_list.push(s.clone());

      return Ok(TypeInfo::new_external(path));
    }


  Err(())
}


pub fn
read_parameter(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  s = id.clone();

      cur.advance(2);

        if let Some(subdir) = cur.get_directory_with_name("type")
        {
            if let Ok(t) = read_type(&subdir)
            {
              let  com = Component::Type(t);

              return Ok(Declaration::new(s,com));
            }
        }
    }


  Err(())
}


pub fn
read_parameter_list(dir: &Directory)-> Result<Vec<Declaration>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Declaration> = Vec::new();

    while let Some(subdir) = cur.seek_directory_with_name("member")
    {
        if let Ok(m) = read_parameter(&subdir)
        {
          ls.push(m);

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
read_type_list(dir: &Directory)-> Result<Vec<TypeInfo>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<TypeInfo> = Vec::new();

    while let Some(subdir) = cur.seek_directory_with_name("type")
    {
        if let Ok(t) = read_type(&subdir)
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
read_tuple(dir: &Directory)-> Result<TypeInfo,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(subdir) = cur.get_directory_with_name("type_list")
    {
        if let Ok(ls) = read_type_list(&subdir)
        {
          return Ok(TypeInfo::new_tuple(ls));
        }
    }


  Err(())
}


pub fn
read_function_reference(dir: &Directory)-> Result<TypeInfo,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

    if let Some(subdir) = cur.get_directory_with_name("type_list")
    {
        if let Ok(ls) = read_type_list(&subdir)
        {
          cur.advance(1);

            if let Some(subdir) = cur.seek_directory_with_name("type")
            {
                if let Ok(ti) = read_type(&subdir)
                {
                  return Ok(TypeInfo::new_function_reference(ls,ti))
                }

              else
                {
                }
            }
        }

      else
        {
        }
    }


  Err(())
}


pub fn
read_primitive(dir: &Directory)-> Result<TypeInfo,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(s) = cur.get_keyword()
    {
      let  mut path = Path::new();

      path.identifier_list.push(s.clone());

      return Ok(TypeInfo::new_external(path));
    }


  Err(())
}




