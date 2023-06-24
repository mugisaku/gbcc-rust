

use super::{
  Type,
};


use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
  Library
};

use super::r#struct::Struct;
use crate::language::expression::Expression;


pub enum
Value
{
  Unspecified,
  Expression(ExpressionIndex),

}


impl
Value
{


pub fn
print(&self, lib: &Library)
{
    match self
    {
  Value::Unspecified=>{}
  Value::Expression(ei)=>{}
    }
}


}




pub struct
Enumerator
{
  pub(crate) name: String,

  pub(crate) value: Value,

}


impl
Enumerator
{


pub fn
print(&self, lib: &Library)
{
  print!("{}",&self.name);

  self.value.print(lib);
}


}




pub struct
Enum
{
  pub(crate) member_list: Vec<Enumerator>,

}


impl
Enum
{


pub fn
new()-> Enum
{
  Enum{ member_list: Vec::new()}
}


pub fn
from(ls: Vec<Enumerator>)-> Enum
{
  Enum{member_list: ls}
}


pub fn
find(&self, name: &str)-> Option<&Value>
{
    for e in &self.member_list
    {
        if e.name == name
        {
          return Some(&e.value);
        }
    }


  None
}


pub fn
print(&self, lib: &Library)
{
  println!("{{");

    for e in &self.member_list
    {
      e.print(lib);

      println!(",");
    }


  println!("}}");
}


}




