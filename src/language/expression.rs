

use crate::syntax::parser::Directory;
use crate::syntax::parser::ObjectData;
use crate::syntax::parser::Cursor;
use super::typesystem::TypeInfo;
use super::element::*;
use super::value::*;




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
read_expression(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

    if let Some(dir) = cur.seek_directory("unary_operation")
    {
      self.read_unary_operation(dir);

        while cur.advance(1)
        {
            if let Some(dir) = cur.get_directory_with_name("binary_operator")
            {
              self.read_binary_operator(dir);

              cur.advance(1);

                if let Some(dir) = cur.seek_directory("unary_operation")
                {
                  self.read_unary_operation(dir);
                }
            }
        }
    }
}


pub fn
read_unary_operation(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

    while let Some(dir) = cur.get_directory_with_name("unary_operator")
    {
      self.read_unary_operator(dir);

      cur.advance(1);
    }


    if let Some(dir) = cur.seek_directory("operand")
    {
      self.read_operand(dir);

      cur.advance(1);

        while let Some(dir) = cur.get_directory_with_name("primary_operation")
        {
          self.read_primary_operator(dir);

          cur.advance(1);
        }
    }
}


pub fn
read_unary_operator(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

    if let Some(s) = cur.get_mark()
    {
           if s == "~"{self.push_unary_operator(UnaryOperator::Not);}
      else if s == "!"{self.push_unary_operator(UnaryOperator::LogicalNot);}
      else if s == "-"{self.push_unary_operator(UnaryOperator::Neg);}
      else if s == "*"{self.push_unary_operator(UnaryOperator::Dereference);}
      else if s == "&"{self.push_unary_operator(UnaryOperator::Address);}
    }
}


pub fn
read_binary_operator(&mut self, dir: &Directory)
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
    }
}


pub fn
read_primary_operator(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

    if let Some(codir) = cur.get_directory()
    {
      let  name = codir.get_name();

           if name == "access"   {self.read_access(&codir);}
      else if name == "subscript"{self.read_subscript(&codir);}
      else if name == "call"     {self.read_call(&codir);}
    }
}


pub fn
read_access(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(o) = cur.get()
    {
        if let ObjectData::Identifier(s) = o.get_data()
        {
          let  po = PrimaryOperator::Access(s.clone());

          self.push_primary_operator(po);
        }
    }
}


pub fn
read_subscript(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(edir) = cur.get_directory_with_name("expression")
    {
      let  e = Expression::from(&edir);

      let  po = PrimaryOperator::Subscript(e);

      self.push_primary_operator(po);
    }
}


pub fn
read_call(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

  let mut  args: Vec<Expression> = Vec::new();

    if let Some(edir) = cur.get_directory_with_name("expression")
    {
      args.push(Expression::from(&edir));

        while cur.advance(2)
        {
            if let Some(eedir) = cur.get_directory_with_name("expression")
            {
              args.push(Expression::from(&eedir));
            }
        }
    }


  let  po = PrimaryOperator::Call(args);

  self.push_primary_operator(po);
}


pub fn
read_operand(&mut self, dir: &Directory)
{
  let mut  cur = Cursor::from(dir);

    if let Some(o) = cur.get()
    {
        match o.get_data()
        {
      ObjectData::Integer(i)=>   {self.push_operand(Operand::Integer(*i));},
      ObjectData::Floating(f)=>  {self.push_operand(Operand::Floating(*f));},
      ObjectData::Character(c)=> {self.push_operand(Operand::Character(*c));},
      ObjectData::String(s)=>    {self.push_operand(Operand::String(s.clone()));},
      ObjectData::Identifier(s)=>{self.push_operand(Operand::Identifier(s.clone()));},
      ObjectData::Mark(s)=>
          {
              if **s == "("
              {
                cur.advance(1);

                  if let Some(edir) = cur.get_directory_with_name("expression")
                  {
                    let  e = Expression::from(&edir);

                    self.push_operand(Operand::Expression(e));
                  }
              }
          },
      _=>{},
        }
    }
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


pub struct
BinaryOperation
{
  operator: BinaryOperator,

   left_operand: Expression,
  right_operand: Expression,

}


pub struct
PrimaryOperation
{
  operator: PrimaryOperator,

  operand: Expression,

}


pub struct
AssignOperation
{
  operator: AssignOperator,

   left_operand: Expression,
  right_operand: Expression,

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
from(dir: &Directory)-> Expression
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

                  return Expression::Empty;
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
      Expression::Empty;
    }

  else
    if l == 1
    {
        if let Some(e) = buf.pop()
        {
          return e;
        }
    }


  Expression::Empty
}


