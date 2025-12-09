



#[derive(Clone)]
pub struct
Node
{
  name: String,

  list: Vec<Value>,

}


impl
Node
{


pub fn
new(name: &str)-> Self
{
  Self{
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
  self.list.push(Value::Null);

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
    name: name.to_string(),

    list: Vec::new(),

  };


  self.list.push(Value::Node(Box::new(nd)));

    if let Some(last) = self.list.last_mut()
    {
        if let Value::Node(nd) = last
        {
          return nd;
        }
    }


  panic!();
}


pub fn
add_node(&mut self, nd: Self)
{
  self.list.push(Value::Node(Box::new(nd)));
}


pub fn
find_node(&self, name: &str)-> Option<&Self>
{
    for v in &self.list
    {
        if let Value::Node(nd) = v
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
        if let Value::Node(nd) = v
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
      v.print_to(buf);
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
      v.print();

      print!("\n");
    }


  print!("}}\n");
}


}




#[derive(Clone)]
pub enum
Value
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
Value
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
        if let Value::Bool(b) = v
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
        if let Value::Char(c) = v
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
        if let Value::Int(i) = v
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
        if let Value::Uint(u) = v
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
        if let Value::Float(f) = v
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
        if let Value::String(s) = v
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
        if let Value::SemiString(ss) = v
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
        if let Value::SemiString(s) = v
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
        if let Value::Identifier(s) = v
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
        if let Value::Keyword(k) = v
        {
          return true;
        }
    }


  false
}


pub fn
select_keyword(&self, s: &str)-> bool
{
    if let Some(v) = self.current()
    {
        if let Value::Keyword(k) = v
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
        if let Value::Node(nd) = v
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
        if let Value::Node(nd) = v
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
        if let Value::Node(nd) = v
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
        if let Value::Node(nd) = &self.r.list[i]
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

        if let Value::Node(nd) = &self.r.list[i]
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
      nd.print();
    }
}



}




