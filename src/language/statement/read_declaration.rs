

use std::cell::Cell;

use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
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
  r#union::Union,
  r#enum::Enum,
  r#enum::Enumerator,
  r#enum::Value,
  function_signature::FunctionSignature,
  read_type::read_type,
};


use super::{
  Definition,
  Declaration,
  Var, Fn, Block, Statement,
};


use crate::language::statement::read_statement::{
  read_block,
};


pub fn
read_parameter(dir: &Directory, lib: &mut Library)-> Result<(String,TypeIndex),()>
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
              return Ok((name,lib.push_type(ty)));
            }
        }
    }


  Err(())
}


pub fn
read_parameter_list(dir: &Directory, lib: &mut Library)-> Result<Vec<(String,TypeIndex)>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<(String,TypeIndex)> = Vec::new();

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

      let  mut fnsig = FunctionSignature::new();

      cur.advance(1);

        if let Some(parals_d) = cur.get_directory_with_name("parameter_list")
        {
            if let Ok(parals) = read_parameter_list(parals_d,lib)
            {
              let  mut name_ls: Vec<String> = Vec::new();

                for para in parals
                {
                  name_ls.push(para.0);

                  fnsig.parameter_list.push(para.1);
                }


              cur.advance(1);

                if let Some(ty_d) = cur.seek_directory_with_name("type")
                {
                    if let Ok(ty) = read_type(ty_d,lib)
                    {
                      fnsig.return_type_index = lib.push_type(ty);

                      cur.advance(1);
                    }
                }


                if let Some(blk_d) = cur.seek_directory_with_name("block")
                {
                  let  _ = lib.open_space(name.as_str());

                    if let Ok(blk) = read_block(blk_d,lib)
                    {
                      let  f = Fn{signature: fnsig, parameter_name_list: name_ls, block: blk};

                      let  decl = Declaration::new(&name,Definition::Fn(f));

                      let  _ = lib.close_space();

                      return Ok(decl);
                    }

                  else
                    {
                      let  _ = lib.close_space();
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_Var(dir: &Directory, lib: &mut Library)-> Result<(String,Var),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      let  mut var = Var{type_index: TypeIndex{value: 0}, expression_index_opt: None};

      cur.advance(1);

        if let Some(ty_d) = cur.seek_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d,lib)
            {
              var.type_index = lib.push_type(ty);
            }


          cur.advance(1);
        }


        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_d,lib)
            {
              var.expression_index_opt = Some(lib.push_expression(e));
            }
        }


      return Ok((name,var));
    }


  Err(())
}


pub fn
read_var(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,var)) = read_Var(dir,lib)
    {
      let  def = Definition::Var(var);

      let  decl = Declaration::new(&name,def);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_static(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,var)) = read_Var(dir,lib)
    {
      let  def = Definition::Static(var);

      let  decl = Declaration::new(&name,def);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_const(dir: &Directory, lib: &mut Library)-> Result<Declaration,()>
{
    if let Ok((name,var)) = read_Var(dir,lib)
    {
      let  def = Definition::Const(var);

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




