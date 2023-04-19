

use crate::syntax::{
  Directory,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  OperandCore,
  Operand,
  PrefixOperator,
  PostfixOperator,
  BinaryOperator,
  Operator,
  ExpressionTail,
  Expression,
};


//use super::make_expression::make_expression;



pub fn
read_expression_tail(dir: &Directory)-> Result<ExpressionTail,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(b_dir) = cur.get_directory_with_name("binary_operator")
    {
        if let Ok(b) = read_binary_operator(b_dir)
        {
          cur.advance(1);

            if let Some(o_dir) = cur.get_directory_with_name("operand")
            {
                if let Ok(o) = read_operand(o_dir)
                {
                  return Ok(ExpressionTail{operator: b, operand: o});
                }
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
        if let Ok(o) = read_operand(o_dir)
        {
          let  mut tail_ls: Vec<ExpressionTail> = Vec::new();

          cur.advance(1);

            while let Some(tail_dir) = cur.get_directory_with_name("expression_tail")
            {
                if let Ok(tail) = read_expression_tail(tail_dir)
                {
                  tail_ls.push(tail);

                  cur.advance(1);
                }

              else
                {
                  return Err(());
                }
            }


          return Ok(Expression{operand: o, tail_list: tail_ls});
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
      else if s ==   "="{return Ok(BinaryOperator::Assign);}
      else if s ==  "+="{return Ok(BinaryOperator::AddAssign);}
      else if s ==  "-="{return Ok(BinaryOperator::SubAssign);}
      else if s ==  "*="{return Ok(BinaryOperator::MulAssign);}
      else if s ==  "/="{return Ok(BinaryOperator::DivAssign);}
      else if s ==  "%="{return Ok(BinaryOperator::RemAssign);}
      else if s == "<<="{return Ok(BinaryOperator::ShlAssign);}
      else if s == ">>="{return Ok(BinaryOperator::ShrAssign);}
      else if s ==  "&="{return Ok(BinaryOperator::AndAssign);}
      else if s ==  "|="{return Ok(BinaryOperator::OrAssign);}
      else if s ==  "^="{return Ok(BinaryOperator::XorAssign);}
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
      else if name == "name_qresolution"{return read_name_resolution(subdir);}
      else if name == "increment"       {return Ok(PostfixOperator::Increment);}
      else if name == "decrement"       {return Ok(PostfixOperator::Increment);}
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
          let  boxed = Box::new(e);

          return Ok(PostfixOperator::Subscript(boxed));
        }
    }


  Err(())
}


pub fn
read_call(dir: &Directory)-> Result<PostfixOperator,()>
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


  Ok(PostfixOperator::Call(args))
}


pub fn
read_name_resolution(dir: &Directory)-> Result<PostfixOperator,()>
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      return Ok(PostfixOperator::NameResolution(s.clone()));
    }


  Err(())
}


pub fn
read_operand_core(dir: &Directory)-> Result<OperandCore,()>
{
  let  mut cur = Cursor::new(dir);

    if let Some(o) = cur.get_object()
    {
        match o.get_data()
        {
      ObjectData::Integer(i)=>   {return Ok(OperandCore::Integer(*i));},
      ObjectData::Floating(f)=>  {return Ok(OperandCore::Floating(*f));},
      ObjectData::Character(c)=> {return Ok(OperandCore::Character(*c));},
      ObjectData::String(s)=>    {return Ok(OperandCore::String(s.clone()));},
      ObjectData::Identifier(s)=>{return Ok(OperandCore::Identifier(s.clone()));},
      ObjectData::OthersString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_dir) = cur.get_directory_with_name("expression")
                  {
                      if let Ok(e) = read_expression(e_dir)
                      {
                        let  boxed = Box::new(e);

                        return Ok(OperandCore::Expression(boxed));
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
read_operand(dir: &Directory)-> Result<Operand,()>
{
  let  mut cur = Cursor::new(dir);

  let  mut  pre_ls:  Vec<PrefixOperator> = Vec::new();
  let  mut post_ls: Vec<PostfixOperator> = Vec::new();

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
        if let Ok(core) = read_operand_core(core_dir)
        {
          cur.advance(1);

            while let Some(post_dir) = cur.get_directory_with_name("postfix_operator")
            {
                if let Ok(post) = read_postfix_operator(post_dir)
                {
                  cur.advance(1);

                  post_ls.push(post);
                }

              else
                {
                  return Err(());
                }
            }


          return Ok(Operand{prefix_operator_list: pre_ls, core, postfix_operator_list: post_ls});
        }
    }


  Err(())
}




