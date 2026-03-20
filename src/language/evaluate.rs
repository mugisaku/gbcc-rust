

use std::convert::{From, TryFrom};

use super::*;
use super::scope::*;
use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::decl::*;
use super::asm::*;
use super::evaluate_unary::*;
use super::evaluate_binary::*;




#[derive(Clone)]
pub struct
ValueProcess
{
  table: AsmTable,
     ty: Ty,

}


impl
ValueProcess
{


pub fn
new(table: AsmTable, ty: Ty)-> Self{Self{table,ty}}


pub fn
get_table(&self)-> &AsmTable{&self.table}


pub fn
get_table_mut(&mut self)-> &mut AsmTable{&mut self.table}


pub fn
get_ty(&self)-> &Ty{&self.ty}


pub fn
set_ty(&mut self, ty: Ty){self.ty = ty;}


}




impl From<bool> for ValueProcess{fn from(b: bool)-> Self{
  let  mut table = AsmTable::new();

  table.push_li_bool(b);

  Self::new(table,Ty::Bool)
}}

impl From<i64> for ValueProcess{fn from(i: i64)-> Self{
  let  mut table = AsmTable::new();

  table.push_li_int(i);

  Self::new(table,Ty::Int)
}}

impl From<f64> for ValueProcess{fn from(f: f64)-> Self{
  let  mut table = AsmTable::new();

  table.push_li_float(f);

  Self::new(table,Ty::Int)
}}

impl TryFrom<EvalResult> for ValueProcess{  type Error = ();  fn try_from(res: EvalResult)-> Result<Self,Self::Error>{
    match res
    {
  EvalResult::Value(vp)=>{Ok(vp)}
  EvalResult::Deref(mut ls,ma)=>
    {
        match ma
        {
      MemAcc::I8=>
        {
          ls.push_opcode(Opcode::Ld8);
          ls.push_opcode(Opcode::Sx8);

          Ok(ValueProcess::new(ls,Ty::Int))
        }
      MemAcc::I16=>
        {
          ls.push_opcode(Opcode::Ld16);
          ls.push_opcode(Opcode::Sx16);

          Ok(ValueProcess::new(ls,Ty::Int))
        }
      MemAcc::I32=>
        {
          ls.push_opcode(Opcode::Ld32);
          ls.push_opcode(Opcode::Sx32);

          Ok(ValueProcess::new(ls,Ty::Int))
        }
      MemAcc::I64=>
        {
          ls.push_opcode(Opcode::Ld64);

          Ok(ValueProcess::new(ls,Ty::Int))
        }
      MemAcc::F32=>
        {
          ls.push_opcode(Opcode::Ld32);
          ls.push_opcode(Opcode::B32toF);

          Ok(ValueProcess::new(ls,Ty::Float))
        }
      MemAcc::F64=>
        {
          ls.push_opcode(Opcode::Ld64);

          Ok(ValueProcess::new(ls,Ty::Float))
        }
      _=>{Err(())}
        }
    }

  EvalResult::Type(ty)=>{Err(())}

  EvalResult::Void=>
    {
      let  mut table = AsmTable::new();

      table.push_opcode(Opcode::Push0);

      Ok(Self::new(table,Ty::Void))
    }
  EvalResult::Bool(b) =>{Ok(Self::from(b))}
  EvalResult::Int(i)  =>{Ok(Self::from(i))}
  EvalResult::Float(f)=>{Ok(Self::from(f))}
  EvalResult::String(_)=>{Err(())}
  EvalResult::Binary(_)=>{Err(())}

  EvalResult::Err=>{Err(())}
    }
}}




#[derive(Clone)]
pub enum
MemAcc
{
  I8, I16, I32, I64,
  F32, F64,

}


#[derive(Clone)]
pub enum
EvalResult
{
  Value(ValueProcess),
  Deref(AsmTable,MemAcc),

  Type(Ty),

  Void,
  Bool(bool),
    Int(i64),
  Float(f64),

  String(String),
  Binary(Vec<u8>),

  Err,

}


