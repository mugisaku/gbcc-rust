

use super::decl::*;
use super::expr::*;
use super::ty::*;
use super::opcode::*;
use super::evaluate::*;
use super::execute::*;




pub struct
FunctionDef
{
  parameter_list: Vec<SizedField>,

  return_ty: SizedTy,

  opcode_list: Vec<Opcode>,

  offset: usize,

}


impl
FunctionDef
{


pub fn
new()-> Self
{
  Self{
    parameter_list: Vec::new(),
    return_ty: SizedTy::Void,
    opcode_list: Vec::new(),
    offset: 0,
  }
}


}




pub struct
TempVar
{
  ty: SizedTy,

  offset: usize,

}


impl
TempVar
{


pub fn
new()-> Self
{
  Self{
    ty: SizedTy::Void,
    offset: 0,
  }
}


pub fn
new_with_ty(ty: SizedTy)-> Self
{
  Self{
    ty,
    offset: 0,
  }
}


pub fn
get_ty(&self)-> &SizedTy
{
  &self.ty
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


}



pub struct
PermVar
{
  ty: SizedTy,

  data_opt: Option<Vec<u8>>,

  offset: usize,

}


impl
PermVar
{


pub fn
new()-> Self
{
  Self{
    ty: SizedTy::Void,
    data_opt: None,
    offset: 0,
  }
}


pub fn
new_void()-> Self
{
  Self{
    ty: SizedTy::Void,
    data_opt: None,
    offset: 0,
  }
}


pub fn
from_8bits(ty: SizedTy, bits: u8)-> Self
{
  Self{
    ty,
    data_opt: Some(vec![bits]),
    offset: 0,
  }
}


pub fn
from_16bits(ty: SizedTy, bits: u16)-> Self
{
  Self{
    ty,
    data_opt: Some(bits.to_be_bytes().to_vec()),
    offset: 0,
  }
}


pub fn
from_32bits(ty: SizedTy, bits: u32)-> Self
{
  Self{
    ty,
    data_opt: Some(bits.to_be_bytes().to_vec()),
    offset: 0,
  }
}


pub fn
from_64bits(ty: SizedTy, bits: u64)-> Self
{
  Self{
    ty,
    data_opt: Some(bits.to_be_bytes().to_vec()),
    offset: 0,
  }
}


pub fn  from_bool(b: bool)-> Self{Self::from_8bits(SizedTy::Bool,if b{1} else{0})}
pub fn     from_i8(i: i8)->    Self{Self::from_8bits( SizedTy::I8   ,i as u8)}
pub fn    from_i16(i: i16)->   Self{Self::from_16bits(SizedTy::I16  ,i as u16)}
pub fn    from_i32(i: i32)->   Self{Self::from_32bits(SizedTy::I32  ,i as u32)}
pub fn    from_i64(i: i64)->   Self{Self::from_64bits(SizedTy::I64  ,i as u64)}
pub fn  from_isize(i: isize)-> Self{Self::from_64bits(SizedTy::ISize,i as u64)}

pub fn     from_u8(u: u8)->    Self{Self::from_8bits( SizedTy::U8   ,u)}
pub fn    from_u16(u: u16)->   Self{Self::from_16bits(SizedTy::U16  ,u)}
pub fn    from_u32(u: u32)->   Self{Self::from_32bits(SizedTy::U32  ,u)}
pub fn    from_u64(u: u64)->   Self{Self::from_64bits(SizedTy::U64  ,u)}
pub fn  from_usize(u: usize)-> Self{Self::from_64bits(SizedTy::USize,u as u64)}

pub fn  from_f32(f: f32)-> Self{Self::from_32bits(SizedTy::F32,f.to_bits())}
pub fn  from_f64(f: f64)-> Self{Self::from_64bits(SizedTy::F64,f.to_bits())}


pub fn
get_ty(&self)-> &SizedTy
{
  &self.ty
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
try_from_eval_result_with_ty(res: EvalResult, ty: &SizedTy)-> Result<Self,()>
{
    match res
    {
  EvalResult::Int(i)=>
    {
        match ty
        {
      SizedTy::I8   =>{if let Ok(new_i) =    i8::try_from(i){return Ok(PermVar::from_i8(   new_i));}}
      SizedTy::I16  =>{if let Ok(new_i) =   i16::try_from(i){return Ok(PermVar::from_i16(  new_i));}}
      SizedTy::I32  =>{if let Ok(new_i) =   i32::try_from(i){return Ok(PermVar::from_i32(  new_i));}}
      SizedTy::I64  =>{if let Ok(new_i) =   i64::try_from(i){return Ok(PermVar::from_i64(  new_i));}}
      SizedTy::ISize=>{if let Ok(new_i) = isize::try_from(i){return Ok(PermVar::from_isize(new_i));}}
      _=>{}
        }
    }
  EvalResult::Uint(u)=>
    {
        match ty
        {
      SizedTy::U8   =>{if let Ok(new_u) =    u8::try_from(u){return Ok(PermVar::from_u8(   new_u));}}
      SizedTy::U16  =>{if let Ok(new_u) =   u16::try_from(u){return Ok(PermVar::from_u16(  new_u));}}
      SizedTy::U32  =>{if let Ok(new_u) =   u32::try_from(u){return Ok(PermVar::from_u32(  new_u));}}
      SizedTy::U64  =>{                                      return Ok(PermVar::from_u64(      u)); }
      SizedTy::USize=>{if let Ok(new_u) = usize::try_from(u){return Ok(PermVar::from_usize(new_u));}}
      _=>{}
        }
    }
  EvalResult::Float(f)=>
    {
        match ty
        {
      SizedTy::F32=>{if f.abs() <= (f32::MAX as f64){return Ok(PermVar::from_f32(f as f32));}}
      SizedTy::F64=>{return Ok(PermVar::from_f64(f));}
      _=>{}
        }
    }
  EvalResult::Void=>{if let SizedTy::Void = ty{return Ok(Self::new_void());}}
  EvalResult::Bool(b)=>{if let SizedTy::Bool = ty{return Ok(PermVar::from_bool(b));}}

  EvalResult::I8(i)   =>{if let SizedTy::I8    = ty{return Ok(PermVar::from_i8( i));}}
  EvalResult::I16(i)  =>{if let SizedTy::I16   = ty{return Ok(PermVar::from_i16(i));}}
  EvalResult::I32(i)  =>{if let SizedTy::I32   = ty{return Ok(PermVar::from_i32(i));}}
  EvalResult::I64(i)  =>{if let SizedTy::I64   = ty{return Ok(PermVar::from_i64(i));}}
  EvalResult::ISize(i)=>{if let SizedTy::ISize = ty{return Ok(PermVar::from_isize(i));}}

  EvalResult::U8(u)   =>{if let SizedTy::U8    = ty{return Ok(PermVar::from_u8( u));}}
  EvalResult::U16(u)  =>{if let SizedTy::U16   = ty{return Ok(PermVar::from_u16(u));}}
  EvalResult::U32(u)  =>{if let SizedTy::U32   = ty{return Ok(PermVar::from_u32(u));}}
  EvalResult::U64(u)  =>{if let SizedTy::U64   = ty{return Ok(PermVar::from_u64(u));}}
  EvalResult::USize(u)=>{if let SizedTy::USize = ty{return Ok(PermVar::from_usize(u));}}

  EvalResult::F32(f)=>{if let SizedTy::F32 = ty{return Ok(PermVar::from_f32(f));}}
  EvalResult::F64(f)=>{if let SizedTy::F64 = ty{return Ok(PermVar::from_f64(f));}}
  _=>{}
    }


  Err(())
}


}


impl
std::convert::TryFrom<EvalResult> for PermVar
{


type Error = ();


fn
try_from(res: EvalResult)-> Result<PermVar,Self::Error>
{
    match res
    {
  EvalResult::Int(i)  =>{Ok(PermVar::from_i64(i))}
  EvalResult::Uint(u) =>{Ok(PermVar::from_u64(u))}
  EvalResult::Float(f)=>{Ok(PermVar::from_f64(f))}

  EvalResult::Void=>{Ok(PermVar::new_void())}
  EvalResult::Bool(b)=>{Ok(PermVar::from_bool(b))}

  EvalResult::I8(i)   =>{Ok(PermVar::from_i8( i))}
  EvalResult::I16(i)  =>{Ok(PermVar::from_i16(i))}
  EvalResult::I32(i)  =>{Ok(PermVar::from_i32(i))}
  EvalResult::I64(i)  =>{Ok(PermVar::from_i64(i))}
  EvalResult::ISize(i)=>{Ok(PermVar::from_isize(i))}

  EvalResult::U8(u)   =>{Ok(PermVar::from_u8( u))}
  EvalResult::U16(u)  =>{Ok(PermVar::from_u16(u))}
  EvalResult::U32(u)  =>{Ok(PermVar::from_u32(u))}
  EvalResult::U64(u)  =>{Ok(PermVar::from_u64(u))}
  EvalResult::USize(u)=>{Ok(PermVar::from_usize(u))}

  EvalResult::F32(f)=>{Ok(PermVar::from_f32(f))}
  EvalResult::F64(f)=>{Ok(PermVar::from_f64(f))}
  _=>{Err(())}
    }
}

}




pub enum
SymbolKind
{
  Null,
  None,

  Type(SizedTy),
  ConstVar(EvalResult),
  TempVar(TempVar),
  PermVar(PermVar),
  Function(FunctionDef),

}


pub struct
Symbol
{
  index: usize,

  name: String,

  decl_kind: DeclKind,

  kind: SymbolKind,

  namespace: String,

}


impl
Symbol
{


pub fn
new(name: &str, kind: SymbolKind)-> Self
{
  Self{
    index: 0,
    name: name.to_string(),
    decl_kind: DeclKind::Undef,
    kind,
    namespace: String::new(),
  }
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_index(&self)-> usize
{
  self.index
}


pub fn
get_kind(&self)-> &SymbolKind
{
  &self.kind
}


pub fn
get_kind_mut(&mut self)-> &mut SymbolKind
{
  &mut self.kind
}


pub fn
get_decl_Kind(&self)-> &DeclKind
{
  &self.decl_kind
}


pub fn
release_decl_kind(&mut self)-> DeclKind
{
  let  mut k = DeclKind::Undef;

  std::mem::swap(&mut self.decl_kind,&mut k);

  k
}


pub fn
reset_decl_kind(&mut self, k: DeclKind)
{
  self.decl_kind = k;
}


pub fn
get_namespace(&self)-> &String
{
  &self.namespace
}


pub fn
print(&self)
{
  println!("namespace: {}",&self.namespace);

  self.decl_kind.print();
}


}




struct
Node
{
  symbol_index: usize,

  reference_count: std::cell::Cell<usize>,
  is_alive: std::cell::Cell<bool>,

  required_index_list: Vec<usize>,

}


impl
Node
{


pub fn
new(symbol_index: usize)-> Self
{
  Self{
    symbol_index,
    reference_count: std::cell::Cell::new(0),
    is_alive: std::cell::Cell::new(true),
    required_index_list: Vec::new(),
  }
}


pub fn
is_alive(&self)-> bool
{
  self.is_alive.get()
}


pub fn
kill(&self)
{
  self.is_alive.set(false)
}


pub fn
get_count(&self)-> usize
{
  self.reference_count.get()
}


pub fn
increase(&self)
{
  let  n = self.reference_count.get();

  self.reference_count.set(n+1);
}


pub fn
decrease(&self)
{
  let  n = self.reference_count.get();

  self.reference_count.set(n-1);
}


}




pub struct
SymbolTable
{
  core: Vec<Symbol>,

   type_index_list: Vec<usize>,
  const_index_list: Vec<usize>,
   perm_index_list: Vec<usize>,
  function_index_list: Vec<usize>,

}


impl
SymbolTable
{


pub fn
new()-> Self
{
  let  mut tbl = Self{
    core: Vec::new(),
      type_index_list: Vec::new(),
     const_index_list: Vec::new(),
      perm_index_list: Vec::new(),
  function_index_list: Vec::new(),
  };


  tbl.add("",Decl::new_primitive_type("void",SizedTy::Void));

  tbl
}


pub fn
add(&mut self, namespace: &str, decl: Decl)
{
  let  (decl_name,decl_kind) = decl.decompose();

    if let DeclKind::Space(mut sp) = decl_kind
    {
      let  mut new_namespace = namespace.to_string();

        if decl_name.len() != 0
        {
          new_namespace.push_str("::");

          new_namespace.push_str(&decl_name);
        }


        for child in sp.get_decl_list_mut()
        {
          let  mut tmp = Decl::new();

          std::mem::swap(child,&mut tmp);

          self.add(&new_namespace,tmp);
        }
    }

  else
    {
      let  index = self.core.len();

        match &decl_kind
        {
      DeclKind::Type(_)    =>{self.type_index_list.push(index);}
      DeclKind::Var(_)     =>{self.perm_index_list.push(index);}
      DeclKind::Static(_)  =>{self.perm_index_list.push(index);}
      DeclKind::Const(_)   =>{self.const_index_list.push(index);}
      DeclKind::Function(_)=>{self.function_index_list.push(index);}
      _=>{}
        }


      let  sym = Symbol{
        index,
        name: decl_name,
        decl_kind,
        kind: SymbolKind::Null,
        namespace: namespace.to_string(),
      };


      self.core.push(sym);
    }
}


pub fn
search_required_for_sized_ty(&self, ty: &SizedTy, buf: &mut Vec<usize>)
{
    match ty
    {
  SizedTy::Pointer(ty)  =>{self.search_required_for_ty(ty,buf);}
  SizedTy::Reference(ty)=>{self.search_required_for_ty(ty,buf);}
  SizedTy::Function(ls,ret_ty)=>
    {
      self.search_required_for_ty(ret_ty,buf);

        for ty in ls
        {
          self.search_required_for_ty(ty,buf);
        }
    }
  _=>{}
    }
}


pub fn
search_required_for_ty(&self, ty: &Ty, buf: &mut Vec<usize>)
{
    match ty
    {
  Ty::Tuple(ls)=>
    {
        for f in ls
        {
          self.search_required_for_ty(f.get_ty(),buf);
        }
    }
  Ty::Struct(ls)=>
    {
        for f in ls
        {
          self.search_required_for_ty(f.get_ty(),buf);
        }
    }
  Ty::Union(ls)=>
    {
        for f in ls
        {
          self.search_required_for_ty(f.get_ty(),buf);
        }
    }
  Ty::Enum(ls)=>
    {
        for e in ls
        {
        }
    }
  Ty::Alias(s)=>
    {
        if let Some(sym) = self.find(s)
        {
          buf.push(sym.get_index());
        }
    }
  Ty::Array(ty,e)=>
    {
      self.search_required_for_ty(ty,buf);
    }
  Ty::Sized(ty)=>
    {
      self.search_required_for_sized_ty(ty,buf);
    }
  _=>{}
    }
}


pub fn
search_required_for_expr(&self, e: &Expr, buf: &mut Vec<usize>)
{
    match e
    {
  Expr::Identifier(s)=>
    {
        if let Some(sym) = self.find(s)
        {
          buf.push(sym.get_index());
        }
    }
  Expr::AccessOp(e,_)=>
    {
      self.search_required_for_expr(e,buf);
    }
  Expr::SubscriptOp(e,i_e)=>
    {
      self.search_required_for_expr(  e,buf);
      self.search_required_for_expr(i_e,buf);
    }
  Expr::CallOp(f,args)=>
    {
      self.search_required_for_expr(f,buf);

        for a in args
        {
          self.search_required_for_expr(a,buf);
        }
    }
  Expr::Expr(e)=>
    {
      self.search_required_for_expr(e,buf);
    }
  Expr::UnaryOp(o,_)=>
    {
      self.search_required_for_expr(o,buf);
    }
  Expr::BinaryOp(l,r,_)=>
    {
      self.search_required_for_expr(l,buf);
      self.search_required_for_expr(r,buf);
    }
  _=>{}
    }
}


fn
make_node_list(&self)-> Vec<Node>
{
  let  mut node_ls = Vec::<Node>::new();

    for sym in &self.core
    {
      let  mut nd = Node::new(sym.index);

      let  ls = &mut nd.required_index_list;

        match &sym.decl_kind
        {
      DeclKind::Type(ty)=>{self.search_required_for_ty(ty,ls);}
      DeclKind::Var(o)=>
        {
          self.search_required_for_ty(o.get_ty(),ls);
          self.search_required_for_expr(o.get_expr(),ls);
        }
      DeclKind::Const(o)=>
        {
          self.search_required_for_ty(o.get_ty(),ls);
          self.search_required_for_expr(o.get_expr(),ls);
        }
      DeclKind::Static(o)=>
        {
          self.search_required_for_ty(o.get_ty(),ls);
          self.search_required_for_expr(o.get_expr(),ls);
        }
      DeclKind::Function(f)=>
        {
          self.search_required_for_ty(f.get_return_ty(),ls);

            for para in f.get_parameter_list()
            {
              self.search_required_for_ty(para.get_ty(),ls);
            }
        }
      _=>{panic!();}
        }


      node_ls.push(nd);
    }


  node_ls
}


fn
initialize_node_list(node_ls: &Vec<Node>)
{
    for nd in node_ls
    {
        for req_i in &nd.required_index_list
        {
          &node_ls[*req_i].increase();
        }
    }
}


fn
collect_noref(node_ls: &Vec<Node>, stack: &mut Vec<usize>)-> Result<(),()>
{
  let  mut buf = Vec::<&Node>::new();

    for nd in node_ls
    {
        if nd.is_alive()
        {
            if nd.get_count() == 0
            {
              buf.push(nd);
            }
        }
    }


    if buf.is_empty()
    {
      return Err(());
    }


    for nd in buf
    {
        for i in &nd.required_index_list
        {
          node_ls[*i].decrease();
        }


      stack.push(nd.symbol_index);

      nd.kill();
    }


  Ok(())
}


fn
get_topological_sorted_index_list(&self)-> Result<Vec<usize>,()>
{
  let  mut stack = Vec::<usize>::new();
  let  mut node_ls = self.make_node_list();

  Self::initialize_node_list(&node_ls);

    while stack.len() != node_ls.len()
    {
        if Self::collect_noref(&node_ls,&mut stack).is_err()
        {
          println!("循環参照を検出");

          return Err(());
        }
    }


  Ok(stack)
}


fn
make_kind_from_ty(&self, ty: &Ty)-> Result<SymbolKind,()>
{
  let  ctx = ExecContext::new_with_symbol_table(self);

    if let Ok(sized) = ty.get_sized(&ctx)
    {
      return Ok(SymbolKind::Type(sized));
    }


  Err(())
}




fn
make_const_var_from_object(&self, obj: &Object)-> Result<SymbolKind,()>
{
  let  ctx = ExecContext::new_with_symbol_table(self);

  let  res = evaluate(obj.get_expr(),&ctx);

    if let Ty::Undef = obj.get_ty()
    {
    }

  else
    if let Ok(sized) = obj.get_ty().get_sized(&ctx)
    {
    }


  Err(())
}


fn
make_perm_var_from_object(&self, obj: &Object)-> Result<SymbolKind,()>
{
  let  ctx = ExecContext::new_with_symbol_table(self);

  let  res = evaluate(obj.get_expr(),&ctx);

    if let Ty::Undef = obj.get_ty()
    {
        if let Ok(pv) = PermVar::try_from(res)
        {
          return Ok(SymbolKind::PermVar(pv));
        }
    }

  else
    if let Ok(sized) = obj.get_ty().get_sized(&ctx)
    {
        if let Ok(pv) = PermVar::try_from_eval_result_with_ty(res,&sized)
        {
          return Ok(SymbolKind::PermVar(pv));
        }
    }


  Err(())
}


fn
make_kind_from_function(&self, f: &Function)-> Result<SymbolKind,()>
{
  let  ctx = ExecContext::new_with_symbol_table(self);

    if let Ok(sized_ty) = f.get_return_ty().get_sized(&ctx)
    {
      let  mut parameter_list = Vec::<SizedField>::new();

      let  mut offset = 0usize;

      let  ctx = ExecContext::new_with_symbol_table(self);

        for p in f.get_parameter_list()
        {
            if let Ok(mut sized_field) = p.get_sized(&ctx)
            {
              offset = super::get_word_aligned(offset+sized_field.get_ty().get_info().get_size());

              sized_field.set_offset(offset);

              parameter_list.push(sized_field);
            }

          else
            {
              return Err(());
            }
        }


      let  fd = FunctionDef{parameter_list, return_ty: sized_ty, opcode_list: Vec::new(), offset: 0};

      return Ok(SymbolKind::Function(fd));
    }


  Err(())
}


pub fn
complete(&mut self)-> Result<(),()>
{
    if let Ok(ls) = self.get_topological_sorted_index_list()
    {
        for i in ls
        {
          let  res = match &self.core[i].decl_kind
            {
          DeclKind::Type(ty)   =>{self.make_kind_from_ty(ty)}
          DeclKind::Static(o)  =>{self.make_perm_var_from_object(o)}
          DeclKind::Const(o)   =>{self.make_const_var_from_object(o)}
          DeclKind::Function(f)=>{self.make_kind_from_function(f)}
          _=>{return Err(());}
            };


            if let Ok(symk) = res
            {
              self.core[i].kind = symk;
            }
        }


      return Ok(());
    }


  Err(())
}


pub fn
allocate_objects(&mut self, mut offset: usize)
{
    for i in &self.perm_index_list
    {
        if let SymbolKind::PermVar(pv) = &mut self.core[*i].kind
        {
          offset = super::get_word_aligned(offset);

          pv.offset = offset;

          offset += pv.ty.get_info().get_size();
        }
    }
}


pub fn
build_function_codes(&mut self)-> Result<(),()>
{
    for i in &self.function_index_list
    {
        if let DeclKind::Function(fdecl) = &self.core[*i].decl_kind
        {
          let  mut ctx = ExecContext::new_with_symbol_table(self);

          match execute_block_as_const(fdecl.get_block(),&mut ctx)
          {
        ExecResult::Return(opt)=>
          {
              if let Some(eval_res) = opt
              {
                eval_res.print();
println!();
              }
          }
        _=>{}
          }

/*
            if let Ok(ls) = fdecl.get_block().codify(self)
            {
                if let SymbolKind::Function(fdef) = &mut self.core[*i].kind
                {
                  fdef.opcode_list = ls;

                  continue;
                }
            }
*/
        }


      return Err(());
    }


  Ok(())
}


pub fn
get(&self, i: usize)-> &Symbol
{
  &self.core[i]
}


pub fn
find(&self, name: &str)-> Option<&Symbol>
{
    for sym in &self.core
    {
        if &sym.name == name
        {
          return Some(sym);
        }
    }


  None
}


pub fn
find_mut(&mut self, name: &str)-> Option<&mut Symbol>
{
    for sym in &mut self.core
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
  println!("const objects{{");

    for i in &self.const_index_list
    {
      self.core[*i].print();

      println!("\n");
    }


  println!("}}\n\nstatic objects{{");

    for i in &self.perm_index_list
    {
      self.core[*i].print();

      println!("\n");
    }


  println!("}}\n\nfunction objects{{");

    for i in &self.function_index_list
    {
      self.core[*i].print();

      println!("\n");
    }


  println!("}}");
}

}


impl<'a>
std::convert::From<Decl> for SymbolTable
{


fn
from(decl: Decl)-> SymbolTable
{
  let  mut tbl = SymbolTable::new();

  let  _ = tbl.add("",decl);

  tbl
}


}




