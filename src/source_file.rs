

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
to_error(&self, msg: String)-> Error
{
  Error::new_with_source_info(self,msg)
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
skip_until_appears_newline(&mut self)-> Result<(),String>
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


  Err(format!("コメントラインが正しく終了していない"))
}


pub fn
skip_until_appears_end_of_comment_block(&mut self)-> Result<(),String>
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


  Err(format!("コメントブロックが正しく終了していない"))
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


pub fn
to_error(&self, msg: String)-> Error
{
  self.info.to_error(msg)
}


}




#[derive(Clone)]
pub struct
Error
{
  source_info_opt: Option<SourceInfo>,

  message: String,

  child_opt: Option<Box<Self>>,

}


impl
Error
{


pub fn
new(message: String)-> Self
{
  Self{
    source_info_opt: None,
    message,
    child_opt: None,
  }
}


pub fn
new_with_source_info(source_info: &SourceInfo, message: String)-> Self
{
  Self{
    source_info_opt: Some(source_info.clone()),
    message,
    child_opt: None,
  }
}


pub fn
wrap(mut self, child: Self)-> Self
{
  self.child_opt = Some(Box::new(child));

  self
}


pub fn
to_string(&self)-> String
{
  let  mut s = String::new();

    if let Some(info) = &self.source_info_opt
    {
      s.push_str(&info.to_string());

      s.push_str("\n");
    }


  s.push_str(&self.message);

  s.push_str("\n");

    if let Some(child) = &self.child_opt
    {
      s.push_str(&child.to_string());

      s.push_str("\n");
    }


  s
}


pub fn
print(&self)
{
  let  s = self.to_string();

  println!("{}",&s);
}


}




