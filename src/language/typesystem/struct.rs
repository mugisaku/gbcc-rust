

use std::cell::Cell;

use super::{
  Type,
};


use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
  Library
};


pub struct
Member
{
  pub(crate) name: String,

  pub(crate) type_index: TypeIndex,

}


impl
Member
{


pub fn
print(&self, lib: &Library)
{
    if self.name.len() != 0
    {
      print!("{}: ",&self.name);
    }


  lib.print_type(self.type_index);

  print!(")");
}


}


pub fn
print_member_list(ls: &Vec<Member>, lib: &Library)
{
    for m in ls
    {
      m.print(lib);

      println!(",");
    }
}




pub struct
Struct
{
  pub(crate) member_list: Vec<Member>,

}


impl
Struct
{


pub fn
new()-> Struct
{
  Struct{member_list: Vec::new()}
}


pub fn
from(ls: Vec<(String,TypeIndex)>)-> Struct
{
  let  mut st = Struct::new();

    for e in ls
    {
      st.member_list.push(Member{name: e.0, type_index: e.1});
    }


  st
}


pub fn
push(&mut self, m: Member)
{
  self.member_list.push(m);
}


pub fn
add(&mut self, name: &str, ti: TypeIndex)
{
  self.member_list.push(Member{ name: String::from(name), type_index: ti});
}


pub fn
merge(&mut self, ls: Vec<Member>)
{
    for m in ls
    {
      self.member_list.push(m);
    }
}


pub fn  get_member_list(&self)-> &Vec<Member>{&self.member_list}


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
print(&self, lib: &Library)
{
  print!("{{");

  print_member_list(&self.member_list,lib);

  print!("}}");
}


}





