

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
read_fn(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("")
    {
    }


  Err(())
}


pub fn
read_var(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("")
    {
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

    while let Some(o) = cur.get_object()
    {
        if let ObjectData::Directory(d) = o.get_data()
        {
          let  d_name = d.get_name();

            if d_name == "statement"
            {
                if let Ok(stmt) = read_statement(d)
                {
                  stmts.push(stmt);
                }
            }

          else
            if d_name == "expression"
            {
                if let Ok(e) = read_expression(d)
                {
                  stmts.push(Statement::Expression(e));
                }
            }
        }

      cur.advance(1);
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




