

use crate::node::*;

use crate::source_file::{
  SourceInfo,
  Error,

};

use super::machine::{
  CORE_NUMBER,
   STACK_SIZE,

};


use crate::syntax::*;
use super::*;
use super::expr::*;
use super::stmt::*;
use super::scope::*;
use super::assemble::assemble;
use super::asm::Opcode;
use super::font14::*;
use super::font8::*;
use super::tplg_sort::*;
use super::evaluate::*;
use super::exec::*;




pub struct
StringSet
{
  set: Vec<String>,

  fail_records: Vec<(SourceInfo,String)>,

}


impl
StringSet
{


pub fn
new()-> Self
{
  Self{set: Vec::new(), fail_records: Vec::new()}
}


pub fn
insert(&mut self, new_s: &str)
{
    for s in &self.set
    {
        if s == new_s
        {
          return;
        }
    }


  self.set.push(new_s.to_string());
}


pub fn
record_fail(&mut self, srcinf: &SourceInfo, s: &str)
{
  self.fail_records.push((srcinf.clone(),s.to_string()));
}


}




pub struct
StaticSet
{
  set: Vec<(SourceInfo,String,StorageInfo)>,

}


impl
StaticSet
{


pub fn
new()-> Self
{
  Self{set: Vec::new()}
}


pub fn
insert_string(&mut self, srcinf: &SourceInfo, new_s: &str)-> String
{
    for (_,name,inf) in &self.set
    {
        if &inf.content == new_s.as_bytes()
        {
          return name.clone();
        }
    }


  let  mut inf = StorageInfo::new();

  inf.ty_kind = TyKind::U8;
  inf.length = new_s.len()+1;
  inf.is_utf8 = true;

    for b in new_s.as_bytes()
    {
      inf.content.push(*b);
    }


  inf.content.push(0);


  let  n = self.set.len();

  let  name = format!(".STATIC{}",n);

  self.set.push((srcinf.clone(),name.clone(),inf));

  name
}


pub fn
insert_storage(&mut self, srcinf: &SourceInfo, inf: StorageInfo)-> String
{
  let  n = self.set.len();

  let  name = format!(".STATIC{}",n);

  self.set.push((srcinf.clone(),name.clone(),inf));

  name
}


}




#[derive(Clone)]
pub enum
TyKind
{
  Undef, Void,

  I8, I16, I32, I64,
  U8, U16, U32,

  Struct(Vec<Field>,(usize,usize)),

  Fn,

  Alias(String,*const TyKind),

}


impl
TyKind
{


pub fn
collect_identifier(&self, set: &DeclSet, ss: &mut StringSet)
{
    match self
    {
  Self::Struct(ls,_)=>
    {
        for f in ls
        {
          f.ty_kind.collect_identifier(set,ss);
        }
    }
  Self::Alias(name,_)=>
    {
      ss.insert(name);
    }
  _=>{}
    }
}


pub fn
get_size_and_align(&self)-> (usize,usize)
{
    match self
    {
  Self::Undef=>{(0,0)}
  Self::Void=>{(0,0)}
  Self::I8 =>{(1,1)}
  Self::I16=>{(2,2)}
  Self::I32=>{(4,4)}
  Self::I64=>{(8,8)}
  Self::U8 =>{(1,1)}
  Self::U16=>{(2,2)}
  Self::U32=>{(4,4)}
  Self::Struct(_,szal)=>{*szal}
  Self::Fn=>{(8,8)}
  Self::Alias(name,ptr)=>
    {
        if *ptr == std::ptr::null()
        {
          panic!("{}の参照先が不明",name);
        }

      else
        {
          unsafe{&**ptr}.get_size_and_align()
        }
    }
    }
}


pub fn
resolve_alias(&mut self, set: &DeclSet)-> Result<(),String>
{
    match self
    {
  Self::Struct(ls,_)=>
    {
        for f in ls
        {
          f.ty_kind.resolve_alias(set);
        }
    }
  Self::Alias(name,ptr)=>
    {
        if let Some(decl) = set.search(name)
        {
            if let DeclKind::Ty(k) = &decl.kind
            {
              *ptr = k as *const TyKind;
            }

          else
            {return Err(format!("{}は型ではない",name));}
        }

      else
        {return Err(format!("{}という型が見付からない",name));}
    }
  _=>{}
    }


  Ok(())
}


pub fn
finalize_struct(&mut self)
{
    if let Self::Struct(ls,(sz,al)) = self
    {
      *sz = 0usize;
      *al = 0usize;

        for f in ls
        {
          let  (f_sz,f_al) = f.ty_kind.get_size_and_align();

          f.offset = Align(f_al).get(*sz);

          *sz = f.offset+f_sz;

          *al = std::cmp::max(*al,f_al);
        }


      *sz = Align(*al).get(*sz);
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Undef=>{print!("undef");}
  Self::Void=>{print!("void");}
  Self::I8 =>{print!("i8");}
  Self::I16=>{print!("i16");}
  Self::I32=>{print!("i32");}
  Self::I64=>{print!("i64");}
  Self::U8 =>{print!("u8");}
  Self::U16=>{print!("u16");}
  Self::U32=>{print!("u32");}
  Self::Struct(ls,(sz,al))=>
    {
      println!("struct{{");

        for f in ls
        {
          print!("  ");

          f.print();

          print!(",\n");
        }


      println!("}}(size: {}, align: {})",sz,al);
    }
  Self::Fn=>{print!("fn");}
  Self::Alias(name,_)=>
    {
      print!("{}",name);
    }
    }
}


}


