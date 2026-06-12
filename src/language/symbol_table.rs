

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

  Io,

  Str(String,Vec<u8>),
  Field(usize),

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


fn
make_str_bytes(dk: &str, ik: &StrInitKind, symtbl: &SymbolTable)-> Vec<u8>
{
    if (dk == "i8") || (dk == "u8")
    {
        if let StrInitKind::String(s) = ik
        {
          return s.as_bytes().to_vec();
        }
    }


  let  mut   tmp = Vec::<i64>::new();
  let  mut bytes = Vec::<u8>::new();

    match ik
    {
  StrInitKind::String(s)=>
    {
        for c in s.chars()
        {
          tmp.push(c as i64);
        }
    }
  StrInitKind::ExprList(ls)=>
    {
        for e in ls
        {
          tmp.push(evaluate_const(e,symtbl,None).unwrap());
        }
    }
    }


    if (dk == "i8") || (dk == "u8")
    {
        for i in tmp
        {
          bytes.push(i as u8);
        }
    }

  else
    if (dk == "i16") || (dk == "u16")
    {
        for i in tmp
        {
          let  tmp_bytes = (i as i16).to_ne_bytes();

            for b in tmp_bytes
            {
              bytes.push(b);
            }
        }
    }

  else
    if (dk == "i32") || (dk == "u32")
    {
        for i in tmp
        {
          let  tmp_bytes = (i as i32).to_ne_bytes();

            for b in tmp_bytes
            {
              bytes.push(b);
            }
        }
    }

  else
    if dk == "i64"
    {
        for i in tmp
        {
          let  tmp_bytes = i.to_ne_bytes();

            for b in tmp_bytes
            {
              bytes.push(b);
            }
        }
    }


  bytes
}


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
  DeclKind::Io=>
    {
      Self{
        name,
        kind: SymbolKind::Io,
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Str(dk,sik)=>
    {
      let  bytes = Self::make_str_bytes(&dk,&sik,symtbl);

      Self{
        name,
        kind: SymbolKind::Str(dk,bytes),
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Field(e)=>
    {
      let  res = evaluate_const(&e,symtbl,None);

      Self{
        name,
        kind: SymbolKind::Field(res.unwrap() as usize),
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
  SymbolKind::Io          =>{print!("io");}
  SymbolKind::Str(_,_)    =>{print!("str");}
  SymbolKind::Field(_)    =>{print!("field");}
  SymbolKind::Fn(_)       =>{print!("fn");}
    }


  print!(" {}: ",&self.name);

    match &self.kind
    {
  SymbolKind::Const(res)    =>{print!("{}",res);}
  SymbolKind::GlobalVar(res)=>{print!("{}",res);}
  SymbolKind::Io            =>{}
  SymbolKind::Str(ty,data)  =>{print!("{} {{..{}}}",ty,data.len());}
  SymbolKind::Field(sz)     =>{print!("{}",sz);}
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
process_io_offset(&mut self, start: usize)-> usize
{
  let  mut pos = start;

    for sym in &mut self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Io=>
        {
          sym.offset = get_word_aligned(pos)            ;
                                        pos += WORD_SIZE;
        }
      _=>{}
        }
    }


  get_word_aligned(pos)
}


fn
process_data_offset(&mut self, start: usize)-> usize
{
  let  mut pos = start;

    for sym in &mut self.symbols
    {
        match &sym.kind
        {
      SymbolKind::GlobalVar(_)
     |SymbolKind::Fn(_)=>
        {
          sym.offset = get_word_aligned(pos)            ;
                                        pos += WORD_SIZE;
        }
      _=>{}
        }
    }


  get_word_aligned(pos)
}


fn
process_str_offset(&mut self, start: usize)-> usize
{
  let  mut pos = start;

    for sym in &mut self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Str(_,bytes)=>
        {
          sym.offset = get_word_aligned(pos)              ;
                                        pos += bytes.len();
        }
      _=>{}
        }
    }


  get_word_aligned(pos)
}


fn
process_field_offset(&mut self, start: usize)-> usize
{
  let  mut pos = start;

    for sym in &mut self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Field(sz)=>
        {
          sym.offset = get_word_aligned(pos)     ;
                                        pos += sz;
        }
      _=>{}
        }
    }


  get_word_aligned(pos)
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


fn
get_const_or(&mut self, s: &str, def: usize)-> usize
{
    if let Some(v) = self.find_const(s)
    {
      v as usize
    }

  else
    {
      self.add_const(s,def as i64);

      def
    }
}



pub fn
generate_exec(&mut self)-> Exec
{
  let  mut exec = Exec::new_with_memory();

  let   data_start = self.process_io_offset(256);
  let    str_start = self.process_data_offset(data_start);
  let  field_start = self.process_str_offset(str_start);
  let  stack_start = self.process_field_offset(field_start);

  self.add_const("STACK_START",stack_start as i64);

  let  stack_size = self.get_const_or("STACK_SIZE",1024*32);

  let  callstack_start = get_word_aligned(stack_start+stack_size);

  self.add_const("CALLSTACK_START",callstack_start as i64);

  let  callstack_size = self.get_const_or("CALLSTACK_SIZE",1024*32);

  let  text_start = get_word_aligned(callstack_start+callstack_size);


  let  mut pos = text_start;

    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Fn(fd)=>
        {
          let   ptr_minsym = MiniSymbol{offset: sym.offset, name: sym.name.clone(), kind: MiniSymbolKind::Data};
          let  text_minsym = MiniSymbol{offset:        pos, name: sym.name.clone(), kind: MiniSymbolKind::Text};

          exec.mini_symbols.push( ptr_minsym);
          exec.mini_symbols.push(text_minsym);


          let  mut text = assemble(fd,self);

          text.finalize();

          let  bytes = text.to_bytes();

            for i in 0..bytes.len()
            {
              exec.memory[pos+i] = bytes[i];
            }


          exec.texts.push((sym.name.clone(),text));

          let  pos_bytes = pos.to_ne_bytes();

            for i in 0..pos_bytes.len()
            {
              exec.memory[sym.offset+i] = pos_bytes[i];
            }


          pos += bytes.len();
        }
      SymbolKind::Const(v)=>
        {
          exec.mini_symbols.push(MiniSymbol{offset: 0, name: sym.name.clone(), kind: MiniSymbolKind::Const(*v)});
        }
      SymbolKind::GlobalVar(res)=>
        {
          let  res_bytes = res.to_ne_bytes();

            for i in 0..res_bytes.len()
            {
              exec.memory[sym.offset+i] = res_bytes[i];
            }


          exec.mini_symbols.push(MiniSymbol{offset: sym.offset, name: sym.name.clone(), kind: MiniSymbolKind::Data});
        }
      SymbolKind::Io=>
        {
          exec.mini_symbols.push(MiniSymbol{offset: sym.offset, name: sym.name.clone(), kind: MiniSymbolKind::Io});
        }
      SymbolKind::Str(ty,bytes)=>
        {
            for i in 0..bytes.len()
            {
              exec.memory[sym.offset+i] = bytes[i];
            }


          let  n = bytes.len()/if (ty ==  "i8") || (ty ==  "u8"){1}
                          else if (ty == "i16") || (ty == "u16"){2}
                          else if (ty == "i32") || (ty == "u32"){4}
                          else if (ty == "i64")                 {8}
                          else{panic!();};

          exec.mini_symbols.push(MiniSymbol{offset: sym.offset, name: sym.name.clone(), kind: MiniSymbolKind::Str(ty.clone(),n)});
        }
      SymbolKind::Field(sz)=>
        {
          exec.mini_symbols.push(MiniSymbol{offset: sym.offset, name: sym.name.clone(), kind: MiniSymbolKind::Field(*sz)});
        }
      _=>{}
        }
    }


  exec
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
add_const(&mut self, name: &str, v: i64)
{
  let  sym = Symbol{
    name: name.to_string(),
    kind: SymbolKind::Const(v),
    offset: 0,
    deps_parent_list: Vec::new(),
    deps_child_list: Vec::new(),
  };


  self.symbols.push(sym);
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
pub enum
MiniSymbolKind
{
  Data, Text, Const(i64), Str(String,usize), Field(usize), Io,

}


#[derive(Clone)]
pub struct
MiniSymbol
{
  offset: usize,
    name: String,

  kind: MiniSymbolKind,

}


impl
MiniSymbol
{


pub fn  get_offset(&self)-> usize{self.offset}
pub fn  get_name(&self)-> &String{&self.name}
pub fn  get_kind(&self)-> &MiniSymbolKind{&self.kind}


}




pub struct
Exec
{
  mini_symbols: Vec<MiniSymbol>,

  texts: Vec<(String,AsmText)>,

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
    mini_symbols: Vec::new(),
    texts: Vec::new(),
    memory: Vec::new(),
  }
}


