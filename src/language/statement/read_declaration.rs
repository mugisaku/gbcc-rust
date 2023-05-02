

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::Expression;
use crate::language::expression::read_expression::read_expression;
use crate::language::typesystem::{
  TypeNote,
  r#struct::Struct,
  r#union::Union,
  r#enum::Enum,
  r#enum::Enumerator,
  r#enum::Value,
  function_signature::FunctionSignature,
  read_type_note::read_type_note,
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
read_parameter(dir: &Directory)-> Result<(String,TypeNote),()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type_note")
        {
            if let Ok(ty) = read_type_note(ty_d)
            {
              return Ok((name,ty));
            }
        }
    }


  Err(())
}


pub fn
read_parameter_list(dir: &Directory)-> Result<Vec<(String,TypeNote)>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<(String,TypeNote)> = Vec::new();

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
read_fn(dir: &Directory)-> Result<Statement,()>
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
            if let Ok(parals) = read_parameter_list(parals_d)
            {
                for para in parals
                {
                  fnsig.parameter.add(&para.0,para.1);
                }


              cur.advance(1);

                if let Some(ty_d) = cur.seek_directory_with_name("type_note")
                {
                    if let Ok(ty) = read_type_note(ty_d)
                    {
                      fnsig.return_type_note = ty;

                      cur.advance(1);
                    }
                }


                if let Some(blk_d) = cur.seek_directory_with_name("block")
                {
                    if let Ok(blk) = read_block(blk_d)
                    {
                      let  f = Fn{signature: fnsig, block: blk};

                      let  decl = Declaration::new(&name,Definition::Fn(f));

                      return Ok(Statement::Declaration(decl));
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_Var(dir: &Directory)-> Result<(String,Var),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      let  mut var = Var{type_note: TypeNote::Unspecified, expression_opt: None};

      cur.advance(1);

        if let Some(ty_d) = cur.seek_directory_with_name("type_note")
        {
            if let Ok(ty) = read_type_note(ty_d)
            {
              var.type_note = ty;
            }


          cur.advance(1);
        }


        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_d)
            {
              var.expression_opt = Some(e);
            }
        }


      return Ok((name,var));
    }


  Err(())
}


pub fn
read_var(dir: &Directory)-> Result<Statement,()>
{
    if let Ok((name,var)) = read_Var(dir)
    {
      let  def = Definition::Var(var);

      let  decl = Declaration::new(&name,def);

      return Ok(Statement::Declaration(decl));
    }


  Err(())
}


pub fn
read_static(dir: &Directory)-> Result<Statement,()>
{
    if let Ok((name,var)) = read_Var(dir)
    {
      let  def = Definition::Static(var);

      let  decl = Declaration::new(&name,def);

      return Ok(Statement::Declaration(decl));
    }


  Err(())
}


pub fn
read_const(dir: &Directory)-> Result<Statement,()>
{
    if let Ok((name,var)) = read_Var(dir)
    {
      let  def = Definition::Const(var);

      let  decl = Declaration::new(&name,def);

      return Ok(Statement::Declaration(decl));
    }


  Err(())
}


pub fn
read_struct(dir: &Directory)-> Result<Statement,()>
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
              let  st = Struct::from(ls);

              let  def = Definition::Struct(st);

              let  decl = Declaration::new(&name,def);

              return Ok(Statement::Declaration(decl));
            }
        }
    }


  Err(())
}


pub fn
read_union(dir: &Directory)-> Result<Statement,()>
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
              let  un = Union::from(ls);

              let  def = Definition::Union(un);

              let  decl = Declaration::new(&name,def);

              return Ok(Statement::Declaration(decl));
            }
        }
    }


  Err(())
}


pub fn
read_enumerator(dir: &Directory)-> Result<Enumerator,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

      let  mut en = Enumerator{name, value: Value::Unspecified};

        if let Some(expr_d) = cur.get_directory_with_name("expression")
        {
            if let Ok(expr) = read_expression(expr_d)
            {
              en.value = Value::Expression(expr);
            }
        }


      return Ok(en);
    }


  Err(())
}


pub fn
read_enumerator_list(dir: &Directory)-> Result<Vec<Enumerator>,()>
{
  let  mut cur = Cursor::new(dir);
  let  mut ls: Vec<Enumerator> = Vec::new();

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
read_enum(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.seek_directory_with_name("enumerator_list")
        {
            if let Ok(ls) = read_enumerator_list(ls_d)
            {
              let  en = Enum::from(ls);

              let  def = Definition::Enum(en);

              let  decl = Declaration::new(&name,def);

              return Ok(Statement::Declaration(decl));
            }
        }
    }


  Err(())
}


pub fn
read_alias(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(ty_d) = cur.get_directory_with_name("type_note")
        {
            if let Ok(ty) = read_type_note(ty_d)
            {
              let  def = Definition::Alias(ty);

              let  decl = Declaration::new(&name,def);

              return Ok(Statement::Declaration(decl));
            }
        }
    }


  Err(())
}




