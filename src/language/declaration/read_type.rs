

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use super::{
  TypeItem,
  TypeItemKeeper,
  Parameter,

};


pub fn
read_type(dir: &Directory)-> Result<TypeItem,()>
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
      return Ok(TypeItem::ByName(s.clone()));
    }


  Err(())
}


pub fn
read_parameter(dir: &Directory)-> Result<Parameter,()>
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
              return Ok(Parameter{name: s, type_item_keeper: TypeItemKeeper::new(t)});
            }
        }
    }


  Err(())
}


pub fn
read_parameter_list(dir: &Directory)-> Result<Vec<Parameter>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Parameter> = Vec::new();

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
read_type_list(dir: &Directory)-> Result<Vec<Parameter>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Parameter> = Vec::new();

    while let Some(subdir) = cur.seek_directory_with_name("type")
    {
        if let Ok(t) = read_type(&subdir)
        {
          ls.push(Parameter{name: String::new(), type_item_keeper: TypeItemKeeper::new(t)});

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
read_tuple(dir: &Directory)-> Result<TypeItem,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(subdir) = cur.get_directory_with_name("type_list")
    {
        if let Ok(ls) = read_type_list(&subdir)
        {
          return Ok(TypeItem::Tuple(ls));
        }
    }


  Err(())
}


pub fn
read_function_reference(dir: &Directory)-> Result<TypeItem,()>
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
                  return Ok(TypeItem::FunctionReference(TypeItemKeeper::new(ti),ls))
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
read_primitive(dir: &Directory)-> Result<TypeItem,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(s) = cur.get_keyword()
    {
           if s == "bool" {return Ok(TypeItem::Bool);}
      else if s == "char" {return Ok(TypeItem::Char);}
      else if s == "void" {return Ok(TypeItem::Void);}
      else if s == "i8"   {return Ok(TypeItem::I8);}
      else if s == "i16"  {return Ok(TypeItem::I16);}
      else if s == "i32"  {return Ok(TypeItem::I32);}
      else if s == "i64"  {return Ok(TypeItem::I64);}
      else if s == "isize"{return Ok(TypeItem::ISize);}
      else if s == "u8"   {return Ok(TypeItem::U8);}
      else if s == "u16"  {return Ok(TypeItem::U16);}
      else if s == "u32"  {return Ok(TypeItem::U32);}
      else if s == "u64"  {return Ok(TypeItem::U64);}
      else if s == "usize"{return Ok(TypeItem::USize);}
      else if s == "f32"  {return Ok(TypeItem::F32);}
      else if s == "f64"  {return Ok(TypeItem::F64);}
    }


  Err(())
}




