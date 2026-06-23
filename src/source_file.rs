

use std::rc::Rc;
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
from_string(s: &str)-> SourceFile
{
  let  mut srcf = SourceFile{ path: String::new(), lines: Vec::new()};

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
from_file(path: &str)-> Result<SourceFile,()>
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
get_character(&self, cur: Cursor)-> Option<char>
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




#[derive(Clone)]
pub struct
SourceInfo
{
  filepath: Rc<String>,

  cursor: Cursor,

}


impl
SourceInfo
{


pub fn
new()-> Self
{
  Self{ filepath: Rc::new(String::new()), cursor: Cursor::new()}
}


pub fn
set_filepath(&mut self, filepath: &str)
{
  self.filepath = Rc::new(String::from(filepath));
}


pub fn
set_cursor(&mut self, cur: &Cursor)
{
  self.cursor.x = cur.x;
  self.cursor.y = cur.y;
}


pub fn  get_filepath(&self)-> &String{&*self.filepath}
pub fn  get_x(&self)-> usize{self.cursor.get_x()}
pub fn  get_y(&self)-> usize{self.cursor.get_y()}


pub fn
to_string(&self)-> String
{
  format!("[file: {} X: {:05} Y: {:05}]",self.get_filepath(),1+self.get_x(),1+self.get_y())
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
new_with_source_info(source_info: SourceInfo, message: String)-> Self
{
  Self{
    source_info_opt: Some(source_info),
    message,
    child_opt: None,
  }
}


pub fn
join(mut self, child: Self)-> Self
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