#[derive(Clone)]
pub struct
Field
{
  name: String,

  ty_kind: TyKind,

  offset: usize,

}


impl
Field
{


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_ty_kind(&self)-> &TyKind
{
  &self.ty_kind
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
print(&self)
{
  print!("(offset: {}) {}: ",self.offset,&self.name);

  self.ty_kind.print();
}


}




pub struct
FnDecl
{
  parameter_names: Vec<String>,

  block: Block,

}


impl
FnDecl
{


pub fn  get_parameter_names(&self)-> &Vec<String>{&self.parameter_names}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
print(&self)
{
  print!("(");

    for name in &self.parameter_names
    {
      print!("{}, ",name);
    }


  print!(")");

  print!("\n");

  self.block.print();

  print!("\n");
}


}




pub struct
StorageInfo
{
  length: usize,
  length_expr_opt: Option<Expr>,

  ty_kind: TyKind,

  expr_list_opt: Option<Vec<Expr>>,

  content: Vec<u8>,

  is_utf8: bool,

}


impl
StorageInfo
{


pub fn
new()-> Self
{
  Self{
    length: 0,
    length_expr_opt: None,
    ty_kind: TyKind::Undef,
    expr_list_opt: None,
    content: Vec::new(),
    is_utf8: false,
  }
}


pub fn
collect_identifier(&self, set: &DeclSet, ss: &mut StringSet)
{
    if let Some(e) = &self.length_expr_opt
    {
      e.collect_identifier(set,ss);
    }


    if let Some(ls) = &self.expr_list_opt
    {
        for e in ls
        {
          e.collect_identifier(set,ss);
        }
    }
}


pub fn
collect_static(&mut self, ss: &mut StaticSet)
{
    if let Some(e) = &mut self.length_expr_opt
    {
      e.collect_static(ss);
    }


    if let Some(ls) = &mut self.expr_list_opt
    {
        for e in ls
        {
          e.collect_static(ss);
        }
    }
}


pub fn
get_length(&self)-> usize
{
  self.length
}


pub fn
get_length_expr_opt(&self)-> &Option<Expr>
{
  &self.length_expr_opt
}


pub fn
get_size(&self)-> usize
{
  let  (sz,_) = self.ty_kind.get_size_and_align();

  sz*self.length
}


pub fn
get_ty_kind(&self)-> &TyKind
{
  &self.ty_kind
}


pub fn
print(&self)
{
  print!("[");

    if let Some(e) = &self.length_expr_opt
    {
      e.print();
    }

  else
    {
      print!("{}",self.length);
    }


  print!("]: ");

  self.ty_kind.print();

    if let Some(ls) = &self.expr_list_opt
    {
      print!("{{");

        for e in ls
        {
          e.print();

          print!(",");
        }


      print!("}}");
    }

  else
    if self.is_utf8
    {
        if let Ok(s) = str::from_utf8(&self.content)
        {
          print!("\"{}\"",s);
        }
    }
}


}




pub enum
DeclKind
{
  Undef,

  Const(Expr,i64),
  Static(StorageInfo),
     Var(StorageInfo),

  LocalStatic(String),

  Enum(Vec<String>),

  Ty(TyKind),
  Fn(FnDecl),

  Asm(Vec<Opcode>),
  AsmWithArgc(Vec<Opcode>),

  Mod(Box<DeclSet>),

}


impl
DeclKind
{


pub fn
print(&self, name: &str)
{
    match self
    {
  DeclKind::Undef=>{print!("undef {}",name);}
  DeclKind::Const(e,i)=>
    {
      print!("const {}",name);

      print!(" = ");

      e.print();

      print!(" = {}",*i);
    }
  DeclKind::Static(inf)=>
    {
      print!("static {}",name);

      inf.print();
    }
  DeclKind::Var(inf)=>
    {
      print!("var {}",name);

      inf.print();
    }
  DeclKind::LocalStatic(name)=>
    {
      print!("local static {}",name);
    }
  DeclKind::Enum(ls)=>
    {
      print!("enum{{");

        for s in ls
        {
          print!("{}, ",s);
        }


      print!("}}");
    }
  DeclKind::Ty(ty)=>
    {
      print!("type {}: ",name);

      ty.print();
    }
  DeclKind::Fn(f)=>
    {
      print!("fn {}",name);

      f.print();
    }
  DeclKind::Asm(ops)=>
    {
      print!("asm{{");

        for op in ops
        {
          print!("{},",op.to_str());
        }


      print!("}}");
    }
  DeclKind::AsmWithArgc(ops)=>
    {
      print!("asm with argc{{");

        for op in ops
        {
          print!("{},",op.to_str());
        }


      print!("}}");
    }
  DeclKind::Mod(set)=>
    {
      println!("mod {}{{",name);

      set.print();

      println!("\n}}");
    }
    }
}


}




pub struct
Decl
{
  source_info: SourceInfo,

  set_ptr: *mut DeclSet,

  name: String,

  kind: DeclKind,

  offset: usize,

  deps_parent_names: Vec<String>,
   deps_child_names: Vec<String>,

}


impl
Decl
{


pub fn
new()-> Self
{
  Self{
    source_info: SourceInfo::new(),

    set_ptr: std::ptr::null_mut(),

    name: String::new(),

    kind: DeclKind::Undef,

    offset: 0,

    deps_parent_names: Vec::new(),
     deps_child_names: Vec::new(),
  }
}


pub fn
new_ty(name: &str, k: TyKind)-> Self
{
  let  mut decl = Self::new();

  decl.name = name.to_string();

  decl.kind = DeclKind::Ty(k);

  decl
}


pub fn
new_sys()-> Self
{
  let  mut decl = Self::new();

  let  sys = DeclSet::new_sys();

  decl.name.push_str("sys");

  decl.kind = DeclKind::Mod(Box::new(sys));

  decl
}


pub fn
new_asm(name: &str, ops: Vec<Opcode>)-> Self
{
  let  mut decl = Self::new();

  decl.name = name.to_string();

  decl.kind = DeclKind::Asm(ops);

  decl
}


pub fn
new_asm_with_argc(name: &str, ops: Vec<Opcode>)-> Self
{
  let  mut decl = Self::new();

  decl.name = name.to_string();

  decl.kind = DeclKind::AsmWithArgc(ops);

  decl
}


pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_qualified_name(&self)-> String
{
  let  q: &str = if self.set_ptr != std::ptr::null_mut(){&unsafe{&*self.set_ptr}.qualifier} else{""};

  format!("{}{}",q,&self.name)
}


pub fn
get_kind(&self)-> &DeclKind
{
  &self.kind
}


pub fn
get_kind_mut(&mut self)-> &mut DeclKind
{
  &mut self.kind
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
collect_identifier(&self, set: &DeclSet, ss: &mut StringSet)
{
    match &self.kind
    {
  DeclKind::Const(e,_)=>{e.collect_identifier(set,ss);}
  DeclKind::Static(inf)=>{inf.collect_identifier(set,ss);}
  DeclKind::Var(inf)=>{inf.collect_identifier(set,ss);}
  DeclKind::Ty(k)=>{k.collect_identifier(set,ss);}
  DeclKind::Mod(set)=>{set.collect_identifier(ss);}
  _=>{}
    }
}


pub fn
collect_static(&mut self, ss: &mut StaticSet)
{
    match &mut self.kind
    {
  DeclKind::Const(e,_)=>{e.collect_static(ss);}
  DeclKind::Static(inf)=>{inf.collect_static(ss);}
  DeclKind::Var(inf)=>{inf.collect_static(ss);}
  DeclKind::Mod(set)=>{set.collect_static(ss);}
  _=>{}
    }
}


fn
initialize(dst: &mut [u8], src: &[Expr], k: &TyKind, set: &DeclSet)-> Result<(),Error>
{
    match k
    {
  TyKind::Struct(ls,(_,_))=>
    {
      let  mut src_i = 0usize;

        for f in ls
        {
          Self::initialize(&mut dst[f.offset..],&src[src_i..],&f.ty_kind,set);

          
        }
    }
  TyKind::Alias(_,ptr)=>
    {
      Self::initialize(dst,src,unsafe{&**ptr},set);
    }
  _=>{}
    }


  Ok(())
}


pub fn
build_const_data(&mut self)-> Result<(),Error>
{
  let  srcinf = &self.source_info;
  let     set = unsafe{&*self.set_ptr};

    match &mut self.kind
    {
  DeclKind::Const(e,v)=>
    {
        match evaluate_const(&e,set,None)
        {
      Some(i)=>{*v = i;}
      None=>{return Err(srcinf.to_error(format!("constの初期化に失敗")));}
        }
    }
  DeclKind::Static(inf)=>
    {
        if let Some(e) = &inf.length_expr_opt
        {
            match evaluate_const(&e,set,None)
            {
          Some(i)=>{inf.length = i as usize;}
          None=>{return Err(srcinf.to_error(format!("staticの要素数の算出に失敗")));}
            }
        }


        if inf.content.len() == 0
        {
          let  sz = inf.get_size();

          inf.content.resize(sz,0);

            if let Some(ls) = &inf.expr_list_opt
            {
                for e in ls
                {
                }
            }
        }
    }
  DeclKind::Var(_)=>
    {
      return Err(srcinf.to_error(format!("グローバル変数の宣言はvarではなくstaticを使ってください")));
    }
  DeclKind::Ty(k)=>
    {
        if let Err(msg) = k.resolve_alias(set)
        {
          return Err(srcinf.to_error(msg));
        }


      k.finalize_struct();
    }
  _=>{}
    }


  Ok(())
}


pub fn
read(s: &str)-> Result<Self,Error>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    match crate::syntax::parse::parse_from_string(s,dic,"declaration")
    {
   Ok(nd)=>
    {
      let  mut cur = nd.cursor();

        if let Some(decl_nd) = cur.select_node("declaration")
        {
          read_decl(decl_nd)
        }

      else
        {Err(Error::new(format!("no decl")))}
    }
  Err(e)=>{Err(e)}
    }
}


pub fn
print(&self)
{
  self.kind.print(&self.name);

  println!("");

    for s in &self.deps_parent_names
    {
      println!("** requires {}",s);
    }


    for s in &self.deps_child_names
    {
      println!("** required by {}",s);
    }
}


}




pub fn
read_parameter_list(start_nd: &Node)-> Vec<String>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<String>::new();

  cur.advance(1);

    if let Some(first_id) = cur.get_identifier()
    {
      ls.push(first_id.clone());

      cur.advance(1);

        while let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(p_id) = cur.get_identifier()
            {
              ls.push(p_id.clone());

              cur.advance(1);
            }
        }
    }


  ls
}


