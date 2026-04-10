

use std::rc::Rc;

use crate::node::*;
use super::asm::*;
use super::expr::*;
use super::evaluate_const::*;
use super::evaluate::*;
use super::symbol_table::*;
use super::*;




#[derive(Clone)]
pub enum
TyNode
{
  Root(String),

  Pointer(Box<TyNode>),
  Reference(Box<TyNode>),
  Array(Box<TyNode>,Expr),
  Struct(Vec<(String,TyNode)>),
   Union(Vec<(String,TyNode)>),
  Function{parameter_ty_nodes: Vec<TyNode>, return_ty_node: Box<TyNode>},

}


impl
TyNode
{


pub fn   new_void()-> Self{Self::Root("void".to_string())}
pub fn   new_bool()-> Self{Self::Root("bool".to_string())}
pub fn     new_i8()-> Self{Self::Root("i8".to_string())}
pub fn    new_i16()-> Self{Self::Root("i16".to_string())}
pub fn    new_i32()-> Self{Self::Root("i32".to_string())}
pub fn    new_i64()-> Self{Self::Root("i64".to_string())}
pub fn  new_isize()-> Self{Self::Root("isize".to_string())}
pub fn     new_u8()-> Self{Self::Root("u8".to_string())}
pub fn    new_u16()-> Self{Self::Root("u16".to_string())}
pub fn    new_u32()-> Self{Self::Root("u32".to_string())}
pub fn    new_u64()-> Self{Self::Root("u64".to_string())}
pub fn  new_usize()-> Self{Self::Root("usize".to_string())}
pub fn    new_f32()-> Self{Self::Root("f32".to_string())}
pub fn    new_f64()-> Self{Self::Root("f64".to_string())}


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
        for (_,tn) in ls
        {
          tn.collect(buf);
        }
    }
  Self::Union(ls)=>
    {
        for (_,tn) in ls
        {
          tn.collect(buf);
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


pub fn   is_pointer(&self)-> bool{if let Self::Pointer(_)   = self{true} else{false}}
pub fn is_reference(&self)-> bool{if let Self::Reference(_) = self{true} else{false}}
pub fn  is_function(&self)-> bool{if let Self::Function{..} = self{true} else{false}}
pub fn     is_array(&self)-> bool{if let Self::Array(_,_)   = self{true} else{false}}
pub fn      is_root(&self)-> bool{if let Self::Root(_)      = self{true} else{false}}


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

        for (_,tn) in ls
        {
          tn.print_to(buf);

          buf.push_str(",");
        }


      buf.push_str("}}");
    }
  Self::Union(ls)=>
    {
      buf.push_str("union{{");

        for (_,tn) in ls
        {
          tn.print_to(buf);

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
  ty_name: String,

  offset: usize,

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
  Array(String,usize),
  Pointer(String),
  Reference(String),
  Struct(Vec<Field>),
  Union(Vec<Field>),
  Enum,
  Function{parameter_ty_names: Vec<String>, return_ty_name: String},

}




pub struct
Ty
{
  name: String,
  kind: TyKind,

   size: usize,
  align: usize,

}


impl
Ty
{


pub fn
wrap_and_add(ty: Ty)-> Rc<Self>
{
  add_ty(Rc::new(ty))
}


pub fn
build_and_add(ty_node: &TyNode, symtbl: &SymbolTable)-> Rc<Self>
{
    match ty_node
    {
  TyNode::Pointer(tn)=>
    {
      let  target = Self::build_and_add(tn,symtbl);

      Self::wrap_and_add(Self{
        name: format!("*{}",&target.name),
        kind: TyKind::Pointer(target.name.clone()),
        size: WORD_SIZE,
        align: WORD_SIZE,
      })
    }
  TyNode::Reference(tn)=>
    {
      let  target = Self::build_and_add(tn,symtbl);

      Self::wrap_and_add(Self{
        name: format!("&{}",&target.name),
        kind: TyKind::Reference(target.name.clone()),
        size: WORD_SIZE,
        align: WORD_SIZE,
      })
    }
  TyNode::Array(tn,e)=>
    {
      let  res = evaluate_const(e,symtbl,None);

        if let EvalConstResult::Int(n) = res
        {
          let  target = Self::build_and_add(tn,symtbl);

          Self::wrap_and_add(Self{
            name: format!("{}[{}]",&target.name,n),
            kind: TyKind::Array(target.name.clone(),n as usize),
            size: target.size*(n as usize),
            align: target.align,
          })
        }

      else
        {panic!();}
    }
  TyNode::Struct(ls)=>
    {
      let  mut name = "struct{".to_string();

      let  mut field_table = Vec::<Field>::new();

      let  mut offset = 0usize;
      let  mut  align = 0usize;

        for (f_name,f_tn) in ls
        {
          let  target = Self::build_and_add(f_tn,symtbl);

          name.push_str(&target.name);
          name.push(',');

          offset = Align(target.align).get(offset);

          let  f = Field{name: f_name.clone(), ty_name: target.name.clone(), offset};

          field_table.push(f);

          offset += target.size;

          align = std::cmp::max(align,target.align);
        }


      name.push('}');

      Self::wrap_and_add(Self{
        name,
        kind: TyKind::Struct(field_table),
        size: Align(align).get(offset),
        align: align,
      })
    }
  TyNode::Union(ls)=>
    {
      let  mut name = "union{".to_string();

      let  mut field_table = Vec::<Field>::new();

      let  mut  size = 0usize;
      let  mut align = 0usize;

        for (f_name,f_tn) in ls
        {
          let  target = Self::build_and_add(f_tn,symtbl);

          name.push_str(&target.name);
          name.push(',');

          let  f = Field{name: f_name.clone(), ty_name: target.name.clone(), offset: 0};

          field_table.push(f);

           size = std::cmp::max( size, target.size);
          align = std::cmp::max(align,target.align);
        }


      name.push('}');

      Self::wrap_and_add(Self{
        name,
        kind: TyKind::Union(field_table),
        size: size,
        align: align,
      })
    }
  TyNode::Function{parameter_ty_nodes,return_ty_node}=>
    {
      let  mut name = "fn(".to_string();

      let  mut parameter_ty_names = Vec::<String>::new();

        for tn in parameter_ty_nodes
        {
          let  target = Self::build_and_add(tn,symtbl);

          name.push_str(&target.name);
          name.push(',');

          parameter_ty_names.push(target.name.clone());
        }


      let  target = Self::build_and_add(return_ty_node,symtbl);

      name.push_str(")->");
      name.push_str(&target.name);

      Self::wrap_and_add(Self{
        name,
        kind: TyKind::Function{parameter_ty_names, return_ty_name: target.name.clone()},
        size: WORD_SIZE,
        align: WORD_SIZE,
      })
    }
  TyNode::Root(s)=>
    {
        if let Some(sym) = symtbl.find_symbol(s)
        {
todo!();
        }


      find_ty(s).unwrap()
    }
    }
}


pub fn
new_basic(name: &str, kind: TyKind, size: usize)-> Rc<Self>
{
  Rc::new(Self{
    name: name.to_string(),
    kind,
    size,
    align: size,
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
find_field(&self, name: &str)-> Option<&Field>
{
    match &self.kind
    {
  TyKind::Struct(ls)=>{Self::find_field_in(ls,name)}
  TyKind::Union(ls) =>{Self::find_field_in(ls,name)}
  _=>{None}
    }
}


pub fn
print(&self)
{
  print!("name:{}, size: {}, align: {}\n",&self.name,self.size,self.align);
}


}




static  mut TABLE: Vec<Rc<Ty>> = Vec::new();


pub fn
install_basic_types()
{
  let   void_ty = Ty::new_basic( "void",TyKind::Void ,0);
  let   bool_ty = Ty::new_basic( "bool",TyKind::Bool ,1);
  let     i8_ty = Ty::new_basic(   "i8",TyKind::Int  ,1);
  let    i16_ty = Ty::new_basic(  "i16",TyKind::Int  ,2);
  let    i32_ty = Ty::new_basic(  "i32",TyKind::Int  ,4);
  let    i64_ty = Ty::new_basic(  "i64",TyKind::Int  ,8);
  let  isize_ty = Ty::new_basic("isize",TyKind::Int  ,8);
  let     u8_ty = Ty::new_basic(   "u8",TyKind::Uint ,1);
  let    u16_ty = Ty::new_basic(  "u16",TyKind::Uint ,2);
  let    u32_ty = Ty::new_basic(  "u32",TyKind::Uint ,4);
  let    u64_ty = Ty::new_basic(  "u64",TyKind::Uint ,8);
  let  usize_ty = Ty::new_basic("usize",TyKind::Uint ,8);
  let    f32_ty = Ty::new_basic(  "f32",TyKind::Float,4);
  let    f64_ty = Ty::new_basic(  "f64",TyKind::Float,8);

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
add_ty(ty: Rc<Ty>)-> Rc<Ty>
{
    if let Some(existed) = find_ty(&ty.name)
    {
      existed
    }

  else
    {
      unsafe{TABLE.push(Rc::clone(&ty));}

      ty
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




