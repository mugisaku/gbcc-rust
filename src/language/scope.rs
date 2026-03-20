

use std::cell::Cell;
use std::rc::Rc;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::symbol_table::*;




pub enum
LocalSymbolKind
{
  Parameter,

  Static,
  Const,
    Var,

}


pub struct
LocalSymbol
{
  name: String,

  kind: LocalSymbolKind,

  ty: Ty,

  value: SymbolValue,

  offset: usize,
    size: usize,

}


impl
LocalSymbol
{


pub fn
new_parameter(name: &str, ty: Ty, offset: usize)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Parameter,
    ty,
    value: SymbolValue::Void,
    offset: offset+WORD_SIZE,
    size: WORD_SIZE,
  }
}


pub fn
new_const_bool(name: &str, b: bool)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    ty: Ty::Bool,
    value: SymbolValue::Bool(b),
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
    ty: Ty::Int,
    value: SymbolValue::Int(i),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_const_int_v(name: &str, i: i64)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Const,
    ty: Ty::Int,
    value: SymbolValue::IntV(std::cell::Cell::new(i)),
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
    ty: Ty::Float,
    value: SymbolValue::Float(f),
    offset: 0,
    size: WORD_SIZE,
  }
}


pub fn
new_var(name: &str, ty: Ty, offset: usize)-> Self
{
  Self{
    name: name.to_string(),
    kind: LocalSymbolKind::Var,
    ty,
    value: SymbolValue::Void,
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
get_ty(&self)-> &Ty
{
  &self.ty
}


pub fn
get_value(&self)-> &SymbolValue
{
  &self.value
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
new_root(name_ls: &Vec<String>, ty_ls: &Vec<Ty>)-> Self
{
  let  mut symbol_list = Vec::<LocalSymbol>::new();

  let  mut name_iter = name_ls.iter();
  let  mut   ty_iter =   ty_ls.iter();

  let  mut off = 0usize;

    while let Some(name) = name_iter.next()
    {
        if let Some(ty) = ty_iter.next()
        {
          let  sym = LocalSymbol::new_parameter(name,ty.clone(),off);

          off = sym.offset;

          symbol_list.push(sym);
        }

      else{panic!();}
    }


  Self{
    previous_opt: None,
    symbol_list,
    offset: 0,
    offset_max: Rc::new(Cell::new(0)),
  }
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
add_var(&mut self, name: &str, ty: Ty)-> usize
{
  let  offset = self.offset;

  let  sym = LocalSymbol::new_var(name,ty,offset);

  self.offset = get_word_aligned(offset+sym.size);

  self.symbol_list.push(sym);

  self.update_offset_max();

  offset
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