pub fn
read_const(start_nd: &Node)-> (String,Expr)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(2);

        if let Some(e_nd) = cur.select_node("expression")
        {
          let  expr = read_expr(e_nd);

          return (name,expr);
        }
    }


  panic!();
}


pub fn
read_number_of_elements(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      let  e = read_expr(nd);

      return e;
    }


  panic!();
}


pub fn
read_storage_info(start_nd: &Node)-> StorageInfo
{
  let  mut cur = start_nd.cursor();

  let  mut inf = StorageInfo::new();

    if let Some(nd) = cur.select_node("number_of_elements")
    {
      inf.length_expr_opt = Some(read_number_of_elements(nd));

      cur.advance(1);
    }

  else
    {
      inf.length  = 1;
    }


    if cur.is_semi_string()
    {
      cur.advance(1);

      inf.ty_kind = read_ty_spec(cur.get_node().unwrap());

      cur.advance(1);

        if let Some(nd) = cur.select_node("expression_list")
        {
          inf.expr_list_opt = Some(read_expr_list(nd));
        }
    }

  else
    {
      inf.ty_kind = TyKind::I64;
    }


  inf
}


pub fn
read_var(start_nd: &Node)-> (String,StorageInfo)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(nd) = cur.select_node("storage_info")
        {
          let  inf = read_storage_info(nd);

          return (name,inf);
        }

      else
        {
          let  mut inf = StorageInfo::new();

          inf.length  = 1;
          inf.ty_kind = TyKind::I64;

            if cur.is_semi_string()
            {
              cur.advance(1);

                if let Some(nd) = cur.select_node("expression")
                {
                  inf.expr_list_opt = Some(vec![read_expr(nd)]);
                }
            }

          else
            {
                for _ in 0..8
                {
                  inf.content.push(0);
                }
            }


          return (name,inf);
        }
    }


  panic!();
}


