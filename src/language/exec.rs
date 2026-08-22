

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::asm::*;
use super::scope::*;
use super::evaluate::*;

use crate::source_file::{
  SourceInfo,
  Error,

};




pub struct
Exec
{
  symbols: Vec<Symbol>,

  texts: Vec<(String,usize,AsmText)>,

  memory: Vec<u8>,

}


impl
Exec
{


pub const MEMORY_SIZE: usize = 0x4000000;

pub const MEMORY_MASK1: usize = 0x3FFFFFF;
pub const MEMORY_MASK2: usize = 0x3FFFFFE;
pub const MEMORY_MASK4: usize = 0x3FFFFFC;
pub const MEMORY_MASK8: usize = 0x3FFFFF8;


pub const fn
new()-> Self
{
  Self{
    symbols: Vec::new(),
    texts: Vec::new(),
    memory: Vec::new(),
  }
}


pub fn
new_with_memory()-> Self
{
  let  mut e = Self{
    symbols: Vec::new(),
    texts: Vec::new(),
    memory: Vec::new(),
  };


  e.initialize_memory();

  e
}


pub fn
initialize_memory(&mut self)
{
  self.memory.resize(Self::MEMORY_SIZE,0);
}


pub fn
get_symbols(&self)-> &Vec<Symbol>
{
  &self.symbols
}


pub fn
add_symbol(&mut self, sym: Symbol)
{
  self.symbols.push(sym);
}


pub fn
get_memory(&self)-> &Vec<u8>
{
  &self.memory
}


pub fn
get_memory_mut(&mut self)-> &mut Vec<u8>
{
  &mut self.memory
}


pub fn
get_memory_slice_mut(&mut self, start: usize)-> &mut [u8]
{
  &mut self.memory[start..]
}



pub fn
get_ptr(&self, off: usize)-> *const u8
{
  unsafe{self.memory.as_ptr().add(off)}
}


pub fn
get_mut_ptr(&mut self, off: usize)-> *mut u8
{
  unsafe{self.memory.as_mut_ptr().add(off)}
}


pub fn
get_u8(&self, off: usize)-> u8
{
  unsafe{*self.get_ptr(off&Self::MEMORY_MASK1)}
}


pub fn
put_u8(&mut self, off: usize, v: u8)
{
  unsafe{*self.get_mut_ptr(off&Self::MEMORY_MASK1) = v;}
}


pub fn
get_u16(&self, off: usize)-> u16
{
  unsafe{*(self.get_ptr(off&Self::MEMORY_MASK2) as *const u16)}
}


pub fn
put_u16(&mut self, off: usize, v: u16)
{
  unsafe{*(self.get_mut_ptr(off&Self::MEMORY_MASK2) as *mut u16) = v;}
}


pub fn
get_u32(&self, off: usize)-> u32
{
  unsafe{*(self.get_ptr(off&Self::MEMORY_MASK4) as *const u32)}
}


pub fn
put_u32(&mut self, off: usize, v: u32)
{
  unsafe{*(self.get_mut_ptr(off&Self::MEMORY_MASK4) as *mut u32) = v;}
}


pub fn
get_u64(&self, off: usize)-> u64
{
  unsafe{*(self.get_ptr(off&Self::MEMORY_MASK8) as *const u64)}
}


pub fn
put_u64(&mut self, off: usize, v: u64)
{
  unsafe{*(self.get_mut_ptr(off&Self::MEMORY_MASK8) as *mut u64) = v;}
}


pub fn
put_bytes(&mut self, mut off: usize, bytes: &[u8])
{
    for b in bytes
    {
      self.memory[off] = *b;

      off += 1;
    }
}


pub fn
add_text(&mut self, txt: (String,usize,AsmText))
{
  self.texts.push(txt);
}


pub fn
find_data_address(&self, name: &str)-> Option<usize>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Static(_) = sym.get_kind()
        {
            if sym.get_name() == name
            {
              return Some(sym.get_offset() as usize);
            }
        }
    }


  None
}


pub fn
find_const(&self, name: &str)-> Option<i64>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Const(v) = sym.get_kind()
        {
            if sym.get_name() == name
            {
              return Some(*v);
            }
        }
    }


  None
}


pub fn
find_entry_point(&self, name: &str)-> Option<usize>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Text = sym.get_kind()
        {
            if sym.get_name() == name
            {
              return Some(sym.get_offset() as usize);
            }
        }
    }


  None
}


pub fn
print_memory_to(&self, buf: &mut String)
{
    for sym in &self.symbols
    {
      let  off = sym.get_offset() as usize;

      buf.push_str(sym.get_name());

        match sym.get_kind()
        {
      SymbolKind::Static(len)=>
        {
          buf.push_str(&format!("(addr: {})",off));
        }
      SymbolKind::Const(v)=>{buf.push_str(&format!(": {}",v));}
      _=>{}
        }


      buf.push_str("\n");
    }
}


pub fn
print_memory(&self)
{
  let  mut buf = String::new();

  self.print_memory_to(&mut buf);

  print!("{}",&buf);
}


pub fn
print_text_to(&self, buf: &mut String)
{
    for (name,start,text) in &self.texts
    {
      buf.push_str(&format!("{}({}){{\n",name,start));

      text.print_to(buf,*start);

      buf.push_str("}\n");
    }
}


pub fn
print_text(&self)
{
  let  mut buf = String::new();

  self.print_text_to(&mut buf);

  print!("{}",&buf);
}


}




