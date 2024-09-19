

pub mod dictionary;
pub mod read_dictionary;
pub mod parse;


use crate::source_file::{
  to_string,
};


use crate::token::{
  Token,
  TokenInfo,
  TokenData,
};


pub fn
print_indent(n: usize)
{
    for _ in 0..n
    {
      print!("  ");
    }
}




#[derive(Clone)]
pub struct
Directory
{
  pub(crate) name: String,

  pub(crate) object_list: Vec<Object>,

}


impl
Directory
{


pub fn
new(name: &str)-> Directory
{
  Directory{ name: String::from(name), object_list: Vec::new()}
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
print(&self, n: usize)
{
  print!("[{}] START\n",&self.name);

    for o in &self.object_list
    {
      o.print(n+1);

      print!("\n");
    }


  print_indent(n);

  print!("[{}] END",&self.name);
}


}




#[derive(Clone)]
pub enum
ObjectData
{
  Null,

  Identifier(String),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(String),

  OthersString(String),
  Keyword(String),

  Directory(Directory),

}


#[derive(Clone)]
pub struct
Object
{
  token_info: Option<TokenInfo>,
        data: ObjectData,

}


impl
Object
{


pub fn
get_token_info(&self)-> &Option<TokenInfo>
{
  &self.token_info
}


pub fn
get_data(&self)-> &ObjectData
{
  &self.data
}


pub fn
print(&self, n: usize)
{
  print_indent(n);

    match &self.data
    {
  ObjectData::Null=>{},

  ObjectData::Integer(i)=>{print!("{}",i);},
  ObjectData::Floating(f)=>{print!("{}",f);},
  ObjectData::String(s)=>{print!("\"{}\"",s);},
  ObjectData::Identifier(s)=>{print!("{}",s);},
  ObjectData::Character(c)=>{print!("\'{}\'",c);},

  ObjectData::OthersString(s)=>{print!("{}",s);},
  ObjectData::Keyword(s)=>{print!("\'{}",s);},

  ObjectData::Directory(d)=>
        {
          d.print(n);
        },
    }
}


}




pub struct
Cursor<'a>
{
  directory: &'a Directory,

  position: usize,

}


impl<'a>
Cursor<'a>
{


pub fn
new(dir: &'a Directory)-> Cursor<'a>
{
  Cursor{directory: dir, position: 0}
}


pub fn
advance(&mut self, n: usize)
{
  self.position += n;
}


pub fn
is_not_finished(&self)-> bool
{
  self.position < self.directory.object_list.len()
}


pub fn
get_object(&self)-> Option<&Object>
{
    if self.is_not_finished()
    {
      return Some(&self.directory.object_list[self.position]);
    }


  None
}


pub fn
test_keyword(&self, s: &str)-> bool
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::Keyword(kw) = &o.data
        {
          return kw == s;
        }
    }


  false
}


pub fn
get_keyword(&self)-> Option<&String>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::Keyword(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_others_string(&self)-> Option<&String>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::OthersString(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::Identifier(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_string(&self)-> Option<&String>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::String(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_directory(&self)-> Option<&Directory>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::Directory(d) = &o.data
        {
          return Some(d);
        }
    }


  None
}


pub fn
get_directory_with_name(&self, name: &str)-> Option<&Directory>
{
    if let Some(o) = self.get_object()
    {
        if let ObjectData::Directory(d) = &o.data
        {
            if d.name == name
            {
              return Some(d);
            }
        }
    }


  None
}


pub fn
seek_directory_with_name(&mut self, name: &str)-> Option<&Directory>
{
  let  tmp = self.position;

    while let None = self.get_directory_with_name(name)
    {
        if self.is_not_finished()
        {
          self.advance(1);
        }

      else
        {
          self.position = tmp;

          return None;
        }
    }


  self.get_directory()
}




}




