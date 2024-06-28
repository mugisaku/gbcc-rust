

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::{
  Expression,

};

use super::read_expression;
use super::read_type;
use crate::language::typesystem::{
  TypeInfo,

};


use super::{
  Component,
  Declaration,
  Storage,
  Function,

};


use crate::language::statement::{
  Statement,

};


use super::{
  read_type::read_type,
  read_expression::read_expression,
  read_statement::read_statement_list,

};


pub fn
read_parameter(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d)
            {
              let  com = Component::Type(ty);

              return Ok(Declaration::new(name,com));
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

    while let Some(d) = cur.seek_directory_with_name("parameter")
    {
        if let Ok(para) = read_parameter(d)
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
read_fn(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.get_directory_with_name("parameter_list")
        {
            if let Ok(para_ls) = read_parameter_list(parals_d)
            {
              cur.advance(1);

              let  mut ret_ty = TypeInfo::new_void();

                if let Some(ty_d) = cur.seek_directory_with_name("type")
                {
                    if let Ok(ty) = read_type(ty_d)
                    {
                      ret_ty = ty;

                      cur.advance(1);
                    }
                }


                if let Some(stmts_d) = cur.seek_directory_with_name("statement_list")
                {
                    if let Ok(stmts) = read_statement_list(stmts_d)
                    {
                      let  f = Function::new()
                              .set_parameter_space(para_ls)
                              .set_return_type_info(ret_ty)
                              .set_statement_list(stmts)
                              ;


                      let  decl = Declaration::new(name,Component::Fn(f));

                      return Ok(decl);
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_storage(dir: &Directory)-> Result<(String,Storage),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      let  mut sto = Storage{type_info: TypeInfo::new_void(), expression: Expression::None};

      cur.advance(1);

        if let Some(ty_d) = cur.seek_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d)
            {
              sto.type_info = ty;
            }


          cur.advance(1);
        }


        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_d)
            {
              sto.expression = e;
            }
        }


      return Ok((name,sto));
    }


  Err(())
}


pub fn
read_var(dir: &Directory)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir)
    {
      let  com = Component::Var(sto);

      let  decl = Declaration::new(name,com);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_static(dir: &Directory)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir)
    {
      let  com = Component::Static(sto);

      let  decl = Declaration::new(name,com);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_const(dir: &Directory)-> Result<Declaration,()>
{
    if let Ok((name,sto)) = read_storage(dir)
    {
      let  com = Component::Const(sto);

      let  decl = Declaration::new(name,com);

      return Ok(decl);
    }


  Err(())
}


pub fn
read_struct(dir: &Directory)-> Result<Declaration,()>
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
            if let Ok(ls) = read_parameter_list(ls_d)
            {
              let  st = Struct::from(ls);

              let  def = Component::Struct(st);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
            }
*/
        }
    }


  Err(())
}


pub fn
read_union(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("member_list")
        {
            if let Ok(ls) = read_parameter_list(ls_d)
            {
/*
              let  un = Union::from(ls);

              let  def = Component::Union(un);

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
*/
            }
        }
    }


  Err(())
}


pub fn
read_enumerator(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(2);

      let  com = Component::Enumerator(Expression::Integer(0));

      let  mut en = Declaration::new(name,com);

        if let Some(expr_d) = cur.get_directory_with_name("expression")
        {
            if let Ok(expr) = read_expression(expr_d)
            {
//              en.value = Value::Expression(expr);
            }
        }


      return Ok(en);
    }


  Err(())
}


pub fn
read_enumerator_list(dir: &Directory)-> Result<Vec<Declaration>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<Declaration> = Vec::new();

    while let Some(d) = cur.seek_directory_with_name("enumerator")
    {
        if let Ok(en) = read_enumerator(d)
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
read_enum(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("enumerator_list")
        {
            if let Ok(ls) = read_enumerator_list(ls_d)
            {
/*
              let  ti = TypeItem::Enum(ls);

              let  def = Component::Type(TypeItemNode::new(ti));

              let  decl = Declaration::new(&name,def);

              return Ok(decl);
*/
            }
        }
    }


  Err(())
}


pub fn
read_alias(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type")
        {
            if let Ok(ty) = read_type(ty_d)
            {
              let  com = Component::Type(TypeInfo::new_void());

              let  decl = Declaration::new(name,com);

              return Ok(decl);
            }
        }
    }


  Err(())
}


pub fn
read_declaration(dir: &Directory)-> Result<Declaration,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "fn"
        {
          return read_fn(d);
        }

      else
        if d_name == "var"
        {
          return read_var(d);
        }

      else
        if d_name == "static"
        {
          return read_static(d);
        }

      else
        if d_name == "const"
        {
          return read_const(d);
        }

      else
        if d_name == "struct"
        {
          return read_struct(d);
        }

      else
        if d_name == "union"
        {
          return read_union(d);
        }

      else
        if d_name == "enum"
        {
          return read_enum(d);
        }

      else
        if d_name == "alias"
        {
          return read_alias(d);
        }
    }


  Err(())
}




