

use std::convert::From;

#[derive(Clone,PartialEq)]
pub struct
OpId
{
  pub(crate) code: usize,

}


impl
OpId
{


const fn
make_code_internal(b: &[u8], i: usize, len: usize, bufsz: usize, code: usize)-> usize
{
    if (bufsz != 0) && (i < len)
    {
      Self::make_code_internal(b,i+1,len,bufsz-1,code|((b[i] as usize)<<(8*i)))
    }

  else
    {
      code
    }
}


const fn
make_code(s: &str)-> usize
{
    if s.len() < 8
    {
      return Self::make_code_internal(s.as_bytes(),0,s.len(),8,0);
    }


  panic!();
}


pub fn
print(&self)
{
  let  bytes = self.code.to_le_bytes();

  print!("{}",str::from_utf8(&bytes).unwrap());
}


}




impl
From<&str> for OpId
{


fn
from(s: &str)-> Self
{
  Self{code: Self::make_code(s)}
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
  Void,

  Identifier(String),

    Int(i64),
   Uint(u64),
  Float(f64),

  String(String),

  Table(Vec<TableElement>),

  SubExpression(Box<Expression>),

     AccessOp(Box<Expression>,String),
       CallOp(Box<Expression>,Vec<Expression>),
  SubscriptOp(Box<Expression>,Box<Expression>),

   UnaryOp(OpId,Box<Expression>),
  BinaryOp(OpId,Box<Expression>,Box<Expression>),

}


impl
Expression
{


/*
pub fn
read(s: &str)-> Self
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
*/


pub fn
print(&self)
{
    match self
    {
  Expression::Void=>{print!("void");},
  Expression::Identifier(s)=>{print!("{}",s);},
  Expression::Int(i)=>{print!("{}",*i);},
  Expression::Uint(u)=>{print!("{}",*u);},
  Expression::Float(f)=>{print!("{}",*f);},
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
  Expression::UnaryOp(o,e)=>
        {
          o.print();
          e.print();
        },
  Expression::CallOp(f,args)=>
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
  Expression::SubscriptOp(target,index)=>
        {
          target.print();

          print!("[");

          index.print();

          print!("]");
        },
  Expression::AccessOp(target,name)=>
        {
          target.print();

          print!(".{}",name);
        },
  Expression::BinaryOp(o,l,r)=>
        {
          l.print();
          o.print();
          r.print();
        },
    }
}


}