pub fn
new_with_memory()-> Self
{
  let  mut e = Self{
    mini_symbols: Vec::new(),
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
get_mini_symbols(&self)-> &Vec<MiniSymbol>
{
  &self.mini_symbols
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



fn
get_ptr(&self, off: usize)-> *const u8
{
  unsafe{self.memory.as_ptr().add(off)}
}


fn
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
find_const(&self, name: &str)-> Option<i64>
{
    for sym in &self.mini_symbols
    {
        if let MiniSymbolKind::Const(v) = &sym.kind
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
    for sym in &self.mini_symbols
    {
        if let MiniSymbolKind::Io = &sym.kind
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
find_entry_point(&self, name: &str)-> Option<usize>
{
    for sym in &self.mini_symbols
    {
        if let MiniSymbolKind::Text = &sym.kind
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
    for sym in &self.mini_symbols
    {
      let  off = sym.get_offset();

      print!("{}",sym.get_name());

        match &sym.kind
        {
      MiniSymbolKind::Data
     |MiniSymbolKind::Io=>
        {
          print!("(addr: {})",off);

          let  i64_ptr = unsafe{self.memory.as_ptr().add(off)} as *const i64;

          println!(": {}",unsafe{*i64_ptr});
        }
      MiniSymbolKind::Const(v)=>{println!(": {}",v);}
      MiniSymbolKind::Str(ty,n)=>
        {
          let  base = sym.offset;

          print!(" {} = {{",ty);

               if ty ==  "i8"{for i in 0..*n{print!("{},",self.get_u8( base+i    )       );}}
          else if ty ==  "u8"{for i in 0..*n{print!("0x{:X},",self.get_u8( base+i    ) as  i8);}}
          else if ty == "i16"{for i in 0..*n{print!("{},",self.get_u16(base+(2*i))       );}}
          else if ty == "u16"{for i in 0..*n{print!("0x{:X},",self.get_u16(base+(2*i)) as i16);}}
          else if ty == "i32"{for i in 0..*n{print!("{},",self.get_u32(base+(4*i))       );}}
          else if ty == "u32"{for i in 0..*n{print!("0x{:X},",self.get_u32(base+(4*i)) as i32);}}
          else if ty == "i64"{for i in 0..*n{print!("{},",self.get_u64(base+(8*i)) as i64);}}
          else{panic!("{}",ty);}

          println!("}}");
        }
      MiniSymbolKind::Field(sz)=>
        {
          println!(" {}: {{...}}",*sz);
        }
      _=>{println!("");}
        }
    }
}


pub fn
print_text_to(&self, buf: &mut String)
{
    for (name,text) in &self.texts
    {
      buf.push_str(name);

      buf.push_str("{\n");

      text.print_to(buf);

      buf.push_str("}\n");
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




