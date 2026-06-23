

use crate::source_file::SourceInfo;


#[derive(Clone)]
pub struct
Node
{
  source_info: SourceInfo,

  name: String,

  list: Vec<Value>,

}


impl
Node
{


pub fn
new(source_info: SourceInfo, name: &str)-> Self
{
  Self{
    source_info,

    name: name.to_string(),

    list: Vec::new(),

  }
}


pub fn
get_list(&self)-> &Vec<Value>
{
  &self.list
}


pub fn
new_value(&mut self)-> &mut Value
{
  let  v = Value{source_info: SourceInfo::new(), kind: ValueKind::Null};

  self.list.push(v);

    if let Some(last) = self.list.last_mut()
    {
      return last;
    }


  panic!();
}


pub fn
add_value(&mut self, v: Value)
{
  self.list.push(v);
}


pub fn
add_value_list(&mut self, mut ls: Vec<Value>)
{
  self.list.append(&mut ls);
}


pub fn
new_node(&mut self, name: &str)-> &mut Node
{
  let  nd = Self{
    source_info: SourceInfo::new(),

    name: name.to_string(),

    list: Vec::new(),

  };


  let  kind = ValueKind::Node(Box::new(nd));

  let  v = Value{source_info: SourceInfo::new(), kind};

  self.list.push(v);

    if let Some(last) = self.list.last_mut()
    {
        if let ValueKind::Node(nd) = &mut last.kind
        {
          return nd;
        }
    }


  panic!();
}


pub fn
add_node(&mut self, nd: Self)
{
  let  kind = ValueKind::Node(Box::new(nd));

  let  v = Value{source_info: SourceInfo::new(), kind};

  self.list.push(v);
}


pub fn
find_node(&self, name: &str)-> Option<&Self>
{
    for v in &self.list
    {
        if let ValueKind::Node(nd) = &v.kind
        {
            if &nd.name == name
            {
              return Some(nd);
            }
        }
    }


  None
}


pub fn
find_node_mut(&mut self, name: &str)-> Option<&mut Self>
{
    for v in &mut self.list
    {
        if let ValueKind::Node(nd) = &mut v.kind
        {
            if &nd.name == name
            {
              return Some(nd);
            }
        }
    }


  None
}


pub fn
get_length(&self)-> usize
{
  self.list.len()
}


pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
set_source_info(&mut self, info: SourceInfo)
{
  self.source_info = info;
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
set_name(&mut self, name: String)
{
  self.name = name;
}


pub fn
cursor(&self)-> Cursor
{
  Cursor{r: self, i: 0}
}


pub fn
to_string(&self)-> String
{
  let  mut buf = String::new();

  self.print_to(&mut buf);

  buf
}


pub fn
print_to(&self, buf: &mut String)
{
  buf.push_str(&self.name);

    for v in &self.list
    {
      buf.push('<');
      v.kind.print_to(buf);
      buf.push('>');
    }
}


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  print!("{{\n");

    for v in &self.list
    {
      v.kind.print();

      print!("\n");
    }


  print!("}}\n");
}


}




#[derive(Clone)]
pub enum
ValueKind
{
  Null,

  Node(Box<Node>),

  Bool(bool),
  Char(char),

    Int(i64),
   Uint(u64),
  Float(f64),

      String(String),
  SemiString(String),
  Identifier(String),
     Keyword(String),

}


impl
ValueKind
{


pub fn
to_string(&self)-> String
{
  let  mut buf = String::new();

  self.print_to(&mut buf);

  buf
}


pub fn
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Null     =>{buf.push_str("null");}
  Self::Node(nd) =>{nd.print_to(buf);}
  Self::Char(c)  =>{buf.push_str(&c.to_string());}
  Self::Bool(b)  =>{buf.push_str(&b.to_string());}
  Self::Int(i)   =>{buf.push_str(&i.to_string());}
  Self::Uint(u)  =>{buf.push_str(&u.to_string());}
  Self::Float(f) =>{buf.push_str(&f.to_string());}
  Self::String(s)=>
    {
      buf.push('\"');
      buf.push_str(s);
      buf.push('\"');
    }
  Self::SemiString(s)=>
    {
      buf.push('\"');
      buf.push_str(s);
      buf.push('\"');
    }
  Self::Identifier(s)=>
    {
      buf.push('$');
      buf.push_str(s);
    }
  Self::Keyword(s)=>
    {
      buf.push_str(s);
    }
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Null     =>{print!("null");}
  Self::Node(nd) =>{nd.print();}
  Self::Bool(b)  =>{print!("{}",*b);}
  Self::Char(c)  =>{print!("{}",*c);}
  Self::Int(i)   =>{print!("{}",*i);}
  Self::Uint(u)  =>{print!("{}",*u);}
  Self::Float(f) =>{print!("{}",*f);}
  Self::String(s)=>{print!("{}",s);}
  Self::SemiString(s)=>{print!("{}",s);}
  Self::Identifier(s)=>{print!("{}",s);}
  Self::Keyword(s)=>{print!("{}",s);}
    }
}


}




