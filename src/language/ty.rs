

use std::rc::Rc;

use crate::node::*;
use super::asm::*;
use super::expr::*;
use super::evaluate_const::*;
use super::evaluate::*;
use super::symbol_table::*;
use super::decl::ParameterDecl;
use super::*;




#[derive(Clone)]
pub enum
TyNode
{
  Root(String),

  Pointer(Box<TyNode>),
  Reference(Box<TyNode>),
  Array(Box<TyNode>,Expr),
  Struct(Vec<ParameterDecl>),
   Union(Vec<ParameterDecl>),
    Enum(Vec<(String,)>),
  Function{parameter_ty_nodes: Vec<TyNode>, return_ty_node: Box<TyNode>},

}


impl
TyNode
{


pub fn
read(s: &str)-> Result<TyNode,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"type",None)
    {
      return Ok(read_ty(&nd));
    }


  Err(())
}


pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    match self
    {
  Self::Pointer(tn)  =>{tn.collect(buf);}
  Self::Reference(tn)=>{tn.collect(buf);}
  Self::Array(tn,e)=>
    {
      tn.collect(buf);
       e.collect(buf);
    }
  Self::Struct(ls)=>
    {
        for p in ls
        {
          p.get_ty_node().collect(buf);
        }
    }
  Self::Union(ls)=>
    {
        for p in ls
        {
          p.get_ty_node().collect(buf);
        }
    }
  Self::Enum(ls)=>
    {
        for (name,) in ls
        {
//          tn.collect(buf);
        }
    }
  Self::Function{parameter_ty_nodes,return_ty_node}=>
    {
        for tn in parameter_ty_nodes
        {
          tn.collect(buf);
        }


      return_ty_node.collect(buf);
    }
  _=>{}
    }
}


pub fn
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Pointer(tn)  =>{  buf.push_str("*");  tn.print_to(buf);}
  Self::Reference(tn)=>{  buf.push_str("&");  tn.print_to(buf);}
  Self::Array(tn,e)=>
    {
      tn.print_to(buf);
      buf.push_str("[");
      e.print_to(buf);
      buf.push_str("]");
    }
  Self::Struct(ls)=>
    {
      buf.push_str("struct{{");

        for p in ls
        {
          buf.push_str(p.get_name());

          buf.push_str(":");

          p.get_ty_node().print_to(buf);

          buf.push_str(",");
        }


      buf.push_str("}}");
    }
  Self::Union(ls)=>
    {
      buf.push_str("union{{");

        for p in ls
        {
          buf.push_str(p.get_name());

          buf.push_str(":");

          p.get_ty_node().print_to(buf);

          buf.push_str(",");
        }


      buf.push_str("}}");
    }
  Self::Enum(ls)=>
    {
      buf.push_str("enum{{");

        for (name,) in ls
        {
          buf.push_str(name);

          buf.push_str(",");
        }


      buf.push_str("}}");
    }
  Self::Function{parameter_ty_nodes,return_ty_node}=>
    {
      buf.push_str("fn(");

        for tn in parameter_ty_nodes
        {
          tn.print_to(buf);

          buf.push_str(",");
        }


      buf.push_str(")->");

      return_ty_node.print_to(buf);
    }
  Self::Root(s)=>{buf.push_str(s);}
    }
}


pub fn
to_string(&self)-> String
{
  let  mut buf = String::new();

  self.print_to(&mut buf);

  buf
}


pub fn
print(&self)
{
  print!("{}",&self.to_string());
}

}




pub fn
read_ty(start_nd: &Node)-> TyNode
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      print!("{}",s);

      cur.advance(1);
    }


    if let Some(s) = cur.get_identifier()
    {
      return TyNode::Root(s.clone());
    }


  panic!();
}




pub struct
Field
{
  name: String,

  ty: Rc<Ty>,

  offset: usize,

}


pub struct
Enumerator
{
   name: String,
  value: i64,
}


impl
Enumerator
{


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_value(&self)-> i64
{
  self.value
}


}




pub enum
TyKind
{
  Unknown,

  Void,
  Bool,
  Int,
  Uint,
  Float,
  Array(Rc<Ty>,usize),
  Pointer(Rc<Ty>),
  Reference(Rc<Ty>),
  Struct(Vec<Field>),
  Union(Vec<Field>),
  Enum(Vec<Enumerator>),
  Function{parameter_tys: Vec<Rc<Ty>>, return_ty: Rc<Ty>},

}




