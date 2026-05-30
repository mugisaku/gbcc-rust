

use std::rc::Rc;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::asm::*;
use super::scope::*;
use super::assemble::assemble;
use super::evaluate::*;
use super::evaluate_const::*;
use super::tplg_sort::*;




pub enum
SymbolKind
{
      Const(i64),
  GlobalVar(i64),

  Fn(FnDecl),

}




struct
Source
{
  decl: Decl,

  deps_parent_list: Vec<String>,
   deps_child_list: Vec<String>,

}


pub struct
Symbol
{
  name: String,

  kind: SymbolKind,

  offset: usize,

  deps_parent_list: Vec<String>,
   deps_child_list: Vec<String>,

}


impl
Symbol
{


pub fn
build(src: Source, symtbl: &SymbolTable)-> Self
{
  let  (name,kind) = src.decl.expire();

  let  deps_parent_list = src.deps_parent_list;
  let   deps_child_list =  src.deps_child_list;

    match kind
    {
  DeclKind::Undef=>{panic!("symbol build error: {} is undef",&name);}
  DeclKind::Const(e)=>
    {
      let  res = evaluate_const(&e,symtbl,None);

      Self{
        name,
        kind: SymbolKind::Const(res.unwrap()),
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Var(e)=>
    {
      let  res = evaluate_const(&e,symtbl,None);

      Self{
        name,
        kind: SymbolKind::GlobalVar(res.unwrap()),
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Fn(fd)=>
    {
      Self{
        name,
        kind: SymbolKind::Fn(fd),
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  _=>{panic!();}
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
get_offset(&self)-> usize
{
  self.offset
}


pub fn
get_reference_count(&self)-> usize
{
  self.deps_child_list.len()
}


pub fn
print(&self)
{
    match &self.kind
    {
  SymbolKind::Const(_)    =>{print!("const");}
  SymbolKind::GlobalVar(_)=>{print!("(g)var");}
  SymbolKind::Fn(_)       =>{print!("fn");}
    }


  print!(" {}: ",&self.name);

    match &self.kind
    {
  SymbolKind::Const(res)    =>{print!("{}",res);}
  SymbolKind::GlobalVar(res)=>{print!("{}",res);}
  SymbolKind::Fn(_,)=>{}
    }


  println!("");
  println!("offset: {}",self.offset);
  println!("");

    for name in &self.deps_parent_list
    {
      println!("this requires {}",name);
    }


    for name in &self.deps_child_list
    {
      println!("this is required by {}",name);
    }


  println!("reference count: {}",self.get_reference_count());

  println!("");
}


}




pub struct
SymbolTable
{
  symbols: Vec<Symbol>,

}


impl
SymbolTable
{


pub const fn
new()-> Self
{
  Self{
    symbols: Vec::new(),
  }
}


fn
join_child(srcs: &mut Vec<Source>, parent_name: &String, child_name: String)
{
    for src in srcs
    {
        if src.decl.get_name() == parent_name
        {
          src.deps_child_list.push(child_name);

          return;
        }
    }


  panic!("join_child error: {} and {}",parent_name,&child_name);
}


fn
make_tplg_sorted_names(srcs: &Vec<Source>)-> Vec<String>
{
  let  mut buf = Vec::<TplgNode>::new();

    for src in srcs
    {
      let  nd = TplgNode::new(src.decl.get_name().clone(),src.deps_child_list.clone(),src.deps_parent_list.len());

      buf.push(nd);
    }


    if let Ok(names) = tplg_sort(buf)
    {
      return names;
    }


  panic!();
}


fn
take_source(srcs: &mut Vec<Source>, name: &str)-> Source
{
    for src in srcs
    {
        if src.decl.get_name() == name
        {
          let  mut tmp = Source{decl: Decl::new(), deps_parent_list: Vec::new(), deps_child_list: Vec::new()};

          std::mem::swap(src,&mut tmp);

          return tmp;
        }
    }


  panic!();
}


fn
generate_symbols(&mut self, mut srcs: Vec<Source>)
{
  let  names = Self::make_tplg_sorted_names(&srcs);

    for name in names
    {
      let  src = Self::take_source(&mut srcs,&name);

      let  sym = Symbol::build(src,self);

      self.symbols.push(sym);
    }
}


pub fn
build(decls: Vec<Decl>)-> Result<Self,()>
{
  let  mut tbl = Self::new();

  let  mut srcs = Vec::<Source>::new();

    for decl in decls
    {
      let  src = Source{decl, deps_parent_list: Vec::new(), deps_child_list: Vec::new()};

      srcs.push(src);
    }


    for i in 0..srcs.len()
    {
      let  mut buf = Vec::<Collectible>::new();

      srcs[i].decl.collect(&mut buf);

        for co in buf
        {
            match co
            {
          Collectible::Identifier(s)=>
            {
              let  child_name = srcs[i].decl.get_name().clone();

              Self::join_child(&mut srcs,&s,child_name);

              srcs[i].deps_parent_list.push(s);
            }
            }
        }
    }


  tbl.generate_symbols(srcs);

  Ok(tbl)
}


fn
generate_data(&mut self, start: usize)-> Vec<u8>
{
  let  mut bytes = Vec::<u8>::new();
  let  mut pos = start;

    for sym in &mut self.symbols
    {
      sym.offset = get_word_aligned(pos)            ;
                                    pos += WORD_SIZE;
    }


  bytes.resize(pos-start,0);

    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Const(res)=>
        {
          let  res_bytes = res.to_ne_bytes();

            for i in 0..res_bytes.len()
            {
              bytes[(sym.offset-start)+i] = res_bytes[i];
            }
        }
      SymbolKind::GlobalVar(res)=>
        {
          let  res_bytes = res.to_ne_bytes();

            for i in 0..res_bytes.len()
            {
              bytes[(sym.offset-start)+i] = res_bytes[i];
            }
        }
      _=>{}
        }
    }


  bytes
}


pub fn
find_text_offset(ls: &Vec<(String,Vec<u8>,usize)>, name: &str)-> usize
{
    for (text_name,_,offset) in ls
    {
        if text_name == name
        {
          return *offset;
        }
    }


  panic!();
}


pub fn
generate_exec(&mut self)-> Exec
{
  let  mut exec = Exec::new();

  exec.data_start      = self.find_const("DATA_START").unwrap() as usize;
  exec.text_start      = self.find_const("TEXT_START").unwrap() as usize;
  exec.heap_start      = self.find_const("HEAP_START").unwrap() as usize;
  exec.heap_size       = self.find_const("HEAP_SIZE").unwrap() as usize;
  exec.stack_start     = self.find_const("STACK_START").unwrap() as usize;
  exec.stack_size      = self.find_const("STACK_SIZE").unwrap() as usize;
  exec.callstack_start = self.find_const("CALLSTACK_START").unwrap() as usize;
  exec.callstack_size  = self.find_const("CALLSTACK_SIZE").unwrap() as usize;

  exec.data_start      = get_word_aligned(exec.data_start);
  exec.text_start      = get_word_aligned(exec.text_start);
  exec.heap_start      = get_word_aligned(exec.heap_start);
  exec.stack_start     = get_word_aligned(exec.stack_start);
  exec.callstack_start = get_word_aligned(exec.callstack_start);

  exec.data_bytes = self.generate_data(exec.data_start);

  let  mut pos = exec.text_start;

    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Fn(fd)=>
        {
          let   ptr_minsym = MiniSymbol{offset: sym.offset, name: sym.name.clone(), is_text: false};
          let  text_minsym = MiniSymbol{offset:        pos, name: sym.name.clone(), is_text:  true};

          exec.mini_symbols.push( ptr_minsym);
          exec.mini_symbols.push(text_minsym);


          let  mut text = assemble(fd,self);
println!("{}{{",&sym.name);
text.print();
println!("}}");

          text.finalize();

          let  bytes = text.to_bytes();

            for b in &bytes
            {
              exec.text_bytes.push(*b);
            }


          exec.texts.push((sym.name.clone(),text));

          let  pos_bytes = pos.to_ne_bytes();

            for i in 0..pos_bytes.len()
            {
              exec.data_bytes[(sym.offset-exec.data_start)+i] = pos_bytes[i];
            }


          pos += bytes.len();
        }
      SymbolKind::GlobalVar(_)=>
        {
          exec.mini_symbols.push(MiniSymbol{offset: sym.offset, name: sym.name.clone(), is_text: false});
        }
      _=>{}
        }
    }


  exec
}




pub fn
find_symbol_index(&self, name: &str)-> Option<usize>
{
    for i in 0..self.symbols.len()
    {
        if &self.symbols[i].name == name
        {
          return Some(i);
        }
    }


  None
}


pub fn
find_symbol(&self, name: &str)-> Option<&Symbol>
{
    for sym in &self.symbols
    {
        if &sym.name == name
        {
          return Some(sym);
        }
    }


  None
}


pub fn
find_symbol_mut(&mut self, name: &str)-> Option<&mut Symbol>
{
    for sym in &mut self.symbols
    {
        if &sym.name == name
        {
          return Some(sym);
        }
    }


  None
}


pub fn
find_const(&self, name: &str)-> Option<i64>
{
    if let Some(sym) = self.find_symbol(name)
    {
        if let SymbolKind::Const(v) = &sym.kind
        {
          return Some(*v);
        }
    }


  None
}


pub fn
print(&self)
{
  println!("}}\nglobal symbols{{");

    for sym in &self.symbols
    {
      sym.print();

      println!("");
    }


  println!("}}");
}


}




#[derive(Clone)]
pub struct
MiniSymbol
{
  offset: usize,
    name: String,

  is_text: bool,

}


impl
MiniSymbol
{


pub fn  get_offset(&self)-> usize{self.offset}
pub fn  get_name(&self)-> &String{&self.name}
pub fn  is_text(&self)-> bool{self.is_text}


}




pub struct
Exec
{
  mini_symbols: Vec<MiniSymbol>,

  texts: Vec<(String,AsmText)>,

  data_start: usize,
  data_bytes: Vec<u8>,

  text_start: usize,
  text_bytes: Vec<u8>,

  heap_start: usize,
  heap_size: usize,

  stack_start: usize,
  stack_size: usize,

  callstack_start: usize,
  callstack_size: usize,

}


impl
Exec
{


pub const fn
new()-> Self
{
  Self{
    mini_symbols: Vec::new(),
    texts: Vec::new(),
    data_start: 0,
    data_bytes: Vec::new(),
    text_start: 0,
    text_bytes: Vec::new(),
    heap_start: 0,
    heap_size: 0,
    stack_start: 0,
    stack_size: 0,
    callstack_start: 0,
    callstack_size: 0,
  }
}


pub fn
get_mini_symbols(&self)-> &Vec<MiniSymbol>
{
  &self.mini_symbols
}


pub fn
get_data_start(&self)-> usize
{
  self.data_start
}


pub fn
get_data_bytes(&self)-> &Vec<u8>
{
  &self.data_bytes
}


pub fn
get_text_start(&self)-> usize
{
  self.text_start
}


pub fn
get_text_bytes(&self)-> &Vec<u8>
{
  &self.text_bytes
}


pub fn
get_heap_start(&self)-> usize
{
  self.heap_start
}


pub fn
get_heap_size(&self)-> usize
{
  self.heap_size
}


pub fn
get_stack_start(&self)-> usize
{
  self.stack_start
}


pub fn
get_stack_size(&self)-> usize
{
  self.stack_size
}


pub fn
get_callstack_start(&self)-> usize
{
  self.callstack_start
}


pub fn
get_callstack_size(&self)-> usize
{
  self.callstack_size
}


pub fn
get_memory_size(&self)-> usize
{
  let  mut sz = 0usize;

  sz = std::cmp::max(sz,self.data_start+self.data_bytes.len());
  sz = std::cmp::max(sz,self.text_start+self.text_bytes.len());
  sz = std::cmp::max(sz,self.heap_start+self.heap_size);
  sz = std::cmp::max(sz,self.stack_start+self.stack_size);
  sz = std::cmp::max(sz,self.callstack_start+self.callstack_size);

  sz
}


pub fn
generate_memory(&self)-> Vec<u8>
{
  let  mut mem = Vec::<u8>::new();

  let  memsz = self.get_memory_size();

    if self.data_start+self.data_bytes.len() >= memsz
    {
      panic!();
    }


    if self.text_start+self.text_bytes.len() >= memsz
    {
      panic!();
    }


  mem.resize(memsz,0);

    unsafe
    {
        for i in 0..self.data_bytes.len()
        {
          let  v = *self.data_bytes.get_unchecked(i);

          *mem.get_unchecked_mut(self.data_start+i) = v;
        }


        for i in 0..self.text_bytes.len()
        {
          let  v = *self.text_bytes.get_unchecked(i);

          *mem.get_unchecked_mut(self.text_start+i) = v;
        }
    }


  mem
}


pub fn
find_entry_point(&self, name: &str)-> Option<usize>
{
    for sym in &self.mini_symbols
    {
        if sym.is_text && (&sym.name == name)
        {
          return Some(sym.offset);
        }
    }


  None
}


pub fn
print_memory(&self, mem: &Vec<u8>)
{
    for sym in &self.mini_symbols
    {
      let  off = sym.get_offset();

      print!("{}({})",sym.get_name(),off);

        if sym.is_text()
        {
          println!("");
        }

      else
        {
          let  i64_ptr = unsafe{mem.as_ptr().add(off)} as *const i64;

          println!(": {}",unsafe{*i64_ptr});
        }
    }
}


pub fn
print_text(&self)
{
    for (name,text) in &self.texts
    {
      println!("{}\n{{",name);

      text.print();

      println!("}}");
    }
}


}




