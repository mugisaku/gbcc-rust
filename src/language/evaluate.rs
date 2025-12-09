

use super::*;
use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::decl::*;
use super::opcode::*;
use super::evaluate_unary::*;
use super::evaluate_binary::*;
use super::execute::*;




#[derive(Clone)]
pub struct
Opdata
{
  ty: SizedTy,

  ty_info: TyInfo,

  opcode_list: Vec<Opcode>,

}


impl
Opdata
{


pub fn
new()-> Self
{
  Self{
    ty: SizedTy::Void,
    ty_info: TyInfo::new(),
    opcode_list: Vec::new(),
  }
}


pub fn
from_ty_and_opcode(ty: SizedTy, op: Opcode)-> Self
{
  let  ty_info = ty.get_info();

  Self{
    ty,
    ty_info,
    opcode_list: vec![op],
  }
}


pub fn
try_from_ty_and_int_lit(ty: SizedTy, i: i64)-> Result<Self,()>
{
  let  ty_info = ty.get_info();

    match ty
    {
  SizedTy::I8=>
    {
        if let Ok(new_i) = i8::try_from(i)
        {
          return Ok(Self{
            ty,
            ty_info,
            opcode_list: vec![Opcode::Pushi(new_i as i64)],
          });
        }
    }
  _=>{}
    }


  Err(())
}


pub fn
new_addfp(ty: SizedTy, off: isize)-> Self
{
  let  ty_info = ty.get_info();

  Self{
    ty,
    ty_info,
    opcode_list: vec![Opcode::Pushi(off as i64),Opcode::Addfp],
  }
}


pub fn
get_ty(&self)-> &SizedTy
{
  &self.ty
}


pub fn
set_ty(&mut self, ty: SizedTy)
{
  self.ty_info = ty.get_info();
  self.ty = ty;
}


pub fn
get_ty_info(&self)-> &TyInfo
{
  &self.ty_info
}


pub fn
get_opcode_list_mut(&mut self)-> &mut Vec<Opcode>
{
  &mut self.opcode_list
}


pub fn
add_ld(&mut self)
{
       if self.ty.is_bool() {self.opcode_list.push(Opcode::Ldu8);}
  else if self.ty.is_i8()   {self.opcode_list.push(Opcode::Ldi8);}
  else if self.ty.is_i16()  {self.opcode_list.push(Opcode::Ldi16);}
  else if self.ty.is_i32()  {self.opcode_list.push(Opcode::Ldi32);}
  else if self.ty.is_i64()  {self.opcode_list.push(Opcode::Ld64);}
  else if self.ty.is_isize(){self.opcode_list.push(Opcode::Ld64);}
  else if self.ty.is_u8()   {self.opcode_list.push(Opcode::Ldu8);}
  else if self.ty.is_u16()  {self.opcode_list.push(Opcode::Ldu16);}
  else if self.ty.is_u32()  {self.opcode_list.push(Opcode::Ldu32);}
  else if self.ty.is_u64()  {self.opcode_list.push(Opcode::Ld64);}
  else if self.ty.is_usize(){self.opcode_list.push(Opcode::Ld64);}
  else if self.ty.is_f32()  {self.opcode_list.push(Opcode::Ldf32);}
  else if self.ty.is_f64()  {self.opcode_list.push(Opcode::Ld64);}
}


pub fn
to_reference(mut self)-> Self
{
  let  new_ty = SizedTy::Reference(Ty::Sized(Box::new(self.ty.clone())));

  self.set_ty(new_ty);

  self
}


pub fn
print(&self)
{
}


}



impl
std::convert::From<bool> for Opdata
{fn from(b: bool)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::Bool,Opcode::Pushu(to_u64_from_bool(b)))}}


impl
std::convert::From<i8> for Opdata
{fn from(i: i8)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::I8,Opcode::Pushi(i as i64))}}


impl
std::convert::From<i16> for Opdata
{fn from(i: i16)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::I16,Opcode::Pushi(i as i64))}}


impl
std::convert::From<i32> for Opdata
{fn from(i: i32)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::I32,Opcode::Pushi(i as i64))}}


impl
std::convert::From<i64> for Opdata
{fn from(i: i64)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::I64,Opcode::Pushi(i as i64))}}


impl
std::convert::From<isize> for Opdata
{fn from(i: isize)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::ISize,Opcode::Pushi(i as i64))}}


impl
std::convert::From<u8> for Opdata
{fn from(u: u8)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::U8,Opcode::Pushu(u as u64))}}


impl
std::convert::From<u16> for Opdata
{fn from(u: u16)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::U16,Opcode::Pushu(u as u64))}}


impl
std::convert::From<u32> for Opdata
{fn from(u: u32)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::U32,Opcode::Pushu(u as u64))}}


impl
std::convert::From<u64> for Opdata
{fn from(u: u64)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::U64,Opcode::Pushu(u as u64))}}


impl
std::convert::From<usize> for Opdata
{fn from(u: usize)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::USize,Opcode::Pushu(u as u64))}}


impl
std::convert::From<f32> for Opdata
{fn from(f: f32)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::F32,Opcode::Pushf(f as f64))}}


impl
std::convert::From<f64> for Opdata
{fn from(f: f64)-> Opdata{Opdata::from_ty_and_opcode(SizedTy::F64,Opcode::Pushf(f as f64))}}