pub struct
Ty
{
  name: String,
  kind: TyKind,

   size: usize,
  align: usize,

  default_data: EvalConstResult,

}


impl
Ty
{


pub fn
new_basic(name: &str, kind: TyKind, size: usize, default_data: EvalConstResult)-> Rc<Self>
{
  Rc::new(Self{
    name: name.to_string(),
    kind,
    size,
    align: size,
    default_data,
  })
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_kind(&self)-> &TyKind
{
  &self.kind
}


pub fn
get_size(&self)-> usize
{
  self.size
}


pub fn
get_align(&self)-> usize
{
  self.align
}


pub fn
construct(&self, args: Vec<EvalConstResult>)-> EvalConstResult
{
    match &self.name
    {
  (s) if s == "i8"=>
    {
        if args.len() == 1{args[0].clone().to_int_if_uint().to_i8_if_int()}
      else{EvalConstResult::Err}
    }
  (s) if s == "i16"=>
    {
        if args.len() == 1{args[0].clone().to_int_if_uint().to_i16_if_int()}
      else{EvalConstResult::Err}
    }
  (s) if s == "i32"=>
    {
        if args.len() == 1{args[0].clone().to_int_if_uint().to_i32_if_int()}
      else{EvalConstResult::Err}
    }
  (s) if s == "i64"=>
    {
        if args.len() == 1{args[0].clone().to_int_if_uint().to_i64_if_int()}
      else{EvalConstResult::Err}
    }
  (s) if s == "isize"=>
    {
        if args.len() == 1{args[0].clone().to_int_if_uint().to_isize_if_int()}
      else{EvalConstResult::Err}
    }
  (s) if s == "u8"=>
    {
        if args.len() == 1{args[0].clone().to_u8_if_uint()}
      else{EvalConstResult::Err}
    }
  (s) if s == "u16"=>
    {
        if args.len() == 1{args[0].clone().to_u16_if_uint()}
      else{EvalConstResult::Err}
    }
  (s) if s == "u32"=>
    {
        if args.len() == 1{args[0].clone().to_u32_if_uint()}
      else{EvalConstResult::Err}
    }
  (s) if s == "u64"=>
    {
        if args.len() == 1{args[0].clone().to_u64_if_uint()}
      else{EvalConstResult::Err}
    }
  (s) if s == "usize"=>
    {
        if args.len() == 1{args[0].clone().to_usize_if_uint()}
      else{EvalConstResult::Err}
    }
  (s) if s == "f32"=>
    {
        if args.len() == 1{args[0].clone().to_f32_if_float()}
      else{EvalConstResult::Err}
    }
  (s) if s == "f64"=>
    {
        if args.len() == 1{args[0].clone().to_f64_if_float()}
      else{EvalConstResult::Err}
    }
  _=>{EvalConstResult::Err}
    }
}


pub fn
get_default(&self)-> &EvalConstResult
{
  &self.default_data
}


fn
find_field_in<'a>(ls: &'a Vec<Field>, name: &str)-> Option<&'a Field>
{
    for f in ls
    {
        if &f.name == name
        {
          return Some(f);
        }
    }


  None
}


fn
find_enumerator_in(ls: &Vec<Enumerator>, name: &str)-> Option<i64>
{
    for en in ls
    {
        if en.name == name
        {
          return Some(en.value);
        }
    }


  None
}


fn
find_field(&self, name: &str)-> Option<&Field>
{
    match &self.kind
    {
  TyKind::Struct(ls)=>{Self::find_field_in(ls,name)}
  TyKind::Union(ls) =>{Self::find_field_in(ls,name)}
  _=>{None}
    }
}


fn
find_enumerator(&self, name: &str)-> Option<i64>
{
    match &self.kind
    {
  TyKind::Enum(ls) =>{Self::find_enumerator_in(ls,name)}
  _=>{None}
    }
}


pub fn
print(&self)
{
  println!("name: {}, size: {}, align: {}",&self.name,self.size,self.align);

    match &self.kind
    {
  TyKind::Struct(ls)=>
    {
      println!("fields{{");

        for f in ls
        {
          println!("{}(off: {}) ,",&f.name,f.offset);
        }


      println!("}}");
    }
  TyKind::Union(ls)=>
    {
      println!("fields{{");

        for f in ls
        {
          println!("{}(off: {}) ,",&f.name,f.offset);
        }


      println!("}}");
    }
  _=>{}
    }
}


}




