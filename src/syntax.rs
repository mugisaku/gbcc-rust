

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


pub struct
Object
{
  token_info: TokenInfo,
        data: ObjectData,

}


impl
Object
{


pub fn
get_token_info(&self)-> &TokenInfo
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




pub fn
get_object(objs: &Vec<Object>, pos: usize)-> Option<&Object>
{
    if pos < objs.len()
    {
      return Some(&objs[pos]);
    }


  None
}


pub fn
test_keyword(objs: &Vec<Object>, pos: usize, s: &str)-> bool
{
    if let Some(o) = get_object(objs,pos)
    {
        if let ObjectData::Keyword(kw) = &o.data
        {
          return kw == s;
        }
    }


  false
}


pub fn
get_keyword(objs: &Vec<Object>, pos: usize)-> Option<&String>
{
    if let Some(o) = get_object(objs,pos)
    {
        if let ObjectData::Keyword(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_identifier(objs: &Vec<Object>, pos: usize)-> Option<&String>
{
    if let Some(o) = get_object(objs,pos)
    {
        if let ObjectData::Identifier(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_directory(objs: &Vec<Object>, pos: usize)-> Option<&Directory>
{
    if let Some(o) = get_object(objs,pos)
    {
        if let ObjectData::Directory(d) = &o.data
        {
          return Some(d);
        }
    }


  None
}


pub fn
get_directory_with_name<'a>(objs: &'a Vec<Object>, pos: usize, name: &str)-> Option<&'a Directory>
{
    if let Some(o) = get_object(objs,pos)
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




