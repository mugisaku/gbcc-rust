

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::Expression;
use crate::language::expression::read_expression::read_expression;


use super::{
  Block, ConditionalBlock, Statement, Program,
};


use crate::language::statement::read_declaration::{
  read_fn,
  read_struct,
  read_union,
  read_enum,
  read_alias,
  read_var,
  read_static,
  read_const,
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
read_else_if(dir: &Directory)-> Result<ConditionalBlock,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
      return read_conditional_block(blk_d);
    }


  Err(())
}


pub fn
read_else(dir: &Directory)-> Result<Block,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("block")
    {
      return read_block(blk_d);
    }


  Err(())
}


pub fn
read_if(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
        if let Ok(top_blk) = read_conditional_block(blk_d)
        {
          cur.advance(1);

          let  mut elif_ls: Vec<ConditionalBlock> = Vec::new();
          let  mut  el_opt: Option<Block> = None;

            while let Some(elif_d) = cur.seek_directory_with_name("else_if")
            {
                if let Ok(elif) = read_else_if(elif_d)
                {
                  elif_ls.push(elif);

                  cur.advance(1);
                }
            }


            if let Some(el_d) = cur.seek_directory_with_name("else")
            {
                if let Ok(el) = read_else(el_d)
                {
                  el_opt = Some(el);
                }
            }


          return Ok(Statement::If(top_blk,elif_ls,el_opt));
        }
    }


  Err(())
}


pub fn
read_while(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
        if let Ok(blk) = read_conditional_block(blk_d)
        {
          return Ok(Statement::While(blk));
        }
    }


  Err(())
}


pub fn
read_loop(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("block")
    {
        if let Ok(blk) = read_block(blk_d)
        {
          return Ok(Statement::Loop(blk));
        }
    }


  Err(())
}


pub fn
read_conditional_block(dir: &Directory)-> Result<ConditionalBlock,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(blk_d) = cur.get_directory_with_name("block")
            {
                if let Ok(blk) = read_block(blk_d)
                {
                  return Ok(ConditionalBlock{expression: e, block: blk});
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


  Ok(Block{statement_list: stmts})
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
          return read_if(d);
        }

      else
        if d_name == "while"
        {
          return read_while(d);
        }

      else
        if d_name == "loop"
        {
          return read_loop(d);
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
            if let Ok(decl) = read_fn(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "var"
        {
            if let Ok(decl) = read_var(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "static"
        {
            if let Ok(decl) = read_static(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "const"
        {
            if let Ok(decl) = read_const(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "struct"
        {
            if let Ok(decl) = read_struct(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "union"
        {
            if let Ok(decl) = read_union(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "enum"
        {
            if let Ok(decl) = read_enum(d)
            {
              return Ok(Statement::Declaration(decl));
            }
        }

      else
        if d_name == "alias"
        {
            if let Ok(decl) = read_alias(d)
            {
              return Ok(Statement::Declaration(decl));
            }
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




