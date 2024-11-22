

use super::space::{
  VariableDecl,

};


use super::statement::{
  Statement,
  Block,

};


use super::memory::{
  Memory,

};


use super::evaluator::{
  Instruction,

};


use super::type_info::{
  WORD_SIZE,
  Align,
  Parameter,
  TypeKind,
  TypeInfo,
  StorageInfo,

};




pub enum
SymbolKind
{
  Type(TypeInfo),
  Constant(TypeInfo,Vec<u8>),

  Global(TypeInfo,Vec<u8>),
   Local(TypeInfo),
  Argument(TypeInfo),
  Function(Vec<(String,TypeInfo)>,TypeInfo),

}


pub struct
Symbol
{
  pub(crate) name: String,
  pub(crate) offset: usize,
  pub(crate) kind: SymbolKind,

}


pub struct
SymbolDirectory
{
  pub(crate) parent_ptr: *const Self,

  pub(crate) symbol_list: Vec<Symbol>,

  pub(crate) allocation_size:   usize,

}


impl
SymbolDirectory
{


pub fn
new(parent: &Self)-> Self
{
  Self{
    parent_ptr: parent as *const Self,
    symbol_list: Vec::new(),
    allocation_size: 0,
  }
}


pub fn
new_as_root()-> Self
{
  Self{
    parent_ptr: std::ptr::null(),
    symbol_list: Vec::new(),
    allocation_size: 0,
  }
}


pub fn
build_symbol_block(blk: &Block, dir: &mut Self)
{
}


pub fn
add_type(&mut self, name: &str, ti: TypeInfo)
{
  let  sym = Symbol{name: name.to_string(), offset: 0, kind: SymbolKind::Type(ti)};

  self.symbol_list.push(sym);
}


pub fn
add_constant(&mut self, name: &str, ti: TypeInfo, b: Vec<u8>)
{
  let  sym = Symbol{name: name.to_string(), offset: 0, kind: SymbolKind::Constant(ti,b)};

  self.symbol_list.push(sym);
}


pub fn
add_global(&mut self, name: &str, ti: TypeInfo, b: Vec<u8>)
{
  let  offset = Align::default().correct(self.allocation_size);

  let  sz = ti.get_size();

  let  sym = Symbol{name: name.to_string(), offset, kind: SymbolKind::Global(ti,b)};

  self.symbol_list.push(sym);

  self.allocation_size = offset+sz;
}


pub fn
add_function(&mut self, name: &str, params: Vec<(String,TypeInfo)>, ret_ti: TypeInfo)
{
  let  offset = Align::default().correct(self.allocation_size);

  let  sym = Symbol{name: name.to_string(), offset, kind: SymbolKind::Function(params,ret_ti)};

  self.symbol_list.push(sym);

  self.allocation_size = offset+WORD_SIZE;
}


pub fn
add_local(&mut self, name: &str, ti: TypeInfo)
{
  let  offset = Align::default().correct(self.allocation_size);

  let  sz = ti.get_size();

  let  sym = Symbol{name: name.to_string(), offset, kind: SymbolKind::Local(ti)};

  self.symbol_list.push(sym);

  self.allocation_size = offset+sz;
}


pub fn
add_argument(&mut self, name: &str, ti: TypeInfo)
{
  self.allocation_size += ti.get_size();

  let  offset = Align::default().correct(self.allocation_size);

  let  sym = Symbol{name: name.to_string(), offset, kind: SymbolKind::Argument(ti)};

  self.symbol_list.push(sym);
}


pub fn
check_name_duplication(&self)-> bool
{
  let  mut name_ls: Vec<&str> = Vec::new();

    for sym in &self.symbol_list
    {
        for name in &name_ls
        {
            if &sym.name == name
            {
              return true;
            }
        }


      name_ls.push(&sym.name);
    }


  false
}


pub fn
get_allocation_offset_max(&self, base: usize)-> usize
{
  let  next_base = Align::default().correct(base+self.allocation_size);

  let  mut max: usize = next_base;

    for sym in &self.symbol_list
    {
        if let SymbolKind::Function(_,_) = &sym.kind
        {
//              max = std::cmp::max(max,symblk.directory.get_allocation_offset_max(next_base));
        }
    }


  max
}


fn
get_parent(&self)-> Option<&Self>
{
    if self.parent_ptr != std::ptr::null()
    {
      return Some(unsafe{&*self.parent_ptr});
    }


  None
}


pub fn
find_any(&self, name: &str)-> Option<&Symbol>
{
    for sym in self.symbol_list.iter().rev()
    {
        if &sym.name == name
        {
          return Some(sym);
        }
    }


    if let Some(parent) = self.get_parent()
    {
      return parent.find_any(name);
    }


  None
}


pub fn
find_type(&self, name: &str)-> Option<&TypeInfo>
{
    for sym in self.symbol_list.iter().rev()
    {
        if &sym.name == name
        {
            if let SymbolKind::Type(ti) = &sym.kind
            {
              return Some(&ti);
            }
        }
    }


    if let Some(parent) = self.get_parent()
    {
      return parent.find_type(name);
    }


  None
}


}




