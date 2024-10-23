

#[derive(Clone)]
pub enum
UnaryOperator
{
  Neg,
  Not,
  LogicalNot,
  Deref,

}


impl
UnaryOperator
{


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::Neg=>{print!("-");},
  UnaryOperator::Not=>{print!("~");},
  UnaryOperator::LogicalNot=>{print!("!");},
  UnaryOperator::Deref=>{print!("*");},
    }
}


pub fn
print_mnemonic(&self)
{
    match self
    {
  UnaryOperator::Neg=>{print!("neg");},
  UnaryOperator::Not=>{print!("not");},
  UnaryOperator::LogicalNot=>{print!("logical_not");},
  UnaryOperator::Deref=>{print!("deref");},
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


pub fn
print_mnemonic(&self)
{
    match self
    {
  BinaryOperator::Add=>{print!("add");},
  BinaryOperator::Sub=>{print!("sub");},
  BinaryOperator::Mul=>{print!("mul");},
  BinaryOperator::Div=>{print!("div");},
  BinaryOperator::Rem=>{print!("rem");},
  BinaryOperator::Shl=>{print!("shl");},
  BinaryOperator::Shr=>{print!("shr");},
  BinaryOperator::And=>{print!("and");},
  BinaryOperator::Or=>{print!("or");},
  BinaryOperator::Xor=>{print!("xor");},
  BinaryOperator::Eq=>{print!("eq");},
  BinaryOperator::Neq=>{print!("neq");},
  BinaryOperator::Lt=>{print!("lt");},
  BinaryOperator::Lteq=>{print!("lteq");},
  BinaryOperator::Gt=>{print!("gt");},
  BinaryOperator::Gteq=>{print!("gteq");},
  BinaryOperator::LogicalAnd=>{print!("logical_and");},
  BinaryOperator::LogicalOr=>{print!("logical_or");},
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


pub fn
get_relational_operator(&self)-> Option<BinaryOperator>
{
    match self
    {
  AssignOperator::Nop=>{None},
  AssignOperator::Add=>{Some(BinaryOperator::Add)},
  AssignOperator::Sub=>{Some(BinaryOperator::Sub)},
  AssignOperator::Mul=>{Some(BinaryOperator::Mul)},
  AssignOperator::Div=>{Some(BinaryOperator::Div)},
  AssignOperator::Rem=>{Some(BinaryOperator::Rem)},
  AssignOperator::Shl=>{Some(BinaryOperator::Shl)},
  AssignOperator::Shr=>{Some(BinaryOperator::Shr)},
  AssignOperator::And=>{Some(BinaryOperator::And)},
  AssignOperator::Or=>{Some(BinaryOperator::Or)},
  AssignOperator::Xor=>{Some(BinaryOperator::Xor)},
    }
}


}




#[derive(Clone,PartialEq)]
pub struct
Path
{
  pub(crate) identifier_list: Vec<String>,
}


impl
Path
{


pub fn
new()-> Path
{
  Path{identifier_list: Vec::new()}
}


pub fn
add(mut self, name: &str)-> Path
{
    if name.len() != 0
    {
      self.identifier_list.push(name.to_string());
    }


  self
}


pub fn
push(&mut self, name: &str)
{
    if name.len() != 0
    {
      self.identifier_list.push(name.to_string());
    }
}


pub fn
pop(&mut self)-> Option<String>
{
  self.identifier_list.pop()
}


pub fn
as_strings(&self)-> &Vec<String>
{
  &self.identifier_list
}


pub fn
to_string(&self)-> String
{
  let  mut s = String::new();

    if let Some(first) = self.identifier_list.first()
    {
      s.push_str(first);

        for i in 1..self.identifier_list.len()
        {
          s.push_str("::");
          s.push_str(&self.identifier_list[i]);
        }
    }


  s
}


pub fn
print(&self)
{
    if let Some(first) = self.identifier_list.first()
    {
      print!("{}",first);

        for i in 1..self.identifier_list.len()
        {
          print!("::{}",&self.identifier_list[i]);
        }
    }
}


}




#[derive(Clone)]
pub struct
TableElement
{
  pub(crate)       name: String,
  pub(crate) expression: Expression,

}


impl
TableElement
{


pub fn
new(name: String, expression: Expression)-> Self
{
  Self{
          name,
    expression,
  }
}


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.expression.print();
}


}




#[derive(Clone)]
pub enum
Expression
{
  Identifier(String),
  Boolean(bool),
  Integer(u64),
  Floating(f64),
  String(String),

  Table(Vec<TableElement>),

  SubExpression(Box<Expression>),

  Access(Box<Expression>,String),
  Call(Box<Expression>,Vec<Expression>),
  Subscript(Box<Expression>,Box<Expression>),

  Unary(UnaryOperator,Box<Expression>),
  Binary(BinaryOperator,Box<Expression>,Box<Expression>),

}


impl
Expression
{


pub fn
read(s: &str)-> Expression
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        if let Some(d_dir) = cur.get_directory_with_name("expression")
        {
          return super::read::read_expression(d_dir);
        }
    }


  panic!();
}


pub fn
print(&self)
{
    match self
    {
  Expression::Identifier(s)=>{print!("{}",s);},
  Expression::Boolean(b)=>{print!("{}",b);},
  Expression::Integer(u)=>{print!("{}",u);},
  Expression::Floating(f)=>{print!("{}",f);},
  Expression::String(s)=>{print!("\"{}\"",s);},
  Expression::Table(ls)=>
        {
          print!("[");

            for e in ls
            {
              e.print();

              print!(", ");
            }


          print!("]");
        },
  Expression::SubExpression(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
  Expression::Unary(o,e)=>
        {
          o.print();
          e.print();
        },
  Expression::Call(f,args)=>
        {
          f.print();

          print!("(");

            for e in args
            {
              e.print();

              print!(", ");
            }


          print!(")");
        },
  Expression::Subscript(target,index)=>
        {
          target.print();

          print!("[");

          index.print();

          print!("]");
        },
  Expression::Access(target,name)=>
        {
          target.print();

          print!(".{}",name);
        },
  Expression::Binary(o,l,r)=>
        {
          l.print();
          o.print();
          r.print();
        },
    }
}


}




