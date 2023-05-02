

use super::TypeNote;
use super::r#struct::Struct;
use crate::language::expression::Expression;


#[derive(Clone)]
pub enum
Value
{
  Unspecified,
  Expression(Expression),
  Struct(Struct),

}


impl
Value
{


pub fn
print(&self)
{
    match self
    {
  Value::Unspecified=>{}
  Value::Expression(e)=>{}
  Value::Struct(st)=>{}
    }
}


}




#[derive(Clone)]
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
print(&self)
{
  print!("{}",&self.name);

  self.value.print();
}


}




#[derive(Clone)]
pub struct
Enum
{
  member_list: Vec<Enumerator>,

   size: Option<usize>,
  align: Option<usize>,

}


impl
Enum
{


pub fn
new()-> Enum
{
  Enum{ member_list: Vec::new(), size: None, align: None}
}


pub fn
from(ls: Vec<Enumerator>)-> Enum
{
  Enum{ member_list: ls, size: None, align: None}
}


pub fn   get_size(&self)-> &Option<usize>{&self.size}
pub fn  get_align(&self)-> &Option<usize>{&self.align}


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
print_id(&self, buf: &mut String)
{
    for m in &self.member_list
    {
//      m.type_info.print_id(buf);
    }
}


pub fn
print(&self)
{
  println!("{{");

    for e in &self.member_list
    {
      e.print();

      println!(",");
    }


  println!("}}");
}


}




