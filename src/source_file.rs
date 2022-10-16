

use std::fs::File;
use std::io::prelude::*;


pub struct
SourceFile
{
     path: String,
  content: String,

}


impl
SourceFile
{


pub fn
open(path: &str)-> SourceFile
{
  let mut  f = File::open(path).unwrap();

  let mut  s = String::new();

  let  _ = f.read_to_string(&mut s);

  SourceFile{
    path: String::from(path),
    content: s,
  }
}


pub fn
from(s: &str)-> SourceFile
{
  SourceFile{
       path: String::from(""),
    content: String::from(s),
  }
}


pub fn
get_path(&self)-> &String
{
  &self.path
}


pub fn
get_content(&self)-> &String
{
  &self.content
}


}




