

use std::rc::Rc;
use crate::token::print_string;
use crate::source_file::Error;


#[derive(Clone)]
pub enum
Expression
{
  Expression(Box<Expression>),

  Option(    Box<Expression>),
  Repetition(Box<Expression>),

  Identifier(String),
  Keyword(String),
  String(String),

  IdentifierLiteral,
  NumberLiteral,
  CharacterLiteral,
  StringLiteral,

  BinaryOperation(Box<Expression>,Box<Expression>,String),

}


impl
Expression
{


pub fn
test(&self, dic: &Dictionary)-> Result<(),String>
{
    match self
    {
  Expression::Expression(e)=>{e.test(dic)}
  Expression::Option(e)=>    {e.test(dic)}
  Expression::Repetition(e)=>{e.test(dic)}
  Expression::Identifier(s)=>
    {
        if let None = dic.find(s)
        {
          Err(format!("definition <{}> not found.\n",s))
        }

      else
        {Ok(())}
    }
  Expression::Keyword(_)=>{Ok(())}
  Expression::String(_)=>{Ok(())}
  Expression::IdentifierLiteral=>{Ok(())}
  Expression::NumberLiteral=>{Ok(())}
  Expression::CharacterLiteral=>{Ok(())}
  Expression::StringLiteral=>{Ok(())}

  Expression::BinaryOperation(l,r,op)=>
    {
        match l.test(dic)
        {
      Ok(())=>
        {
            match r.test(dic)
            {
          Ok(())=>{Ok(())}
          Err(s)=>{Err(s)}
            }
        }
      Err(s)=>{Err(s)}
        }
    }
    }
}


pub fn
print(&self)
{
    match self
    {
  Expression::Expression(e)=>
    {
      print!("(");
      e.print();
      print!(")");
    }
  Expression::Option(e)=>
    {
      print!("[");
      e.print();
      print!("]");
    }
  Expression::Repetition(e)=>
    {
      print!("{}","{");
      e.print();
      print!("{}","}");
    }
  Expression::Identifier(s)=>
    {
      print_string(s);
    }
  Expression::Keyword(s)=>
    {
      print!("\'");
      print_string(s);
    }
  Expression::String(s)=>
    {
      print!("\"");
      print_string(s);
      print!("\"");
    }
  Expression::IdentifierLiteral=>{print!(".Identifier");}
  Expression::NumberLiteral=>{print!(".Number");}
  Expression::CharacterLiteral=>{print!(".Character");}
  Expression::StringLiteral=>{print!(".String");}

  Expression::BinaryOperation(l,r,op)=>
    {
      l.print();

      print!(" {} ",op);

      r.print();
    }
    }
}


}




#[derive(Clone)]
pub struct
Definition
{
        name: String,
  expression: Expression,

}


impl
Definition
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
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_expression(&self)-> &Expression
{
  &self.expression
}


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.expression.print();

  print!(";");
}


}




pub struct
Dictionary
{
  definition_list: Vec<Definition>,

}


impl
Dictionary
{


pub fn
new()-> Self
{
  Self{definition_list: Vec::new()}
}


pub fn
make_from_string(s: &str)-> Result<Self,Error>
{
  let  file = Rc::new(crate::source_file::SourceFile::from_string(s));

  super::read_dictionary::read_dictionary(&file)
}


pub fn
find(&self, name: &str)-> Option<&Definition>
{
    for def in &self.definition_list
    {
        if def.get_name() == name
        {
          return Some(def);
        }
    }


  None
}


pub fn
add(&mut self, def: Definition)
{
  self.definition_list.push(def);
}


pub fn
test(&self)-> Result<(),String>
{
    for def in &self.definition_list
    {
        if let Err(s) = def.expression.test(self)
        {
          return Err(s);
        }
    }


  Ok(())
}




pub fn
print(&self)
{
  print!("{{\n");

    for def in &self.definition_list
    {
      def.print();

      print!("\n\n");
    }


  print!("}}\n\n");
}




}




