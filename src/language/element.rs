

use std::rc::Rc;

use super::expression::*;


pub enum
Element
{
  Operand(Operand),
  Operator(Operator),
}


pub enum
Operand
{
  Identifier(Rc<String>),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(Rc<String>),
  Expression(Expression),

}


pub enum
Operator
{
  Primary(PrimaryOperation),
  Unary(UnaryOperator),
  Binary(BinaryOperator),
  Assign(AssignOperator),

}


impl
Element
{


pub fn
print(&self)
{
    match self
    {
  Element::Operand(o)=>{o.print();},
  Element::Operator(o)=>{o.print();},
    }
}


}


impl
Operand
{


pub fn
print(&self)
{
    match self
    {
  Operand::Identifier(s)=>{print!("{}",s);},
  Operand::Integer(u)=>{print!("{}",u);},
  Operand::Floating(f)=>{print!("{}",f);},
  Operand::Character(c)=>{print!("{}",c);},
  Operand::String(s)=>{print!("\"{}\"",s);},
  Operand::Expression(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
    }
}


}


impl
Operator
{


pub fn
get_priority(&self)-> usize
{
    match self
    {
  Operator::Primary(o)=>{return 3;},
  Operator::Unary(o)=>  {return 2;},
  Operator::Binary(o)=> {return 1;},
  Operator::Assign(o)=> {return 0;},
    }
}


pub fn
print(&self)
{
    match self
    {
  Operator::Primary(o)=>{o.print();},
  Operator::Unary(o)=>{o.print();},
  Operator::Binary(o)=>{o.print();},
  Operator::Assign(o)=>{o.print();},
    }
}


}




pub enum
PrimaryOperation
{
  Access(Rc<String>),
  Subscript(Expression),
  Call(Vec<Expression>),

}


impl
PrimaryOperation
{


pub fn
print(&self)
{
    match self
    {
  PrimaryOperation::Access(s)=>{print!(".{}",s);},
  PrimaryOperation::Subscript(e)=>
        {
          print!("[");
          e.print();
          print!("]");
        },
  PrimaryOperation::Call(args)=>
        {
          print!("(");

            for e in args
            {
              e.print();

              print!(", ");
            }

          print!(")");
        },
    }
}


}




pub enum
UnaryOperator
{
  Nop,
  Neg,
  Not,
  Address,
  Dereference,
  LogicalNot,
  PrefixIncrement,
  PrefixDecrement,

}


impl
UnaryOperator
{


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::Nop=>{print!("");},
  UnaryOperator::Neg=>{print!("-");},
  UnaryOperator::Not=>{print!("~");},
  UnaryOperator::Address=>{print!("&");},
  UnaryOperator::Dereference=>{print!("*");},
  UnaryOperator::LogicalNot=>{print!("!");},
  UnaryOperator::PrefixIncrement=>{print!("++");},
  UnaryOperator::PrefixDecrement=>{print!("--");},
    }
}


}


pub enum
BinaryOperator
{
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,
  Eq,
  Neq,
  Lt,
  Lteq,
  Gt,
  Gteq,
  LogicalOr,
  LogicalAnd,

}


impl
BinaryOperator
{


pub fn
print(&self)
{
    match self
    {
  BinaryOperator::Add=>{print!("+");},
  BinaryOperator::Sub=>{print!("-");},
  BinaryOperator::Mul=>{print!("*");},
  BinaryOperator::Div=>{print!("/");},
  BinaryOperator::Rem=>{print!("%");},
  BinaryOperator::Shl=>{print!("<<");},
  BinaryOperator::Shr=>{print!(">>");},
  BinaryOperator::And=>{print!("&");},
  BinaryOperator::Or=>{print!("|");},
  BinaryOperator::Xor=>{print!("^");},
  BinaryOperator::Eq=>{print!("==");},
  BinaryOperator::Neq=>{print!("!=");},
  BinaryOperator::Lt=>{print!("<");},
  BinaryOperator::Lteq=>{print!("<=");},
  BinaryOperator::Gt=>{print!(">");},
  BinaryOperator::Gteq=>{print!(">=");},
  BinaryOperator::LogicalAnd=>{print!("&&");},
  BinaryOperator::LogicalOr=>{print!("||");},
    }
}


}




pub enum
AssignOperator
{
  Assign,
  AddAssign,
  SubAssign,
  MulAssign,
  DivAssign,
  RemAssign,
  ShlAssign,
  ShrAssign,
  AndAssign,
  OrAssign,
  XorAssign,

}




impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Assign=>{print!("=");},
  AssignOperator::AddAssign=>{print!("+=");},
  AssignOperator::SubAssign=>{print!("-=");},
  AssignOperator::MulAssign=>{print!("*=");},
  AssignOperator::DivAssign=>{print!("/=");},
  AssignOperator::RemAssign=>{print!("%=");},
  AssignOperator::ShlAssign=>{print!("<<=");},
  AssignOperator::ShrAssign=>{print!(">>=");},
  AssignOperator::AndAssign=>{print!("&=");},
  AssignOperator::OrAssign=>{print!("|=");},
  AssignOperator::XorAssign=>{print!("^=");},
    }
}


}