static  mut TABLE: Vec<Rc<Ty>> = Vec::new();


pub fn
install_basic_types()
{
  let   void_ty = Ty::new_basic( "void",TyKind::Void ,0,EvalConstResult::Void);
  let   bool_ty = Ty::new_basic( "bool",TyKind::Bool ,1,EvalConstResult::Bool(false));
  let     i8_ty = Ty::new_basic(   "i8",TyKind::Int  ,1,EvalConstResult::I8(0));
  let    i16_ty = Ty::new_basic(  "i16",TyKind::Int  ,2,EvalConstResult::I16(0));
  let    i32_ty = Ty::new_basic(  "i32",TyKind::Int  ,4,EvalConstResult::I32(0));
  let    i64_ty = Ty::new_basic(  "i64",TyKind::Int  ,8,EvalConstResult::I64(0));
  let  isize_ty = Ty::new_basic("isize",TyKind::Int  ,8,EvalConstResult::ISize(0));
  let     u8_ty = Ty::new_basic(   "u8",TyKind::Uint ,1,EvalConstResult::U8(0));
  let    u16_ty = Ty::new_basic(  "u16",TyKind::Uint ,2,EvalConstResult::U16(0));
  let    u32_ty = Ty::new_basic(  "u32",TyKind::Uint ,4,EvalConstResult::U32(0));
  let    u64_ty = Ty::new_basic(  "u64",TyKind::Uint ,8,EvalConstResult::U64(0));
  let  usize_ty = Ty::new_basic("usize",TyKind::Uint ,8,EvalConstResult::USize(0));
  let    f32_ty = Ty::new_basic(  "f32",TyKind::Float,4,EvalConstResult::F32(0.0));
  let    f64_ty = Ty::new_basic(  "f64",TyKind::Float,8,EvalConstResult::F64(0.0));

    unsafe
    {
      TABLE.push(void_ty);
      TABLE.push(bool_ty);
      TABLE.push(i8_ty);
      TABLE.push(i16_ty);
      TABLE.push(i32_ty);
      TABLE.push(i64_ty);
      TABLE.push(isize_ty);
      TABLE.push(u8_ty);
      TABLE.push(u16_ty);
      TABLE.push(u32_ty);
      TABLE.push(u64_ty);
      TABLE.push(usize_ty);
      TABLE.push(f32_ty);
      TABLE.push(f64_ty);
    }
}


pub fn
add_ty(ty: Ty)-> Rc<Ty>
{
    if let Some(existed) = find_ty(&ty.name)
    {
      existed
    }

  else
    {
      add_ty_unchecked(ty)
    }
}


pub fn
add_ty_unchecked(ty: Ty)-> Rc<Ty>
{
  let  rc = Rc::new(ty);

  unsafe{TABLE.push(Rc::clone(&rc));}

  rc
}


pub fn
add_ty_from_node(tn: &TyNode, symtbl: &SymbolTable)-> Rc<Ty>
{
    match tn
    {
  TyNode::Pointer(tn)=>
    {
      let  target = add_ty_from_node(tn,symtbl);

      get_pointer_ty(target)
    }
  TyNode::Reference(tn)=>
    {
      let  target = add_ty_from_node(tn,symtbl);

      get_reference_ty(target)
    }
  TyNode::Array(tn,e)=>
    {
      let  res = evaluate_const(e,symtbl,None);

        if let EvalConstResult::Uint(n) = res
        {
          let  target = add_ty_from_node(tn,symtbl);

          get_array_ty(target,n as usize)
        }

      else
        {panic!();}
    }
  TyNode::Struct(ls)=>
    {
      let  mut fields = Vec::<Field>::new();

        for p in ls
        {
          let  ty = add_ty_from_node(p.get_ty_node(),symtbl);

          let  f = Field{name: p.get_name().clone(), ty, offset: 0};

          fields.push(f);
        }


      get_struct_ty(fields)
    }
  TyNode::Union(ls)=>
    {
      let  mut fields = Vec::<Field>::new();

        for p in ls
        {
          let  ty = add_ty_from_node(p.get_ty_node(),symtbl);

          let  f = Field{name: p.get_name().clone(), ty, offset: 0};

          fields.push(f);
        }


      get_union_ty(fields)
    }
  TyNode::Enum(ls)=>
    {
      let  mut en_table = Vec::<Enumerator>::new();

        for (en_name,) in ls
        {
          let  value = en_table.len() as i64;

          let  en = Enumerator{name: en_name.clone(), value};

          en_table.push(en);
        }


      get_enum_ty(en_table)
    }
  TyNode::Function{parameter_ty_nodes,return_ty_node}=>
    {
      let  mut parameter_tys = Vec::<Rc<Ty>>::new();

        for tn in parameter_ty_nodes
        {
          let  ty = add_ty_from_node(tn,symtbl);

          parameter_tys.push(ty);
        }


      let  return_ty = add_ty_from_node(return_ty_node,symtbl);

      get_function_ty(parameter_tys,return_ty)
    }
  TyNode::Root(s)=>
    {
        if let Some(sym) = symtbl.find_symbol(s)
        {
          find_ty(sym.get_ty_name()).unwrap()
        }

      else
        {
          find_ty(s).unwrap()
        }
    }
    }
}


