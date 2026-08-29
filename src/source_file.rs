

use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

mod read_string;
mod read_number;
mod read_token;




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
new()-> Self
{
  Self{path: String::new(), lines: Vec::new()}
}


pub fn
from_string(s: &str)-> Self
{
  let  mut srcf = Self::new();

  let  mut buf: Vec<char> = Vec::new();

    for c in s.chars()
    {
      buf.push(c);

        if c == '\n'
        {
          srcf.lines.push(buf);

          buf = Vec::new();
        }
    }


    if buf.len() != 0
    {
      srcf.lines.push(buf);
    }


  srcf
}


pub fn
from_file(path: &str)-> Result<Self,()>
{
    if let Ok(mut f) = File::open(path)
    {
      let  mut s = String::new();

      let  _ = f.read_to_string(&mut s);

      let  mut srcf = SourceFile::from_string(s.as_str());

      srcf.path = path.to_string();

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
get_character(&self, x: usize, y: usize)-> Option<char>
{
    if y < self.lines.len()
    {
      let  ln = &self.lines[y];

        if x < ln.len()
        {
          return Some(ln[x]);
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


pub fn
print_line_to(&self, y: usize, x_opt: Option<usize>, buf: &mut String)
{
    if let Some(ln) = self.lines.get(y)
    {
        if let Some(x) = x_opt
        {
          buf.push_str("   ");

            if x != 0
            {
                for i in 0..(x-1)
                {
                  buf.push(if ln[i] >= '　'{'　'} else{' '});
                }
            }


          buf.push_str("↓\n");
        }


      buf.push_str(">> ");

        for c in ln
        {
          buf.push(*c);
        }
    }
}


}




#[derive(Clone)]
pub struct
SourceInfo
{
  file: Rc<SourceFile>,

  x: usize,
  y: usize,

}


impl
SourceInfo
{


pub fn
new()-> Self
{
  Self{file: Rc::new(SourceFile::new()), x: 0, y: 0}
}


pub fn
from_file(file: &Rc<SourceFile>)-> Self
{
  Self{file: Rc::clone(file), x: 0, y: 0}
}


pub fn
get_file(&self)-> &Rc<SourceFile>
{
  &self.file
}


pub fn
to_string(&self)-> String
{
  let  mut s = format!("[file: \"{}\" x: {} y: {}]\n",self.file.get_path(),1+self.x,1+self.y);

    if self.y >= 2{self.file.print_line_to(self.y-2,None,&mut s);}
    if self.y >= 1{self.file.print_line_to(self.y-1,None,&mut s);}


  self.file.print_line_to(self.y  ,Some(self.x),&mut s);
  self.file.print_line_to(self.y+1,None        ,&mut s);
  self.file.print_line_to(self.y+2,None        ,&mut s);

  s
}


pub fn
to_message(&self)-> Message
{
  Message::new(self.to_string())
}


pub fn
print(&self)
{
  let  s = self.to_string();

  print!("{}",&s);
}


}




#[derive(Clone)]
pub struct
SourceReader
{
  info: SourceInfo,

}


impl
SourceReader
{


pub fn
new(file: &Rc<SourceFile>)-> Self
{
  Self{info: SourceInfo::from_file(file)}
}


pub fn  get_x(&self)-> usize{self.info.x}
pub fn  get_y(&self)-> usize{self.info.y}

pub fn  as_info(&self)-> &SourceInfo{&self.info}


pub fn
get_character(&self)-> Option<char>
{
  self.info.file.get_character(self.get_x(),self.get_y())
}


pub fn
advance(&mut self)
{
  self.info.x += 1;
}


pub fn
newline(&mut self)
{
  self.info.x  = 0;
  self.info.y += 1;
}


pub fn
is_space(c: char)-> bool
{
  (c ==  ' ') ||
  (c == '\n') ||
  (c == '\t') ||
  (c == '\r')
}


pub fn
skip_until_appears_newline(&mut self)-> Result<(),Message>
{
    while let Some(c) = self.get_character()
    {
      self.advance();

        if c == '\n'
        {
          self.newline();

          return Ok(());
        }
    }


  Err(self.info.to_message()+"コメントラインが正しく終了していない")
}


pub fn
skip_until_appears_end_of_comment_block(&mut self)-> Result<(),Message>
{
    while let Some(first) = self.get_character()
    {
      self.advance();

        if first == '\n'
        {
          self.newline();
        }

      else
        if first == '*'
        {
            if let Some(second) = self.get_character()
            {
              self.advance();

                if second == '/'
                {
                  return Ok(());
                }
            }
        }
    }


  Err(self.info.to_message()+"コメントブロックが正しく終了していない")
}


pub fn
skip_spaces(&mut self)
{
    while let Some(c) = self.get_character()
    {
        if Self::is_space(c)
        {
            if c == '\n'
            {
              self.newline();
            }

          else
            {
              self.advance();
            }
        }

      else
        {
          break;
        }
    }
}


}




#[derive(Clone)]
pub struct
Message
{
  strings: Vec<String>,

}


impl
Message
{


pub fn
new(s: String)-> Self
{
  Self{
    strings: vec![s],
  }
}


pub fn
to_string(&self)-> String
{
  let  mut buf = String::new();

    for s in &self.strings
    {
      buf.push_str(s);
      buf.push('\n');
    }


  buf
}


pub fn
print(&self)
{
    for s in &self.strings
    {
      println!("{}",s);
    }
}


}


impl
std::ops::Add<String> for Message
{


type Output = Message;

fn
add(mut self, s: String)-> Self::Output
{
  self.strings.push(s);

  self
}


}


impl
std::ops::Add<&str> for Message
{


type Output = Message;

fn
add(mut self, s: &str)-> Self::Output
{
  self.strings.push(s.to_string());

  self
}


}


impl
std::ops::Add<Message> for Message
{


type Output = Message;

fn
add(mut self, msg: Message)-> Self::Output
{
  self.strings.push(msg.to_string());

  self
}


}


impl
std::convert::From<&str> for Message
{


fn
from(s: &str)-> Self
{
  Self::new(s.to_string())
}


}




