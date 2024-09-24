

use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  UnaryOperator,
  BinaryOperator,
  AssignOperator,
  Expression,

};


use crate::language::dynamic_space::{
  Function,
  Block,
  Statement,
  Declaration,

};


use crate::language::dynamic_value::{
  Value,

};


pub fn
read_parameter_list(dir: &Directory)-> Result<Vec<String>,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<String> = Vec::new();

  cur.advance(1);

    while let Some(s) = cur.get_identifier()
    {
      ls.push(s.clone());

      cur.advance(2);
    }


  Ok(ls)
}

 
pub fn
read_fn(dir: &Directory)-> Result<(String,Function),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.get_directory_with_name("parameter_list")
        {
            if let Ok(parameter_list) = read_parameter_list(parals_d)
            {
              cur.advance(1);

                if let Some(stmts_d) = cur.seek_directory_with_name("statement_list")
                {
                    if let Ok(block) = read_block(stmts_d)
                    {
                      let  f = Function{parameter_list, block};

                      return Ok((name,f));
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_variable(dir: &Directory)-> Result<(String,Option<Expression>),()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_d)
            {
              return Ok((name,Some(e)));
            }
        }


      return Ok((name,None));
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
            if let Ok((name,f)) = read_fn(d)
            {
              return Ok(Declaration::Fn(name,f));
            }
        }

      else
        if d_name == "let"
        {
            if let Ok((name,e_opt)) = read_variable(d)
            {
              return Ok(Declaration::Let(name,e_opt));
            }
        }

      else
        if d_name == "const"
        {
            if let Ok((name,e_opt)) = read_variable(d)
            {
                if let Some(e) = e_opt
                {
                  return Ok(Declaration::Const(name,e));
                }
            }
        }
    }


  Err(())
}




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
          return Ok(Statement::Return(Some(e)));
        }
    }


  Ok(Statement::Return(None))
}


pub fn
read_if(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut cond_blk_ls: Vec<(Expression,Block)> = Vec::new();

  let  mut else_blk_opt: Option<Block> = None;

  cur.advance(1);

    while let Some(expr_d) = cur.get_directory_with_name("expression")
    {
        if let Ok(condition) = read_expression(expr_d)
        {
          cur.advance(1);

            if let Some(ls_d) = cur.get_directory_with_name("statement_list")
            {
                if let Ok(blk) = read_block(ls_d)
                {
                  cur.advance(1);

                  cond_blk_ls.push((condition,blk));
                }
            }


            if cur.test_keyword("else")
            {
              cur.advance(1);

                if cur.test_keyword("if")
                {
                  cur.advance(1);
                }

              else
                if let Some(else_d) = cur.get_directory_with_name("statement_list")
                {
                    if let Ok(else_blk) = read_block(else_d)
                    {
                      else_blk_opt = Some(else_blk);

                      break;
                    }

                  else
                    {
                      return Err(());
                    }
                }
            }
        }

      else
        {
          return Err(());
        }
    }


  Ok(Statement::If(cond_blk_ls,else_blk_opt))
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
                if let Ok(blk) = read_block(ls_d)
                {
                  return Ok(Statement::While(condition,blk));
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
        if let Ok(blk) = read_block(ls_d)
        {
          return Ok(Statement::Loop(blk));
        }
    }


  Err(())
}


pub fn
read_for(dir: &Directory)-> Result<Statement,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      let  name = s.clone();

      cur.advance(2);

        if let Some(expr_d) = cur.get_directory_with_name("expression")
        {
            if let Ok(expr) = read_expression(expr_d)
            {
              cur.advance(1);

                if let Some(blk_d) = cur.get_directory_with_name("statement_list")
                {
                    if let Ok(blk) = read_block(blk_d)
                    {
                      return Ok(Statement::For(name,expr,blk));
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
read_print_s(dir: &Directory)-> Result<String,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_string()
    {
      return Ok(s.clone());
    }


  Err(())
}


pub fn
read_print_v(dir: &Directory)-> Result<String,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      return Ok(s.clone());
    }


  Err(())
}


pub fn
read_block(dir: &Directory)-> Result<Block,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut statement_list: Vec<Statement> = Vec::new();

  cur.advance(1);

    while let Some(d) = cur.get_directory()
    {
        if let Ok(stmt) = read_statement(d)
        {
          statement_list.push(stmt);
        }


      cur.advance(1);
    }


  Ok(Block{statement_list})
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
        if d_name == "for"
        {
          return read_for(d);
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
        if d_name == "let"
        {
            if let Ok((name,e_opt)) = read_variable(d)
            {
              return Ok(Statement::Let(name,e_opt));
            }
        }

      else
        if d_name == "const"
        {
            if let Ok((name,e_opt)) = read_variable(d)
            {
              return Ok(Statement::Const(name,e_opt));
            }
        }

      else
        if d_name == "print_s"
        {
            if let Ok(s) = read_print_s(d)
            {
              return Ok(Statement::PrintS(s));
            }
        }

      else
        if d_name == "print_v"
        {
            if let Ok(s) = read_print_v(d)
            {
              return Ok(Statement::PrintV(s));
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




pub fn
read_expression(dir: &Directory)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(o_dir) = cur.get_directory_with_name("operand")
    {
        if let Ok(mut e) = read_operand(o_dir)
        {
          cur.advance(1);

            while let Some(b_dir) = cur.get_directory_with_name("binary_operator")
            {
                if let Ok(b) = read_binary_operator(b_dir)
                {
                  cur.advance(1);

                    if let Some(next_o_dir) = cur.get_directory_with_name("operand")
                    {
                        if let Ok(next_e) = read_operand(next_o_dir)
                        {
                          cur.advance(1);

                          let  l = Box::new(     e);
                          let  r = Box::new(next_e);

                          e = Expression::Binary(b,l,r);
                        }
                    }

                  else
                    {
                      return Err(());
                    }
                }

              else
                {
                  return Err(());
                }
            }


          return Ok(e);
        }
    }


  Err(())
}




pub fn
read_unary_operator(dir: &Directory)-> Result<UnaryOperator,()>
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s == "~"{return Ok(UnaryOperator::Not);}
      else if s == "!"{return Ok(UnaryOperator::LogicalNot);}
      else if s == "-"{return Ok(UnaryOperator::Neg);}
      else if s == "*"{return Ok(UnaryOperator::Deref);}
    }


  Err(())
}


pub fn
read_binary_operator(dir: &Directory)-> Result<BinaryOperator,()>
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s ==  "+"{return Ok(BinaryOperator::Add);}
      else if s ==  "-"{return Ok(BinaryOperator::Sub);}
      else if s ==  "*"{return Ok(BinaryOperator::Mul);}
      else if s ==  "/"{return Ok(BinaryOperator::Div);}
      else if s ==  "%"{return Ok(BinaryOperator::Rem);}
      else if s == "<<"{return Ok(BinaryOperator::Shl);}
      else if s == ">>"{return Ok(BinaryOperator::Shr);}
      else if s ==  "&"{return Ok(BinaryOperator::And);}
      else if s ==  "|"{return Ok(BinaryOperator::Or);}
      else if s ==  "^"{return Ok(BinaryOperator::Xor);}
      else if s == "=="{return Ok(BinaryOperator::Eq);}
      else if s == "!="{return Ok(BinaryOperator::Neq);}
      else if s ==  "<"{return Ok(BinaryOperator::Lt);}
      else if s == "<="{return Ok(BinaryOperator::Lteq);}
      else if s ==  ">"{return Ok(BinaryOperator::Gt);}
      else if s == ">="{return Ok(BinaryOperator::Gteq);}
      else if s == "&&"{return Ok(BinaryOperator::LogicalAnd);}
      else if s == "||"{return Ok(BinaryOperator::LogicalOr);}
    }


  Err(())
}


