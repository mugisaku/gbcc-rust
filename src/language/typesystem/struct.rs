

use std::cell::Cell;

use super::{
  Type,
};


use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  Library
};


pub struct
Member
{
  pub(crate) name: String,

  pub(crate) r#type: Type,

}


impl
Member
{


pub fn
print(&self)
{
    if self.name.len() != 0
    {
      print!("{}: ",&self.name);
    }


  self.r#type.print();

  print!(")");
}


}




pub struct
Struct
{
  pub(crate) member_list: Vec<Member>,

}


#[allow(dead_code)]
impl
Struct
{


pub fn
new()-> Struct
{
  Struct{member_list: Vec::new()}
}


pub fn
from(ls: Vec<Member>)-> Struct
{
  Struct{member_list: ls}
}


pub fn
push(&mut self, m: Member)
{
  self.member_list.push(m);
}


pub fn
find(&self, name: &str)-> Option<&Member>
{
    for m in &self.member_list
    {
        if m.name == name
        {
          return Some(&m);
        }
    }


  None
}


pub fn
get(&self, i: usize)-> Option<&Member>
{
    if i < self.member_list.len()
    {
      return Some(&self.member_list[i]);
    }


  None
}


pub fn
print(&self)
{
  print!("{{");

    for m in &self.member_list
    {
      m.print();

      print!(", ");
    }


  print!("}}");
}


}





