

use super::TypeNote;


#[derive(Clone)]
pub struct
Enumerator
{
  name: String,

  value: i64,

}


impl
Enumerator
{


pub fn
print(&self)
{
  print!("{} = {}",&self.name,self.value);
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


pub fn   get_size(&self)-> &Option<usize>{&self.size}
pub fn  get_align(&self)-> &Option<usize>{&self.align}


pub fn
find(&self, name: &str)-> Option<i64>
{
    for e in &self.member_list
    {
        if e.name == name
        {
          return Some(e.value);
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
    for e in &self.member_list
    {
      e.print();

      println!(",");
    }
}


}