pub fn
read_enum(start_nd: &Node)-> Vec<String>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<String>::new();

  cur.advance(2);

    while let Some(s) = cur.get_identifier()
    {
      ls.push(s.clone());

      cur.advance(1);

        if let Some(_) = cur.get_semi_string()
        {
          cur.advance(1);
        }
    }


  ls
}




pub fn
read_field(start_nd: &Node)-> Field
{
  let  mut cur = start_nd.cursor();

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(nd) = cur.select_node("type_spec")
        {
          let  ty_kind = read_ty_spec(nd);

          return Field{name, ty_kind, offset: 0};
        }
    }


  todo!();
}


pub fn
read_struct(start_nd: &Node)-> Vec<Field>
{
  let  mut cur = start_nd.cursor();

  let  mut buf = Vec::<Field>::new();

  cur.advance(2);

    while let Some(nd) = cur.select_node("field")
    {
      let  f = read_field(nd);

      buf.push(f);

      cur.advance(1);

        if cur.is_semi_string()
        {
          cur.advance(1);
        }
    }


  buf
}


pub fn
read_ty_spec(start_nd: &Node)-> TyKind
{
  let  mut cur = start_nd.cursor();

    if let Some(nd) = cur.get_node()
    {
        match nd.get_name()
        {
      (s) if s == "struct"=>
        {
          let  flds = read_struct(nd);

          return TyKind::Struct(flds,(0,0));
        }
      (s) if s == "qualified_identifier"=>
        {
          let  id = read_qualified_identifier(nd);

          return TyKind::Alias(id,std::ptr::null());
        }
      _=>{}
        }
    }


  panic!();
}


pub fn
read_ty_decl(start_nd: &Node)-> (String,TyKind)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

        if let Some(nd) = cur.select_node("type_spec")
        {
          let  kind = read_ty_spec(nd);

          return (name,kind);
        }
    }


  panic!();
}


pub fn
read_fn_decl(start_nd: &Node)-> (String,FnDecl)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.select_node("parameter_list")
        {
          let  parameter_names = read_parameter_list(parals_d);

          cur.advance(1);

            if let Some(blk_d) = cur.select_node("block")
            {
              let  block = read_block(blk_d);

              let  f = FnDecl{parameter_names, block};

              return (name,f);
            }
        }
    }


  panic!();
}