pub fn
get_pointer_ty_name(base_name: &str)-> String
{
  format!("*{}",base_name)
}


pub fn
get_pointer_ty_by_name(name: &str)-> Rc<Ty>
{
  let  ptr_ty_name = get_pointer_ty_name(name);

    if let Some(existed) = find_ty(&ptr_ty_name)
    {
      existed
    }

  else
    if let Some(base) = find_ty(name)
    {
      get_pointer_ty(base)
    }

  else
    {panic!();}
}


pub fn
get_pointer_ty(ty: Rc<Ty>)-> Rc<Ty>
{
  let  ptr_ty_name = get_pointer_ty_name(&ty.name);

    if let Some(existed) = find_ty(&ptr_ty_name)
    {
      existed
    }

  else
    {
      add_ty(Ty{
        name: ptr_ty_name.clone(),
        kind: TyKind::Pointer(Rc::clone(&ty)),
        size: WORD_SIZE,
        align: WORD_SIZE,
        default_data: EvalConstResult::Complex(ptr_ty_name,vec![EvalConstResult::NullPointer]),
      })
    }
}


pub fn
get_reference_ty_name(base_name: &str)-> String
{
  format!("&{}",base_name)
}


pub fn
get_reference_ty_by_name(name: &str)-> Rc<Ty>
{
  let  ref_ty_name = get_reference_ty_name(name);

    if let Some(existed) = find_ty(&ref_ty_name)
    {
      existed
    }

  else
    if let Some(base) = find_ty(name)
    {
      get_reference_ty(base)
    }

  else
    {panic!();}
}


pub fn
get_reference_ty(ty: Rc<Ty>)-> Rc<Ty>
{
  let  ref_ty_name = get_reference_ty_name(&ty.name);

    if let Some(existed) = find_ty(&ref_ty_name)
    {
      existed
    }

  else
    {
      add_ty(Ty{
        name: ref_ty_name.clone(),
        kind: TyKind::Reference(Rc::clone(&ty)),
        size: WORD_SIZE,
        align: WORD_SIZE,
        default_data: EvalConstResult::Complex(ref_ty_name,vec![EvalConstResult::NullPointer]),
      })
    }
}


pub fn
get_array_ty_name(base_name: &str, n: usize)-> String
{
  format!("{}[{}]",base_name,n)
}


pub fn
get_array_ty_by_name(name: &str, n: usize)-> Rc<Ty>
{
  let  arr_ty_name = get_array_ty_name(name,n);

    if let Some(existed) = find_ty(&arr_ty_name)
    {
      existed
    }

  else
    if let Some(base) = find_ty(name)
    {
      get_array_ty(base,n)
    }

  else
    {panic!();}
}


pub fn
get_array_ty(ty: Rc<Ty>, n: usize)-> Rc<Ty>
{
  let  arr_ty_name = get_array_ty_name(&ty.name,n);

    if let Some(existed) = find_ty(&arr_ty_name)
    {
      existed
    }

  else
    {
      let  mut buf = Vec::<EvalConstResult>::new();

        for _ in 0..n
        {
          buf.push(ty.get_default().clone());
        }


      add_ty(Ty{
        name: arr_ty_name.clone(),
        kind: TyKind::Array(Rc::clone(&ty),n),
        size: ty.size*n,
        align: ty.align,
        default_data: EvalConstResult::Complex(arr_ty_name,buf),
      })
    }
}




