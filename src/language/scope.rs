

use std::rc::{Rc,Weak};
use std::cell::Cell;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::evaluate_const::*;
use super::symbol_table::*;




pub enum
LocalSymbolKind
{
  Static,
  Const,
    Var,

  IntV(Cell<usize>),

}


pub struct
LocalSymbol
{
  name: String,

  kind: LocalSymbolKind,

  value: EvalConstResult,

  ty_name: String,

  offset: usize,
    size: usize,

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
    value: EvalConstResult::Bool(b),
    ty_name: "bool".to_string(),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_const_int(name: &str, i: i64)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    value: EvalConstResult::Int(i),
    ty_name: "i64".to_string(),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_int_v(name: &str)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::IntV(Cell::new(0)),
    value: EvalConstResult::Void,
    ty_name: "i64".to_string(),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_const_float(name: &str, f: f64)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    value: EvalConstResult::Float(f),
    ty_name: "f64".to_string(),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_var(name: &str, ty_name: &str, offset: usize)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Var,
    value: EvalConstResult::Void,
    ty_name: ty_name.to_string(),
    offset,
    size: WORD_SIZE,
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
get_value(&self)-> &EvalConstResult
{
  &self.value
}


pub fn
get_ty_name(&self)-> &String
{
  &self.ty_name
}


pub fn
get_ty(&self)-> Rc<Ty>
{
  find_ty(&self.ty_name).unwrap()
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


    for pd in decl.get_parameter_decl_list()
    {
      let  ty = add_ty_from_node(pd.get_ty_node(),tbl);

      scp.add_var(pd.get_name(),ty.get_name());
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
add_const_float(&mut self, name: &str, f: f64)
{
  let  sym = LocalSymbol::new_const_float(name,f);

  self.symbol_list.push(sym);
}


pub fn
add_var(&mut self, name: &str, ty_name: &str)-> usize
{
  let  offset = self.offset;

  let  sym = LocalSymbol::new_var(name,ty_name,offset);

  self.offset = get_word_aligned(offset+sym.size);

  self.symbol_list.push(sym);

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




