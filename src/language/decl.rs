

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
use super::font14::*;
use super::font8::*;
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
StorageInfo
{
  length: usize,
  length_expr_opt: Option<Expr>,

  ty_kind: TyKind,

  init_exprs_opt: Option<Vec<Expr>>,

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
    ty_kind: TyKind::Void,
    init_exprs_opt: None,
    content: Vec::new(),
    is_utf8: false,
  }
}


pub fn
from_string(s: &str)-> Self
{
  let  mut inf = Self::new();

  inf.length = s.len()+1;
  inf.is_utf8 = true;

    for b in s.as_bytes()
    {
      inf.content.push(*b);
    }


  inf.content.push(0);

  inf
}


pub fn
from_data(data: Vec<u8>, k: TyKind)-> Self
{
  let  element_size = k.get_size();

  let  length = if element_size != 0{data.len()/element_size} else{0};


  Self{
    length,
    length_expr_opt: None,
    ty_kind: k,
    init_exprs_opt: None,
    content: data,
    is_utf8: false,
  }
}


pub fn
collect_identifier(&self, pj: &Project, ss: &mut StringSet)
{
    if let Some(e) = &self.length_expr_opt
    {
      e.collect_identifier(pj,ss);
    }


    if let Some(exprs) = &self.init_exprs_opt
    {
        for e in exprs
        {
          e.collect_identifier(pj,ss);
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


    if let Some(exprs) = &mut self.init_exprs_opt
    {
        for e in exprs
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
get_ty_kind(&self)-> &TyKind
{
  &self.ty_kind
}


pub fn
get_init_exprs_opt(&self)-> &Option<Vec<Expr>>
{
  &self.init_exprs_opt
}


pub fn
get_size(&self)-> usize
{
  self.ty_kind.get_size()*self.length
}


pub fn
get_content(&self)-> &Vec<u8>
{
  &self.content
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

    if let Some(exprs) = &self.init_exprs_opt
    {
      print!("{{");

        for e in exprs
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
  DeclKind::Fn(f)=>
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
new_static(srcinf: SourceInfo, name: String, si: StorageInfo)-> Self
{
  let  mut decl = Decl::new();

  decl.source_info = srcinf;

  decl.name = name;

  decl.kind = DeclKind::Static(si);

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
  DeclKind::Static(inf)=>{inf.collect_identifier(pj,ss);}
  DeclKind::Var(inf)=>{inf.collect_identifier(pj,ss);}
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
  _=>{}
    }
}


pub fn
initialize_content(dst: &mut [u8], exprs: &[Expr], mut n: usize, k: &TyKind, pj: &Project)-> Result<(),Message>
{
  let  mut ptr = dst.as_mut_ptr();

  let  sz = k.get_size();

    for e in exprs
    {
        if n == 0
        {
          break;
        }


        match evaluate_const(e,pj,None)
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
      None=>{return Err(e.get_source_info().to_message()+"initialize_content error: const value eval is failed")}
        }


      n -= 1;
    }


  Ok(())
}


pub fn
build_const_data(&mut self, pj: &Project)-> Result<(),Message>
{
  let  srcinf = &self.source_info;

    match &mut self.kind
    {
  DeclKind::Const(e,v)=>
    {
        match evaluate_const(&e,pj,None)
        {
      Some(i)=>{*v = i;}
      None=>{return Err(srcinf.to_message()+"constの初期化に失敗");}
        }
    }
  DeclKind::Static(inf)=>
    {
        if let Some(e) = &inf.length_expr_opt
        {
            match evaluate_const(&e,pj,None)
            {
          Some(i)=>{inf.length = i as usize;}
          None=>{return Err(srcinf.to_message()+"staticの要素数の算出に失敗");}
            }
        }


        if inf.content.len() == 0
        {
          let  sz = inf.get_size();

          inf.content.resize(sz,0);

            if let Some(exprs) = &inf.init_exprs_opt
            {
                if let Err(msg) = Self::initialize_content(&mut inf.content,exprs,inf.length,&inf.ty_kind,pj)
                {
                  return Err(srcinf.to_message()+msg);
                }
            }
        }
    }
  DeclKind::Var(_)=>
    {
      return Err(srcinf.to_message()+"グローバル変数の宣言はvarではなくstaticを使ってください");
    }
  _=>{}
    }


  Ok(())
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
      inf.length = 1;
    }


    if let Some(_) = cur.get_semi_string()
    {
      cur.advance(1);

        if let Some(s) = cur.get_keyword()
        {
          inf.ty_kind =
                 if s ==   "i8"{TyKind::I8  }
            else if s ==  "i16"{TyKind::I16 }
            else if s ==  "i32"{TyKind::I32 }
            else if s ==  "i64"{TyKind::I64 }
            else if s ==   "u8"{TyKind::U8  }
            else if s ==  "u16"{TyKind::U16 }
            else if s ==  "u32"{TyKind::U32 }
            else{panic!();}
          ;


          cur.advance(1);

            if let Some(nd) = cur.select_node("expression_list")
            {
              inf.init_exprs_opt = Some(read_expr_list(nd));
            }
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
                  let  e = read_expr(nd);

                  inf.init_exprs_opt = Some(vec![e]);
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
        {
          return Err(decl.source_info.to_message()+format!("{} is unknown decl",nd_name));
        }


      return Ok(decl);
    }


  Err(decl.source_info.to_message()+"read_decl error")
}




