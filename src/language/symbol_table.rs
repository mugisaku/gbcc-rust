

use std::rc::Rc;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::scope::*;
use super::assemble::assemble;
use super::evaluate::*;
use super::evaluate_const::*;
use super::tplg_sort::*;




pub enum
SymbolKind
{
  Ty,

      Const(EvalConstResult),
  GlobalVar(EvalConstResult),

  Fn(FnDecl,Vec<u8>,usize),

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
build(tbl: &SymbolTable, src: Source, offset: usize)-> Self
{
  let  (name,kind) = src.decl.expire();

  let  deps_parent_list = src.deps_parent_list;
  let   deps_child_list =  src.deps_child_list;

    match kind
    {
  DeclKind::Undef=>{panic!("symbol build error: {} is undef",&name);}
  DeclKind::Const(tn_opt,e)=>
    {
      let  res = evaluate_const(&e,tbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::Const(res),
            ty_name,
            offset,
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
      let  res = evaluate_const(&e,tbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::GlobalVar(res),
            ty_name,
            offset,
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
      let  res = evaluate_const(&e,tbl,None);

        if let Some(ty_name) = res.get_ty_name()
        {
          Self{
            name,
            kind: SymbolKind::GlobalVar(res),
            ty_name,
            offset,
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

      let  ty = add_ty_from_node(&tynd,tbl);

      let  ty_name = ty.get_name().clone();

      Self{
        name,
        kind: SymbolKind::Fn(fd,Vec::new(),0),
        ty_name,
        offset,
        deps_parent_list,
        deps_child_list,
      }
    }
  DeclKind::Ty(tn)=>
    {
      let  ty = add_ty_from_node(&tn,tbl);

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
get_ty(&self)-> Rc<Ty>
{
  find_ty(&self.ty_name).unwrap()
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
  SymbolKind::Fn(_,_,_)   =>{print!("fn");}
  SymbolKind::Ty          =>{print!("ty");}
    }


  print!(" {}: ",&self.name);

    match &self.kind
    {
  SymbolKind::Const(res)    =>{res.print();}
  SymbolKind::GlobalVar(res)=>{res.print();}
  SymbolKind::Fn(_,_,_)=>{}
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

  image_size: usize,

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
    image_size: 0,
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
process_collectibles(srcs: &mut Vec<Source>)-> Vec<String>
{
  let  mut strs = Vec::<String>::new();

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

              Self::join_child(srcs,&s,child_name);

              srcs[i].deps_parent_list.push(s);
            }
          Collectible::String(s)=>{strs.push(s);}
            }
        }
    }


  strs
}


fn
install_string_literals(&mut self, strs: Vec<String>, mut pos: usize)-> usize
{
    for s in strs
    {
        if let None = self.find_string_offset(&s)
        {
          let  len = s.len();

          self.string_table.push((s,pos));

          pos += len;
        }
    }


  pos
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
install_global_vars(&mut self, mut srcs: Vec<Source>, mut pos: usize)-> usize
{
  let  names = Self::make_tplg_sorted_names(&srcs);

  let  gvar_start = pos;

    for name in names
    {
      let  src = Self::take_source(&mut srcs,&name);

      let  sym = Symbol::build(self,src,pos);

      pos += find_ty(&sym.ty_name).unwrap().get_size();

      pos = get_word_aligned(pos);

      self.symbols.push(sym);
    }


println!("global values are allocated on {} - {}",gvar_start,pos);

  let  prog_start = pos;

    for i in 0..self.symbols.len()
    {
      let  mut tmp = Vec::<u8>::new();

        if let SymbolKind::Fn(fd,_,_) = &self.symbols[i].kind
        {
          tmp = assemble(fd,self);
        }


        if let SymbolKind::Fn(_,bytes,offset) = &mut self.symbols[i].kind
        {
           *bytes = tmp;
          *offset = pos;

          pos += bytes.len();
        }
    }


println!("progs are allocated on {} - {}",prog_start,pos);

  pos
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


  let  strs = Self::process_collectibles(&mut srcs);

  let  mut alloc_pos = 0usize;

  alloc_pos = tbl.install_string_literals(strs,alloc_pos);

  tbl.image_size = tbl.install_global_vars(srcs,alloc_pos);

  Ok(tbl)
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
ExecImage
{
  bytes: Vec<u8>,

  entry_point: usize,

}


impl
ExecImage
{


pub fn
new()-> Self
{
  Self{
    bytes: Vec::new(),
    entry_point: 0,
  }
}


pub fn
build(tbl: &SymbolTable)-> Self
{
  let  mut img = Self::new();

  img.bytes.resize(tbl.image_size,0);

    for sym in &tbl.symbols
    {
        match &sym.kind
        {
      SymbolKind::GlobalVar(res)=>
        {
            match res
            {
          EvalConstResult::Bool(b) =>{img.write_u64(sym.offset,if *b{1} else{0});}
          EvalConstResult::Int(i)  =>{img.write_u64(sym.offset,*i as u64);}
          EvalConstResult::Float(f)=>{img.write_u64(sym.offset,f.to_bits());}
          _=>{}
            }
        }
      SymbolKind::Fn(_,_,offset)=>
        {
          img.write_u64(sym.offset,*offset as u64);

            if &sym.name == "main"
            {
              img.entry_point = *offset;

              println!("entry_point: {}",img.entry_point);
            }
        }
      _=>{}
        }
    }


    for sym in &tbl.symbols
    {
        match &sym.kind
        {
      SymbolKind::Fn(_,bytes,offset)=>
        {
            for i in 0..bytes.len()
            {
              img.bytes[offset+i] = bytes[i];
            }
        }
      _=>{}
        }
    }


  img
}


pub fn
get_bytes(&self)-> &Vec<u8>
{
  &self.bytes
}


pub fn
get_entry_point(&self)-> usize
{
  self.entry_point
}


pub fn
write_u64(&mut self, offset: usize, v: u64)
{
  let  src_bytes = v.to_ne_bytes();

  let  mut ptr = unsafe{self.bytes.as_mut_ptr().add(offset)};

    for b in src_bytes
    {
      unsafe{*ptr = b};

      ptr = unsafe{ptr.add(1)};
    }
}


pub fn
write_str(&mut self, offset: usize, s: &str)
{
  let  mut ptr = unsafe{self.bytes.as_mut_ptr().add(offset)};

    for b in s.as_bytes()
    {
      unsafe{*ptr = *b};

      ptr = unsafe{ptr.add(1)};
    }
}


}




