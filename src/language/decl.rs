

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
use super::asm::AsmText;
use super::font14::*;
use super::font8::*;
use super::tplg_sort::*;
use super::evaluate::*;
use super::evaluate_const::*;
use super::exec::*;




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




pub enum
StorageKind
{
  Word(Option<Expr>,i64),

   EmptyField(Expr,usize),

  FilledField(Vec<u8>),

  String(String,Vec<u8>),

}


impl
StorageKind
{


pub fn
collect_identifier(&self, set: &DeclSet, ss: &mut StringSet)
{
    match self
    {
  Self::Word(e_opt,_)=>{if let Some(e) = e_opt{e.collect_identifier(set,ss);}}
  Self::EmptyField(e,_)=>{e.collect_identifier(set,ss);}
  _=>{}
    }
}


pub fn
collect_string(&self, ss: &mut StringSet)
{
    match self
    {
  Self::Word(e_opt,_)=>{if let Some(e) = e_opt{e.collect_string(ss)}}
  Self::EmptyField(e,_)=>{e.collect_string(ss)}
  _=>{}
    }
}


pub fn
get_size(&self)-> usize
{
    match self
    {
  Self::Word(_,_)=>{WORD_SIZE}
  Self::EmptyField(_,i)=>{*i}
  Self::FilledField(bytes)=>{bytes.len()}
  Self::String(_,bytes)=>{bytes.len()}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Word(e_opt,_)=>
    {
        if let Some(e) = e_opt
        {
          print!(" = ");

          e.print();
        }
    }
  Self::EmptyField(e,i)=>
    {
      print!("[");

      e.print();

      print!("]");
    }
  Self::FilledField(bytes)=>
    {
      print!("{{...}}");
    }
  Self::String(s,_)=>
    {
      print!(": \"{}\"",s);
    }
    }
}


}




pub enum
DeclKind
{
  Undef,

  Const(Expr,i64),
  Static(StorageKind),
     Var(StorageKind),

  Enum(Vec<String>),

  Fn(FnDecl),

  InlineAsm(AsmText),

  Class(Box<DeclSet>),

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
  DeclKind::Static(k)=>
    {
      print!("static {}",name);

      k.print();
    }
  DeclKind::Var(k)=>
    {
      print!("var {}",name);

      k.print();
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
  DeclKind::InlineAsm(txt)=>
    {
      print!("asm{{");

      txt.print(0);

      print!("__");
    }
  DeclKind::Class(set)=>
    {
      println!("class {}{{",name);

      set.print();

      println!("\n}}");
    }
    }
}


}




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
insert(&mut self, new_s: String)
{
    for s in &self.set
    {
        if s == &new_s
        {
          return;
        }
    }


  self.set.push(new_s);
}


pub fn
record_fail(&mut self, srcinf: &SourceInfo, s: String)
{
  self.fail_records.push((srcinf.clone(),s));
}


pub fn
expire(self)-> Vec<String>
{
  self.set
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
  DeclKind::Static(k)=>{k.collect_identifier(set,ss);}
  DeclKind::Var(k)=>{k.collect_identifier(set,ss);}
  DeclKind::Class(set)=>{set.collect_identifier(ss);}
  _=>{}
    }
}


pub fn
collect_string(&self, ss: &mut StringSet)
{
    match &self.kind
    {
  DeclKind::Const(e,_)=>{e.collect_string(ss);}
  DeclKind::Static(k)=>{k.collect_string(ss);}
  DeclKind::Var(k)=>{k.collect_string(ss);}
  DeclKind::Class(set)=>{set.collect_string(ss);}
  _=>{}
    }
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
      EvalResult::Const(i)=>{*v = i;}
      _=>{return Err(srcinf.to_error(format!("constの初期化に失敗")));}
        }
    }
  DeclKind::Static(k)=>
    {
        match k
        {
      StorageKind::Word(e_opt,v)=>
        {
            if let Some(e) = e_opt
            {
                match evaluate_const(&e,set,None)
                {
              EvalResult::Const(i)=>{*v = i;}
              _=>{return Err(srcinf.to_error(format!("varの初期化に失敗")));}
                }
            }
        }
      StorageKind::EmptyField(e,sz)=>
        {
            match evaluate_const(e,set,None)
            {
          EvalResult::Const(i)=>{*sz = i as usize;}
          _=>{return Err(srcinf.to_error(format!("varの初期化に失敗")));}
            }
        }
      StorageKind::FilledField(bytes)=>
        {
        }
      StorageKind::String(s,bytes)=>
        {
          bytes.clear();

            for c in s.chars()
            {
                for b in (c as u16).to_ne_bytes()
                {
                  bytes.push(b);
                }
            }
        }
        }
    }
  DeclKind::Var(_)=>
    {
      return Err(srcinf.to_error(format!("グローバル変数の宣言はvarではなくstaticを使ってください")));
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
read_expr_for_init(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

   cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      return read_expr(nd);
    }


  panic!();
}


