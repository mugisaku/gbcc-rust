

use std::cell::Cell;

use crate::language::library::{
  ExpressionIndex,
  DeclarationIndex,
  StringIndex,
  Library
};

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::Expression;
use crate::language::expression::read_expression::read_expression;
use crate::language::typesystem::{
  Type,
  r#struct::Struct,
  r#enum::Enum,
  r#enum::Enumerator,
  r#enum::Value,
  read_type::read_type,
};


use super::{
  Definition,
  Declaration,
  Storage, Function, Block, Statement,
};


use crate::language::statement::read_statement::{
  read_block,
};


pub fn
read_parameter(dir: &Directory, lib: &mut Library)-> Result<DeclarationIndex,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d,lib)
            {
              let  def = Definition::Parameter(Storage{r#type: ty, expression_index_opt: None});

              let  decl = Declaration{name: name, definition: def};

              return Ok(lib.push_declaration(decl));
            }
        }
    }


  Err(())
}


pub fn
read_parameter_list(dir: &Directory, lib: &mut Library)-> Result<Vec<DeclarationIndex>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<DeclarationIndex> = Vec::new();

    while let Some(d) = cur.seek_directory_with_name("parameter")
    {
        if let Ok(para) = read_parameter(d,lib)
        {
          ls.push(para);

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
read_fn(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.get_directory_with_name("parameter_list")
        {
            if let Ok(para_ls) = read_parameter_list(parals_d,lib)
            {
              cur.advance(1);

              let  mut ret_ty = Type::Void;

                if let Some(ty_d) = cur.seek_directory_with_name("type")
                {
                    if let Ok(ty) = read_type(ty_d,lib)
                    {
                      ret_ty = ty;

                      cur.advance(1);
                    }
                }


                if let Some(blk_d) = cur.seek_directory_with_name("block")
                {
                    if let Ok(blk) = read_block(blk_d,lib)
                    {
                      let  bi = lib.push_block(blk);

                      let  f = Function{parameter_list: para_ls, return_type: ret_ty, block_index: bi};

                      let  decl = Declaration::new(&name,Definition::Fn(f));

                      return Ok(decl);
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_storage(dir: &Directory, lib: &mut Library)-> Result<(String,Storage),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      let  mut sto = Storage{r#type: Type::Void, expression_index_opt: None};

      cur.advance(1);

        if let Some(ty_d) = cur.seek_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d,lib)
            {
              sto.r#type = ty;
            }


          cur.advance(1);
        }


        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_d,lib)
            {
              sto.expression_index_opt = Some(lib.push_expression(e));
            }
        }


      return Ok((name,sto));
    }


  Err(())
}


pub fn
read_var(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir,lib)
    {
      let  def = Definition::Var(sto);

      let  decl = Declaration::new(&name,def);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_static(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir,lib)
    {
      let  def = Definition::Static(sto);

      let  decl = Declaration::new(&name,def);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_const(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir,lib)
    {
      let  def = Definition::Const(sto);

      let  decl = Declaration::new(&name,def);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_struct(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("member_list")
        {
/*
            if let Ok(ls) = read_parameter_list(ls_d,lib)
            {
              let  st = Struct::from(ls);

              let  def = Definition::Struct(st);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
            }
*/
        }
    }


  Err(())
}


pub fn
read_union(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("member_list")
        {
            if let Ok(ls) = read_parameter_list(ls_d,lib)
            {
/*
              let  un = Union::from(ls);

              let  def = Definition::Union(un);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
*/
            }
        }
    }


  Err(())
}


pub fn
read_enumerator(dir: &Directory, lib: &mut Library)-> Result<Enumerator,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

      let  mut en = Enumerator{name, value: Value::Unspecified};

        if let Some(expr_d) = cur.get_directory_with_name("expression")
        {
            if let Ok(expr) = read_expression(expr_d,lib)
            {
              en.value = Value::Expression(lib.push_expression(expr));
            }
        }


      return Ok(en);
    }


  Err(())
}


pub fn
read_enumerator_list(dir: &Directory, lib: &mut Library)-> Result<Vec<Enumerator>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<Enumerator> = Vec::new();

    while let Some(d) = cur.seek_directory_with_name("enumerator")
    {
        if let Ok(en) = read_enumerator(d,lib)
        {
          ls.push(en);

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
read_enum(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("enumerator_list")
        {
            if let Ok(ls) = read_enumerator_list(ls_d,lib)
            {
              let  en = Enum::from(ls);

              let  def = Definition::Enum(en);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
            }
        }
    }


  Err(())
}


pub fn
read_alias(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d,lib)
            {
              let  def = Definition::Alias(ty);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
            }
        }
    }


  Err(())
}


pub fn
read_declaration(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "fn"
        {
          return read_fn(d,lib);
        }

      else
        if d_name == "var"
        {
          return read_var(d,lib);
        }

      else
        if d_name == "static"
        {
          return read_static(d,lib);
        }

      else
        if d_name == "const"
        {
          return read_const(d,lib);
        }

      else
        if d_name == "struct"
        {
          return read_struct(d,lib);
        }

      else
        if d_name == "union"
        {
          return read_union(d,lib);
        }

      else
        if d_name == "enum"
        {
          return read_enum(d,lib);
        }

      else
        if d_name == "alias"
        {
          return read_alias(d,lib);
        }
    }


  Err(())
}




