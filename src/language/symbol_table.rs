

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::scope::*;
use super::program::*;
use super::evaluate::*;
use super::evaluate_const::*;
use super::tplg_sort::*;




pub enum
SymbolKind
{
  Null,

  Ty(TyNode),

      Const(Expr,EvalConstResult),
  GlobalVar(Expr,EvalConstResult),

  Fn(FnDecl),

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
new()-> Self
{
  Self{
    name: String::new(),
    kind: SymbolKind::Null,
    ty_name: String::new(),
    offset: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_ty(name: String, ty_node: TyNode)-> Self
{
  Self{
    name,
    kind: SymbolKind::Ty(ty_node),
    ty_name: String::new(),
    offset: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_const(name: String, e: Expr)-> Self
{
  Self{
    name,
    kind: SymbolKind::Const(e,EvalConstResult::Void),
    ty_name: String::new(),
    offset: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_global(name: String, e: Expr)-> Self
{
  Self{
    name,
    kind: SymbolKind::GlobalVar(e,EvalConstResult::Void),
    ty_name: String::new(),
    offset: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
  }
}


pub fn
new_fn(name: String, fndecl: FnDecl)-> Self
{
  Self{
    name,
    kind: SymbolKind::Fn(fndecl),
    ty_name: String::new(),
    offset: 0,
    deps_parent_list: Vec::new(),
     deps_child_list: Vec::new(),
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
  SymbolKind::Null          =>{print!("null");}
  SymbolKind::Const(_,_)    =>{print!("const");}
  SymbolKind::GlobalVar(_,_)=>{print!("(g)var");}
  SymbolKind::Fn(_)         =>{print!("fn");}
  SymbolKind::Ty(_)         =>{print!("ty");}
    }


  print!(" {}: ",&self.name);

    match &self.kind
    {
  SymbolKind::Null          =>{}
  SymbolKind::Const(e,_)    =>{e.print();}
  SymbolKind::GlobalVar(e,_)=>{e.print();}
  SymbolKind::Fn(fd)=>{fd.get_block().print();}
  SymbolKind::Ty(_)=>{}
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
find_program(&self, name: &str)-> Option<&Program>
{
    for prog in &self.programs
    {
        if prog.get_name() == name
        {
          return Some(prog);
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
add_ty(&mut self, name: &str, ty_node: TyNode)
{
  let  sym = Symbol::new_ty(name.to_string(),ty_node);

  self.symbols.push(sym);
}


pub fn
add_fn(&mut self, name: &str, fd: FnDecl)
{
  let  sym = Symbol::new_fn(name.to_string(),fd);

  self.symbols.push(sym);
}


pub fn
add_const(&mut self, name: &str, e: Expr)
{
  let  sym = Symbol::new_const(name.to_string(),e);

  self.symbols.push(sym);
}


pub fn
add_global(&mut self, name: &str, e: Expr)
{
  let  sym = Symbol::new_global(name.to_string(),e);

  self.symbols.push(sym);
}


pub fn
add(&mut self, mut decl: Decl)
{
  let  decl_name = decl.release_name();
  let  decl_k    = decl.release_kind();

    match decl_k
    {
  DeclKind::Undef=>{}
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
  DeclKind::Struct(ls)=>
    {
      self.add_ty(&decl_name,TyNode::Struct(ls))
    }
  DeclKind::Union(ls)=>
    {
      self.add_ty(&decl_name,TyNode::Union(ls))
    }
  DeclKind::Enum(ls)=>
    {
      self.add_ty(&decl_name,TyNode::Enum(ls))
    }
    }
}


fn
process_collectibles(&mut self)
{
    for sym in &mut self.symbols
    {
      sym.deps_parent_list.clear();
      sym.deps_child_list.clear();
    }


    for i in 0..self.symbols.len()
    {
      let  mut buf = Vec::<Collectible>::new();

        match &self.symbols[i].kind
        {
      SymbolKind::Const(e,_)    =>{e.collect(&mut buf);}
      SymbolKind::GlobalVar(e,_)=>{e.collect(&mut buf);}
      SymbolKind::Fn(fd)        =>{fd.get_block().collect(&mut buf);}
      SymbolKind::Ty(ty_node)   =>{ty_node.collect(&mut buf);}
      _=>{panic!();}
        }


        for co in buf
        {
            match co
            {
          Collectible::Identifier(s)=>
            {
              let  sym_name = self.symbols[i].name.clone();

              self.find_symbol_mut(&s).unwrap().deps_child_list.push(sym_name);

              self.symbols[i].deps_parent_list.push(s);
            }
          Collectible::String(s)=>{self.add_string_literal(s);}
            }
        }
    }
}


pub fn
make_tplg_node_list(&self)-> Vec<TplgNode>
{
  let  mut buf = Vec::<TplgNode>::new();

    for sym in &self.symbols
    {
      let  nd = TplgNode::new(sym.name.clone(),sym.deps_child_list.clone(),sym.deps_parent_list.len());

      buf.push(nd);
    }


  buf
}


pub fn
build_value(&mut self, name: &str, alloc_off: &mut usize)
{
  println!("building {}...",name);

  let  i = self.find_symbol_index(name).unwrap();

  let  mut tmp = Symbol::new();

  std::mem::swap(&mut self.symbols[i],&mut tmp);

    match &mut tmp.kind
    {
  SymbolKind::Ty(ty_node)=>
    {
      let  ty = Ty::build_and_add(ty_node,self);

      tmp.name = ty.get_name().clone();
    }
  SymbolKind::Const(e,res)=>
    {
      *res = evaluate_const(e,self,None);

        match res
        {
      EvalConstResult::Void    =>{tmp.ty_name = "void".to_string();}
      EvalConstResult::Bool(_) =>{tmp.ty_name = "bool".to_string();}
      EvalConstResult::Int(_)  =>{tmp.ty_name = "i64".to_string();}
      EvalConstResult::Float(_)=>{tmp.ty_name = "f64".to_string();}
      _=>{  println!("build const error: {} ",&tmp.name);  e.print();  panic!();}
        }
    }
  SymbolKind::GlobalVar(e,res)=>
    {
      *res = evaluate_const(e,self,None);

      tmp.offset = *alloc_off;

        match res
        {
      EvalConstResult::Void    =>{tmp.ty_name = "void".to_string();}
      EvalConstResult::Bool(_) =>{tmp.ty_name = "bool".to_string();}
      EvalConstResult::Int(_)  =>{tmp.ty_name = "i64".to_string();}
      EvalConstResult::Float(_)=>{tmp.ty_name = "f64".to_string();}
      _=>{panic!();}
        }


      let  ty = find_ty(&tmp.ty_name).unwrap();

      *alloc_off = get_word_aligned(*alloc_off+ty.get_size());
    }
  SymbolKind::Fn(fd)=>
    {
      let  tynd = fd.get_ty_node();

      let  ty = Ty::build_and_add(&tynd,self);

      tmp.ty_name = ty.get_name().clone();

      tmp.offset = *alloc_off;

      *alloc_off = get_word_aligned(*alloc_off+WORD_SIZE);
    }
  _=>{}
    }


  std::mem::swap(&mut self.symbols[i],&mut tmp);
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

    if let Ok(names) = tplg_sort(nodes)
    {
      let  gval_start = pos;

        for name in names
        {
          self.build_value(&name,&mut pos);
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

    if let SymbolKind::Fn(fd) = &sym.kind
    {
      let  mut parameter_names = Vec::<String>::new();

        for decl in fd.get_parameter_decl_list()
        {
          parameter_names.push(decl.get_name().clone());
        }


      let  ty = find_ty(&sym.ty_name).unwrap();

        if let TyKind::Function{parameter_ty_names, return_ty_name} = ty.get_kind()
        {
          let  mut lid = LabelID::new();

          let  scp = Scope::new_root(&parameter_names,parameter_ty_names);

          let  mut output = AsmTable::new();

          fd.get_block().process(self,return_ty_name,&mut lid,None,&scp,&mut output);

          let  prog = Program::new(&sym.name,scp.get_offset_max(),output);

          self.programs.push(prog);
        }
    }


  Ok(())
}


pub fn
build(&mut self)-> Result<ExecImage,()>
{
  self.process_collectibles();

  let  first_pos = self.allocate_strings();
  let   last_pos = self.allocate_global_vars(first_pos);

  let  mut img = ExecImage::new();

  img.bytes.resize(last_pos,0);

    for sym in &self.symbols
    {
        match &sym.kind
        {
      SymbolKind::GlobalVar(_,res)=>
        {
            match res
            {
          EvalConstResult::Bool(b) =>{img.write_u64(sym.offset,if *b{1} else{0});}
          EvalConstResult::Int(i)  =>{img.write_u64(sym.offset,*i as u64);}
          EvalConstResult::Float(f)=>{img.write_u64(sym.offset,f.to_bits());}
          _=>{}
            }
        }
      SymbolKind::Fn{..}=>
        {
          let  prog = self.find_program(&sym.name).unwrap();

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


  println!("}}\nprograms{{");

    for prog in &self.programs
    {
      prog.print_lines();

      println!("");

      prog.print_bytes();

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