pub fn
read_var(start_nd: &Node)-> (String,StorageKind)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      let  mut k = StorageKind::Word(None,0);

      cur.advance(1);

        if let Some(nd) = cur.select_node("init_as_word")
        {
          let  e = read_expr_for_init(nd);

          k = StorageKind::Word(Some(e),0);
        }

      else
        if let Some(nd) = cur.select_node("init_as_field")
        {
          let  e = read_expr_for_init(nd);

          k = StorageKind::EmptyField(e,0);
        }

      else
        if let Some(nd) = cur.select_node("init_by_data")
        {
todo!();
        }


      return (name,k);
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
read_class(start_nd: &Node)-> Result<(String,DeclSet),Error>
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
          let  (name,k) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Var(k);
        }

      else
        if nd_name == "static"
        {
          let  (name,k) = read_var(nd);

          decl.name = name;
          decl.kind = DeclKind::Static(k);
        }

      else
        if nd_name == "const"
        {
          let  (name,expr) = read_const(nd);

          decl.name = name;
          decl.kind = DeclKind::Const(expr,0);
        }

      else
        if nd_name == "class"
        {
            match read_class(nd)
            {
          Ok((name,set))=>
            {
              decl.name = name;
              decl.kind = DeclKind::Class(Box::new(set));
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

  size: usize,

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
    size: 0,
  }
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


      Ok(Box::new(set))
    }
  Err(e)=>{Err(e)}
    }
}


pub fn
make_name_for_string(s: &str)-> String
{
  format!(".S:{}",s)
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


                if let DeclKind::Class(set) = &decl.kind
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
find_string(&self, s: &str)-> Option<&Decl>
{
  let  name = DeclSet::make_name_for_string(s);

  self.find(&name)
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
collect_string(&self, ss: &mut StringSet)
{
    for decl in &self.decls
    {
      decl.collect_string(ss);
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

        if let DeclKind::Class(set) = &mut decl.kind
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

        if let DeclKind::Class(set) = &mut decl.kind
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


        if let DeclKind::Class(set) = &mut self.decls[i].kind
        {
          set.process_deps_relationship();
        }
    }


  Ok(())
}


fn
process_data_offset(&mut self, start: usize)-> usize
{
  let  mut pos = start;

  self.size = 0;

    for decl in &mut self.decls
    {
        match &mut decl.kind
        {
      DeclKind::Static(k)=>
        {
          decl.offset = get_word_aligned(pos)               ;
                                         pos += k.get_size();
        }
      DeclKind::Var(k)=>
        {
          panic!();
        }
      DeclKind::Fn(_)=>
        {
          decl.offset = get_word_aligned(pos)            ;
                                         pos += WORD_SIZE;
        }
      DeclKind::Class(set)=>
        {
          set.process_data_offset(0);
        }
      _=>{}
        }
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
  let  mut ss = StringSet::new();

  self.collect_string(&mut ss);

    for s in ss.set
    {
      let  mut decl = Decl::new();

      decl.name = Self::make_name_for_string(&s);

      let  k = StorageKind::String(s,Vec::new());

      decl.kind = DeclKind::Static(k);

      self.insert(decl);
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
      DeclKind::Class(set)=>
        {
            match set.write_to_exec(exec,pos)
            {
          Ok(())=>{}
          Err(e)=>{return Err(e);}
            }
        }
      DeclKind::Fn(fd)=>
        {
          let   ptr_sym = Symbol::new(decl.offset,q_name.clone(),SymbolKind::Data);
          let  text_sym = Symbol::new(       *pos,q_name.clone(),SymbolKind::Text);

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
          exec.add_symbol(Symbol::new(0,q_name.clone(),SymbolKind::Const(*v)));
        }
      DeclKind::Static(k)=>
        {
            match k
            {
          StorageKind::Word(_,v)=>
            {
              exec.put_u64(decl.offset,*v as u64);

              exec.add_symbol(Symbol::new(decl.offset,q_name,SymbolKind::Data));
            }
          StorageKind::EmptyField(_,sz)=>
            {
              exec.add_symbol(Symbol::new(decl.offset,q_name,SymbolKind::Field(*sz)));
            }
          StorageKind::FilledField(bytes)=>
            {
              exec.put_bytes(decl.offset,bytes);

              exec.add_symbol(Symbol::new(decl.offset,q_name,SymbolKind::Field(bytes.len())));
            }
          StorageKind::String(_,bytes)=>
            {
              exec.put_bytes(decl.offset,bytes);

              exec.add_symbol(Symbol::new(decl.offset,q_name,SymbolKind::Field(bytes.len())));
            }
            }
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


  exec.add_symbol(Symbol::new(0,"HEAP_START".to_string(),SymbolKind::Const(pos as i64)));

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

  let  k = StorageKind::FilledField(new_data);

  decl.name = name.to_string();
  decl.kind = DeclKind::Static(k);

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