pub fn
read_mod(start_nd: &Node)-> Result<(String,DeclSet),Error>
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(2);

      let  mut set = DeclSet::new();

        while let Some(nd) = cur.select_node("declaration")
        {
            match read_decl(nd)
            {
          Ok(decl)=>
            {
                match set.insert(decl)
                {
              Ok(())=>{cur.advance(1);}
              Err(e)=>{return Err(e);}
                }
            }
          Err(e)=>{return Err(e);}
            }
        }


      return Ok((name,set));
    }


  panic!();
}



pub fn
read_decl(start_nd: &Node)-> Result<Decl,Error>
{
  let  mut decl = Decl::new();

  decl.source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

    if let Some(nd) = cur.get_node()
    {
      let  nd_name = nd.get_name();

        if nd_name == "empty"
        {
        }

      else
        if nd_name == "fn"
        {
          let  (name,f) = read_fn_decl(nd);

          decl.name = name;
          decl.kind = DeclKind::Fn(f);
        }

      else
        if nd_name == "enum"
        {
          let  ls = read_enum(nd);

          decl.kind = DeclKind::Enum(ls);
        }

      else
        if nd_name == "var"
        {
          let  (name,inf) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Var(inf);
        }

      else
        if nd_name == "static"
        {
          let  (name,inf) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Static(inf);
        }

      else
        if nd_name == "const"
        {
          let  (name,expr) = read_const(nd);

          decl.name = name;
          decl.kind = DeclKind::Const(expr,0);
        }

      else
        if nd_name == "type"
        {
          let  (name,ty) = read_ty_decl(nd);

          decl.name = name;
          decl.kind = DeclKind::Ty(ty);
        }

      else
        if nd_name == "mod"
        {
            match read_mod(nd)
            {
          Ok((name,set))=>
            {
              decl.name = name;
              decl.kind = DeclKind::Mod(Box::new(set));
            }
          Err(e)=>{return Err(e);}
            }
        }

      else
        {
          return Err(decl.source_info.to_error(format!("{} is unknown decl",nd_name)));
        }


      return Ok(decl);
    }


  Err(decl.source_info.to_error(format!("read_decl error")))
}




pub struct
DeclSet
{
  parent_ptr: *mut Self,
    decl_ptr: *mut Decl,

  qualifier: String,

  decls: Vec<Box<Decl>>,

}


impl
DeclSet
{


pub fn
new()-> Self
{
  Self{
    parent_ptr: std::ptr::null_mut(),
      decl_ptr: std::ptr::null_mut(),

    qualifier: String::new(),

    decls: Vec::new(),

  }
}


pub fn
new_sys()-> Self
{
  let  mut set = Self::new();

  set.decls.push(Box::new(Decl::new_asm_with_argc("spawn",vec![Opcode::Spw])));
  set.decls.push(Box::new(Decl::new_asm("id",vec![Opcode::Pushid])));
  set.decls.push(Box::new(Decl::new_asm("pc",vec![Opcode::Pushpc])));
  set.decls.push(Box::new(Decl::new_asm("fp",vec![Opcode::Pushfp])));
  set.decls.push(Box::new(Decl::new_asm("sp",vec![Opcode::Pushsp])));
  set.decls.push(Box::new(Decl::new_asm("input",vec![Opcode::Pushinput])));
  set.decls.push(Box::new(Decl::new_asm("timer",vec![Opcode::Pushtimer])));

  set
}


pub fn
get_parent(&self)-> Option<&Self>
{
    if self.parent_ptr != std::ptr::null_mut()
    {
      Some(unsafe{&*self.parent_ptr})
    }

  else
    {
      None
    }
}


pub fn
get_parent_mut(&mut self)-> Option<&mut Self>
{
    if self.parent_ptr != std::ptr::null_mut()
    {
      Some(unsafe{&mut *self.parent_ptr})
    }

  else
    {
      None
    }
}


pub fn
get_root_ptr(&self)-> *const Self
{
  let  mut r = self;

    while r.parent_ptr != std::ptr::null_mut()
    {
      r = unsafe{&*r.parent_ptr};
    }


  r as *const Self
}


pub fn
get_root(&self)-> &Self
{
  unsafe{&*self.get_root_ptr()}
}


pub fn
get_root_mut(&mut self)-> &mut Self
{
  unsafe{&mut *(self.get_root_ptr() as *mut Self)}
}


pub fn
get_qualifier(&self)-> &String
{
  &self.qualifier
}


pub fn
as_decl(&self)-> &Decl
{
  unsafe{&*self.decl_ptr}
}




pub fn
read(s: &str)-> Result<Box<Self>,Error>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    match crate::syntax::parse::parse_from_string(s,dic,"declaration")
    {
  Ok(nd)=>
    {
      let  mut cur = nd.cursor();

      let  mut set = Self::new();

        while let Some(decl_nd) = cur.select_node("declaration")
        {
            match read_decl(decl_nd)
            {
          Ok(decl)=>
            {
                match set.insert(decl)
                {
              Ok(())=>{cur.advance(1);}
              Err(e)=>{return Err(e);}
                }
            }
          Err(e)=>{return Err(e);}
            }
        }


      let  sys = Decl::new_sys();

      set.decls.push(Box::new(sys));
      set.decls.push(Box::new(Decl::new_ty("void",TyKind::Void)));
      set.decls.push(Box::new(Decl::new_ty( "i8",   TyKind::I8)));
      set.decls.push(Box::new(Decl::new_ty( "u8",   TyKind::U8)));
      set.decls.push(Box::new(Decl::new_ty("i16",  TyKind::I16)));
      set.decls.push(Box::new(Decl::new_ty("u16",  TyKind::U16)));
      set.decls.push(Box::new(Decl::new_ty("i32",  TyKind::I32)));
      set.decls.push(Box::new(Decl::new_ty("u32",  TyKind::U32)));
      set.decls.push(Box::new(Decl::new_ty("i64",  TyKind::I64)));

      Ok(Box::new(set))
    }
  Err(e)=>{Err(e)}
    }
}


