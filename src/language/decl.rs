

use crate::node::*;

use crate::source_file::{
  SourceInfo,
  Message,

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
use super::tplg_sort::*;
use super::evaluate::*;
use super::project::*;
use super::exec::*;




#[derive(Clone)]
pub enum
TyKind
{
  Void,

  I8, I16, I32, I64,
  U8, U16, U32,

}


impl
TyKind
{


pub fn
get_size(&self)-> usize
{
    match self
    {
  Self::Void=>{0}
  Self::I8 =>{1}
  Self::I16=>{2}
  Self::I32=>{4}
  Self::I64=>{8}
  Self::U8 =>{1}
  Self::U16=>{2}
  Self::U32=>{4}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Void=>{print!("void");}
  Self::I8 =>{print!("i8");}
  Self::I16=>{print!("i16");}
  Self::I32=>{print!("i32");}
  Self::I64=>{print!("i64");}
  Self::U8 =>{print!("u8");}
  Self::U16=>{print!("u16");}
  Self::U32=>{print!("u32");}
    }
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
Initializer
{
  start_expr_opt: Option<Expr>,

  expr: Expr,

}


impl
Initializer
{


pub fn
get_start_expr_opt(&self)-> &Option<Expr>
{
  &self.start_expr_opt
}


pub fn
get_expr(&self)-> &Expr
{
  &self.expr
}


}




pub struct
VarDecl
{
  length: usize,
  length_expr_opt: Option<Expr>,

  ty_kind: TyKind,

  initializers_opt: Option<Vec<Initializer>>,

  content: Vec<u8>,

}


impl
VarDecl
{


pub fn
new()-> Self
{
  Self{
    length: 0,
    length_expr_opt: None,
    ty_kind: TyKind::Void,
    initializers_opt: None,
    content: Vec::new(),
  }
}


pub fn
from_bytes(bytes: Vec<u8>, k: TyKind)-> Self
{
  let  element_size = k.get_size();

  let  length = if element_size != 0{bytes.len()/element_size} else{0};


  Self{
    length,
    length_expr_opt: None,
    ty_kind: k,
    initializers_opt: None,
    content: bytes,
  }
}


pub fn
collect_identifier(&self, pj: &Project, ss: &mut StringSet)
{
    if let Some(e) = &self.length_expr_opt
    {
      e.collect_identifier(pj,ss);
    }


    if let Some(inits) = &self.initializers_opt
    {
        for init in inits
        {
            if let Some(e) = &init.start_expr_opt
            {
              e.collect_identifier(pj,ss);
            }


          init.expr.collect_identifier(pj,ss);
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


    if let Some(inits) = &mut self.initializers_opt
    {
        for init in inits
        {
            if let Some(e) = &mut init.start_expr_opt
            {
              e.collect_static(ss);
            }


          init.expr.collect_static(ss);
        }
    }
}


fn
initialize_content(dst: &mut [u8], inits: &[Initializer], n: usize, k: &TyKind, pj: &Project)-> Result<(),Message>
{
  let  sz = k.get_size();

  let  mut ptr = dst.as_mut_ptr();
  let      end = unsafe{ptr.add(sz*n)};

    for init in inits
    {
        if let Some(e) = &init.start_expr_opt
        {
            if let Some(i) = evaluate_const(e,pj,None)
            {
              ptr = unsafe{dst.as_mut_ptr().add(sz*(i as usize))};
            }

          else
            {
              return Err(Message::from("initialize_content error: インデックス値の算出に失敗"));
            }
        }


        if ptr >= end
        {
          return Err(Message::from("initialize_content error: ptr over end"));
        }


        match evaluate_const(&init.expr,pj,None)
        {
      Some(i)=>
        {
            match k
            {
          TyKind::I8 =>{*unsafe{&mut *(ptr as *mut  i8)} = i as  i8;}
          TyKind::I16=>{*unsafe{&mut *(ptr as *mut i16)} = i as i16;}
          TyKind::I32=>{*unsafe{&mut *(ptr as *mut i32)} = i as i32;}
          TyKind::I64=>{*unsafe{&mut *(ptr as *mut i64)} = i       ;}
          TyKind::U8 =>{*unsafe{&mut *(ptr as *mut  u8)} = i as  u8;}
          TyKind::U16=>{*unsafe{&mut *(ptr as *mut u16)} = i as u16;}
          TyKind::U32=>{*unsafe{&mut *(ptr as *mut u32)} = i as u32;}
          _=>
            {
              return Err(Message::from("initialize_content error: maybe void"));
            }
            }


          ptr = unsafe{ptr.add(sz)};
        }
      None=>{return Err(init.expr.get_source_info().to_message()+"initialize_content error: const value eval is failed")}
        }
    }


  Ok(())
}


pub fn
build_content(&mut self, srcinf: &SourceInfo, pj: &Project)-> Result<usize,Message>
{
    if let Some(e) = &self.length_expr_opt
    {
        match evaluate_const(&e,pj,None)
        {
      Some(i)=>{self.length = i as usize;}
      None=>{return Err(srcinf.to_message()+"要素数の算出に失敗");}
        }
    }


  let  sz = self.get_size();

    if self.content.len() == 0
    {
      self.content.resize(sz,0);

        if let Some(inits) = &self.initializers_opt
        {
            if let Err(msg) = Self::initialize_content(&mut self.content,inits,self.length,&self.ty_kind,pj)
            {
              return Err(srcinf.to_message()+msg);
            }
        }
    }


  Ok(sz)
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
get_ty_kind(&self)-> &TyKind
{
  &self.ty_kind
}


pub fn
get_initializers_opt(&self)-> &Option<Vec<Initializer>>
{
  &self.initializers_opt
}


pub fn
get_content(&self)-> &Vec<u8>
{
  &self.content
}


pub fn
get_size(&self)-> usize
{
  self.ty_kind.get_size()*self.length
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

    if let Some(inits) = &self.initializers_opt
    {
      print!("{{");

      let  mut  n = 3;

        for init in inits
        {
            if let Some(e) = &init.start_expr_opt
            {
              e.print();
            }


          init.expr.print();

          print!(",");

          n -= 1;

            if n == 0
            {
              print!(" ...");

              break;
            }
        }


      print!("}}");
    }
}


}




pub enum
DeclKind
{
  Undef,

  Const(Expr,i64),
  Static(VarDecl),
     Var(VarDecl),

  LocalStatic(String),

     String(String),
  U16String(Vec<u16>),
  U32String(Vec<u32>),

  Enum(Vec<String>),

  Fn(FnDecl),

}


impl
DeclKind
{


pub fn
print(&self, name: &str)
{
    match self
    {
  Self::Undef=>{print!("undef {}",name);}
  Self::Const(e,i)=>
    {
      print!("const {}",name);

      print!(" = ");

      e.print();

      print!(" = {}",*i);
    }
  Self::Static(v)=>
    {
      print!("static {}",name);

      v.print();
    }
  Self::Var(v)=>
    {
      print!("var {}",name);

      v.print();
    }
  Self::LocalStatic(name)=>
    {
      print!("local static {}",name);
    }
  Self::String(s)=>{print!("static  {}: u8\"{}\"",name,s);}
  Self::U16String(s)=>
    {
      print!("static  {}: u16\"",name);

        for c in s
        {
          print!("{}",char::from_u32(*c as u32).unwrap());
        }


      print!("\"");
    }
  Self::U32String(s)=>
    {
      print!("static  {}: u32\"",name);

        for c in s
        {
          print!("{}",char::from_u32(*c).unwrap());
        }


      print!("\"");
    }
  Self::Enum(ls)=>
    {
      print!("enum{{");

        for s in ls
        {
          print!("{}, ",s);
        }


      print!("}}");
    }
  Self::Fn(f)=>
    {
      print!("fn {}",name);

      f.print();
    }
    }
}


}




pub struct
Decl
{
  source_info: SourceInfo,

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

    name: String::new(),

    kind: DeclKind::Undef,

    offset: 0,

    deps_parent_names: Vec::new(),
     deps_child_names: Vec::new(),
  }
}


pub fn
new_const(name: &str, v: i64)-> Self
{
  let  mut decl = Decl::new();

  decl.name.push_str(name);

  decl.kind = DeclKind::Const(Expr::from_int(v),v);

  decl
}


pub fn
new_static(srcinf: SourceInfo, name: String, v: VarDecl)-> Self
{
  let  mut decl = Decl::new();

  decl.source_info = srcinf;

  decl.name = name;

  decl.kind = DeclKind::Static(v);

  decl
}


pub fn
new_string(srcinf: SourceInfo, name: String, mut s: String)-> Self
{
  let  mut decl = Decl::new();

  decl.source_info = srcinf;

  decl.name = name;

  s.push('\0');

  decl.kind = DeclKind::String(s);

  decl
}


pub fn
new_u16string(srcinf: SourceInfo, name: String, s: &str)-> Self
{
  let  mut decl = Decl::new();

  decl.source_info = srcinf;

  decl.name = name.to_string();

  let  mut buf = Vec::<u16>::new();

    for c in s.chars()
    {
      buf.push(c as u16);
    }


  buf.push(0);

  decl.kind = DeclKind::U16String(buf);

  decl
}


pub fn
new_u32string(srcinf: SourceInfo, name: String, s: &str)-> Self
{
  let  mut decl = Decl::new();

  decl.source_info = srcinf;

  decl.name = name;

  let  mut buf = Vec::<u32>::new();

    for c in s.chars()
    {
      buf.push(c as u32);
    }


  buf.push(0);

  decl.kind = DeclKind::U32String(buf);

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
set_offset(&mut self, off: usize)
{
  self.offset = off;
}


pub fn
get_size(&self)-> usize
{
    match &self.kind
    {
  DeclKind::Static(inf)=>{inf.get_size()}
  DeclKind::Var(_)=>{panic!();}
  DeclKind::Fn(_)=>{WORD_SIZE}
  _=>{0}
    }
}


pub fn
get_deps_parent_names(&self)-> &Vec<String>
{
  &self.deps_parent_names
}


pub fn
get_deps_child_names(&self)-> &Vec<String>
{
  &self.deps_child_names
}


pub fn
add_deps_parent_name(&mut self, s: String)
{
  self.deps_parent_names.push(s);
}


pub fn
add_deps_child_name(&mut self, s: String)
{
  self.deps_child_names.push(s);
}


pub fn
collect_identifier(&self, pj: &Project, ss: &mut StringSet)
{
    match &self.kind
    {
  DeclKind::Const(e,_)=>{e.collect_identifier(pj,ss);}
  DeclKind::Static(v) =>{v.collect_identifier(pj,ss);}
  DeclKind::Var(v)    =>{v.collect_identifier(pj,ss);}
  _=>{}
    }
}


pub fn
collect_static(&mut self, ss: &mut StaticSet)
{
    match &mut self.kind
    {
  DeclKind::Const(e,_)=>{e.collect_static(ss);}
  DeclKind::Static(v) =>{v.collect_static(ss);}
  DeclKind::Var(v)    =>{v.collect_static(ss);}
  DeclKind::Fn(f)     =>{f.block.collect_static(ss);}
  _=>{}
    }
}


pub fn
build_const_data(&mut self, pj: &Project)-> Result<usize,Message>
{
  let  srcinf = &self.source_info;

    match &mut self.kind
    {
  DeclKind::Const(e,v)=>
    {
        match evaluate_const(&e,pj,None)
        {
      Some(i)=>
        {
          *v = i;

          Ok(0)
        }
      None=>{Err(srcinf.to_message()+"constの初期化に失敗")}
        }
    }
  DeclKind::Static(v)=>
    {
      Ok(v.build_content(srcinf,pj)?)
    }
  DeclKind::Var(_)=>
    {
      Err(srcinf.to_message()+"グローバル変数の宣言はvarではなくstaticを使ってください")
    }
  DeclKind::Fn(_)=>
    {
      Ok(WORD_SIZE)
    }
  DeclKind::String(s)   =>{Ok(  s.len())}
  DeclKind::U16String(s)=>{Ok(2*s.len())}
  DeclKind::U32String(s)=>{Ok(4*s.len())}
  _=>{Ok(0)}
    }
}


pub fn
read(s: &str)-> Result<Self,Message>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  nd = crate::syntax::parse::parse_from_string("",s,dic,"declaration")?;

  let  mut cur = nd.cursor();

    if let Some(decl_nd) = cur.select_node("declaration")
    {
      read_decl(decl_nd)
    }

  else
    {Err(Message::new(format!("no decl")))}
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
read_initializer(start_nd: &Node)-> Initializer
{
  let  mut cur = start_nd.cursor();

  let  mut start_expr_opt = Option::<Expr>::None;

    if let Some(nd) = cur.select_node("subsc")
    {
      let  e = read_subsc(nd);

      start_expr_opt = Some(e);

      cur.advance(2);
    }


    if let Some(nd) = cur.select_node("expression")
    {
      return Initializer{start_expr_opt, expr: read_expr(nd)};
    }


  panic!();
}


pub fn
read_initializer_list(start_nd: &Node)-> Vec<Initializer>
{
  let  mut cur = start_nd.cursor();

  let  mut buf = Vec::<Initializer>::new();

  cur.advance(1);

    while let Some(nd) = cur.select_node("initializer")
    {
      buf.push(read_initializer(nd));

      cur.advance(1);

        if cur.is_semi_string()
        {
          cur.advance(1);
        }
    }


  buf
}


pub fn
read_storage_info(start_nd: &Node)-> (TyKind,Option<Vec<Initializer>>)
{
  let  mut cur = start_nd.cursor();

  let  mut k = TyKind::I64;
  let  mut inits_opt = Option::<Vec<Initializer>>::None;

  cur.advance(1);

    if let Some(s) = cur.get_keyword()
    {
      k =    if s ==   "i8"{TyKind::I8  }
        else if s ==  "i16"{TyKind::I16 }
        else if s ==  "i32"{TyKind::I32 }
        else if s ==  "i64"{TyKind::I64 }
        else if s ==   "u8"{TyKind::U8  }
        else if s ==  "u16"{TyKind::U16 }
        else if s ==  "u32"{TyKind::U32 }
        else{panic!();}
      ;
    }


  cur.advance(1);

    if let Some(nd) = cur.select_node("initializer_list")
    {
      inits_opt = Some(read_initializer_list(nd));
    }


  (k,inits_opt)
}


pub fn
read_var(start_nd: &Node)-> (String,VarDecl)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

      let  mut v = VarDecl::new();

      v.ty_kind = TyKind::I64;

        if let Some(nd) = cur.select_node("subsc")
        {
          v.length_expr_opt = Some(read_subsc(nd));

          cur.advance(1);
        }

      else
        {
          v.length = 1;
        }


        if cur.is_semi_string()
        {
          cur.advance(1);

            if let Some(nd) = cur.select_node("expression")
            {
              let  init = Initializer{start_expr_opt: None, expr: read_expr(nd)};

              v.initializers_opt = Some(vec![init]);
            }
        }

      else
        if let Some(nd) = cur.select_node("storage_info")
        {
          let  (k,inits_opt) = read_storage_info(nd);

          v.ty_kind = k;
          v.initializers_opt = inits_opt;
        }

      else
        {
            for _ in 0..8
            {
              v.content.push(0);
            }
        }


      return (name,v);
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
read_decl(start_nd: &Node)-> Result<Decl,Message>
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
          let  (name,v) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Var(v);
        }

      else
        if nd_name == "static"
        {
          let  (name,v) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Static(v);
        }

      else
        if nd_name == "const"
        {
          let  (name,expr) = read_const(nd);

          decl.name = name;
          decl.kind = DeclKind::Const(expr,0);
        }

      else
        {
          return Err(decl.source_info.to_message()+format!("{} is unknown decl",nd_name));
        }


      return Ok(decl);
    }


  Err(decl.source_info.to_message()+"read_decl error")
}




