

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
  function_signature::FunctionSignature,
  read_type_note::read_type_note,
};


use super::{
  Definition,
  Declaration,
  Var, Fn, Block, Statement, Program,
};


pub fn
read_return(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.seek_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(d)
        {
          return Ok(Statement::Return(Some(e)));
        }
    }


  Ok(Statement::Return(None))
}


pub fn
read_if(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(d) = cur.get_directory_with_name("block_statement")
    {
    }


  Ok(Statement::Return(None))
}


pub fn
read_while(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(blk_d) = cur.get_directory_with_name("block_statement")
            {
                if let Ok(blk) = read_block(blk_d)
                {
                  return Ok(Statement::While(blk));
                }
            }
        }
    }


  Err(())
}


pub fn
read_for(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
    }


  Err(())
}


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
      let  def = Definition::Struct(Struct::new());

      let  decl = Declaration::new(id,def);

      return Ok(Statement::Declaration(decl));
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
      let  def = Definition::Union(Union::new());

      let  decl = Declaration::new(id,def);

      return Ok(Statement::Declaration(decl));
    }


  Err(())
}


pub fn
read_enum(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  def = Definition::Enum(Enum::new());

      let  decl = Declaration::new(id,def);

      return Ok(Statement::Declaration(decl));
    }


  Err(())
}


pub fn
read_block(dir: &Directory)-> Result<Block,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut stmts: Vec<Statement> = Vec::new();

    while let Some(d) = cur.get_directory()
    {
        if let Ok(stmt) = read_statement(d)
        {
          stmts.push(stmt);

          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


  Ok(Block{ condition: None, statement_list: stmts})
}


pub fn
read_statement(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
        if s == ";"
        {
          return Ok(Statement::Empty);
        }
    }

  else
    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d.get_name() == "if"
        {
          return read_if(d);
        }

      else
        if d_name == "block"
        {
            if let Ok(blk) = read_block(d)
            {
              return Ok(Statement::Block(blk));
            }
        }

      else
        if d_name == "if"
        {
          return read_while(d);
        }

      else
        if d_name == "while"
        {
          return read_while(d);
        }

      else
        if d_name == "for"
        {
          return read_for(d);
        }

      else
        if d_name == "break"
        {
          return Ok(Statement::Break);
        }

      else
        if d_name == "continue"
        {
          return Ok(Statement::Continue);
        }

      else
        if d_name == "return"
        {
          return read_return(d);
        }

      else
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
        if d_name == "expression"
        {
            if let Ok(e) = crate::language::expression::read_expression::read_expression(d)
            {
              return Ok(Statement::Expression(e));
            }
        }
    }


  Err(())
}


pub fn
read_program(dir: &Directory)-> Result<Program,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<Statement> = Vec::new();

    while let Some(d) = cur.get_directory()
    {
        if let Ok(st) = read_statement(d)
        {
          ls.push(st);

          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


  Ok(Program{statement_list: ls})
}