pub fn
find(&self, name: &str)-> Option<&Decl>
{
    for decl in &self.decls
    {
        if &decl.name == name
        {
          return Some(decl);
        }
    }


  None
}


pub fn
find_mut(&mut self, name: &str)-> Option<&mut Decl>
{
    for decl in &mut self.decls
    {
        if &decl.name == name
        {
          return Some(decl);
        }
    }


  None
}


fn
search_downwards(&self, q_name: &str, exclude: &str)-> Option<usize>
{
    if q_name.starts_with(&self.qualifier)
    {
        for decl in &self.decls
        {
            if &decl.name != exclude
            {
                if decl.get_qualified_name() == q_name
                {
                  return Some((&**decl) as *const Decl as usize);
                }


                if let DeclKind::Mod(set) = &decl.kind
                {
                    if let Some(u) = set.search_downwards(q_name,"")
                    {
                      return Some(u)
                    }
                }
            }
        }
    }


  None
}


fn
search_internal(&self, q_name: &str, exclude: &str)-> Option<usize>
{
    if let Some(u) = self.search_downwards(q_name,exclude)
    {
      return Some(u);
    }


    if let Some(parent) = self.get_parent()
    {
      return parent.search_internal(q_name,self.as_decl().get_name());
    }


  None
}


pub fn
search(&self, q_name: &str)-> Option<&Decl>
{
    if let Some(u) = self.search_internal(q_name,"")
    {
      return Some(unsafe{&*(u as *const Decl)})
    }


  None
}


pub fn
search_mut(&mut self, q_name: &str)-> Option<&mut Decl>
{
    if let Some(u) = self.search_internal(q_name,"")
    {
      return Some(unsafe{&mut *(u as *mut Decl)})
    }


  None
}


pub fn
search_by_qualified_name_mut(&mut self, qname: &str)-> Option<&mut Decl>
{
  todo!();
}


pub fn
find_const(&self, name: &str)-> Option<i64>
{
    if let Some(decl) = self.find(name)
    {
        if let DeclKind::Const(_,v) = &decl.kind
        {
          return Some(*v);
        }
    }


  None
}


pub fn
add_const(&mut self, name: &str, v: i64)
{
  let  mut decl = Decl::new();

  decl.name.push_str(name);
  decl.kind = DeclKind::Const(Expr::from_int(v),v);

  self.insert(decl);
}


pub fn
insert(&mut self, mut decl: Decl)-> Result<(),Error>
{
    if let DeclKind::Undef = &decl.kind
    {
      Ok(())
    }

  else
    if let DeclKind::Enum(ls) = &decl.kind
    {
         for (i,s) in ls.iter().enumerate()
         {
           let  e = Expr::from_int(i as i64);

           let  mut const_decl = Decl::new();

           const_decl.name = s.clone();
           const_decl.kind = DeclKind::Const(e,i as i64);

             match self.insert(const_decl)
             {
           Ok(())=>{}
           Err(e)=>{return Err(e);}
             }
         }


      self.decls.push(Box::new(decl));

      Ok(())
    }

  else
   if self.find(&decl.name).is_some()
    {
      Err(decl.source_info.to_error(format!("{}という名前は既に存在している",&decl.name)))
    }

  else
    {
      self.decls.push(Box::new(decl));

      Ok(())
    }
}


pub fn
collect_identifier(&self, ss: &mut StringSet)
{
    for decl in &self.decls
    {
      decl.collect_identifier(self,ss);
    }
}


fn
collect_static(&mut self, ss: &mut StaticSet)
{
    for decl in &mut self.decls
    {
      decl.collect_static(ss);
    }
}


fn
collect_as_tplg_nodes(&mut self, buf: &mut Vec<TplgNode>)
{
    for decl in &mut self.decls
    {
      let  value = decl.as_mut() as *mut Decl as usize;

      let  nd = TplgNode::new(&decl.get_qualified_name(),
                              value,
                              &decl.deps_child_names,
                              decl.deps_parent_names.len());

      buf.push(nd);

        if let DeclKind::Mod(set) = &mut decl.kind
        {
          set.collect_as_tplg_nodes(buf);
        }
    }
}