impl
std::convert::TryFrom<EvalResult> for Opdata
{


type Error = ();


fn
try_from(res: EvalResult)-> Result<Opdata,Self::Error>
{
    match res
    {
  EvalResult::Value(o)=>{Ok(o)}
  EvalResult::Dereference(mut o)=>
    {
      o.add_ld();

      Ok(o)
    }
  EvalResult::Bool(b)=>{Ok(Opdata::from(b))}

  EvalResult::I8(i)   =>{Ok(Opdata::from(i))}
  EvalResult::I16(i)  =>{Ok(Opdata::from(i))}
  EvalResult::I32(i)  =>{Ok(Opdata::from(i))}
  EvalResult::I64(i)  =>{Ok(Opdata::from(i))}
  EvalResult::ISize(i)=>{Ok(Opdata::from(i))}

  EvalResult::U8(u)   =>{Ok(Opdata::from(u))}
  EvalResult::U16(u)  =>{Ok(Opdata::from(u))}
  EvalResult::U32(u)  =>{Ok(Opdata::from(u))}
  EvalResult::U64(u)  =>{Ok(Opdata::from(u))}
  EvalResult::USize(u)=>{Ok(Opdata::from(u))}

  EvalResult::F32(f)=>{Ok(Opdata::from(f))}
  EvalResult::F64(f)=>{Ok(Opdata::from(f))}
  _=>{Err(())}
    }
}


}




#[derive(Clone)]
pub enum
EvalResult
{
        Value(Opdata),
  Dereference(Opdata),

  Type(Ty),

    Int(i64),
   Uint(u64),
  Float(f64),

  Void,
  Bool(bool),

  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  ISize(isize),

  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  USize(usize),

  F32(f32),
  F64(f64),

  Err,

}


impl
EvalResult
{


pub fn
is_ok(&self)-> bool
{
  !self.is_err()
}


pub fn
is_err(&self)-> bool
{
  if let Self::Err = self{true} else{false}
}


pub fn
print(&self)
{
    match self
    {
  Self::Value(o)      =>{o.print();}
  Self::Dereference(o)=>{o.print();}

  Self::Type(ty)=>{ty.print();}

  Self::Int(i)  =>{print!("{}: literal   int",*i);}
  Self::Uint(u) =>{print!("{}: literal  uint",*u);}
  Self::Float(f)=>{print!("{}: literal float",*f);}

  Self::Void=>{print!("void");}
  Self::Bool(b)=>{print!("{}: const bool",*b);}

  Self::I8(i)=>{print!("{}: const i8",*i);}
  Self::I16(i)=>{print!("{}: const i16",*i);}
  Self::I32(i)=>{print!("{}: const i32",*i);}
  Self::I64(i)=>{print!("{}: const i64",*i);}
  Self::ISize(i)=>{print!("{}: const isize",*i);}

  Self::U8(u)=>{print!("{}: const u8",*u);}
  Self::U16(u)=>{print!("{}: const u16",*u);}
  Self::U32(u)=>{print!("{}: const u32",*u);}
  Self::U64(u)=>{print!("{}: const u64",*u);}
  Self::USize(u)=>{print!("{}: const usize",*u);}

  Self::F32(f)=>{print!("{}: const f32",*f);}
  Self::F64(f)=>{print!("{}: const f64",*f);}

  Self::Err=>{print!("ERR");}
    }
}


}


impl
std::convert::TryInto<usize> for EvalResult
{


type Error = ();


fn
try_into(self)-> Result<usize,Self::Error>
{
    match self
    {
  EvalResult::Int(i)=>{if let Ok(u) = usize::try_from(i){Ok(u)} else{Err(())}}
  EvalResult::Uint(u)=>{Ok(u as usize)}
  EvalResult::USize(u)=>{Ok(u)}
  _=>{Err(())}
    }
}


}




pub fn
to_f32_from_f64(f: f64)-> Result<f32,()>
{
    if (f >= (f32::MIN as f64))
    && (f <= (f32::MAX as f64))
    {
      Ok(f as f32)
    }

  else{Err(())}    
}




pub fn
evaluate_call(paramss: &Vec<Ty>, args: &Vec<Expr>)-> bool
{
    for e in args
    {
    }


  true
}




pub fn
evaluate(e: &Expr, ctx: &ExecContext)-> EvalResult
{
    match e
    {
  Expr::Void=>
    {
      EvalResult::Void
    }
  Expr::Identifier(s)=>
    {
        if s == "void"
        {
          EvalResult::Void
        }

      else
        if s == "false"
        {
          EvalResult::Bool(false)
        }

      else
        if s == "true"
        {
          EvalResult::Bool(true)
        }

      else
        if let Some(sym) = ctx.find_symbol(s)
        {
            match sym.get_kind()
            {
          SymbolKind::Type(ty)=>{EvalResult::Type(Ty::Sized(Box::new(ty.clone())))}
          SymbolKind::ConstVar(res)=>{res.clone()}
          _=>{EvalResult::Err}
            }
        }

      else
        {
          EvalResult::Err
        }
    }
  Expr::Int(u)=>
    {
      EvalResult::Uint(*u)
    }
  Expr::Float(f)=>
    {
      EvalResult::Float(*f)
    }
  Expr::String(s)=>
    {
      EvalResult::Err
    }
  Expr::AccessOp(e,s)=>
    {
      evaluate(e,ctx)
    }
  Expr::SubscriptOp(e,i_e)=>
    {
      evaluate(e,ctx)
    }
  Expr::CallOp(f,args)=>
    {
      evaluate(f,ctx)
    }
  Expr::Expr(e)=>
    {
      evaluate(e,ctx)
    }
  Expr::UnaryOp(o,op)=>
    {
      evaluate_unary(o,op,ctx)
    }
  Expr::BinaryOp(l,r,op)=>
    {
      evaluate_binary(l,r,op,ctx)
    }
    }
}




