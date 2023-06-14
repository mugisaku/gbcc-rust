

pub mod read_expression;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  Library
};

#[derive(Clone)]
pub enum
OperandCore
{
  Identifier(String),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(StringIndex),
  Expression(ExpressionIndex),

}


impl
OperandCore
{


pub fn
print(&self, lib: &Library)
{
    match self
    {
  OperandCore::Identifier(s)=>{print!("{}",s);},
  OperandCore::Integer(u)=>{print!("{}",u);},
  OperandCore::Floating(f)=>{print!("{}",f);},
  OperandCore::Character(c)=>{print!("{}",c);},
  OperandCore::String(i)=>
        {
            if let Some(s) = lib.get_string(*i)
            {
              print!("\"{}\"",s);
            }
        },
  OperandCore::Expression(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              print!("(");
              e.print(lib);
              print!(")");
            }
        },
    }
}


}




#[derive(Clone)]
pub enum
PostfixOperator
{
  Access(String),
  Subscript(ExpressionIndex),
  Call(Vec<ExpressionIndex>),
  NameResolution(String),
  Increment,
  Decrement,

}


impl
PostfixOperator
{


pub fn
print(&self, lib: &Library)
{
    match self
    {
  PostfixOperator::Access(s)=>{print!(".{}",s);},
  PostfixOperator::Subscript(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              print!("[");
              e.print(lib);
              print!("]");
            }
        },
  PostfixOperator::Call(args)=>
        {
          print!("(");

            for i in args
            {
                if let Some(e) = lib.get_expression(*i)
                {
                  e.print(lib);
                }


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




#[derive(Clone)]
pub enum
AssignOperator
{
  Nop,
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

}


impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Nop=>{print!("=");},
  AssignOperator::Add=>{print!("+=");},
  AssignOperator::Sub=>{print!("-=");},
  AssignOperator::Mul=>{print!("*=");},
  AssignOperator::Div=>{print!("/=");},
  AssignOperator::Rem=>{print!("%=");},
  AssignOperator::Shl=>{print!("<<=");},
  AssignOperator::Shr=>{print!(">>=");},
  AssignOperator::And=>{print!("&=");},
  AssignOperator::Or=>{print!("|=");},
  AssignOperator::Xor=>{print!("^=");},
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
print(&self, lib: &Library)
{
    match self
    {
  Operator::Postfix(o)=>{o.print(lib);},
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
print(&self, lib: &Library)
{
    for o in &self.prefix_operator_list
    {
      o.print();
    }


  self.core.print(lib);

    for o in &self.postfix_operator_list
    {
      o.print(lib);
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

  pub(crate) assign_part_opt: Option<(AssignOperator,ExpressionIndex)>,

}


impl
Expression
{


pub fn
make_from_string(s: &str, lib: &mut Library)-> Result<Expression,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = self::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression_with_assign",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(e_dir) = cur.get_directory()
        {
//          e_dir.print(0);

          return self::read_expression::read_expression(&e_dir,lib);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
print(&self, lib: &Library)
{
  self.operand.print(lib);

    for t in &self.tail_list
    {
      t.operator.print();
      t.operand.print(lib);
    }


    if let Some((a,ei)) = &self.assign_part_opt
    {
      a.print();

        if let Some(e) = lib.get_expression(*ei)
        {
          e.print(lib);
        }
    }
}


}




