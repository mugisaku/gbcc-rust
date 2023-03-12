

use crate::syntax::parser::Directory;
use crate::syntax::parser::ObjectData;
use crate::syntax::parser::Cursor;
use super::typesystem::TypeInfo;
use super::element::*;




pub struct
Source
{
  elements: Vec<Element>,

}


impl
Source
{


pub fn
from(dir: &Directory)-> Source
{
  let  mut src = Source{ elements: Vec::new()};

  src.read_expression(dir);

  src
}


pub fn
push_operand(&mut self, o: Operand)
{
  self.elements.push(Element::Operand(o));
}


pub fn
push_unary_operator(&mut self, o: UnaryOperator)
{
  self.elements.push(Element::Operator(Operator::Unary(o)));
}


pub fn
push_binary_operator(&mut self, o: BinaryOperator)
{
  self.elements.push(Element::Operator(Operator::Binary(o)));
}


pub fn
push_assign_operator(&mut self, o: AssignOperator)
{
  self.elements.push(Element::Operator(Operator::Assign(o)));
}


pub fn
push_primary_operator(&mut self, o: PrimaryOperator)
{
  self.elements.push(Element::Operator(Operator::Primary(o)));
}


pub fn
read_expression(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    if let Some(dir) = cur.seek_directory("unary_operation")
    {
        if self.read_unary_operation(dir).is_ok()
        {
            while cur.advance(1)
            {
                if let Some(dir) = cur.get_directory_with_name("binary_operator")
                {
                    if self.read_binary_operator(dir).is_ok()
                    {
                      cur.advance(1);

                        if let Some(dir) = cur.seek_directory("unary_operation")
                        {
                            if self.read_unary_operation(dir).is_err()
                            {
                              return Err(());
                            }
                        }
                    }

                  else
                    {
                      return Err(());
                    }
                }
            }


          return Ok(());
        }
    }


  Err(())
}


pub fn
read_unary_operation(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    while let Some(dir) = cur.get_directory_with_name("unary_operator")
    {
        if self.read_unary_operator(dir).is_ok()
        {
          cur.advance(1);
        }

      else
        {
          return Err(());
        }
    }


    if let Some(dir) = cur.seek_directory("operand")
    {
        if self.read_operand(dir).is_ok()
        {
          cur.advance(1);

            while let Some(dir) = cur.get_directory_with_name("primary_operation")
            {
                if self.read_primary_operator(dir).is_ok()
                {
                  cur.advance(1);
                }

              else
                {
                  return Err(());
                }
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


  Ok(())
}


pub fn
read_unary_operator(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    if let Some(s) = cur.get_mark()
    {
           if s == "~"{self.push_unary_operator(UnaryOperator::Not);}
      else if s == "!"{self.push_unary_operator(UnaryOperator::LogicalNot);}
      else if s == "-"{self.push_unary_operator(UnaryOperator::Neg);}
      else if s == "*"{self.push_unary_operator(UnaryOperator::Dereference);}
      else if s == "&"{self.push_unary_operator(UnaryOperator::Address);}
      else
        {
          return Err(());
        }

      return Ok(());
    }


  Err(())
}


pub fn
read_binary_operator(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    if let Some(s) = cur.get_mark()
    {
           if s ==  "+"{self.push_binary_operator(BinaryOperator::Add);}
      else if s ==  "-"{self.push_binary_operator(BinaryOperator::Sub);}
      else if s ==  "*"{self.push_binary_operator(BinaryOperator::Mul);}
      else if s ==  "/"{self.push_binary_operator(BinaryOperator::Div);}
      else if s ==  "%"{self.push_binary_operator(BinaryOperator::Rem);}
      else if s == "<<"{self.push_binary_operator(BinaryOperator::Shl);}
      else if s == ">>"{self.push_binary_operator(BinaryOperator::Shr);}
      else if s ==  "&"{self.push_binary_operator(BinaryOperator::And);}
      else if s ==  "|"{self.push_binary_operator(BinaryOperator::Or);}
      else if s ==  "^"{self.push_binary_operator(BinaryOperator::Xor);}
      else if s == "=="{self.push_binary_operator(BinaryOperator::Eq);}
      else if s == "!="{self.push_binary_operator(BinaryOperator::Neq);}
      else if s ==  "<"{self.push_binary_operator(BinaryOperator::Lt);}
      else if s == "<="{self.push_binary_operator(BinaryOperator::Lteq);}
      else if s ==  ">"{self.push_binary_operator(BinaryOperator::Gt);}
      else if s == ">="{self.push_binary_operator(BinaryOperator::Gteq);}
      else if s == "&&"{self.push_binary_operator(BinaryOperator::LogicalAnd);}
      else if s == "||"{self.push_binary_operator(BinaryOperator::LogicalOr);}
      else if s ==   "="{self.push_assign_operator(AssignOperator::Assign);}
      else if s ==  "+="{self.push_assign_operator(AssignOperator::AddAssign);}
      else if s ==  "-="{self.push_assign_operator(AssignOperator::SubAssign);}
      else if s ==  "*="{self.push_assign_operator(AssignOperator::MulAssign);}
      else if s ==  "/="{self.push_assign_operator(AssignOperator::DivAssign);}
      else if s ==  "%="{self.push_assign_operator(AssignOperator::RemAssign);}
      else if s == "<<="{self.push_assign_operator(AssignOperator::ShlAssign);}
      else if s == ">>="{self.push_assign_operator(AssignOperator::ShrAssign);}
      else if s ==  "&="{self.push_assign_operator(AssignOperator::AndAssign);}
      else if s ==  "|="{self.push_assign_operator(AssignOperator::OrAssign);}
      else if s ==  "^="{self.push_assign_operator(AssignOperator::XorAssign);}
      else
        {
          return Err(());
        }

      return Ok(());
    }


  Err(())
}


pub fn
read_primary_operator(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    if let Some(codir) = cur.get_directory()
    {
      let  name = codir.get_name();

           if name == "access"   {self.read_access(&codir);}
      else if name == "subscript"{self.read_subscript(&codir);}
      else if name == "call"     {self.read_call(&codir);}
      else
        {
          return Err(());
        }

      return Ok(());
    }


  Err(())
}


pub fn
read_access(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(o) = cur.get()
    {
        if let ObjectData::Identifier(s) = o.get_data()
        {
          let  po = PrimaryOperator::Access(s.clone());

          self.push_primary_operator(po);

          return Ok(());
        }
    }


  Err(())
}


pub fn
read_subscript(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(edir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = Expression::build(&edir)
        {
          let  po = PrimaryOperator::Subscript(e);

          self.push_primary_operator(po);

          return Ok(());
        }
    }


  Err(())
}


pub fn
read_call(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

  let mut  args: Vec<Expression> = Vec::new();

    if let Some(edir) = cur.get_directory_with_name("expression")
    {
        if let Ok(e) = Expression::build(&edir)
        {
          args.push(e);
        }

      else
        {
          return Err(());
        }


        while cur.advance(2)
        {
            if let Some(eedir) = cur.get_directory_with_name("expression")
            {
                if let Ok(e) = Expression::build(&eedir)
                {
                  args.push(e);
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
    }

  else
    {
      return Err(());
    }


  let  po = PrimaryOperator::Call(args);

  self.push_primary_operator(po);

  Ok(())
}


pub fn
read_operand(&mut self, dir: &Directory)-> Result<(),()>
{
  let mut  cur = Cursor::from(dir);

    if let Some(o) = cur.get()
    {
        match o.get_data()
        {
      ObjectData::Integer(i)=>   {  self.push_operand(Operand::Integer(*i));  return Ok(());},
      ObjectData::Floating(f)=>  {  self.push_operand(Operand::Floating(*f));  return Ok(());},
      ObjectData::Character(c)=> {  self.push_operand(Operand::Character(*c));  return Ok(());},
      ObjectData::String(s)=>    {  self.push_operand(Operand::String(s.clone()));  return Ok(());},
      ObjectData::Identifier(s)=>{  self.push_operand(Operand::Identifier(s.clone()));  return Ok(());},
      ObjectData::Mark(s)=>
          {
              if **s == "("
              {
                cur.advance(1);

                  if let Some(edir) = cur.get_directory_with_name("expression")
                  {
                      if let Ok(e) = Expression::build(&edir)
                      {
                        self.push_operand(Operand::Expression(e));

                        return Ok(());
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
print(&self)
{
    for e in &self.elements
    {
      e.print();
    }
}


}




pub struct
PostfixNotation
{
  elements: Vec<Element>,

}


impl
PostfixNotation
{


pub fn
from(src: Source)-> PostfixNotation
{
  let mut  pn = PostfixNotation{ elements: Vec::new()};

  let mut  stack: Vec<Element> = Vec::new();

  let mut  assigned = false;

    for e in src.elements
    {
      pn.push_element(e,&mut assigned,&mut stack);
    }


    while let Some(e) = stack.pop()
    {
      pn.elements.push(e);
    }


  pn
}


pub fn
push_operator_element(&mut self, e: Element, stack: &mut Vec<Element>)
{
  let  pr = if let Element::Operator(o) = &e
            {
              o.get_priority()
            }

          else{0};


    while let Some(last_e) = stack.last()
    {
        if let Element::Operator(last_o) = last_e
        {
            if last_o.get_priority() >= pr
            {
                if let Some(popped) = stack.pop()
                {
                  self.elements.push(popped);
                }
            }

          else
            {
              break;
            }
        }

      else
        {
          break;
        }
    }


  stack.push(e);
}


pub fn
push_element(&mut self, e: Element, assigned: &mut bool, stack: &mut Vec<Element>)
{
    if let Element::Operand(_) = e
    {
      self.elements.push(e);
    }

  else
    {
        if let Element::Operator(o) = &e
        {
            if let Operator::Assign(_) = o
            {
                if !*assigned
                {
                  *assigned = true;
                }

              else
                {
                  print!("do not multiple assign in one expression.");
                }
            }
        }


      self.push_operator_element(e,stack);
    }
}


}




pub struct
UnaryOperation
{
  operator: UnaryOperator,

  operand: Expression,

}


impl
UnaryOperation
{


pub fn  get_operator(&self)-> &UnaryOperator{&self.operator}
pub fn   get_operand(&self)-> &Expression{&self.operand}


}




pub struct
BinaryOperation
{
  operator: BinaryOperator,

   left_operand: Expression,
  right_operand: Expression,

}


impl
BinaryOperation
{


pub fn  get_operator(&self)-> &BinaryOperator{&self.operator}
pub fn   get_left_operand(&self)-> &Expression{&self.left_operand}
pub fn   get_right_operand(&self)-> &Expression{&self.right_operand}


}




pub struct
PrimaryOperation
{
  operator: PrimaryOperator,

  operand: Expression,

}


impl
PrimaryOperation
{


pub fn  get_operator(&self)-> &PrimaryOperator{&self.operator}
pub fn   get_operand(&self)-> &Expression{&self.operand}


}


pub struct
AssignOperation
{
  operator: AssignOperator,

   left_operand: Expression,
  right_operand: Expression,

}


impl
AssignOperation
{


pub fn  get_operator(&self)-> &AssignOperator{&self.operator}
pub fn   get_left_operand(&self)-> &Expression{&self.left_operand}
pub fn   get_right_operand(&self)-> &Expression{&self.right_operand}


}


pub enum
Expression
{
  Empty,
  Operand(Box<Operand>),
  Unary(Box<UnaryOperation>),
  Binary(Box<BinaryOperation>),
  Primary(Box<PrimaryOperation>),
  Assign(Box<AssignOperation>),

}


impl
Expression
{


pub fn
combine(buf: &mut Vec<Expression>, o: Operator)-> Result<(),&str>
{
    match o
    {
  Operator::Unary(u)=>
        {
            if let Some(operand) = buf.pop()
            {
              let  unop = Box::new(UnaryOperation{ operator: u, operand});

              buf.push(Expression::Unary(unop));

              return Ok(());
            }


          return Err("no operand for unary operation");
        },
  Operator::Binary(b)=>
        {
            if let Some(right) = buf.pop()
            {
                if let Some(left) = buf.pop()
                {
                  let  binop = Box::new(BinaryOperation{ operator: b, left_operand: left, right_operand: right});

                  buf.push(Expression::Binary(binop));

                  return Ok(());
                }


              return Err("no left operand for binary operation");
            }


          return Err("no right operand for binary operation");
        },
  Operator::Primary(p)=>
        {
            if let Some(operand) = buf.pop()
            {
              let  priop = Box::new(PrimaryOperation{ operator: p, operand});

              buf.push(Expression::Primary(priop));

              return Ok(());
            }


          return Err("no operand for primary operation");
        },
  Operator::Assign(a)=>
        {
            if let Some(right) = buf.pop()
            {
                if let Some(left) = buf.pop()
                {
                  let  assop = Box::new(AssignOperation{ operator: a, left_operand: left, right_operand: right});

                  buf.push(Expression::Assign(assop));

                  return Ok(());
                }


              return Err("no left operand for binary operation");
            }


          return Err("no right operand for binary operation");
        },
    }
}


pub fn
build(dir: &Directory)-> Result<Expression,()>
{
  let  src = Source::from(dir);

  let mut  pnt = PostfixNotation::from(src);

  let mut  buf: Vec<Expression> = Vec::new();

    for e in pnt.elements
    {
        match e
        {
      Element::Operator(o)=>
            {
                if let Err(s) = Self::combine(&mut buf,o)
                {
                  print!("{}",s);

                  return Err(());
                }
            },
      Element::Operand(o)=>
            {
              buf.push(Expression::Operand(Box::new(o)));
            },
        }
    }


  let  l = buf.len();

    if l == 0
    {
      return Err(());
    }

  else
    if l == 1
    {
        if let Some(e) = buf.pop()
        {
          return Ok(e);
        }
    }


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  Expression::Empty=>{print!("");},
  Expression::Operand(o)=>{o.print();},
  Expression::Unary(u)=>
        {
          u.operator.print();

          u.operand.print();
        },
  Expression::Binary(b)=>
        {
          b.left_operand.print();

          b.operator.print();

          b.right_operand.print();
        },
  Expression::Primary(p)=>
        {
          p.operand.print();

          p.operator.print();
        },
  Expression::Assign(a)=>
        {
          a.left_operand.print();

          a.operator.print();

          a.right_operand.print();
        },
    }
}


}