pub fn
get_struct_ty_name(fields: &Vec<Field>)-> String
{
  let  mut name = "struct{".to_string();

    for f in fields
    {
      name.push_str(&f.ty.name);
      name.push(',');
    }


  name.push('}');

  name
}


pub fn
get_struct_ty(mut fields: Vec<Field>)-> Rc<Ty>
{
  let  st_ty_name = get_struct_ty_name(&fields);

    if let Some(existed) = find_ty(&st_ty_name)
    {
      existed
    }

  else
    {
      let  mut offset = 0usize;
      let  mut  align = 0usize;

      let  mut buf = Vec::<EvalConstResult>::new();

        for f in &mut fields
        {
          offset = Align(f.ty.align).get(offset);

          f.offset = offset;

          offset += f.ty.size;

          align = std::cmp::max(align,f.ty.align);

          buf.push(f.ty.get_default().clone());
        }


      add_ty(Ty{
        name: st_ty_name.clone(),
        kind: TyKind::Struct(fields),
        size: Align(align).get(offset),
        align: align,
        default_data: EvalConstResult::Complex(st_ty_name,buf),
      })
    }
}


pub fn
get_union_ty_name(fields: &Vec<Field>)-> String
{
  let  mut name = "union{".to_string();

    for f in fields
    {
      name.push_str(&f.ty.name);
      name.push(',');
    }


  name.push('}');

  name
}


pub fn
get_union_ty(mut fields: Vec<Field>)-> Rc<Ty>
{
  let  un_ty_name = get_union_ty_name(&fields);

    if let Some(existed) = find_ty(&un_ty_name)
    {
      existed
    }

  else
    {
      let  mut  size = 0usize;
      let  mut align = 0usize;

      let  mut buf = Vec::<EvalConstResult>::new();

        for f in &fields
        {
           size = std::cmp::max( size,f.ty.size );
          align = std::cmp::max(align,f.ty.align);

          buf.push(f.ty.get_default().clone());
        }


      add_ty(Ty{
        name: un_ty_name.clone(),
        kind: TyKind::Union(fields),
        size,
        align,
        default_data: EvalConstResult::Complex(un_ty_name,buf),
      })
    }
}




pub fn
get_enum_ty_name(enumers: &Vec<Enumerator>)-> String
{
  let  mut name = "enum{".to_string();

    for en in enumers
    {
      name.push_str(&format!("{}:{}",&en.name,en.value));
      name.push(',');
    }


  name.push('}');

  name
}


pub fn
get_enum_ty(mut enumers: Vec<Enumerator>)-> Rc<Ty>
{
  let  en_ty_name = get_enum_ty_name(&enumers);

    if let Some(existed) = find_ty(&en_ty_name)
    {
      existed
    }

  else
    {
      let  mut id = String::new();

        if let Some(en) = enumers.first()
        {
          id = en.name.clone();
        }


      add_ty(Ty{
        name: en_ty_name.clone(),
        kind: TyKind::Enum(enumers),
        size: WORD_SIZE,
        align: WORD_SIZE,
        default_data: EvalConstResult::Enumerator(en_ty_name,id),
      })
    }
}



pub fn
get_function_ty_name(param_tys: &Vec<Rc<Ty>>, ret_ty: &Rc<Ty>)-> String
{
  let  mut name = "fn(".to_string();

    for ty in param_tys
    {
      name.push_str(&ty.name);
      name.push(',');
    }


  name.push_str(")->");
  name.push_str(&ret_ty.name);

  name
}


pub fn
get_function_ty(parameter_tys: Vec<Rc<Ty>>, return_ty: Rc<Ty>)-> Rc<Ty>
{
  let  fn_ty_name = get_function_ty_name(&parameter_tys,&return_ty);

    if let Some(existed) = find_ty(&fn_ty_name)
    {
      existed
    }

  else
    {
      add_ty(Ty{
        name: fn_ty_name.clone(),
        kind: TyKind::Function{parameter_tys, return_ty},
        size: WORD_SIZE,
        align: WORD_SIZE,
        default_data: EvalConstResult::Complex(fn_ty_name,vec![EvalConstResult::NullPointer]),
      })
    }
}


pub fn
find_ty(name: &str)-> Option<Rc<Ty>>
{
    for ty in unsafe{&TABLE}
    {
        if &ty.name == name
        {
          return Some(Rc::clone(ty));
        }
    }


  None
}


pub fn
print_tys()
{
    for ty in unsafe{&TABLE}
    {
      ty.print();

      println!("");
    }
}




