

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  PrefixOperator,
  PostfixOperator,
  BinaryOperator,
  Path,
  Expression,
  ExpressionKeeper,
  StringKeeper,

};




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

                          let  l = ExpressionKeeper::new(     e);
                          let  r = ExpressionKeeper::new(next_e);

                          e = Expression::BinaryOperation(b,l,r);
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
read_prefix_operator(dir: &Directory)-> Result<PrefixOperator,()>
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s == "~"{return Ok(PrefixOperator::Not);}
      else if s == "!"{return Ok(PrefixOperator::LogicalNot);}
      else if s == "-"{return Ok(PrefixOperator::Neg);}
      else if s == "*"{return Ok(PrefixOperator::Dereference);}
      else if s == "&"{return Ok(PrefixOperator::Address);}
      else if s == "++"{return Ok(PrefixOperator::Increment);}
      else if s == "--"{return Ok(PrefixOperator::Decrement);}
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
read_postfix_operator(dir: &Directory)-> Result<PostfixOperator,()>
{
  let  cur = Cursor::new(dir);

    if let Some(subdir) = cur.get_directory()
    {
      let  name = subdir.get_name();

           if name == "access"   {return read_access(subdir);}
      else if name == "subscript"{return read_subscript(subdir);}
      else if name == "call"     {return read_call(subdir);}
      else if name == "increment"{return Ok(PostfixOperator::Increment);}
      else if name == "decrement"{return Ok(PostfixOperator::Decrement);}
    }


  Err(())
}


pub fn
read_access(dir: &Directory)-> Result<PostfixOperator,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(o) = cur.get_object()
    {
        if let ObjectData::Identifier(s) = o.get_data()
        {
          return Ok(PostfixOperator::Access(s.clone()));
        }
    }


  Err(())
}


pub fn
read_subscript(dir: &Directory)-> Result<PostfixOperator,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(e_dir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(e_dir)
        {
          return Ok(PostfixOperator::Subscript(ExpressionKeeper::new(e)));
        }
    }


  Err(())
}


pub fn
read_call(dir: &Directory)-> Result<PostfixOperator,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

  let  mut args: Vec<ExpressionKeeper> = Vec::new();

    if let Some(first_e_dir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = read_expression(first_e_dir)
        {
          args.push(ExpressionKeeper::new(e));

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
              args.push(ExpressionKeeper::new(e));

              cur.advance(2);
            }

          else
            {
              return Err(());
            }
        }
    }


  Ok(PostfixOperator::Call(args))
}


pub fn
read_path(dir: &Directory)-> Result<Path,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut path = Path::new();

    while let Some(id) = cur.get_identifier()
    {
      path.identifier_list.push(id.clone());

      cur.advance(2);
    }


    if path.identifier_list.len() != 0
    {
      return Ok(path);
    }


  Err(())
}


pub fn
read_operand_core(dir: &Directory)-> Result<Expression,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(path_d) = cur.get_directory_with_name("path")
    {
        if let Ok(path) = read_path(path_d)
        {
          return Ok(Expression::Identifier(path));
        }
    }

  else
    if let Some(o) = cur.get_object()
    {
        match o.get_data()
        {
      ObjectData::Integer(i)=>   {return Ok(Expression::Integer(*i));},
      ObjectData::Floating(f)=>  {return Ok(Expression::Floating(*f));},
      ObjectData::Character(c)=> {return Ok(Expression::Character(*c));},
      ObjectData::String(s)=>    {return Ok(Expression::String(StringKeeper::new(s.as_str())));},
      ObjectData::OthersString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_dir) = cur.get_directory_with_name("expression")
                  {
                      if let Ok(e) = read_expression(e_dir)
                      {
                        return Ok(Expression::SubExpression(ExpressionKeeper::new(e)));
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

  let  mut pre_ls: Vec<PrefixOperator> = Vec::new();

    while let Some(pre_dir) = cur.get_directory_with_name("prefix_operator")
    {
        if let Ok(pre) = read_prefix_operator(pre_dir)
        {
          cur.advance(1);

          pre_ls.push(pre);
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
                if let Ok(post) = read_postfix_operator(post_dir)
                {
                  cur.advance(1);

                  let  ek = ExpressionKeeper::new(e);

                  e = Expression::PostfixOperation(post,ek);
                }

              else
                {
                  return Err(());
                }
            }


            while let Some(pre) = pre_ls.pop()
            {
              let  ek = ExpressionKeeper::new(e);

              e = Expression::PrefixOperation(pre,ek);
            }


          return Ok(e);
        }
    }


  Err(())
}




