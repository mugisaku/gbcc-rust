

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::{
  Expression,
  ExpressionKeeper,

};

use crate::language::declaration::read_expression::read_expression;


use crate::language::statement::{
  Block, Statement, AssignOperator,
};


use super::read_declaration::{
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
read_expression_or_assign(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(e_dir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(e_dir)
        {
          cur.advance(1);

            if let Some(o_dir) = cur.get_directory_with_name("assign_operator")
            {
                if let Ok(o) = read_assign_operator(o_dir)
                {
                  cur.advance(1);

                    if let Some(r_dir) = cur.get_directory_with_name("expression")
                    {
                        if let Ok(r) = read_expression(r_dir)
                        {
                          let  lk = ExpressionKeeper::new(e);
                          let  rk = ExpressionKeeper::new(r);

                          return Ok(Statement::Expression(lk,Some((o,rk))));
                        }
                    }
                }

              else
                {
                  return Ok(Statement::Expression(ExpressionKeeper::new(e),None));
                }
            }
        }
    }


  Err(())
}


pub fn
read_assign_operator(dir: &Directory)-> Result<AssignOperator,()>
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s ==   "="{return Ok(AssignOperator::Nop);}
      else if s ==  "+="{return Ok(AssignOperator::Add);}
      else if s ==  "-="{return Ok(AssignOperator::Sub);}
      else if s ==  "*="{return Ok(AssignOperator::Mul);}
      else if s ==  "/="{return Ok(AssignOperator::Div);}
      else if s ==  "%="{return Ok(AssignOperator::Rem);}
      else if s == "<<="{return Ok(AssignOperator::Shl);}
      else if s == ">>="{return Ok(AssignOperator::Shr);}
      else if s ==  "&="{return Ok(AssignOperator::And);}
      else if s ==  "|="{return Ok(AssignOperator::Or);}
      else if s ==  "^="{return Ok(AssignOperator::Xor);}
    }


  Err(())
}


pub fn
read_return(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.seek_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(d)
        {
          return Ok(Statement::Return(Some(ExpressionKeeper::new(e))));
        }
    }


  Ok(Statement::Return(None))
}


pub fn
read_if_list(dir: &Directory)-> Result<Block,()>
{
    if let Ok((first_expr,first_ls)) = read_some_conditional(dir,1)
    {
      let  mut cur = Cursor::new(dir);

      cur.advance(2);

      let  mut block_ls: Vec<Block> = Vec::new();

      block_ls.push(Block::If(ExpressionKeeper::new(first_expr),first_ls));

        while let Some(elif_d) = cur.seek_directory_with_name("else_if")
        {
            if let Ok((expr,ls)) = read_some_conditional(elif_d,2)
            {
              block_ls.push(Block::If(ExpressionKeeper::new(expr),ls));

              cur.advance(1);
            }

          else
            {
              return Err(());
            }
        }


        if let Some(el_d) = cur.seek_directory_with_name("else")
        {
            if let Ok(ls) = read_some_unconditional(el_d,1)
            {
              block_ls.push(Block::Plain(ls));
            }

          else
            {
              return Err(());
            }
        }


      return Ok(Block::IfList(block_ls));
    }


  Err(())
}


pub fn
read_some_conditional(dir: &Directory, skip_n: usize)-> Result<(Expression,Vec<Statement>),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(skip_n);

    if let Some(ls_d) = cur.get_directory_with_name("statement_list_with_condition")
    {
      return read_statement_list_with_condition(ls_d);
    }


  Err(())
}


pub fn
read_some_unconditional(dir: &Directory, skip_n: usize)-> Result<Vec<Statement>,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(skip_n);

    if let Some(ls_d) = cur.get_directory_with_name("statement_list")
    {
      return read_statement_list(ls_d);
    }


  Err(())
}


pub fn
read_for(dir: &Directory)-> Result<Block,()>
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

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "if_list"
        {
          return read_if_list(d);
        }

      else
        if d_name == "while"
        {
            if let Ok((expr,ls)) = read_some_conditional(d,1)
            {
              return Ok(Block::While(ExpressionKeeper::new(expr),ls));
            }
        }

      else
        if d_name == "loop"
        {
            if let Ok(ls) = read_some_unconditional(d,1)
            {
              return Ok(Block::Loop(ls));
            }
        }

      else
        if d_name == "for"
        {
          return read_for(d);
        }

      else
        if d_name == "statement_list"
        {
            if let Ok(ls) = read_statement_list(d)
            {
              return Ok(Block::Plain(ls));
            }
        }
    }


  Err(())
}


pub fn
read_statement_list_with_condition(dir: &Directory)-> Result<(Expression,Vec<Statement>),()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(expr) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(ls_d) = cur.get_directory_with_name("statement_list")
            {
                if let Ok(ls) = read_statement_list(ls_d)
                {
                  return Ok((expr,ls));
                }
            }
        }
    }


  Err(())
}


pub fn
read_statement_list(dir: &Directory)-> Result<Vec<Statement>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut stmts: Vec<Statement> = Vec::new();

  cur.advance(1);

    while let Some(d) = cur.get_directory()
    {
        if let Ok(stmt) = read_statement(d)
        {
          stmts.push(stmt);
        }


      cur.advance(1);
    }


  Ok(stmts)
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

        if d_name == "block"
        {
            if let Ok(blk) = read_block(d)
            {
              return Ok(Statement::Block(blk));
            }
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
        if d_name == "expression_or_assign"
        {
            if let Ok(st) = read_expression_or_assign(d)
            {
              return Ok(st);
            }
        }
    }


  Err(())
}




