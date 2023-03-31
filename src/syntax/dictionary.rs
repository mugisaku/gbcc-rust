

use crate::source_file::to_string;
use crate::token::print_char_string;


pub enum
Operand
{
  One(       Box<Expression>),
  Option(    Box<Expression>),
  Repetition(Box<Expression>),

  Identifier(Vec<char>),
  Keyword(Vec<char>),
  String(Vec<char>),

  IdentifierLiteral,
  IntegerLiteral,
  FloatingLiteral,
  CharacterLiteral,
  StringLiteral,

}


impl
Operand
{


pub fn
test(&self, dic: &Dictionary)-> Result<(),()>
{
    match self
    {
  Operand::One(e)=>      {e.test(dic)},
  Operand::Option(e)=>   {e.test(dic)},
  Operand::Repetition(e)=>{e.test(dic)},
  Operand::Identifier(id)=>
        {
          let  s = to_string(id);

            if let None = dic.find(&s)
            {
              print!("definition <{}> not found.\n",&s);

              return Err(());
            }


          Ok(())
        },
  Operand::Keyword(_)=>{Ok(())},
  Operand::String(_)=>{Ok(())},
  Operand::IdentifierLiteral=>{Ok(())},
  Operand::IntegerLiteral=>{Ok(())},
  Operand::FloatingLiteral=>{Ok(())},
  Operand::CharacterLiteral=>{Ok(())},
  Operand::StringLiteral=>{Ok(())},
    }
}


pub fn
print(&self)
{
    match self
    {
  Operand::One(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
  Operand::Option(e)=>
        {
          print!("[");
          e.print();
          print!("]");
        },
  Operand::Repetition(e)=>
        {
          print!("{}","{");
          e.print();
          print!("{}","}");
        },
  Operand::Identifier(s)=>{print_char_string(s);},
  Operand::Keyword(s)=>
        {
          print!("\'");
          print_char_string(s);
        },
  Operand::String(s)=>
        {
          print!("\"");
          print_char_string(s);
          print!("\"");
        },
  Operand::IdentifierLiteral=>{print!(".Identifier");},
  Operand::IntegerLiteral=>{print!(".Integer");},
  Operand::FloatingLiteral=>{print!(".Floating");},
  Operand::CharacterLiteral=>{print!(".Character");},
  Operand::StringLiteral=>{print!(".String");},
    }
}


}




pub enum
UnaryOperator
{
  Nop,
  Not,

}


impl
UnaryOperator
{


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::Nop=>{},
  UnaryOperator::Not=>{print!("!");},
    }
}


}


pub struct
UnaryOperation
{
  pub(crate) operator: UnaryOperator,
  pub(crate)  operand: Operand,

}


impl
UnaryOperation
{


pub fn
get_operator(&self)-> &UnaryOperator
{
  &self.operator
}


pub fn
get_operand(&self)-> &Operand
{
  &self.operand
}


pub fn
print(&self)
{
  self.operator.print();
  self.operand.print();
}


}




pub enum
BinaryOperator
{
  And,
  Or,
  Arrow,

}


impl
BinaryOperator
{


pub fn
print(&self)
{
    match self
    {
  BinaryOperator::And=>{print!("&");},
  BinaryOperator::Or=> {print!("|");},
  BinaryOperator::Arrow=>{print!("->");},
    }
}


}




pub struct
BinaryOperation
{
  pub(crate) operator: BinaryOperator,

  pub(crate)  left: Operand,
  pub(crate) right: Operand,

}


impl
BinaryOperation
{


pub fn
get_operator(&self)-> &BinaryOperator
{
  &self.operator
}


pub fn
get_left(&self)-> &Operand
{
  &self.left
}


pub fn
get_right(&self)-> &Operand
{
  &self.right
}


pub fn
test(&self, dic: &Dictionary)-> Result<(),()>
{
    if   self.left.test(dic).is_ok()
     && self.right.test(dic).is_ok()
    {
      return Ok(());
    }


  Err(())
}


pub fn
print(&self)
{
  self.left.print();

  print!(" ");

  self.operator.print();

  print!(" ");

  self.right.print();
}


}




pub enum
Expression
{
  Empty,
  Operand(Operand),
  UnaryOperation( UnaryOperation),
  BinaryOperation(BinaryOperation),

}


impl
Expression
{


pub fn
test(&self, dic: &Dictionary)-> Result<(),()>
{
    match self
    {
  Expression::Empty=>{Ok(())},
  Expression::Operand(o)=>{o.test(dic)}
  Expression::UnaryOperation(o)=>{o.operand.test(dic)},
  Expression::BinaryOperation(o)=>{o.test(dic)},
    }
}


pub fn
print(&self)
{
    match self
    {
  Expression::Empty=>{print!("EMPTY");},
  Expression::Operand(o)=>{o.print();},
  Expression::UnaryOperation(o)=>{o.print();},
  Expression::BinaryOperation(o)=>{o.print();},
    }
}


}




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
new(name: &str)-> Definition
{
  Definition{ name: String::from(name), expression: Expression::Empty}
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
set_expression(&mut self, expr: Expression)
{
  self.expression = expr;
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
new()-> Dictionary
{
  Dictionary{definition_list: Vec::new()}
}


pub fn
make_from_file(filepath: &str)-> Result<Dictionary,()>
{
    if let Ok(src) = crate::source_file::SourceFile::open(filepath)
    {
      return super::read_dictionary::read_dictionary(&src);
    }


  Err(())
}


pub fn
make_from_string(s: &str)-> Result<Dictionary,()>
{
  let  src = crate::source_file::SourceFile::from(s);

  return super::read_dictionary::read_dictionary(&src);
}


pub fn
get_first(&self)-> Option<&Definition>
{
  self.definition_list.first()
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
test(&self)-> Result<(),()>
{
    for def in &self.definition_list
    {
        if def.expression.test(self).is_err()
        {
          return Err(());
        }
    }


  Ok(())
}




pub fn
print(&self)
{
    for def in &self.definition_list
    {
      def.print();

      print!("\n\n");
    }
}




}




