

use super::{
  Type,
};

use std::cell::Cell;

use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  Library
};

use super::r#struct::{
  Member,
  print_member_list,
};


pub struct
Union
{
  pub(crate) member_list: Vec<Member>,

}


impl
Union
{


pub fn
new()-> Union
{
  Union{ member_list: Vec::new()}
}


pub fn
from(ls: Vec<(String,Type)>)-> Union
{
  let  mut un = Union::new();

    for e in ls
    {
      un.member_list.push(Member{name: e.0, r#type: e.1, offset_optcel: Cell::new(None)});
    }


  un
}


pub fn
push(&mut self, name: &str, t: Type)
{
  self.member_list.push(Member{name: String::from(name), r#type: t, offset_optcel: Cell::new(None)});
}


pub fn
merge(&mut self, ls: Vec<Member>)
{
    for m in ls
    {
      self.member_list.push(m);
    }
}


pub fn
print(&self, lib: &Library)
{
  print!("{{");

    for m in &self.member_list
    {
      m.print(lib);
      println!(",");
    }


  print!("}}");
}


}




