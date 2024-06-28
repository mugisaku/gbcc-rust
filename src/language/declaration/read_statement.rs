

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};

use crate::language::expression::{
  Expression,

};

use crate::language::declaration::read_expression::read_expression;


use crate::language::statement::{
  ConditionalBlock, Statement, AssignOperator,
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
                          return Ok(Statement::Expression(e,Some((o,r))));
                        }
                    }
                }

              else
                {
                  return Ok(Statement::Expression(e,None));
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
          return Ok(Statement::Return(e));
        }
    }


  Ok(Statement::Return(Expression::None))
}


pub fn
read_if(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut cond_blk_ls: Vec<ConditionalBlock> = Vec::new();

  cur.advance(1);

    while let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(condition) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(ls_d) = cur.get_directory_with_name("statement_list")
            {
                if let Ok(statement_list) = read_statement_list(ls_d)
                {
                  cond_blk_ls.push(ConditionalBlock{condition, statement_list});

                  cur.advance(1);

                    if cur.test_keyword("else")
                    {
                      cur.advance(1);

                        if cur.test_keyword("if")
                        {
                          cur.advance(1);

                          continue;
                        }

                      else
                        if let Some(ls_d) = cur.get_directory_with_name("statement_list")
                        {
                            if let Ok(statement_list) = read_statement_list(ls_d)
                            {
                              cond_blk_ls.push(ConditionalBlock{condition: Expression::None, statement_list});
                            }
                        }
                    }


                  return Ok(Statement::If(cond_blk_ls));
                }

              else
                {
                  break;
                }
            }
        }

      else
        {
          break;
        }
    }


  Err(())
}


pub fn
read_while(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(condition) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(ls_d) = cur.get_directory_with_name("statement_list")
            {
                if let Ok(statement_list) = read_statement_list(ls_d)
                {
                  return Ok(Statement::While(ConditionalBlock{condition, statement_list}));
                }
            }
        }
    }


  Err(())
}


pub fn
read_loop(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(ls_d) = cur.get_directory_with_name("statement_list")
    {
        if let Ok(ls) = read_statement_list(ls_d)
        {
          return Ok(Statement::Loop(ls));
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
read_conditional_block(dir: &Directory)-> Result<ConditionalBlock,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(cond_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(condition) = read_expression(cond_d)
        {
          cur.advance(1);

            if let Some(ls_d) = cur.get_directory_with_name("statement_list")
            {
                if let Ok(statement_list) = read_statement_list(ls_d)
                {
                  return Ok(ConditionalBlock{condition, statement_list});
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

        if d_name == "statement_list"
        {
            if let Ok(ls) = read_statement_list(d)
            {
              return Ok(Statement::Block(ls));
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