impl
EvalResult
{


pub fn
from_value_and_ty(value: &SymbolValue, ty: &Ty)-> Self
{
    match ty
    {
  Ty::Void =>{Self::Void                    }
  Ty::Bool =>{Self::Bool( value.get_bool() )}
  Ty::Int  =>{Self::Int(  value.get_int()  )}
  Ty::Float=>{Self::Float(value.get_float())}
  _=>{Self::Err}
    }
}


pub fn
from_global_addr_and_ty(off: usize, ty: &Ty)-> Self
{
  let  mut table = AsmTable::new();

  table.push_li_global_addr(off);

  Self::Deref(table,MemAcc::I64)
}


pub fn
from_fn_addr_and_ty(off: usize, ty: &Ty)-> Self
{
  let  mut table = AsmTable::new();

  table.push_li_global_addr(off);

  Self::Deref(table,MemAcc::I64)
}


pub fn
from_local_addr_and_ty(off: usize, ty: &Ty)-> Self
{
  let  mut table = AsmTable::new();

  table.push_li_local_addr(off);

  Self::Deref(table,MemAcc::I64)
}


pub fn
from_parameter_addr_and_ty(off: usize, ty: &Ty)-> Self
{
  let  mut table = AsmTable::new();

  table.push_li_parameter_addr(off);

  Self::Deref(table,MemAcc::I64)
}


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
  Self::Value(_)     =>{print!("value");}
  Self::Deref(_,_)=>{print!("deref");}

  Self::Type(ty)=>{ty.print();}

  Self::Void=>{print!("void");}
  Self::Bool(b)=>{print!("bool");}
  Self::Int(i)  =>{print!("{}: literal   int",*i);}
  Self::Float(f)=>{print!("{}: literal float",*f);}

  Self::String(s)=>{print!("literal string");}
  Self::Binary(s)=>{print!("literal binary");}

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
  _=>{Err(())}
    }
}


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
evaluate(e: &Expr, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
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
        if let Some(scp) = scp_opt
        {
            if let Some(lsym) = scp.find(s)
            {
                match lsym.get_kind()
                {
              LocalSymbolKind::Const=>
                {
                  EvalResult::from_value_and_ty(lsym.get_value(),lsym.get_ty())
                }
              LocalSymbolKind::Var=>
                {
                  EvalResult::from_local_addr_and_ty(lsym.get_offset(),lsym.get_ty())
                }
              LocalSymbolKind::Parameter=>
                {
                  EvalResult::from_parameter_addr_and_ty(lsym.get_offset(),lsym.get_ty())
                }
              _=>{EvalResult::Err}
                }
            }

          else{EvalResult::Err}
        }

      else
        if let Some(sym) = tbl.find_symbol(s)
        {
            match sym.get_kind()
            {
          SymbolKind::Const(_)=>
            {
              EvalResult::from_value_and_ty(sym.get_value(),sym.get_ty())
            }
          SymbolKind::GlobalVar(_)=>
            {
              EvalResult::from_global_addr_and_ty(sym.get_offset(),sym.get_ty())
            }
          SymbolKind::Fn{..}=>
            {
              EvalResult::from_fn_addr_and_ty(sym.get_offset(),sym.get_ty())
            }
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
      EvalResult::Int(*u as i64)
    }
  Expr::Float(f)=>
    {
      EvalResult::Float(*f)
    }
  Expr::String(s)=>
    {
      EvalResult::String(s.clone())
    }
  Expr::AccessOp(e,s)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::SubscriptOp(e,i_e)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::CallOp(f,args)=>
    {
      evaluate(f,tbl,scp_opt)
    }
  Expr::Expr(e)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::UnaryOp(o,op)=>
    {
      evaluate_unary(o,op,tbl,scp_opt)
    }
  Expr::BinaryOp(l,r,op)=>
    {
      evaluate_binary(l,r,op,tbl,scp_opt)
    }
    }
}




