

use std::rc::Rc;
use std::cell::Cell;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::evaluate_const::*;




pub enum
SymbolKind
{
  Data,
  Text, 

  Const(i64),
  Static,
     Var,

  Field(usize),

}


pub struct
Symbol
{
  name: String,

  kind: SymbolKind,

  offset: isize,

}


impl
Symbol
{


pub fn
new_data(name: &str, offset: isize)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Data,
    offset,
  }
}


pub fn
new_text(name: &str, offset: isize)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Text,
    offset,
  }
}


pub fn
new_const_bool(name: &str, b: bool)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Const(if b{1} else{0}),
    offset: 0,
  }
}


pub fn
new_const_int(name: &str, i: i64)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Const(i),
    offset: 0,
  }
}


pub fn
new_static(name: &str, offset: isize)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Static,
    offset,
  }
}


pub fn
new_var(name: &str, offset: isize)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Var,
    offset,
  }
}




pub fn
new_field(name: &str, offset: isize, sz: usize)-> Self
{
  Self{
    name: name.to_string(),
    kind: SymbolKind::Field(sz),
    offset,
  }
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_kind(&self)-> &SymbolKind
{
  &self.kind
}


pub fn
get_offset(&self)-> isize
{
  self.offset
}


}




pub struct
Scope<'a>
{
  previous_opt: Option<&'a Scope<'a>>,

  symbols: Vec<Symbol>,

  offset: usize,

  offset_max: Rc<Cell<usize>>,

}


impl<'a>
Scope<'a>
{


pub fn
new_root(decl: &FnDecl)-> Self
{
  let  mut scp = Self{
    previous_opt: None,
    symbols: Vec::new(),
    offset: 0,
    offset_max: Rc::new(Cell::new(0)),
  };


  let  arg_n = decl.get_parameter_names().len() as isize;

  let  mut off = -((WORD_SIZE as isize)*(3+arg_n));

    for name in decl.get_parameter_names()
    {
      scp.symbols.push(Symbol::new_var(name,off));

      off += (WORD_SIZE as isize);
    }


  scp
}


pub fn
new(&'a self)-> Self
{
  Self{
    previous_opt: Some(self),
    symbols: Vec::new(),
    offset: self.offset,
    offset_max: Rc::clone(&self.offset_max),
  }
}


pub fn
update_offset_max(&self)
{
  let  cur = self.offset_max.get();

    if cur < self.offset
    {
      self.offset_max.set(self.offset);
    }
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
get_offset_max(&self)-> usize
{
  self.offset_max.get()
}


pub fn
add_const_bool(&mut self, name: &str, b: bool)
{
  let  sym = Symbol::new_const_bool(name,b);

  self.symbols.push(sym);
}


pub fn
add_const_int(&mut self, name: &str, i: i64)
{
  let  sym = Symbol::new_const_int(name,i);

  self.symbols.push(sym);
}


pub fn
add_var(&mut self, name: &str)-> isize
{
  let  offset = self.offset as isize;

  let  sym = Symbol::new_var(name,offset);

  self.symbols.push(sym);

  self.offset += WORD_SIZE;

  self.update_offset_max();

  offset as isize
}


pub fn
add_static(&mut self, name: &str, offset: usize)
{
  let  sym = Symbol::new_static(name,offset as isize);

  self.symbols.push(sym);
}


pub fn
add_field(&mut self, name: &str, sz: usize)-> isize
{
  let  offset = self.offset as isize;

  let  sym = Symbol::new_field(name,offset,sz);

  self.symbols.push(sym);

  self.offset = get_word_aligned(self.offset+sz);

  self.update_offset_max();

  offset as isize
}


pub fn
find(&'a self, name: &str)-> Option<&'a Symbol>
{
    for decl in &self.symbols
    {
        if &decl.name == name
        {
          return Some(decl);
        }
    }


    if let Some(prev) = self.previous_opt
    {
      return prev.find(name);
    }


  None
}


}




