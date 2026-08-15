

use std::convert::{From, TryFrom};

use crate::source_file::{
  SourceInfo,
  Error,

};

use super::*;
use super::scope::*;
use super::expr::*;
use super::decl::*;
use super::asm::*;




pub enum
Operation
{
  Opcode(Opcode),

  Binary(Operand,Operand,Opcode),
   Unary(Operand,Opcode),

  LoadInt(i64),

  Call(Operand,Vec<Operand>),

}


impl
Operation
{


fn
try_get_const2(l: &Operand, r: &Operand)-> Result<(i64,i64),()>
{
    match l.try_get_const()
    {
  Ok(li)=>
    {
        match r.try_get_const()
        {
      Ok(ri)=>
        {
          Ok((li,ri))
        }
      Err(())=>{Err(())}
        }
    }
  Err(())=>{Err(())}
    }
}


pub fn
to_i64(b: bool)-> i64
{
  if b{1} else{0}
}


fn
to_bool(i: i64)-> bool
{
  i != 0
}


pub fn
try_get_const(&self)-> Result<i64,()>
{
    match self
    {
  Self::Opcode(op)=>{Err(())}
  Self::Binary(lo,ro,op)=>
    {
        match Self::try_get_const2(lo,ro)
        {
      Ok((l,r))=>
        {
            match op
            {
          Opcode::Add=>{Ok(l+r)}
          Opcode::Sub=>{Ok(l-r)}
          Opcode::Mul=>{Ok(l*r)}
          Opcode::Div=>{Ok(l/r)}
          Opcode::Rem=>{Ok(l%r)}
          Opcode::Shl=>{Ok(l<<r)}
          Opcode::Shr=>{Ok(l>>r)}
          Opcode::And=>{Ok(l&r)}
          Opcode::Or =>{Ok(l|r)}
          Opcode::Xor=>{Ok(l^r)}
          Opcode::Eq  =>{Ok(Self::to_i64(l == r))}
          Opcode::Neq =>{Ok(Self::to_i64(l != r))}
          Opcode::Lt  =>{Ok(Self::to_i64(l <  r))}
          Opcode::Lteq=>{Ok(Self::to_i64(l <= r))}
          Opcode::Gt  =>{Ok(Self::to_i64(l >  r))}
          Opcode::Gteq=>{Ok(Self::to_i64(l >= r))}
          Opcode::Land=>{Ok(Self::to_i64(Self::to_bool(l) && Self::to_bool(r)))}
          Opcode::Lor =>{Ok(Self::to_i64(Self::to_bool(l) || Self::to_bool(r)))}
          _=>{Err(())}
            }
        }
      Err(())=>{Err(())}
        }
    }
  Self::Unary(o,op)=>
    {
        match o.try_get_const()
        {
      Ok(i)=>
        {
            match op
            {
          Opcode::Not=>{Ok(!i)}
          Opcode::Lnot=>{Ok(Self::to_i64(i == 0))}
          Opcode::Neg=>{Ok(-i)}
          _=>{Err(())}
            }
        }
      Err(())=>{Err(())}
        }
    }
  Self::LoadInt(i)=>{Ok(*i)}
  Self::Call(f,args)=>{Err(())}
    }
}


pub fn
print_to(&self, txt: &mut AsmText)
{
    match self
    {
  Self::Opcode(op)=>{txt.push_opcode(*op);}
  Self::Binary(l,r,op)=>
    {
      l.print_to(true,txt);
      r.print_to(true,txt);
      txt.push_opcode(*op);
    }
  Self::Unary(o,op)=>
    {
      o.print_to(true,txt);
      txt.push_opcode(*op);
    }
  Self::LoadInt(i)=>{txt.push_i64(*i)}
  Self::Call(f,args)=>
    {
      f.print_to(true,txt);

      let  arg_n = args.len();

        for a in args
        {
          a.print_to(true,txt);
        }


      txt.push_i64(arg_n as i64);

      txt.push_opcode(Opcode::Cal);
    }
    }
}


}




