

use crate::token::Token;
use crate::token::TokenInfo;
use crate::token::TokenData;
use super::dictionary::{
  Definition,
  Dictionary,
  Expression,
  BinaryOperation,
  BinaryOperator,
  Operand,
};


pub fn
print_indent(n: usize)
{
  let mut  i = 0;

    while i < n
    {
      print!("  ");

      i +=1;
    }
}




pub struct
Directory
{
  name: String,

  objects: Vec<Object>,

}


impl
Directory
{


pub fn
new(name: &str)-> Directory
{
  Directory{ name: String::from(name), objects: Vec::new()}
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
print(&self)
{
  print!("[{}] START\n",&self.name);

    for o in &self.objects
    {
      o.print();

      print!("\n");
    }


  print!("[{}] END\n",*self.name);
}


}


pub enum
ObjectData
{
  Null,

  Integer(u64),
  Floating(f64),
  String(String),
  Identifier(String),
  Character(char),

  Mark(String),

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
from(tok: &Token)-> Object
{
  let  inf = tok.get_info().clone();

    match tok.get_data()
    {
  TokenData::Integer(i)=>    {return Object{ token_info: inf, data: ObjectData::Integer(*i)};},
  TokenData::Floating(f)=>   {return Object{ token_info: inf, data: ObjectData::Floating(*f)};},
  TokenData::Character(c)=>  {return Object{ token_info: inf, data: ObjectData::Character(*c)};},
  TokenData::String(s)=>     {return Object{ token_info: inf, data: ObjectData::String(s.clone())};},
  TokenData::Identifier(s)=> {return Object{ token_info: inf, data: ObjectData::Identifier(s.clone())};},
  _=>                        {return Object{ token_info: inf, data: ObjectData::Null};},
    }
}


pub fn
new_as_mark(s: Rc<String>)-> Object
{
  Object{ token_info: TokenInfo::new(""), data: ObjectData::Mark(s)}
}


pub fn
new_as_directory(dir: Directory)-> Object
{
  Object{ token_info: TokenInfo::new(""), data: ObjectData::Directory(dir)}
}


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
print(&self)
{
    match &self.data
    {
  ObjectData::Null=>{},

  ObjectData::Integer(i)=>{print!("{}",i);},
  ObjectData::Floating(f)=>{print!("{}",f);},
  ObjectData::String(s)=>{print!("{}",s);},
  ObjectData::Identifier(s)=>{print!("{}",s);},
  ObjectData::Character(c)=>{print!("{}",c);},

  ObjectData::Mark(s)=>{print!("{}",s);},

  ObjectData::Directory(d)=>
        {
          d.print();
        },
    }
}


}




pub struct
Cursor<'a>
{
  directory: &'a Directory,
      index: usize,

}


impl<'a>
Cursor<'a>
{


pub fn
from(dir: &'a Directory)-> Cursor<'a>
{
  Cursor{ directory: dir, index: 0}
}


pub fn
get(&self)-> Option<&Object>
{
    if self.index < self.directory.objects.len()
    {
      return Some(&self.directory.objects[self.index]);
    }


  None
}


pub fn
test_mark(&self, s: &str)-> bool
{
    if let Some(o) = self.get()
    {
        if let ObjectData::Mark(rcs) = &o.data
        {
          return **rcs == s;
        }
    }


  false
}


pub fn
get_mark(&self)-> Option<&str>
{
    if let Some(o) = self.get()
    {
        if let ObjectData::Mark(rcs) = &o.data
        {
          return Some(&**rcs);
        }
    }


  None
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let Some(o) = self.get()
    {
        if let ObjectData::Identifier(s) = &o.data
        {
          return Some(s);
        }
    }


  None
}


pub fn
get_directory(&self)-> Option<&Directory>
{
    if let Some(o) = self.get()
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
    if let Some(o) = self.get()
    {
        if let ObjectData::Directory(d) = &o.data
        {
            if *d.name == name
            {
              return Some(d);
            }
        }
    }


  None
}


pub fn
seek_directory(&mut self, name: &str)-> Option<&Directory>
{
  let  objs = &self.directory.objects;

  let  l = objs.len();

    while self.index < l
    {
        if let ObjectData::Directory(d) = &objs[self.index].data
        {
            if d.name == name
            {
              return Some(d);
            }
        }


      self.index += 1;
    }


  None
}


pub fn
advance(&mut self, off: usize)-> bool
{
    if self.index < self.directory.objects.len()
    {
      self.index += 1;

      return true;
    }


  false
}


pub fn
is_finished(&self)-> bool
{
  self.index >= self.directory.objects.len()
}


}




pub struct
Packet
{
  index: usize,

  objects: Vec<Object>,

}


pub fn
seek(toks: &Vec<Token>, i: usize)-> usize
{
    if (i+1) < toks.len()
    {
      let  dat = &toks[i].get_data();

        if let TokenData::Space = dat
        {
          return seek(toks,i+1);
        }

      else
        if let TokenData::Newline = dat
        {
          return seek(toks,i+1);
        }
    }


  i
}


pub fn
read_by_string(dic: &Dictionary, toks: &Vec<Token>, i: usize, s: &String)-> Option<Packet>
{
  let mut  buf = new_char_string();

  let mut  offset = 0;

/*
    while offset < s.len()
    {
      let  pos = i+offset;

        if pos < toks.len()
        {
          let  dat = toks[pos].get_data();

            if let TokenData::Identifier(s) = dat
            {
              buf.push_str(&*s);

              offset += 1;

              break;
            }

          else
            if let TokenData::Others(c) = dat
            {
              buf.push(*c);

              offset += 1;
            }

          else
            {
              return None;
            }
        }

      else
        {
          return None;
        }
    }


    if buf == **s
    {
      let  o = Object::new_as_mark(s.clone());

      return Some(Packet{ index: i+offset, objects: vec![o]});
    }
*/


  None
}


pub fn
read_by_identifier(dic: &Dictionary, toks: &Vec<Token>, i: usize, s: &str)-> Option<Packet>
{
    if i < toks.len()
    {
      let  dat = &toks[i].get_data();

        if s == "IDENTIFIER"
        {
            if let TokenData::Identifier(_) = dat
            {
              let  o = Object::from(&toks[i]);

              return Some(Packet{ index: i+1, objects: vec![o]});
            }
        }

      else
        if s == "INTEGER_LITERAL"
        {
            if let TokenData::Integer(_) = dat
            {
              let  o = Object::from(&toks[i]);

              return Some(Packet{ index: i+1, objects: vec![o]});
            }
        }

      else
        if s == "FLOATING_LITERAL"
        {
            if let TokenData::Floating(_) = dat
            {
              let  o = Object::from(&toks[i]);

              return Some(Packet{ index: i+1, objects: vec![o]});
            }
        }

      else
        if s == "CHARACTER_LITERAL"
        {
            if let TokenData::Character(_) = dat
            {
              let  o = Object::from(&toks[i]);

              return Some(Packet{ index: i+1, objects: vec![o]});
            }
        }

      else
        if s == "STRING_LITERAL"
        {
            if let TokenData::String(_) = dat
            {
              let  o = Object::from(&toks[i]);

              return Some(Packet{ index: i+1, objects: vec![o]});
            }
        }

      else
        if let Some(pac) = read_by_name(dic,toks,i,s)
        {
          return Some(pac);
        }
    }


  None
}


pub fn
read_by_primary_expression(dic: &Dictionary, toks: &Vec<Token>, i: usize, pr: &PrimaryExpression)-> Option<Packet>
{
    match pr
    {
  PrimaryExpression::None=>
        {
        },
  PrimaryExpression::One(e)=>
        {
          return read_by_expression(dic,toks,i,&e);
        },
  PrimaryExpression::Option(e)=>
        {
            if let Some(pac) = read_by_expression(dic,toks,i,&e)
            {
              return Some(pac);
            }


          return Some(Packet{ index: i, objects: vec![]});
        },
  PrimaryExpression::Repetition(e)=>
        {
            if let Some(first_pac) = read_by_expression(dic,toks,i,&e)
            {
              let mut  objects: Vec<Object> = first_pac.objects;

              let mut  mi = first_pac.index;

                while let Some(mut pac) = read_by_expression(dic,toks,mi,&e)
                {
                  objects.append(&mut pac.objects);

                  mi = pac.index;
                }


              return Some(Packet{ index: mi, objects: objects});
            }
        },
  PrimaryExpression::Identifier(s)=>
        {
            if let Some(pac) = read_by_identifier(dic,toks,seek(toks,i),s.as_str())
            {
              return Some(pac);
            }
        },
  PrimaryExpression::String(s)=>
        {
            if let Some(pac) = read_by_string(dic,toks,seek(toks,i),s)
            {
              return Some(pac);
            }
        },
    }


  None
}


pub fn
read_by_binary_operation(dic: &Dictionary, toks: &Vec<Token>, i: usize, op: &BinaryOperation)-> Option<Packet>
{
    match op.get_operator()
    {
  BinaryOperator::And=>
        {
            if let Some(mut lpac) = read_by_expression(dic,toks,i,op.get_left())
            {
                if let Some(mut rpac) = read_by_expression(dic,toks,lpac.index,op.get_right())
                {
                  lpac.objects.append(&mut rpac.objects);

                  return Some(Packet{ index: rpac.index, objects: lpac.objects});
                }
            }


          return None;
        },
  BinaryOperator::Or=>
        {
            if let Some(pac) = read_by_expression(dic,toks,i,op.get_left())
            {
              return Some(pac);
            }


            if let Some(pac) = read_by_expression(dic,toks,i,op.get_right())
            {
              return Some(pac);
            }
        },
    }


  None
}


pub fn
read_by_expression(dic: &Dictionary, toks: &Vec<Token>, i: usize, e: &Expression)-> Option<Packet>
{
    match e
    {
  Expression::Empty=>{},
  Expression::UnaryOperation(op)=>
        {
            if let Some(pac) = read_by_expression(dic,toks,i,op.get_operand())
            {
              return Some(pac);
            }
        },
  Expression::BinaryOperation(op)=>
        {
          return read_by_binary_operation(dic,toks,i,op);
        },
  Expression::Primary(pr)=>
        {
          return read_by_primary_expression(dic,toks,i,pr);
        },
    }


  None
}


pub fn
read_by_definition(dic: &Dictionary, toks: &Vec<Token>, i: usize, def: &Definition)-> Option<Packet>
{
    if let Some(mut pac) = read_by_expression(dic,toks,i,def.get_expression())
    {
      let  dir = Directory{ name: def.clone_name(), objects: pac.objects};

      return Some(Packet{ index: pac.index, objects: vec![Object::new_as_directory(dir)]});
    }


  None
}


pub fn
read_by_name(dic: &Dictionary, toks: &Vec<Token>, i: usize, name: &str)-> Option<Packet>
{
    if let Some(def) = dic.find(name)
    {
        if let Some(pac) = read_by_definition(dic,toks,i,def)
        {
          return Some(pac);
        }
    }


  None
}


pub fn
parse(dic: &Dictionary, toks: &Vec<Token>)-> Option<Directory>
{
    if toks.len() == 0
    {
      return None;
    }


  let mut  dir = Directory::new("/");

    if let Some(first) = dic.get_first() 
    {
      let mut  i = 0;

        while let Some(mut pac) = read_by_definition(dic,toks,i,first)
        {
          i = pac.index;

          dir.objects.append(&mut pac.objects);
        }
    }


  Some(dir)
}




