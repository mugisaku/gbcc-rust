

use std::fs::File;
use std::io::prelude::*;


#[derive(Clone,Copy)]
pub struct
Cursor
{
  x: usize,
  y: usize,

}


impl
Cursor
{


pub fn
new()-> Cursor
{
  Cursor{ x:0, y:0}
}


pub fn  get_x(&self)-> usize{self.x}
pub fn  get_y(&self)-> usize{self.y}


pub fn
advance(&mut self)
{
  self.x += 1;
}


pub fn
newline(&mut self)
{
  self.x  = 0;
  self.y += 1;
}


pub fn
print(&self)
{
  print!("[X:{:04}, Y:{:04}]",1+self.x,1+self.y);
}


}




pub struct
SourceFile
{
  path: String,

  lines: Vec<Vec<char>>,

}


impl
SourceFile
{


pub fn
open(path: &str)-> Result<SourceFile,()>
{
    if let Ok(mut f) = File::open(path)
    {
      let  mut s = String::new();

      let  _ = f.read_to_string(&mut s);

      let  mut srcf = SourceFile::from(s.as_str());

      srcf.path = String::from(path);

      return Ok(srcf);
    }


  Err(())
}


pub fn
get_path(&self)-> &String
{
  &self.path
}


pub fn
get_character(&self, cur: &Cursor)-> Option<char>
{
    if cur.y < self.lines.len()
    {
      let  ln = &self.lines[cur.y];

        if cur.x < ln.len()
        {
          return Some(ln[cur.x]);
        }
    }


  None
}


pub fn
print(&self)
{
    for i in 0..self.lines.len()
    {
      let  ln = &self.lines[i];

      print!("{:04} ",i+1);

        for c in ln
        {
          print!("{}",c);
        }
    }
}


}




impl
From<&str> for SourceFile
{


fn
from(s: &str)-> SourceFile
{
  let  mut srcf = SourceFile{ path: String::new(), lines: Vec::new()};

  let  mut buf: Vec<char> = Vec::new();

    for c in s.chars()
    {
      buf.push(c);

        if c == '\n'
        {
          srcf.lines.push(buf.clone());

          buf.clear();
        }
    }


    if buf.len() != 0
    {
      srcf.lines.push(buf);
    }


  srcf
}


}


pub fn
new_char_string()-> Vec<char>
{
  Vec::new()
}


pub fn
to_string(src: &Vec<char>)-> String
{
  let  mut s = String::new();

    for c in src
    {
      s.push(*c);
    }


  s
}