pub enum
Operand
{
  Undef(&'static str),

  Value(Box<Operation>),
  Deref(Box<Operation>,TyKind),

  Err(Error),

}


impl
Operand
{


pub fn
from_bool(b: bool)-> Self
{
  let  i = Operation::to_i64(b);

  Self::from_int(i)
}


pub fn
from_int(i: i64)-> Self
{
  Self::Value(Box::new(Operation::LoadInt(i)))
}


pub fn
from_opcode(op: Opcode)-> Self
{
  Self::Value(Box::new(Operation::Opcode(op)))
}


pub fn
make_load_global(offset: usize, k: TyKind)-> Self
{
  let  o = Operation::LoadInt(offset as i64);

  Self::Deref(Box::new(o),k)
}


pub fn
make_load_local(offset: isize, k: TyKind)-> Self
{
  let  l = Operation::Opcode(Opcode::Pushfp);
  let  r = Operation::LoadInt(offset as i64);

  let  lo = Operand::Value(Box::new(l));
  let  ro = Operand::Value(Box::new(r));

  let  bin = Operation::Binary(lo,ro,Opcode::Add);

  Self::Deref(Box::new(bin),k)
}


pub fn
make_binary(l: Self, r: Self, op: Opcode)-> Self
{
  let  bin = Operation::Binary(l,r,op);

  Self::Value(Box::new(bin))
}


pub fn
make_unary(o: Self, op: Opcode)-> Self
{
  let  un = Operation::Unary(o,op);

  Self::Value(Box::new(un))
}


pub fn
try_get_const(&self)-> Result<i64,()>
{
    match self
    {
  Self::Undef(_)=>{Err(())}
  Self::Value(o)=>{o.try_get_const()}
  Self::Deref(_,_)=>{Err(())}
  Self::Err(_)=>{Err(())}
    }
}


pub fn
get_ty_kind(&self)-> Option<&TyKind>
{
    if let Self::Deref(_,k) = self
    {
      return Some(k);
    }


  None
}


pub fn
print_to(&self, loading: bool, txt: &mut AsmText)
{
    match self
    {
  Self::Undef(_)=>{}
  Self::Value(o)=>{o.print_to(txt);}
  Self::Deref(o,k)=>
    {
      o.print_to(txt);

        if loading
        {
            match k
            {
          TyKind::I8 =>{txt.push_opcode(Opcode::Ld_i8 );}
          TyKind::I16=>{txt.push_opcode(Opcode::Ld_i16);}
          TyKind::I32=>{txt.push_opcode(Opcode::Ld_i32);}
          TyKind::I64=>{txt.push_opcode(Opcode::Ld_i64);}
          TyKind::U8 =>{txt.push_opcode(Opcode::Ld_u8 );}
          TyKind::U16=>{txt.push_opcode(Opcode::Ld_u16);}
          TyKind::U32=>{txt.push_opcode(Opcode::Ld_u32);}
          _=>{panic!();}
            }
        }
    }
  Self::Err(_)=>{}
    }
}


}




pub fn
evaluate_call(f: &Expr, args: &Vec<Expr>, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  srcinf = f.get_source_info();

  let  o = evaluate(f,set,scp_opt);

  let  mut buf = Vec::<Operand>::new();

    for a in args
    {
      buf.push(evaluate(a,set,scp_opt))
    }


  let  opr = Operation::Call(o,buf);

  Operand::Value(Box::new(opr))
}


pub fn
evaluate_access(e: &Expr, s: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  srcinf = e.get_source_info();

  let  o = evaluate(e,set,scp_opt);

  todo!();
}


pub fn
evaluate_reint(e: &Expr, s: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  srcinf = e.get_source_info();

    match evaluate(e,set,scp_opt)
    {
  Operand::Undef(_)=>{Operand::Undef("")}
  Operand::Value(o)=>
    {
           if s ==  "i8"{Operand::Deref(o,TyKind::I8 )}
      else if s == "i16"{Operand::Deref(o,TyKind::I16)}
      else if s == "i32"{Operand::Deref(o,TyKind::I32)}
      else if s == "i64"{Operand::Deref(o,TyKind::I64)}
      else if s ==  "u8"{Operand::Deref(o,TyKind::U8 )}
      else if s == "u16"{Operand::Deref(o,TyKind::U16)}
      else if s == "u32"{Operand::Deref(o,TyKind::U32)}
      else
        {
          Operand::Err(srcinf.to_error(format!("evalute_access error: unknown field {}",s)))
        }
    }
  Operand::Deref(o,k)=>
    {
      if s == "ptr"{Operand::Value(o)}
      else
        {
          Operand::Err(srcinf.to_error(format!("evalute_access error: unknown field {}",s)))
        }
    }
  Operand::Err(e)=>
    {
      Operand::Err(e)
    }
    }
}


pub fn
evaluate_subscr(ref_e: &Expr, idx_e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  srcinf = ref_e.get_source_info();

  let  ref_o = evaluate(ref_e,set,scp_opt);
  let  idx_o = evaluate(idx_e,set,scp_opt);

//  Operand::Deref(Operation::(ref_o,idx_o))
todo!();
}


pub fn
evaluate_decl(decl: &Decl)-> Operand
{
    match decl.get_kind()
    {
  DeclKind::Const(_,i)=>
    {
      Operand::from_int(*i)
    }
  DeclKind::Static(inf)=>
    {
      Operand::make_load_global(decl.get_offset(),inf.get_ty_kind().clone())
    }
  DeclKind::Var(inf)=>
    {
      panic!();
    }
  DeclKind::Fn(_)=>
    {
      Operand::make_load_global(decl.get_offset(),TyKind::Fn)
    }
  _=>{Operand::Err(decl.get_source_info().to_error(format!("evaluate_identifier error: {} is invalid symbol kind",&decl.get_qualified_name())))}
    }
}


pub fn
evaluate_identifier(srcinf: &SourceInfo, name: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
    if let Some(scp) = scp_opt
    {
        if let Some(sym) = scp.find(name)
        {
          return match sym.get_kind()
            {
          SymbolKind::Const(i)=>
            {
              Operand::from_int(*i)
            }
          SymbolKind::Static(_,k)=>
            {
              Operand::make_load_global(sym.get_offset() as usize,k.clone())
            }
          SymbolKind::Var(_,k)=>
            {
              Operand::make_load_local(sym.get_offset(),k.clone())
            }
          _=>{Operand::Err(srcinf.to_error(format!("evaluate_identifier error: {} is invalid local symbol kind",name)))}
            };
        }
    }


    if let Some(decl) = set.search(name)
    {
      evaluate_decl(decl)
    }

  else
    {
      Operand::Err(srcinf.to_error(format!("evaluate_identifier error: {} not found",name)))
    }
}


pub fn
evaluate_unary(e: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  srcinf = e.get_source_info();

  let  o = evaluate(e,set,scp_opt);

    match op
    {
  (s) if s == "-" =>{Operand::make_unary(o,Opcode::Neg)}
  (s) if s == "!" =>{Operand::make_unary(o,Opcode::Lnot)}
  (s) if s == "~" =>{Operand::make_unary(o,Opcode::Not)}
  _=>{Operand::Err(srcinf.to_error(format!("unknown operator {}",op)))}
    }
}


pub fn
evaluate_binary(l: &Expr, r: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  l_srcinf = l.get_source_info();
  let  r_srcinf = r.get_source_info();

  let  lo = evaluate(l,set,scp_opt);
  let  ro = evaluate(r,set,scp_opt);

    match op
    {
  (s) if s == "+" =>{Operand::make_binary(lo,ro,Opcode::Add)}
  (s) if s == "-" =>{Operand::make_binary(lo,ro,Opcode::Sub)}
  (s) if s == "*" =>{Operand::make_binary(lo,ro,Opcode::Mul)}
  (s) if s == "/" =>{Operand::make_binary(lo,ro,Opcode::Div)}
  (s) if s == "%" =>{Operand::make_binary(lo,ro,Opcode::Rem)}
  (s) if s == "<<"=>{Operand::make_binary(lo,ro,Opcode::Shl)}
  (s) if s == ">>"=>{Operand::make_binary(lo,ro,Opcode::Shr)}
  (s) if s == "&" =>{Operand::make_binary(lo,ro,Opcode::And)}
  (s) if s == "|" =>{Operand::make_binary(lo,ro,Opcode::Or)}
  (s) if s == "^" =>{Operand::make_binary(lo,ro,Opcode::Xor)}
  (s) if s == "=="=>{Operand::make_binary(lo,ro,Opcode::Eq)}
  (s) if s == "!="=>{Operand::make_binary(lo,ro,Opcode::Neq)}
  (s) if s == "<" =>{Operand::make_binary(lo,ro,Opcode::Lt)}
  (s) if s == "<="=>{Operand::make_binary(lo,ro,Opcode::Lteq)}
  (s) if s == ">" =>{Operand::make_binary(lo,ro,Opcode::Gt)}
  (s) if s == ">="=>{Operand::make_binary(lo,ro,Opcode::Gteq)}
  (s) if s == "&&"=>{Operand::make_binary(lo,ro,Opcode::Land)}
  (s) if s == "||"=>{Operand::make_binary(lo,ro,Opcode::Lor)}
  _=>{Operand::Err(l_srcinf.to_error(format!("unknown operator {}",op)))}
    }
}


pub fn
evaluate(e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
    match e.get_kind()
    {
  ExprKind::Identifier(s)=>
    {
      evaluate_identifier(e.get_source_info(),s,set,scp_opt)
    }
  ExprKind::Int(i)=>
    {
      Operand::from_int(*i)
    }
  ExprKind::String(_,name)=>
    {
        if let Some(decl) = set.get_root().find(&name)
        {
todo!();
        }

      else
        {
          Operand::Err(e.get_source_info().to_error(format!("{} is not found or string",name)))
        }
    }
  ExprKind::CallOp(f,args)=>
    {
      evaluate_call(f,args,set,scp_opt)
    }
  ExprKind::Expr(e)=>
    {
      evaluate(e,set,scp_opt)
    }
  ExprKind::AccessOp(ins,s)=>
    {
      evaluate_access(ins,s,set,scp_opt)
    }
  ExprKind::ReintOp(ins,s)=>
    {
      evaluate_reint(ins,s,set,scp_opt)
    }
  ExprKind::SubscrOp(ref_o,idx_o)=>
    {
      evaluate_subscr(ref_o,idx_o,set,scp_opt)
    }
  ExprKind::UnaryOp(o,op)=>
    {
      evaluate_unary(o,op,set,scp_opt)
    }
  ExprKind::BinaryOp(l,r,op)=>
    {
      evaluate_binary(l,r,op,set,scp_opt)
    }
    }
}




pub fn
evaluate_const(e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> Option<i64>
{
    match evaluate(e,set,scp_opt)
    {
  Operand::Value(o)=>
    {
        match o.try_get_const()
        {
      Ok(i)=>{Some(i)}
      Err(())=>{None}
        }
    }
  _=>{None}
    }
}




