

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::scope::*;
use super::program::*;
use super::evaluate::*;
use super::tplg_sort::*;




pub enum
SymbolKind
{
      Const(Expr),
  GlobalVar(Expr),

  Fn{parameter_name_list: Vec<String>, block: Block},

}


pub enum
SymbolValue
{
  Void,

  Bool(bool),
    Int(i64),
    IntV(std::cell::Cell<i64>),
   Uint(u64),
  Float(f64),

  Bytes(Vec<u8>),

  ProgramIndex(usize),

}


impl
SymbolValue
{


pub fn get_bool(&self)-> bool{if let Self::Bool(b)  = self{*b} else{false}}
pub fn get_int(&self)->   i64{if let Self::Int(i)   = self{*i} else{0}}
pub fn get_int_v(&self)-> i64{if let Self::IntV(i)  = self{i.get()} else{0}}
pub fn get_uint(&self)->  u64{if let Self::Uint(u)  = self{*u} else{0}}
pub fn get_float(&self)-> f64{if let Self::Float(f) = self{*f} else{0.0}}
pub fn get_bytes(&self)-> &Vec<u8>
{
  static DUMMY: Vec<u8> = Vec::new();

  if let Self::Bytes(s) = self{s} else{&DUMMY}
}


}




pub struct
Symbol
{
  key: SymbolKey,

  name: String,

  kind: SymbolKind,

  value: SymbolValue,

  ty: Ty,

  offset: usize,
    size: usize,

  deps_parent_list: Vec<SymbolKey>,
   deps_child_list: Vec<SymbolKey>,

}


