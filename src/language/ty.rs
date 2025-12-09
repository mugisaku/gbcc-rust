

use crate::node::*;
use super::*;
use super::expr::*;
use super::decl::*;
use super::symbol_table::*;
use super::evaluate::*;
use super::execute::*;




pub static  VOID_ID: &str = "void";
pub static UNDEF_ID: &str = "undef";
pub static  BOOL_ID: &str = "bool";
pub static    I8_ID: &str = "i8";
pub static   I16_ID: &str = "i16";
pub static   I32_ID: &str = "i32";
pub static   I64_ID: &str = "i64";
pub static ISIZE_ID: &str = "isize";
pub static    U8_ID: &str = "u8";
pub static   U16_ID: &str = "u16";
pub static   U32_ID: &str = "u32";
pub static   U64_ID: &str = "u64";
pub static USIZE_ID: &str = "usize";
pub static F32_ID: &str = "f32";
pub static F64_ID: &str = "f64";


#[derive(Clone)]
pub struct
Field
{
  name: String,
    ty: Ty,

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
get_ty(&self)-> &Ty
{
  &self.ty
}


pub fn
get_sized(&self, ctx: &ExecContext)-> Result<SizedField,()>
{
    if let Ok(ty) = self.ty.get_sized(ctx)
    {
      return Ok(SizedField{name: self.name.clone(), ty, offset: 0});
    }


  Err(())
}



pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.ty.print();
}


}




#[derive(Clone)]
pub struct
SizedField
{
  name: String,

  ty: SizedTy,

  offset: usize,

}