/*
pub fn
evaluate(&self)-> Value
{
  let mut  buf: Vec<Value> = Vec::new();

    for e in &self.elements
    {
        match e
        {
      Element::Operand(o)=>{buf.push(Value::from(o));}
      Element::Operator(o)=>{Self::operate(&mut buf,&o)},
        }
    }


    if buf.len() == 1
    {
        if let Some(v) = buf.pop()
        {
          return v;
        }
    }


  Value::Void
}


pub fn
operate(buf: &mut Vec<Value>, o: &Operator)
{
    match o
    {
  Operator::Unary(unop)=>
        {
            if let Some(v) = buf.pop()
            {
              buf.push(Self::operate_unary(&v,unop));
            }
        },
  Operator::Binary(binop)=>
        {
            if let Some(rv) = buf.pop()
            {
                if let Some(lv) = buf.pop()
                {
                  buf.push(Self::operate_binary(&lv,&rv,binop));
                }
            }
        },
  Operator::Primary(primop)=>
        {
            if let Some(v) = buf.pop()
            {
              buf.push(Self::operate_primary(&v,primop));
            }
        },
  Operator::Assign(asop)=>
        {
            if let Some(rv) = buf.pop()
            {
                if let Some(lv) = buf.pop()
                {
                  buf.push(Self::operate_assign(&lv,&rv,asop));
                }
            }
        },
    }
}


pub fn
operate_unary(v: &Value, unop: &UnaryOperator)-> Value
{
    match unop
    {
  UnaryOperator::Not=>{return Value::not(v);},
  UnaryOperator::LogicalNot=>{return Value::logical_not(v);},
  UnaryOperator::Neg=>{return Value::neg(v);},
  _=>{},
    }


  Value::Undefined
}


pub fn
operate_binary(l: &Value, r: &Value, binop: &BinaryOperator)-> Value
{
    match binop
    {
  BinaryOperator::Add=> {return Value::add(l,r);},
  BinaryOperator::Sub=> {return Value::sub(l,r);},
  BinaryOperator::Mul=> {return Value::mul(l,r);},
  BinaryOperator::Div=> {return Value::div(l,r);},
  BinaryOperator::Rem=> {return Value::rem(l,r);},
  BinaryOperator::Shl=> {return Value::shl(l,r);},
  BinaryOperator::Shr=> {return Value::shr(l,r);},
  BinaryOperator::Or=>  {return Value::or(l,r);},
  BinaryOperator::And=> {return Value::and(l,r);},
  BinaryOperator::Xor=> {return Value::xor(l,r);},

  BinaryOperator::Eq=>  {return Value::eq(l,r);},
  BinaryOperator::Neq=> {return Value::neq(l,r);},
  BinaryOperator::Lt=>  {return Value::lt(l,r);},
  BinaryOperator::Lteq=>{return Value::lteq(l,r);},
  BinaryOperator::Gt=>  {return Value::gt(l,r);},
  BinaryOperator::Gteq=>{return Value::gteq(l,r);},

  BinaryOperator::LogicalAnd=>{return Value::logical_and(l,r);},
  BinaryOperator::LogicalOr=> {return Value::logical_or(l,r);},
  _=>{},
    }


  Value::Undefined
}


pub fn
operate_assign(l: &Value, r: &Value, asop: &AssignOperator)-> Value
{
  Value::Undefined
}


pub fn
operate_primary(v: &Value, primop: &PrimaryOperator)-> Value
{
  Value::Undefined
}
*/


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