impl
Symbol
{


pub fn
new_const(key: SymbolKey, name: String, e: Expr)-> Self
{
  Self{
    key,
    name,
    kind: SymbolKind::Const(e),
    value: SymbolValue::Void,
    ty: Ty::Undef,
    offset: 0,
      size: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_global(key: SymbolKey, name: String, e: Expr)-> Self
{
  Self{
    key,
    name,
    kind: SymbolKind::GlobalVar(e),
    value: SymbolValue::Void,
    ty: Ty::Undef,
    offset: 0,
      size: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_fn(key: SymbolKey, name: String, parameter_name_list: Vec<String>, ty: Ty, block: Block)-> Self
{
  Self{
    key,
    name,
    kind: SymbolKind::Fn{parameter_name_list, block},
    value: SymbolValue::Void,
    ty,
    offset: 0,
      size: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
get_key(&self)-> SymbolKey
{
  self.key
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
get_value(&self)-> &SymbolValue
{
  &self.value
}


pub fn
get_value_mut(&mut self)-> &mut SymbolValue
{
  &mut self.value
}


pub fn
get_ty(&self)-> &Ty
{
  &self.ty
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
get_size(&self)-> usize
{
  self.size
}


pub fn
get_reference_count(&self)-> usize
{
  self.deps_child_list.len()
}


pub fn
print(&self, progs: &Vec<Program>)
{
    match &self.kind
    {
  SymbolKind::Const(_)    =>{print!("const");}
  SymbolKind::GlobalVar(_)=>{print!("(g)var");}
  SymbolKind::Fn{..}      =>{print!("fn");}
    }


  print!(" {}({}i): ",&self.name,self.key.0);

    match &self.kind
    {
  SymbolKind::Const(e)    =>{e.print();}
  SymbolKind::GlobalVar(e)=>{e.print();}
  SymbolKind::Fn{ parameter_name_list, block}=>{block.print();}
    }


  print!(" -> ");

  self.ty.print();

  print!(" ");

    match &self.ty
    {
  Ty::Bool =>{print!("{}",self.value.get_bool()) ;}
  Ty::Int  =>{print!("{}",self.value.get_int())  ;}
  Ty::Float=>{print!("{}",self.value.get_float());}
  _=>{}
    }


  println!("");
  println!("offset: {}",self.offset);
  println!("  size: {}",self.size);

  println!("");

    for key in &self.deps_parent_list
    {
      println!("this requires {}",key.0);
    }


    for key in &self.deps_child_list
    {
      println!("this is required by {}",key.0);
    }


  println!("reference count: {}",self.get_reference_count());

    if let SymbolValue::ProgramIndex(i) = &self.value
    {
      progs[*i].print_lines();
      progs[*i].print_bytes();
    }


  println!("");
}


}




#[derive(Clone,Copy,PartialEq)]
pub struct SymbolKey(usize);
impl SymbolKey{pub fn to_number(self)-> usize{self.0}}


pub enum
Embedded
{
  Key(SymbolKey,SymbolKey),
  String(String),

}




pub struct
SymbolTable
{
  symbols: Vec<Symbol>,

  string_table: Vec<(String,usize)>,

  programs: Vec<Program>,

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
    programs: Vec::new(),
  }
}


pub fn
add_string_literal(&mut self, new_s: String)
{
    for (s,_) in &self.string_table
    {
        if &new_s == s
        {
          return;
        }
    }


  self.string_table.push((new_s,0));
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
add_fn(&mut self, name: &str, fd: FnDecl)-> Result<SymbolKey,()>
{
  let  (parameter_name_list,ty,mut block) = fd.decompose();

  let  new_key = SymbolKey(self.symbols.len());

  let  sym = Symbol::new_fn(new_key,name.to_string(),parameter_name_list,ty,block);

  self.symbols.push(sym);

  Ok(new_key)
}


pub fn
add_const(&mut self, name: &str, e: Expr)-> Result<SymbolKey,()>
{
  let  new_key = SymbolKey(self.symbols.len());

  let  sym = Symbol::new_const(new_key,name.to_string(),e);

  self.symbols.push(sym);

  Ok(new_key)
}


pub fn
add_global(&mut self, name: &str, e: Expr)-> Result<SymbolKey,()>
{
  let  new_key = SymbolKey(self.symbols.len());

  let  sym = Symbol::new_global(new_key,name.to_string(),e);

  self.symbols.push(sym);

  Ok(new_key)
}


pub fn
add(&mut self, mut decl: Decl)-> Result<SymbolKey,()>
{
  let  decl_name = decl.release_name();
  let  decl_k    = decl.release_kind();

    match decl_k
    {
  DeclKind::Undef=>{Err(())}
  DeclKind::Const(e)=>
    {
      self.add_const(&decl_name,e)
    }
  DeclKind::Var(e)=>
    {
      self.add_global(&decl_name,e)
    }
  DeclKind::Static(e)=>
    {
      self.add_global(&decl_name,e)
    }
  DeclKind::Fn(fd)=>
    {
      self.add_fn(&decl_name,fd)
    }
    }
}


pub fn
collect_embedded_for_expr(&self, key: SymbolKey, e: &Expr, buf: &mut Vec<Embedded>)
{
    match e
    {
  Expr::Identifier(s)=>
    {
        if let Some(sym) = self.find_symbol(s)
        {
          buf.push(Embedded::Key(sym.key,key));
        }

      else
        {
          println!("{} is not found",s);
        }
    }
  Expr::String(s)=>
    {
      buf.push(Embedded::String(s.clone()));
    }
  Expr::AccessOp(e,_)=>
    {
      self.collect_embedded_for_expr(key,e,buf);
    }
  Expr::SubscriptOp(e,i_e)=>
    {
      self.collect_embedded_for_expr(key,  e,buf);
      self.collect_embedded_for_expr(key,i_e,buf);
    }
  Expr::CallOp(f,args)=>
    {
      self.collect_embedded_for_expr(key,f,buf);

        for a in args
        {
          self.collect_embedded_for_expr(key,a,buf);
        }
    }
  Expr::Expr(e)=>
    {
      self.collect_embedded_for_expr(key,e,buf);
    }
  Expr::UnaryOp(o,_)=>
    {
      self.collect_embedded_for_expr(key,o,buf);
    }
  Expr::BinaryOp(l,r,_)=>
    {
      self.collect_embedded_for_expr(key,l,buf);
      self.collect_embedded_for_expr(key,r,buf);
    }
  _=>{}
    }
}


fn
link_deps(&mut self, parent_key: SymbolKey, child_key: SymbolKey)
{
  self.get_mut(parent_key).deps_child_list.push(  child_key);
  self.get_mut( child_key).deps_parent_list.push(parent_key);
}


fn
process_embedded(&mut self)
{
  let  mut emb_ls = Vec::<Embedded>::new();

    for sym in &mut self.symbols
    {
      sym.deps_parent_list.clear();
      sym.deps_child_list.clear();
    }


    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::Const(e)    =>{self.collect_embedded_for_expr(sym.key,e,&mut emb_ls);}
      SymbolKind::GlobalVar(e)=>{self.collect_embedded_for_expr(sym.key,e,&mut emb_ls);}
      SymbolKind::Fn{..}=>
        {
        }
      _=>{panic!();}
        }
    }


    for emb in emb_ls
    {
        match emb
        {
      Embedded::Key(parent,child)=>{self.link_deps(parent,child);}
      Embedded::String(s)        =>{self.add_string_literal(s);}
        }
    }
}


pub fn
make_tplg_node_list(&self)-> Vec<TplgNode>
{
  let  mut buf = Vec::<TplgNode>::new();

    for sym in &self.symbols
    {
      let  nd = TplgNode::new(sym.key,sym.deps_child_list.clone(),sym.deps_parent_list.len());

      buf.push(nd);
    }


  buf
}


pub fn
build_value(&mut self, key: SymbolKey, alloc_off: &mut usize)-> Result<(),()>
{
  println!("building {}...",&self.get(key).name);

    match &self.get(key).kind
    {
  SymbolKind::Const(e)=>
    {
      let  res = evaluate(e,self,None);

      let  sym = self.get_mut(key);

        match res
        {
      EvalResult::Void   =>{sym.ty = Ty::Void;}
      EvalResult::Bool(b)=>
        {
          sym.value = SymbolValue::Bool(b);
          sym.ty = Ty::Bool;
          sym.size = WORD_SIZE;
        }
      EvalResult::Int(i)=>
        {
          sym.value = SymbolValue::Int(i);
          sym.ty = Ty::Int;
          sym.size = WORD_SIZE;
        }
      EvalResult::Float(f)=>
        {
          sym.value = SymbolValue::Float(f);
          sym.ty = Ty::Float;
          sym.size = WORD_SIZE;
        }
      _=>{println!("build const errorx");  res.print();  return Err(());}
        }


      return Ok(());
    }
  SymbolKind::GlobalVar(e)=>
    {
      let  res = evaluate(e,self,None);

      let  sym = self.get_mut(key);

      sym.offset = *alloc_off;

        match res
        {
      EvalResult::Void   =>{sym.ty = Ty::Void;}
      EvalResult::Bool(b)=>
        {
          sym.value = SymbolValue::Bool(b);
          sym.ty = Ty::Bool;
          sym.size = WORD_SIZE;
        }
      EvalResult::Int(i)=>
        {
          sym.value = SymbolValue::Int(i);
          sym.ty = Ty::Int;
          sym.size = WORD_SIZE;
        }
      EvalResult::Float(f)=>
        {
          sym.value = SymbolValue::Float(f);
          sym.ty = Ty::Float;
          sym.size = WORD_SIZE;
        }
      _=>{return Err(());}
        }


      *alloc_off = get_word_aligned(*alloc_off+sym.size);

      return Ok(());
    }
  SymbolKind::Fn{..}=>
    {
      let  sym = self.get_mut(key);

      sym.offset = *alloc_off;

      sym.size = WORD_SIZE;

      *alloc_off = get_word_aligned(*alloc_off+sym.size);

      return Ok(());
    }
  _=>{}
    }


  Err(())
}


pub fn
allocate_strings(&mut self)-> usize
{
  let  mut pos = 0usize;

    for (s,off) in &mut self.string_table
    {
      *off = pos           ;
             pos += s.len();
    }


  pos
}


pub fn
allocate_global_vars(&mut self, mut pos: usize)-> usize
{
  let  nodes = self.make_tplg_node_list();

    if let Ok(sorted_keys) = tplg_sort(nodes)
    {
      let  gval_start = pos;

        for key in sorted_keys
        {
            if self.build_value(key,&mut pos).is_err()
            {
              println!("build_symbol is failed");

              panic!();
            }
        }


println!("global values are allocated on {} - {}",gval_start,pos);

        for i in 0..self.symbols.len()
        {
            if self.build_fn(i).is_err()
            {
              println!("build_fn is failed");

              panic!();
            }
        }


      let  prog_start = pos;

        for prog in &mut self.programs
        {
            if prog.build(pos).is_err()
            {
              println!("build_fn is failed");

              panic!();
            }


          pos += prog.get_bytes().len();
        }


println!("progs are allocated on {} - {}",prog_start,pos);
    }

  else
    {
      panic!();
    }


  pos
}


pub fn
build_fn(&mut self, i: usize)-> Result<(),()>
{
  let  sym = &self.symbols[i];

    if let SymbolKind::Fn{parameter_name_list, block} = &sym.kind
    {
        if let Ty::Function{parameter_ty_list, return_ty} = &sym.ty
        {
          let  ret_ty_s = return_ty.get_canonical_name();

          let  mut lid = LabelID::new();

          let  scp = Scope::new_root(parameter_name_list,parameter_ty_list);

          let  mut output = AsmTable::new();

          block.process(self,&ret_ty_s,&mut lid,None,&scp,&mut output);

          let  index = self.programs.len();

          self.symbols[i].value = SymbolValue::ProgramIndex(index);

          self.programs.push(Program::new(index,scp.get_offset_max(),output));
        }
    }


  Ok(())
}


pub fn
build(&mut self)-> Result<ExecImage,()>
{
  self.process_embedded();

  let  first_pos = self.allocate_strings();
  let   last_pos = self.allocate_global_vars(first_pos);

  let  mut img = ExecImage::new();

  img.bytes.resize(last_pos,0);

    for sym in &self.symbols
    {
        match &sym.value
        {
      SymbolValue::Bool(b)=>
        {
          img.write_u64(sym.offset,if *b{1} else{0});
        }
      SymbolValue::Int(i)=>
        {
          img.write_u64(sym.offset,*i as u64);
        }
      SymbolValue::Float(f)=>
        {
          img.write_u64(sym.offset,f.to_bits());
        }
      SymbolValue::ProgramIndex(i)=>
        {
          let  prog = self.get_program(*i);

          img.write_u64(sym.offset,prog.get_offset() as u64);

            if &sym.name == "main"
            {
              img.entry_point = prog.get_offset();

              println!("entry_point: {}",img.entry_point);
            }
        }
      _=>{}
        }
    }


    for prog in &self.programs
    {
        for i in 0..prog.get_bytes().len()
        {
          img.bytes[prog.get_offset()+i] = prog.get_bytes()[i];
        }
    }


  Ok(img)
}


pub fn
get(&self, key: SymbolKey)-> &Symbol
{
  &self.symbols[key.0]
}


pub fn
get_program(&self, i: usize)-> &Program
{
  &self.programs[i]
}


pub fn
get_mut(&mut self, key: SymbolKey)-> &mut Symbol
{
  &mut self.symbols[key.0]
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
      sym.print(&self.programs);

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




impl
std::convert::From<Vec<Decl>> for SymbolTable
{


fn
from(ls: Vec<Decl>)-> SymbolTable
{
  let  mut tbl = SymbolTable::new();

    for decl in ls
    {
      tbl.add(decl);
    }


  tbl
}


}