fn
canonicalize(&mut self, parent_ptr: *mut Self, decl_ptr: *mut Decl)
{
  let  self_ptr = self as *mut Self;

  self.parent_ptr = parent_ptr;
  self.decl_ptr   =   decl_ptr;

    for decl in &mut self.decls
    {
      decl.set_ptr = self_ptr;

      let  sub_decl_ptr = decl.as_ref() as *const Decl as *mut Decl;

        if let DeclKind::Mod(set) = &mut decl.kind
        {
          let  parent_q: &str = if parent_ptr != std::ptr::null_mut(){&unsafe{&*parent_ptr}.qualifier} else{""};

          set.qualifier = format!("{}{}::",parent_q,&decl.name);

          set.canonicalize(self_ptr,sub_decl_ptr);
        }
    }
}


fn
process_deps_relationship(&mut self)-> Result<(),Error>
{
    for i in 0..self.decls.len()
    {
      let  mut ss = StringSet::new();

      self.decls[i].collect_identifier(self,&mut ss);

        if ss.fail_records.len() != 0
        {
          let  mut msg = String::new();

            for ((srcinf,s)) in ss.fail_records
            {
              msg.push_str(&format!("{} {} not found\n",&srcinf.to_string(),&s));
            }


          return Err(Error::new(msg));
        }


        for s in ss.set
        {
          let  parent_name = s;
          let   child_name = self.decls[i].get_qualified_name();

            if let Some(parent) = self.get_root_mut().search_mut(&parent_name)
            {
              parent.deps_child_names.push(child_name);

              self.decls[i].deps_parent_names.push(parent_name);
            }

          else
            {panic!();}
        }


        if let DeclKind::Mod(set) = &mut self.decls[i].kind
        {
          set.process_deps_relationship();
        }
    }


  Ok(())
}


fn
process_data_offset(&mut self, start: usize)-> usize
{
  let  mut pos = get_word_aligned(start);

    for decl in &mut self.decls
    {
        match &mut decl.kind
        {
      DeclKind::Static(inf)=>
        {
          decl.offset = pos                  ;
                        pos += inf.get_size();
        }
      DeclKind::Var(k)=>
        {
          panic!();
        }
      DeclKind::Fn(_)=>
        {
          decl.offset = pos             ;
                        pos += WORD_SIZE;
        }
      DeclKind::Mod(set)=>
        {
          pos = set.process_data_offset(pos);
        }
      _=>{}
        }


      pos = get_word_aligned(pos);
    }


  get_word_aligned(pos)
}


fn
install_font8(dst: &mut [u8])
{
  let  mut  iter = FONT8.iter();

    while let Some(unicode) = iter.next()
    {
      let  base = (8*((*unicode) as usize));

        for i in 0..8
        {
          let  bits = (*iter.next().unwrap()) as u8;

          dst[base+i] = bits;
        }
    }
}


fn
install_combi8(dst: &mut [u8])
{
  let  mut  iter = COMBI8.iter();

    while let Some(unicode) = iter.next()
    {
      let  base = (2*((*unicode) as usize));

      let  upper = (*iter.next().unwrap()) as u16;
      let  lower = (*iter.next().unwrap()) as u16;

      let  u_bytes = upper.to_ne_bytes();
      let  l_bytes = lower.to_ne_bytes();

      dst[base  ] = u_bytes[0];
      dst[base+1] = u_bytes[1];
      dst[base+2] = l_bytes[0];
      dst[base+3] = l_bytes[1];
    }
}


fn
install_font14(dst: &mut [u8])
{
  let  mut  iter = FONT14.iter();

    while let Some(unicode) = iter.next()
    {
      const  FULLWIDTH_FIRST: usize = 0xFF01;
      const  FULLWIDTH_LAST: usize  = 0xFF5E;

      let  u = *unicode as usize;

      let  base = 2*14*u;

      let  is_fullwidth_ascii = (u >= FULLWIDTH_FIRST) && (u <= FULLWIDTH_LAST);

        for i in 0..14
        {
          let  bytes = iter.next().unwrap().to_ne_bytes();

          dst[base+(2*i)  ] = bytes[0];
          dst[base+(2*i)+1] = bytes[1];

            if is_fullwidth_ascii
            {
              let  ascii_base = 2*14*(('!' as usize)+u-FULLWIDTH_FIRST);

              dst[ascii_base+(2*i)  ] = bytes[0];
              dst[ascii_base+(2*i)+1] = bytes[1];
            }
        }
    }
}


fn
get_const_or(&mut self, s: &str, defval: usize)-> usize
{
    if let Some(v) = self.find_const(s)
    {
      v as usize
    }

  else
    {
      self.add_const(s,defval as i64);

      defval
    }
}


