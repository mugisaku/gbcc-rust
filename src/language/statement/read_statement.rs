

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  Library
};
use crate::language::expression::Expression;
use crate::language::expression::read_expression::read_expression;


use super::{
  Block, BlockIndex, Statement,
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
read_return(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.seek_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(d,lib)
        {
          return Ok(Statement::Return(Some(lib.push_expression(e))));
        }
    }


  Ok(Statement::Return(None))
}


pub fn
read_else_if(dir: &Directory, lib: &mut Library)-> Result<(Expression,Block),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
      return read_conditional_block(blk_d,lib);
    }


  Err(())
}


pub fn
read_else(dir: &Directory, lib: &mut Library)-> Result<Block,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("block")
    {
      return read_block(blk_d,lib);
    }


  Err(())
}


pub fn
read_if(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
        if let Ok((top_e,top_b)) = read_conditional_block(blk_d,lib)
        {
          cur.advance(1);

          let  mut elif_ls: Vec<(ExpressionIndex,BlockIndex)> = Vec::new();
          let  mut  el_opt: Option<BlockIndex> = None;

            while let Some(elif_d) = cur.seek_directory_with_name("else_if")
            {
                if let Ok((e,b)) = read_else_if(elif_d,lib)
                {
                  let  ei = lib.push_expression(e);
                  let  bi = lib.push_block(b);

                  elif_ls.push((ei,bi));

                  cur.advance(1);
                }
            }


            if let Some(el_d) = cur.seek_directory_with_name("else")
            {
                if let Ok(el_blk) = read_else(el_d,lib)
                {
                  el_opt = Some(lib.push_block(el_blk));
                }
            }


          let  ei = lib.push_expression(top_e);
          let  bi = lib.push_block(top_b);

          return Ok(Statement::If((ei,bi),elif_ls,el_opt));
        }
    }


  Err(())
}


pub fn
read_while(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("conditional_block")
    {
        if let Ok((e,b)) = read_conditional_block(blk_d,lib)
        {
          let  ei = lib.push_expression(e);
          let  bi = lib.push_block(b);

          return Ok(Statement::While((ei,bi)));
        }
    }


  Err(())
}


pub fn
read_loop(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("block")
    {
        if let Ok(blk) = read_block(blk_d,lib)
        {
          return Ok(Statement::Loop(lib.push_block(blk)));
        }
    }


  Err(())
}


pub fn
read_conditional_block(dir: &Directory, lib: &mut Library)-> Result<(Expression,Block),()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(expr_d,lib)
        {
          cur.advance(1);

            if let Some(blk_d) = cur.get_directory_with_name("block")
            {
                if let Ok(blk) = read_block(blk_d,lib)
                {
                  return Ok((e,blk));
                }
            }
        }
    }


  Err(())
}


pub fn
read_for(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
    }


  Err(())
}


pub fn
read_block(dir: &Directory, lib: &mut Library)-> Result<Block,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut stmts: Vec<Statement> = Vec::new();

  cur.advance(1);

    while let Some(d) = cur.get_directory()
    {
        if let Ok(stmt) = read_statement(d,lib)
        {
          stmts.push(stmt);
        }


      cur.advance(1);
    }


  Ok(Block{parent_block_index_opt: None, statement_list: stmts})
}


pub fn
read_statement(dir: &Directory, lib: &mut Library)-> Result<Statement,()>
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

        if d_name == "block"
        {
            if let Ok(blk) = read_block(d,lib)
            {
              return Ok(Statement::Block(lib.push_block(blk)));
            }
        }

      else
        if d_name == "if"
        {
          return read_if(d,lib);
        }

      else
        if d_name == "while"
        {
          return read_while(d,lib);
        }

      else
        if d_name == "loop"
        {
          return read_loop(d,lib);
        }

      else
        if d_name == "for"
        {
          return read_for(d,lib);
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
          return read_return(d,lib);
        }

      else
        if d_name == "fn"
        {
            if let Ok(decl) = read_fn(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "var"
        {
            if let Ok(decl) = read_var(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "static"
        {
            if let Ok(decl) = read_static(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "const"
        {
            if let Ok(decl) = read_const(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "struct"
        {
            if let Ok(decl) = read_struct(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "union"
        {
            if let Ok(decl) = read_union(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "enum"
        {
            if let Ok(decl) = read_enum(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "alias"
        {
            if let Ok(decl) = read_alias(d,lib)
            {
              let  di = lib.push_declaration(decl);

              return Ok(Statement::Declaration(di));
            }
        }

      else
        if d_name == "expression"
        {
            if let Ok(e) = crate::language::expression::read_expression::read_expression(d,lib)
            {
              return Ok(Statement::Expression(lib.push_expression(e)));
            }
        }
    }


  Err(())
}




