

use std::rc::{Rc,Weak};
use std::cell::Cell;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::evaluate_const::*;
use super::symbol_table::*;




pub enum
LocalSymbolKind
{
  Const,
    Var,

  IntV(Cell<i64>),

}


pub struct
LocalSymbol
{
  name: String,

  kind: LocalSymbolKind,

  value: i64,

  offset: usize,

}


impl
LocalSymbol
{


pub fn
new_const_bool(name: &str, b: bool)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    value: if b{1} else{0},
    offset: 0,
  }
}


pub fn
new_const_int(name: &str, i: i64)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    value: i,
    offset: 0,
  }
}


pub fn
new_int_v(name: &str)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::IntV(Cell::new(0)),
    value: 0,
    offset: 0,
  }
}


pub fn
new_var(name: &str, offset: usize)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Var,
    value: 0,
    offset,
  }
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_kind(&self)-> &LocalSymbolKind
{
  &self.kind
}


pub fn
get_value(&self)-> i64
{
  self.value
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


}




pub struct
Scope<'a>
{
  previous_opt: Option<&'a Scope<'a>>,

  symbol_list: Vec<LocalSymbol>,

  offset: usize,

  offset_max: Rc<Cell<usize>>,

}


impl<'a>
Scope<'a>
{


pub fn
new_root(decl: &FnDecl, tbl: &SymbolTable)-> Self
{
  let  mut scp = Self{
    previous_opt: None,
    symbol_list: Vec::new(),
    offset: 0,
    offset_max: Rc::new(Cell::new(0)),
  };


    for name in decl.get_parameter_names()
    {
      scp.add_var(name);
    }


  scp
}


pub fn
new(&'a self)-> Self
{
  Self{
    previous_opt: Some(self),
    symbol_list: Vec::new(),
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
  let  sym = LocalSymbol::new_const_bool(name,b);

  self.symbol_list.push(sym);
}


pub fn
add_const_int(&mut self, name: &str, i: i64)
{
  let  sym = LocalSymbol::new_const_int(name,i);

  self.symbol_list.push(sym);
}


pub fn
add_var(&mut self, name: &str)-> usize
{
  let  offset = self.offset;

  let  sym = LocalSymbol::new_var(name,offset);

  self.symbol_list.push(sym);

  self.offset += WORD_SIZE;

  self.update_offset_max();

  offset
}


pub fn
add_int_v(&mut self, name: &str)
{
  let  sym = LocalSymbol::new_int_v(name);

  self.symbol_list.push(sym);
}


pub fn
find(&'a self, name: &str)-> Option<&'a LocalSymbol>
{
    for sym in &self.symbol_list
    {
        if &sym.name == name
        {
          return Some(sym);
        }
    }


    if let Some(prev) = self.previous_opt
    {
      return prev.find(name);
    }


  None
}


}




