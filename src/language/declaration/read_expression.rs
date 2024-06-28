

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  UnaryOperator,
  BinaryOperator,
  Path,
  Expression,

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