impl
SizedField
{


pub fn
get_name(&self)-> &String
{
  &self.name
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




#[derive(Clone)]
pub enum
SizedTy
{
  Void,

  Bool,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,
  F32, F64,

    Pointer(Ty),
  Reference(Ty),

  Tuple(Vec<SizedField>),
  Struct(Vec<SizedField>),
  Union(Vec<SizedField>),
  Enum(Vec<String>),

  Array(Box<SizedTy>,usize),
  Function(Vec<Ty>,Ty),

}


impl
SizedTy
{


pub fn  is_void(&self)-> bool{if let Self::Void = self{true} else{false}}
pub fn  is_bool(&self)-> bool{if let Self::Bool = self{true} else{false}}
pub fn     is_i8(&self)-> bool{if let Self::I8 = self{true} else{false}}
pub fn    is_i16(&self)-> bool{if let Self::I16 = self{true} else{false}}
pub fn    is_i32(&self)-> bool{if let Self::I32 = self{true} else{false}}
pub fn    is_i64(&self)-> bool{if let Self::I64 = self{true} else{false}}
pub fn  is_isize(&self)-> bool{if let Self::ISize = self{true} else{false}}
pub fn     is_u8(&self)-> bool{if let Self::U8 = self{true} else{false}}
pub fn    is_u16(&self)-> bool{if let Self::U16 = self{true} else{false}}
pub fn    is_u32(&self)-> bool{if let Self::U32 = self{true} else{false}}
pub fn    is_u64(&self)-> bool{if let Self::U64 = self{true} else{false}}
pub fn  is_usize(&self)-> bool{if let Self::USize = self{true} else{false}}
pub fn    is_f32(&self)-> bool{if let Self::F32 = self{true} else{false}}
pub fn    is_f64(&self)-> bool{if let Self::F64 = self{true} else{false}}
pub fn  is_pointer(&self)-> bool{if let Self::Pointer(_) = self{true} else{false}}
pub fn  is_reference(&self)-> bool{if let Self::Reference(_) = self{true} else{false}}
pub fn  is_array(&self)-> bool{if let Self::Array(_,_) = self{true} else{false}}
pub fn  is_tuple(&self)-> bool{if let Self::Tuple(_) = self{true} else{false}}
pub fn  is_struct(&self)-> bool{if let Self::Struct(_) = self{true} else{false}}
pub fn  is_union(&self)-> bool{if let Self::Union(_) = self{true} else{false}}
pub fn  is_enum(&self)-> bool{if let Self::Enum(_) = self{true} else{false}}
pub fn  is_function(&self)-> bool{if let Self::Function(_,_) = self{true} else{false}}


pub fn
is_int(&self)-> bool
{
   self.is_i8()
 ||self.is_i16()
 ||self.is_i32()
 ||self.is_i64()
 ||self.is_isize()
}


pub fn
is_uint(&self)-> bool
{
   self.is_u8()
 ||self.is_u16()
 ||self.is_u32()
 ||self.is_u64()
 ||self.is_usize()
}


pub fn
is_float(&self)-> bool
{
   self.is_f32()
 ||self.is_f64()
}




pub fn
remove_reference(&self)-> Ty
{
    if let Self::Reference(ty) = self
    {
      return ty.clone();
    }


  Ty::Sized(Box::new(self.clone()))
}


pub fn
remove_pointer(&self)-> Ty
{
    if let Self::Pointer(ty) = self
    {
      return ty.clone();
    }


  Ty::Sized(Box::new(self.clone()))
}


pub fn
get_info_from_sized_field_list(ls: &Vec<SizedField>)-> TyInfo
{
  let  mut  size = 0usize;
  let  mut align = 0usize;

    for f in ls
    {
      align = std::cmp::max(align,f.ty.get_info().align);
    }


    if let Some(last) = ls.last()
    {
      size = get_aligned(last.offset+last.ty.get_info().size,align);
    }


  TyInfo{size, align}
}


pub fn
get_info(&self)-> TyInfo
{
    match self
    {
  Self::Void         =>{TyInfo::from_size(0)}
  Self::Bool         =>{TyInfo::from_size(1)}
  Self::I8           =>{TyInfo::from_size(1)}
  Self::I16          =>{TyInfo::from_size(2)}
  Self::I32          =>{TyInfo::from_size(4)}
  Self::I64          =>{TyInfo::from_size(8)}
  Self::ISize        =>{TyInfo::from_size(WORD_SIZE)}
  Self::U8           =>{TyInfo::from_size(1)}
  Self::U16          =>{TyInfo::from_size(2)}
  Self::U32          =>{TyInfo::from_size(4)}
  Self::U64          =>{TyInfo::from_size(8)}
  Self::USize        =>{TyInfo::from_size(WORD_SIZE)}
  Self::F32          =>{TyInfo::from_size(4)}
  Self::F64          =>{TyInfo::from_size(8)}
  Self::Pointer(_)   =>{TyInfo::from_size(WORD_SIZE)}
  Self::Reference(_) =>{TyInfo::from_size(WORD_SIZE)}
  Self::Function(_,_)=>{TyInfo::from_size(WORD_SIZE)}
  Self::Enum(_)      =>{TyInfo::from_size(WORD_SIZE)}
  Self::Tuple(ls)=>
    {
      Self::get_info_from_sized_field_list(ls)
    }
  Self::Struct(ls)=>
    {
      Self::get_info_from_sized_field_list(ls)
    }
  Self::Union(ls)=>
    {
      let  mut  size = 0usize;
      let  mut align = 0usize;

        for f in ls
        {
          let  ti = f.ty.get_info();

          align = std::cmp::max(align,ti.align);
           size = std::cmp::max( size,ti.size );
        }


      size = get_aligned(size,align);

      TyInfo{size, align}
    }
  Self::Array(ty,n)=>
    {
      let  ti = ty.get_info();

      TyInfo{size: ti.size*(*n), align: ti.align}
    }
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
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Void=>{buf.push_str(VOID_ID);}
  Self::Bool=>{buf.push_str(BOOL_ID);}
  Self::I8=>{buf.push_str(I8_ID);}
  Self::I16=>{buf.push_str(I16_ID);}
  Self::I32=>{buf.push_str(I32_ID);}
  Self::I64=>{buf.push_str(I64_ID);}
  Self::ISize=>{buf.push_str(ISIZE_ID);}
  Self::U8=>{buf.push_str(U8_ID);}
  Self::U16=>{buf.push_str(U16_ID);}
  Self::U32=>{buf.push_str(U32_ID);}
  Self::U64=>{buf.push_str(U64_ID);}
  Self::USize=>{buf.push_str(USIZE_ID);}
  Self::F32=>{buf.push_str(F32_ID);}
  Self::F64=>{buf.push_str(F64_ID);}
  Self::Tuple(ls)=>
    {
      buf.push_str("tuple{");

        for f in ls
        {
          f.ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str("}");
    }
  Self::Struct(ls)=>
    {
      buf.push_str("struct{");

        for f in ls
        {
          f.ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str("}");
    }
  Self::Union(ls)=>
    {
      buf.push_str("union{");

        for f in ls
        {
          f.ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str("}");
    }
  Self::Enum(_)=>
    {
      buf.push_str("enum{");
      buf.push_str("}");
    }
  Self::Pointer(ty)=>{  buf.push_str("*");  ty.print_to(buf);}
  Self::Reference(ty)=>{  buf.push_str("&");  ty.print_to(buf);}
  Self::Array(ty,n)=>
    {
      ty.print_to(buf);

      let  s = format!("[{}]",*n);

      buf.push_str(&s);
    }
  Self::Function(ls,ret_ty)=>
    {
      buf.push_str("function(");

        for ty in ls
        {
          ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str(")-> ");

      ret_ty.print_to(buf);
    }
    }
}


pub fn
print(&self)
{
  let  s = self.to_string();

  print!("{}",&s);
}


}




#[derive(Clone)]
pub struct
TyInfo
{
   size: usize,
  align: usize,

}


impl
TyInfo
{


pub fn
new()-> Self
{
  Self{
     size: 0,
    align: 0,
  }
}


pub fn
from_size(sz: usize)-> Self
{
  Self{
     size: sz,
    align: sz,
  }
}


pub fn   get_size(&self)-> usize{self.size}
pub fn  get_align(&self)-> usize{self.align}


}




pub struct
Enumerator
{
}




#[derive(Clone)]
pub enum
Ty
{
  Undef,

  Alias(String),

  Tuple(Vec<Field>),
  Struct(Vec<Field>),
  Union(Vec<Field>),
  Enum(Vec<String>),

  Array(Box<Ty>,Expr),

  Sized(Box<SizedTy>),

}


impl
Ty
{


pub fn
read(s: &str)-> Result<Self,()>
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
is_int(&self)-> bool
{
    if let Self::Sized(ty) = self
    {
      return ty.is_int();
    }


  false
}


pub fn
is_uint(&self)-> bool
{
    if let Self::Sized(ty) = self
    {
      return ty.is_uint();
    }


  false
}


pub fn
is_float(&self)-> bool
{
    if let Self::Sized(ty) = self
    {
      return ty.is_float();
    }


  false
}


pub fn  is_undef(&self)-> bool{if let Self::Undef = self{true} else{false}}
pub fn  is_sized(&self)-> bool{if let Self::Sized(_) = self{true} else{false}}
pub fn  is_alias(&self)-> bool{if let Self::Alias(_) = self{true} else{false}}
pub fn  is_array(&self)-> bool{if let Self::Array(_,_) = self{true} else{false}}
pub fn   is_tuple(&self)-> bool{if let Self::Tuple(_) = self{true} else{false}}
pub fn  is_struct(&self)-> bool{if let Self::Struct(_) = self{true} else{false}}
pub fn   is_union(&self)-> bool{if let Self::Union(_) = self{true} else{false}}
pub fn    is_enum(&self)-> bool{if let Self::Enum(_) = self{true} else{false}}


pub fn
remove_array(&mut self)
{
    if let Self::Array(ty,_) = self
    {
      let  mut tmp_ty = Ty::Undef;

      std::mem::swap(ty.as_mut(),&mut tmp_ty);

      *self = tmp_ty;
    }
}


pub fn
get_sized_field_list(ctx: &ExecContext, ls: &Vec<Field>)-> Result<Vec<SizedField>,()>
{
  let  mut buf = Vec::<SizedField>::new();

  let  mut offset = 0usize;

    for f in ls
    {
        if let Ok(ty) = f.ty.get_sized(ctx)
        {
          let  ty_info = ty.get_info();

          offset = get_aligned(ty_info.align,offset);

          let  next_offset = offset+ty_info.size;

          let  name = f.name.clone();

          let  fld = SizedField{name, ty, offset};

          buf.push(fld);

          offset = next_offset;
        }

      else
        {
          return Err(());
        }
    }


  Ok(buf)
}


pub fn
get_sized(&self, ctx: &ExecContext)-> Result<SizedTy,()>
{
    match self
    {
  Self::Undef=>{Err(())}
  Self::Enum(_)=>{Err(())}
  Self::Alias(s)=>
    {
        if let Some(sym) = ctx.find_symbol(s)
        {
            if let SymbolKind::Type(ty) = sym.get_kind()
            {
              return Ok(ty.clone());
            }
        }


      Err(())
    }
  Self::Tuple(ls)=>
    {
        if let Ok(buf) = Self::get_sized_field_list(ctx,ls)
        {
          Ok(SizedTy::Tuple(buf))
        }

      else
        {
          Err(())
        }
    }
  Self::Struct(ls)=>
    {
        if let Ok(buf) = Self::get_sized_field_list(ctx,ls)
        {
          Ok(SizedTy::Struct(buf))
        }

      else
        {
          Err(())
        }
    }
  Self::Union(ls)=>
    {
      let  mut buf = Vec::<SizedField>::new();

        for f in ls
        {
            if let Ok(ty) = f.ty.get_sized(ctx)
            {
              let  name = f.name.clone();

              let  fld = SizedField{name, ty, offset: 0};

              buf.push(fld);
            }

          else
            {
              return Err(());
            }
        }


      Ok(SizedTy::Union(buf))
    }
  Self::Array(ty,e)=>
    {
        if let Ok(sized_ty) = ty.get_sized(ctx)
        {
          let  res = evaluate(e,ctx);

            if let Result::<usize,()>::Ok(n) = res.try_into()
            {
              return Ok(SizedTy::Array(Box::new(sized_ty),n));
            }
        }


      Err(())
    }
  Self::Sized(ty)=>
    {
      Ok((**ty).clone())
    }
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
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Undef=>{buf.push_str(UNDEF_ID);}
  Self::Tuple(ls)=>
    {
      buf.push_str("tuple{{");

        for f in ls
        {
          f.ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str("}}");
    }
  Self::Struct(ls)=>
    {
      buf.push_str("struct");
      buf.push_str("}}");
    }
  Self::Union(ls)=>
    {
      buf.push_str("union");

        for f in ls
        {
          f.ty.print_to(buf);

          buf.push_str(", ");
        }


      buf.push_str("}}");
    }
  Self::Enum(ls)=>
    {
      buf.push_str("enum");

        for e in ls
        {
          buf.push_str(", ");
        }


      buf.push_str("}}");
    }
  Self::Alias(s)=>{buf.push_str(s);}
  Self::Array(ty,e)=>
    {
      ty.print_to(buf);
      buf.push_str("[");
      e.print();
      buf.push_str("]");
    }
  Self::Sized(ty)=>
    {
      ty.print_to(buf);
    }
    }
}


pub fn
print(&self)
{
  let  s = self.to_string();

  print!("{}",&s);
}


}




pub fn
read_ty(start_nd: &Node)-> Ty
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      print!("{}",s);

      cur.advance(1);
    }


    if let Some(s) = cur.get_identifier()
    {
      return Ty::Alias(s.clone());
    }


  panic!();
}


pub fn
read_field(start_nd: &Node)-> Field
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_identifier()
    {
      let  name = s.clone();

      cur.advance(2);

        if let Some(d) = cur.select_node("type")
        {
          let  ty = read_ty(d);

          return Field{name, ty};
        }
    }


  panic!();
}


pub fn
read_field_list(start_nd: &Node)-> Vec<Field>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<Field>::new();

  cur.advance(1);

    while let Some(p_d) = cur.select_node("parameter")
    {
      let  p = read_field(p_d);

      ls.push(p);

      cur.advance(2);
    }


  ls
}




impl
std::convert::From<SizedTy> for Ty
{


fn
from(ty: SizedTy)-> Ty
{
  Ty::Sized(Box::new(ty))
}


}