pub fn
finalize(&mut self)-> Result<(),Error>
{
  let  mut ss = StaticSet::new();

  self.collect_static(&mut ss);

    for (srcinf,name,k) in ss.set
    {
      let  mut decl = Decl::new();

      decl.source_info = srcinf;
      decl.name = name;
      decl.kind = DeclKind::Static(k);

        match self.insert(decl)
        {
      Ok(())=>{}
      Err(e)=>{return Err(e);}
        }
    }


  self.canonicalize(std::ptr::null_mut(),std::ptr::null_mut());

    match self.process_deps_relationship()
    {
  Ok(())=>
    {
      let  mut tplg_nodes = Vec::<TplgNode>::new();

      self.collect_as_tplg_nodes(&mut tplg_nodes);

        match tplg_sort(tplg_nodes)
        {
      Ok(sorted_values)=>
        {
            for v in sorted_values
            {
              let  decl = unsafe{&mut *(v as *mut Decl)};

                match decl.build_const_data()
                {
              Ok(())=>{}
              Err(e)=>{return Err(e);}
                }
            }
        }
      Err(e)=>{return Err(e);}
        }


      Ok(())
    }
  Err(e)=>{Err(e)}
    }
}


fn
write_to_exec(&self, exec: &mut Exec, pos: &mut usize)-> Result<(),Error>
{
    for decl in &self.decls
    {
      let  q_name = decl.get_qualified_name();

        match &decl.kind
        {
      DeclKind::Mod(set)=>
        {
            match set.write_to_exec(exec,pos)
            {
          Ok(())=>{}
          Err(e)=>{return Err(e);}
            }
        }
      DeclKind::Fn(fd)=>
        {
          let   ptr_sym = Symbol::new_static(&q_name,decl.offset as isize,1,TyKind::Fn);
          let  text_sym = Symbol::new_text(&q_name,*pos as isize);

          exec.add_symbol( ptr_sym);
          exec.add_symbol(text_sym);

            match assemble(&decl.source_info,fd,self)
            {
          Ok(mut text)=>
            {
              text.finalize();

              let  bytes = text.to_bytes();

                if ((*pos)+bytes.len()) > Exec::MEMORY_SIZE
                {
                  return Err(Error::new(format!("プログラムおよびデータが、容量を超えている")));
                }


              exec.put_bytes(*pos,&bytes);

              exec.add_text((decl.name.clone(),*pos,text));

              let  pos_bytes = pos.to_ne_bytes();

              exec.put_bytes(decl.offset,&pos_bytes);

              *pos += bytes.len();
            }
          Err(e)=>{return Err(Error::new(format!("関数{}のアセンブルに失敗",&q_name)).wrap(e));}
            }
        }
      DeclKind::Const(_,v)=>
        {
          exec.add_symbol(Symbol::new_const_int(&q_name,*v));
        }
      DeclKind::Static(inf)=>
        {
          exec.add_symbol(Symbol::new_static(&q_name,decl.offset as isize,inf.length,inf.ty_kind.clone()));
        }
      DeclKind::Var(_)=>
        {
          panic!();
        }
      _=>{}
        }
    }


  Ok(())
}


pub fn
generate_exec(&mut self)-> Result<Exec,Error>
{
  let  mut exec = Exec::new_with_memory();

  let   font8_start = self.process_data_offset(256);
  let  combi8_start = get_word_aligned( font8_start+(   8*0x10000));
  let  font14_start = get_word_aligned(combi8_start+(2* 3*0x10000));
  let   stack_start = get_word_aligned(font14_start+(2*14*0x10000));

  let  stack_size = self.get_const_or("STACK_SIZE",STACK_SIZE*CORE_NUMBER);

  let  text_start = get_word_aligned(stack_start+stack_size);


  self.add_const( "FONT8_START", font8_start as i64);
  self.add_const("COMBI8_START",combi8_start as i64);
  self.add_const("FONT14_START",font14_start as i64);
  self.add_const( "STACK_START", stack_start as i64);

  let  mut pos = text_start;

    match self.write_to_exec(&mut exec,&mut pos)
    {
  Ok(())=>{}
  Err(e)=>{return Err(e);}
    }


  exec.add_symbol(Symbol::new_const_int("HEAP_START",pos as i64));

  Self::install_font8( exec.get_memory_slice_mut(font8_start ));
  Self::install_combi8(exec.get_memory_slice_mut(combi8_start));
  Self::install_font14(exec.get_memory_slice_mut(font14_start));


  Ok(exec)
}




pub fn
add_ex_img(&mut self, name: &str, w: u32, h: u32, data: &Vec<u8>)
{
  let  mut new_data = Vec::<u8>::new();

    for b in w.to_ne_bytes(){new_data.push(b);}
    for b in h.to_ne_bytes(){new_data.push(b);}

  let  mut iter = data.iter();

    while let Some(r_ref) = iter.next()
    {
      let  r = *r_ref as u32;
      let  g = *iter.next().unwrap() as u32;
      let  b = *iter.next().unwrap() as u32;
      let  _ = *iter.next().unwrap() as u32;

      let  pix = (r<<24)
                |(g<<16)
                |(b<< 8);

        for b in pix.to_ne_bytes()
        {
          new_data.push(b);
        }
    }


  let  mut decl = Decl::new();

  let  mut inf = StorageInfo::new();

  inf.length = new_data.len();
  inf.ty_kind = TyKind::U8;
  inf.content = new_data;

  decl.name = name.to_string();
  decl.kind = DeclKind::Static(inf);

  self.insert(decl);
}


pub fn
print(&self)
{
    for decl in &self.decls
    {
      decl.print();

      println!("");
    }
}


}




