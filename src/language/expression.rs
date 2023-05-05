

pub mod read_expression;
pub mod dictionary;




#[derive(Clone)]
pub enum
OperandCore
{
  Identifier(String),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(String),
  Expression(Box<Expression>),

}


impl
OperandCore
{


pub fn
print(&self)
{
    match self
    {
  OperandCore::Identifier(s)=>{print!("{}",s);},
  OperandCore::Integer(u)=>{print!("{}",u);},
  OperandCore::Floating(f)=>{print!("{}",f);},
  OperandCore::Character(c)=>{print!("{}",c);},
  OperandCore::String(s)=>{print!("\"{}\"",s);},
  OperandCore::Expression(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
    }
}


}




#[derive(Clone)]
pub enum
PostfixOperator
{
  Access(String),
  Subscript(Box<Expression>),
  Call(Vec<Expression>),
  NameResolution(String),
  Increment,
  Decrement,

}


impl
PostfixOperator
{


pub fn
print(&self)
{
    match self
    {
  PostfixOperator::Access(s)=>{print!(".{}",s);},
  PostfixOperator::Subscript(o)=>
        {
          print!("[");
          o.print();
          print!("]");
        },
  PostfixOperator::Call(args)=>
        {
          print!("(");

            for o in args
            {
              o.print();

              print!(", ");
            }

          print!(")");
        },
  PostfixOperator::NameResolution(s)=>
        {
          print!("::{}",s);
        },
  PostfixOperator::Increment=>{print!("++");},
  PostfixOperator::Decrement=>{print!("--");},
    }
}


}




#[derive(Clone)]
pub enum
PrefixOperator
{
  Neg,
  Not,
  Address,
  Dereference,
  LogicalNot,
  Increment,
  Decrement,

}


impl
PrefixOperator
{


pub fn
print(&self)
{
    match self
    {
  PrefixOperator::Neg=>{print!("-");},
  PrefixOperator::Not=>{print!("~");},
  PrefixOperator::Address=>{print!("&");},
  PrefixOperator::Dereference=>{print!("*");},
  PrefixOperator::LogicalNot=>{print!("!");},
  PrefixOperator::Increment=>{print!("++");},
  PrefixOperator::Decrement=>{print!("--");},
    }
}


}


#[derive(Clone)]
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

  BinaryOperator::Assign=>{print!("=");},
  BinaryOperator::AddAssign=>{print!("+=");},
  BinaryOperator::SubAssign=>{print!("-=");},
  BinaryOperator::MulAssign=>{print!("*=");},
  BinaryOperator::DivAssign=>{print!("/=");},
  BinaryOperator::RemAssign=>{print!("%=");},
  BinaryOperator::ShlAssign=>{print!("<<=");},
  BinaryOperator::ShrAssign=>{print!(">>=");},
  BinaryOperator::AndAssign=>{print!("&=");},
  BinaryOperator::OrAssign=>{print!("|=");},
  BinaryOperator::XorAssign=>{print!("^=");},
    }
}


}




#[derive(Clone)]
pub enum
Operator
{
  Prefix(PrefixOperator),
  Postfix(PostfixOperator),
  Binary(BinaryOperator),

}


impl
Operator
{


pub fn
get_priority(&self)-> usize
{
    match self
    {
  Operator::Postfix(o)=>{return 3;},
  Operator::Prefix(o)=>  {return 2;},
  Operator::Binary(o)=> {return 1;},
    }
}


pub fn
print(&self)
{
    match self
    {
  Operator::Postfix(o)=>{o.print();},
  Operator::Prefix(o)=>{o.print();},
  Operator::Binary(o)=>{o.print();},
    }
}


}




#[derive(Clone)]
pub struct
Operand
{
  pub(crate) prefix_operator_list: Vec<PrefixOperator>,

  pub(crate) core: OperandCore,

  pub(crate) postfix_operator_list: Vec<PostfixOperator>,

}


impl
Operand
{


pub fn
print(&self)
{
    for o in &self.prefix_operator_list
    {
      o.print();
    }


  self.core.print();

    for o in &self.postfix_operator_list
    {
      o.print();
    }
}


}




#[derive(Clone)]
pub struct
ExpressionTail
{
  pub(crate) operator: BinaryOperator,

  pub(crate) operand: Operand,

}


#[derive(Clone)]
pub struct
Expression
{
  pub(crate) operand: Operand,

  pub(crate) tail_list: Vec<ExpressionTail>,

}


impl
Expression
{


pub fn
make_from_string(s: &str)-> Result<Expression,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = self::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(e_dir) = cur.get_directory()
        {
//          e_dir.print(0);

          return self::read_expression::read_expression(&e_dir);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
print(&self)
{
  self.operand.print();

    for t in &self.tail_list
    {
      t.operator.print();
      t.operand.print();
    }
}


}




