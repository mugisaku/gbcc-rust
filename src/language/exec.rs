

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::asm::*;
use super::scope::*;
use super::evaluate::*;
use super::evaluate_const::*;

use crate::source_file::{
  SourceInfo,
  Error,

};




#[derive(Clone)]
pub enum
SymbolKind
{
  Data, Text, Const(i64), Str(String,usize), Field(usize), Io,

}


#[derive(Clone)]
pub struct
Symbol
{
  offset: usize,
    name: String,

  kind: SymbolKind,

}


impl
Symbol
{


pub fn
new(offset: usize, name: String, kind: SymbolKind)-> Self
{
  Self{offset,name,kind}
}


pub fn  get_offset(&self)-> usize{self.offset}
pub fn  get_name(&self)-> &String{&self.name}
pub fn  get_kind(&self)-> &SymbolKind{&self.kind}


}



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
find_const(&self, name: &str)-> Option<i64>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Const(v) = &sym.kind
        {
            if &sym.name == name
            {
              return Some(*v);
            }
        }
    }


  None
}


pub fn
find_io(&self, name: &str)-> Option<usize>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Io = &sym.kind
        {
            if &sym.name == name
            {
              return Some(sym.offset);
            }
        }
    }


  None
}


pub fn
find_field(&self, name: &str)-> Option<(usize,usize)>
{
    for sym in &self.symbols
    {
        if let SymbolKind::Field(sz) = &sym.kind
        {
            if &sym.name == name
            {
              return Some((sym.offset,*sz));
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
        if let SymbolKind::Text = &sym.kind
        {
            if &sym.name == name
            {
              return Some(sym.offset);
            }
        }
    }


  None
}


pub fn
print_memory(&self)
{
    for sym in &self.symbols
    {
      let  off = sym.get_offset();

      print!("{}",sym.get_name());

        match &sym.kind
        {
      SymbolKind::Data
     |SymbolKind::Io=>
        {
          print!("(addr: {})",off);

          let  i64_ptr = unsafe{self.memory.as_ptr().add(off)} as *const i64;

          println!(": {}",unsafe{*i64_ptr});
        }
      SymbolKind::Const(v)=>{println!(": {}",v);}
      SymbolKind::Str(ty,n)=>
        {
          let  base = off;

          print!(" off: {}, ty: {} = {{",off,ty);
/*
               if ty ==  "i8"{for i in 0..*n{print!("{},",self.get_u8( base+i    )       );}}
          else if ty ==  "u8"{for i in 0..*n{print!("0x{:X},",self.get_u8( base+i    ) as  i8);}}
          else if ty == "i16"{for i in 0..*n{print!("{},",self.get_u16(base+(2*i))       );}}
          else if ty == "u16"{for i in 0..*n{print!("0x{:X},",self.get_u16(base+(2*i)) as i16);}}
          else if ty == "i32"{for i in 0..*n{print!("{},",self.get_u32(base+(4*i))       );}}
          else if ty == "u32"{for i in 0..*n{print!("0x{:X},",self.get_u32(base+(4*i)) as i32);}}
          else if ty == "i64"{for i in 0..*n{print!("{},",self.get_u64(base+(8*i)) as i64);}}
          else{panic!("{}",ty);}
*/
          println!("}}");
        }
      SymbolKind::Field(sz)=>
        {
          println!(" off: {}, sz: {}: {{...}}",off,*sz);
        }
      _=>{println!("");}
        }
    }
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




