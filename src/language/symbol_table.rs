

use std::rc::Rc;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::scope::*;
use super::assemble::assemble;
use super::machine::MachineInfo;
use super::evaluate::*;
use super::evaluate_const::*;
use super::tplg_sort::*;




pub enum
SymbolKind
{
  Ty,

      Const(EvalConstResult),
  GlobalVar(EvalConstResult),

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

  ty_name: String,

  offset: usize,

  deps_parent_list: Vec<String>,
   deps_child_list: Vec<String>,

}


impl
Symbol
{


pub fn
build(src: Source, symtbl: &SymbolTable, tytbl: &mut TyTable)-> Self
{
  let  (name,kind) = src.decl.expire();

  let  deps_parent_list = src.deps_parent_list;
  let   deps_child_list =  src.deps_child_list;

    match kind
    {
  DeclKind::Undef=>{panic!("symbol build error: {} is undef",&name);}
  DeclKind::Const(tn_opt,e)=>
    {
      let  res = evaluate_const(&e,symtbl,tytbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::Const(res),
            ty_name,
            offset: 0,
            deps_parent_list,
            deps_child_list,
          }
        }

      else
        {
          println!("build const error: {} ",&name);
          e.print();

          panic!();
        }
    }
  DeclKind::Var(tn_opt,e)=>
    {
      let  res = evaluate_const(&e,symtbl,tytbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::GlobalVar(res),
            ty_name,
            offset: 0,
            deps_parent_list,
            deps_child_list,
          }
        }

      else
        {
          println!("build var error: {} ",&name);
          e.print();

          panic!();
        }
    }
  DeclKind::Static(tn_opt,e)=>
    {
      let  res = evaluate_const(&e,symtbl,tytbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::GlobalVar(res),
            ty_name,
            offset: 0,
            deps_parent_list,
            deps_child_list,
          }
        }

      else
        {
          println!("build var error: {} ",&name);
          e.print();

          panic!();
        }
    }
  DeclKind::Fn(fd)=>
    {
      let  tynd = fd.make_ty_node();

      let  ty = tytbl.add_from_node(&tynd,symtbl);

      let  ty_name = ty.get_name().clone();

      Self{
        name,
        kind: SymbolKind::Fn(fd),
        ty_name,
        offset: 0,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Ty(tn)=>
    {
      let  ty = tytbl.add_from_node(&tn,symtbl);

      let  ty_name = ty.get_name().clone();

      Self{
        name,
        kind: SymbolKind::Ty,
        ty_name,
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
get_ty_name(&self)-> &String
{
  &self.ty_name
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
  SymbolKind::Ty          =>{print!("ty");}
    }


  print!(" {}: ",&self.name);

    match &self.kind
    {
  SymbolKind::Const(res)    =>{res.print();}
  SymbolKind::GlobalVar(res)=>{res.print();}
  SymbolKind::Fn(_,)=>{}
  SymbolKind::Ty=>{}
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

  string_table: Vec<(String,usize)>,

}


impl
SymbolTable
{


pub fn
new()-> Self
{
  Self{
    symbols: Vec::new(),
    string_table: Vec::new(),
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


  panic!();
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
generate_symbols(&mut self, mut srcs: Vec<Source>, tytbl: &mut TyTable)
{
  let  names = Self::make_tplg_sorted_names(&srcs);

    for name in names
    {
      let  src = Self::take_source(&mut srcs,&name);

      let  sym = Symbol::build(src,self,tytbl);

      self.symbols.push(sym);
    }
}


pub fn
build(decls: Vec<Decl>, tytbl: &mut TyTable)-> Result<Self,()>
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
          Collectible::String(s)=>{tbl.string_table.push((s,0));}
            }
        }
    }


  tbl.generate_symbols(srcs,tytbl);

  Ok(tbl)
}


fn
generate_data(&mut self, tytbl: &TyTable, start: usize)-> Vec<u8>
{
  let  mut bytes = Vec::<u8>::new();
  let  mut pos = start;

    for sym in &mut self.symbols
    {
                   pos = get_word_aligned(pos);
      sym.offset = pos                        ;

      pos += tytbl.find(&sym.ty_name).unwrap().get_size();
    }


    for (s,off) in &mut self.string_table
    {
      *off = pos           ;
             pos += s.len();
    }


  bytes.resize(pos-start,0);

    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Const(res)=>
        {
          let  res_bytes = res.to_bytes(tytbl);

            for i in 0..res_bytes.len()
            {
              bytes[(sym.offset-start)+i] = res_bytes[i];
            }
        }
      SymbolKind::GlobalVar(res)=>
        {
          let  res_bytes = res.to_bytes(tytbl);

            for i in 0..res_bytes.len()
            {
              bytes[(sym.offset-start)+i] = res_bytes[i];
            }
        }
      _=>{}
        }
    }


    for (s,off) in &self.string_table
    {
      let  s_bytes = s.as_bytes();

        for i in 0..s_bytes.len()
        {
          bytes[((*off)-start)+i] = s_bytes[i];
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
generate_exec(&mut self, tytbl: &mut TyTable, mi: &MachineInfo)-> Exec
{
  let  mut exec = Exec::new();

  exec.data_bytes = self.generate_data(tytbl,mi.get_data_start());

  let  mut pos = mi.get_text_start();

    for sym in &self.symbols
    {
        if let SymbolKind::Fn(fd) = &sym.kind
        {
            if &sym.name == "main"
            {
              exec.entry_point = pos;
            }


          let  bytes = assemble(fd,self,tytbl);

            for b in &bytes
            {
              exec.text_bytes.push(*b);
            }


          let  pos_bytes = pos.to_ne_bytes();

            for i in 0..pos_bytes.len()
            {
              exec.data_bytes[(sym.offset-mi.get_data_start())+i] = pos_bytes[i];
            }


          pos += bytes.len();
        }
    }


  exec
}




pub fn
find_string_offset(&self, s: &str)-> Option<usize>
{
    for (stored_s,off) in &self.string_table
    {
        if stored_s == s
        {
          return Some(*off);
        }
    }


  None
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
print(&self)
{
  println!("string literals{{");

    for (s,off) in &self.string_table
    {
      println!("{} {}",s,off);
    }


  println!("}}\nglobal symbols{{");

    for sym in &self.symbols
    {
      sym.print();

      println!("");
    }


  println!("}}");
}


}




pub struct
Exec
{
  data_bytes: Vec<u8>,
  text_bytes: Vec<u8>,

  entry_point: usize,

}


impl
Exec
{


pub fn
new()-> Self
{
  Self{
    data_bytes: Vec::new(),
    text_bytes: Vec::new(),
    entry_point: 0,
  }
}


pub fn
get_data_bytes(&self)-> &Vec<u8>
{
  &self.data_bytes
}


pub fn
get_text_bytes(&self)-> &Vec<u8>
{
  &self.text_bytes
}


pub fn
get_entry_point(&self)-> usize
{
  self.entry_point
}


}