pub fn
read_postfix_operator(dir: &Directory, e: Box<Expression>)-> Result<Expression,()>
{
  let  cur = Cursor::new(dir);

    if let Some(subdir) = cur.get_directory()
    {
      let  name = subdir.get_name();

           if name == "access"   {return read_access(subdir,e);}
      else if name == "subscript"{return read_subscript(subdir,e);}
      else if name == "call"     {return read_call(subdir,e);}
    }


  Err(())
}


pub fn
read_access(dir: &Directory, e: Box<Expression>)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(o) = cur.get_object()
    {
        if let ObjectData::Identifier(s) = o.get_data()
        {
          return Ok(Expression::Access(e,s.clone()));
        }
    }


  Err(())
}


pub fn
read_subscript(dir: &Directory, target_e: Box<Expression>)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(e_dir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(e_dir)
        {
          return Ok(Expression::Subscript(target_e,Box::new(e)));
        }
    }


  Err(())
}


pub fn
read_call(dir: &Directory, fe: Box<Expression>)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

  let  mut args: Vec<Expression> = Vec::new();

    if let Some(first_e_dir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(first_e_dir)
        {
          args.push(e);

          cur.advance(2);
        }

      else
        {
          return Err(());
        }


        while let Some(e_dir) = cur.get_directory_with_name("expression")
        {
            if let Ok(e) = read_expression(e_dir)
            {
              args.push(e);

              cur.advance(2);
            }

          else
            {
              return Err(());
            }
        }
    }


  Ok(Expression::Call(fe,args))
}


pub fn
read_operand_core(dir: &Directory)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      return Ok(Expression::Identifier(id.clone()));
    }

  else
    if let Some(o) = cur.get_object()
    {
        match o.get_data()
        {
      ObjectData::Integer(i)=>   {return Ok(Expression::Integer(*i));},
      ObjectData::Floating(f)=>  {return Ok(Expression::Floating(*f));},
      ObjectData::String(s)=>    {return Ok(Expression::String(s.clone()));},
      ObjectData::OthersString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_dir) = cur.get_directory_with_name("expression")
                  {
                      if let Ok(e) = read_expression(e_dir)
                      {
                        return Ok(Expression::SubExpression(Box::new(e)));
                      }
                  }
              }
          },
      _=>{},
        }
    }


  Err(())
}


pub fn
read_operand(dir: &Directory)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut un_ls: Vec<UnaryOperator> = Vec::new();

    while let Some(un_dir) = cur.get_directory_with_name("unary_operator")
    {
        if let Ok(pre) = read_unary_operator(un_dir)
        {
          cur.advance(1);

          un_ls.push(pre);
        }

      else
        {
          return Err(());
        }
    }


    if let Some(core_dir) = cur.get_directory_with_name("operand_core")
    {
        if let Ok(mut e) = read_operand_core(core_dir)
        {
          cur.advance(1);

            while let Some(post_dir) = cur.get_directory_with_name("postfix_operator")
            {
                if let Ok(new_e) = read_postfix_operator(post_dir,Box::new(e))
                {
                  cur.advance(1);

                  e = new_e;
                }

              else
                {
                  return Err(());
                }
            }


            while let Some(un) = un_ls.pop()
            {
              e = Expression::Unary(un,Box::new(e));
            }


          return Ok(e);
        }
    }


  Err(())
}