#[derive(Clone)]
pub struct
Value
{
  source_info: SourceInfo,
         kind: ValueKind,
}


impl
Value
{


pub fn
new(source_info: SourceInfo, kind: ValueKind)-> Self
{
  Self{
    source_info,
    kind,
  }
}


pub fn
get_source_info(&self)-> &SourceInfo
{&self.source_info}


pub fn
set_source_info(&mut self, info: SourceInfo)
{self.source_info = info;}


pub fn
get_kind(&self)-> &ValueKind
{&self.kind}


pub fn
set_kind(&mut self, kind: ValueKind)
{self.kind = kind;}


}




pub struct
Cursor<'a>
{
  r: &'a Node,
  i: usize,

}


impl<'a>
Cursor<'a>
{


pub fn
advance(&mut self, n: usize)
{
    if self.i < self.r.list.len()
    {
      self.i += n;
    }
}


pub fn
current(&self)-> Option<&Value>
{
    if self.i < self.r.list.len()
    {
      return Some(&self.r.list[self.i]);
    }


  None
}


pub fn
get_bool(&self)-> Option<bool>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Bool(b) = &v.kind
        {
          return Some(*b);
        }
    }


  None
}


pub fn
get_char(&self)-> Option<char>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Char(c) = &v.kind
        {
          return Some(*c);
        }
    }


  None
}


pub fn
get_int(&self)-> Option<i64>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Int(i) = &v.kind
        {
          return Some(*i);
        }
    }


  None
}


pub fn
get_uint(&self)-> Option<u64>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Uint(u) = &v.kind
        {
          return Some(*u);
        }
    }


  None
}


pub fn
get_float(&self)-> Option<f64>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Float(f) = &v.kind
        {
          return Some(*f);
        }
    }


  None
}


pub fn
get_string(&self)-> Option<&String>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::String(s) = &v.kind
        {
          return Some(s);
        }
    }


  None
}


pub fn
is_semi_string(&self)-> bool
{
    if let Some(v) = self.current()
    {
        if let ValueKind::SemiString(ss) = &v.kind
        {
          return true;
        }
    }


  false
}


pub fn
get_semi_string(&self)-> Option<&String>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::SemiString(s) = &v.kind
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Identifier(s) = &v.kind
        {
          return Some(s);
        }
    }


  None
}


pub fn
is_keyword(&self)-> bool
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Keyword(k) = &v.kind
        {
          return true;
        }
    }


  false
}


pub fn
get_keyword(&self)-> Option<&String>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Keyword(k) = &v.kind
        {
          return Some(k);
        }
    }


  None
}


pub fn
select_keyword(&self, s: &str)-> bool
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Keyword(k) = &v.kind
        {
          return s == k;
        }
    }


  false
}


pub fn
is_node(&self)-> bool
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Node(nd) = &v.kind
        {
          return true;
        }
    }


  false
}


pub fn
get_node(&self)-> Option<&Node>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Node(nd) = &v.kind
        {
          return Some(nd);
        }
    }


  None
}


pub fn
select_node(&self, name: &str)-> Option<&Node>
{
    if let Some(v) = self.current()
    {
        if let ValueKind::Node(nd) = &v.kind
        {
            if &nd.name == name
            {
              return Some(nd);
            }
        }
    }


  None
}


pub fn
seek_node(&mut self, name: &str)-> Option<&Node>
{
  let  mut i = self.i;
  let      l = self.r.get_list().len();

  let  mut found = false;

    while i < l
    {
        if let ValueKind::Node(nd) = &self.r.list[i].kind
        {
            if &nd.name == name
            {
              found = true;

              break;
            }
        }


      i += 1;
    }


    if found
    {
      self.i = i;

        if let ValueKind::Node(nd) = &self.r.list[i].kind
        {
          return Some(nd);
        }
    }


  None
}


pub fn
print(&self)
{
  print!("{}: {}\n",&self.r.name,self.i);

    if let Some(nd) = self.current()
    {
      nd.kind.print();
    }
}



}




