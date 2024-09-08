

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  Path,
  Expression,

};

use crate::language::typesystem::{
  Ty,
  TySet,
  Field,

};


pub fn
read_type(dir: &Directory)-> Result<Ty,()>
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
      return read_function_port(&subdir);
    }

  else
    if let Some(s) = cur.get_identifier()
    {
      let  mut path = Path::new();

      path.identifier_list.push(s.clone());

      return Ok(Ty::External(path,None));
    }


  Err(())
}


pub fn
read_type_list(dir: &Directory)-> Result<Vec<Ty>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Ty> = Vec::new();

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
read_tuple(dir: &Directory)-> Result<Ty,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(subdir) = cur.get_directory_with_name("type_list")
    {
        if let Ok(ls) = read_type_list(&subdir)
        {
          let  mut set = TySet::new();

            for ti in ls
            {
              set.add(String::new(),ti);
            }


          return Ok(Ty::Tuple(set));
        }
    }


  Err(())
}


pub fn
read_function_port(dir: &Directory)-> Result<Ty,()>
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
                if let Ok(ty) = read_type(&subdir)
                {
                  return Ok(Ty::FunctionPort(Box::new(ty),ls,false));
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
read_primitive(dir: &Directory)-> Result<Ty,()>
{
  let  mut cur = Cursor::new(dir);
 
    if let Some(s) = cur.get_keyword()
    {
      let  mut path = Path::new();

      path.identifier_list.push(s.clone());

      return Ok(Ty::External(path,None));
    }


  Err(())
}




