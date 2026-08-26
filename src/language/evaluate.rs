

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




enum UseArgs
{No, Yes}


pub enum
Operation
{
  Binary(Operand,Operand,Opcode),
   Unary(Operand,Opcode),

  LoadInt(i64),
  LoadPosFromFp(i64),
  LoadValue(Operand),

     Call(Operand,Vec<Operand>),
  SymCall(SourceInfo,&'static str,Vec<Operand>),

  Subsc(Operand,Operand),

}


impl
Operation
{


fn
try_get_const2(l: &Operand, r: &Operand)-> Result<(i64,i64),Error>
{
  let  l = l.try_get_const()?;
  let  r = r.try_get_const()?;

  Ok((l,r))
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
try_get_const(&self)-> Result<i64,Error>
{
    match self
    {
  Self::Binary(lo,ro,op)=>
    {
      let  (l,r) = Self::try_get_const2(lo,ro)?;

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
      _=>{Err(lo.source_info.to_error(format!("try_get_const error: invalid binary opcode")))}
        }
    }
  Self::Unary(o,op)=>
    {
      let  i = o.try_get_const()?;

        match op
        {
      Opcode::Not=>{Ok(!i)}
      Opcode::Lnot=>{Ok(Self::to_i64(i == 0))}
      Opcode::Neg=>{Ok(-i)}
      _=>{Err(o.source_info.to_error(format!("try_get_const error: invalid unary opcode")))}
        }
    }
  Self::LoadInt(i)=>{Ok(*i)}
  Self::LoadPosFromFp(_)=>{Err(Error::new(format!("try_get_const error: load_pos_from_fp")))}
  Self::LoadValue(o)=>{Err(o.source_info.to_error(format!("try_get_const error: load_value")))}
  Self::Call(f,_) =>{Err(f.source_info.to_error(format!("try_get_const error: call")))}
  Self::SymCall(srcinf,_,_) =>{Err(srcinf.to_error(format!("try_get_const error: sym call")))}
  Self::Subsc(ref_o,_)=>{Err(ref_o.source_info.to_error(format!("try_get_const error: subsc")))}
    }
}


fn
check_symbol(s: &'static str)-> Result<(UseArgs,Opcode),()>
{
       if s == "spawn"{Ok((UseArgs::Yes,Opcode::Spw))}
  else if s ==    "id"{Ok((UseArgs::No,Opcode::Pushid))}
  else if s ==    "fp"{Ok((UseArgs::No,Opcode::Pushfp))}
  else if s ==    "sp"{Ok((UseArgs::No,Opcode::Pushsp))}
  else if s ==    "pc"{Ok((UseArgs::No,Opcode::Pushpc))}
  else if s == "input"{Ok((UseArgs::No,Opcode::Pushinput))}
  else if s == "timer"{Ok((UseArgs::No,Opcode::Pushtimer))}
  else{Err(())}
}


pub fn
write_to(&self, txt: &mut AsmText)-> Result<(),Error>
{
    match self
    {
  Self::Binary(l,r,op)=>
    {
      l.write_to(true,txt)?;
      r.write_to(true,txt)?;

      txt.push_opcode(*op);
    }
  Self::Unary(o,op)=>
    {
      o.write_to(true,txt)?;

      txt.push_opcode(*op);
    }
  Self::LoadInt(i)=>{txt.push_i64(*i)}
  Self::LoadPosFromFp(i)=>
    {
      txt.push_opcode(Opcode::Pushfp);
      txt.push_i64(*i);
      txt.push_opcode(Opcode::Add);
    }
  Self::LoadValue(o)=>{o.write_to(true,txt)?;}
  Self::Call(f,args)=>
    {
      f.write_to(true,txt)?;

      let  arg_n = args.len();

        for a in args
        {
          a.write_to(true,txt)?;
        }


      txt.push_i64(arg_n as i64);

      txt.push_opcode(Opcode::Cal);
    }
  Self::SymCall(srcinf,s,args)=>
    {
        match Self::check_symbol(s)
        {
      Ok((use_args,op))=>
        {
            if let UseArgs = use_args
            {
              let  arg_n = args.len();

                for a in args
                {
                  a.write_to(true,txt)?;
                }


              txt.push_i64(arg_n as i64);
            }


          txt.push_opcode(op);
        }
      Err(())=>
        {
          return Err(srcinf.to_error(format!("write_to error: symcall {}",s)));
        }
        }
    }
  Self::Subsc(ref_o,idx_o)=>
    {
        if let OperandKind::Deref(_,k) = &ref_o.kind
        {
          let  sz = k.get_size();

          ref_o.write_to(false,txt)?;
          idx_o.write_to( true,txt)?;

          txt.push_i64(sz as i64);

          txt.push_opcode(Opcode::Mul);
          txt.push_opcode(Opcode::Add);
        }

      else
        {return Err(ref_o.source_info.to_error(format!("write_to error: subsc for non deref")));}
    }
    }


  Ok(())
}


pub fn
print(&self)
{
    match self
    {
  Self::Binary(l,r,op)=>
    {
      l.print();

      print!(" ");

      op.print();

      print!(" ");

      r.print();
    }
  Self::Unary(o,op)=>
    {
      op.print();

      print!(" ");

      o.print();
    }
  Self::LoadInt(i)=>{print!("{}",*i)}
  Self::LoadPosFromFp(i)=>{print!("load_pos_from_fp{}",*i)}
  Self::LoadValue(o)=>
    {
      print!("ld(");
      o.print();
      print!(")");
    }
  Self::Call(f,args)=>
    {
      f.print();

      print!("(");

      let  arg_n = args.len();

        for a in args
        {
          a.print();

          print!(", ");
        }


      print!(")");
    }
  Self::SymCall(_,s,args)=>
    {
      print!("{}(",s);

      let  arg_n = args.len();

        for a in args
        {
          a.print();

          print!(", ");
        }


      print!(")");
    }
  Self::Subsc(ref_o,idx_o)=>
    {
      ref_o.print();

      print!("[");

      idx_o.print();

      print!("]");
    }
    }
}


}




pub enum
OperandKind
{
   Undef(String),
  Symbol(&'static str),

  Value(Box<Operation>),
  Deref(Box<Operation>,TyKind),

}


pub struct
Operand
{
  source_info: SourceInfo,

  kind: OperandKind,

}


impl
Operand
{


pub fn
make_undef(source_info: SourceInfo, msg: String)-> Self
{
  Self{
    source_info,

    kind: OperandKind::Undef(msg),
  }
}


pub fn
from_bool(source_info: SourceInfo, b: bool)-> Self
{
  let  i = Operation::to_i64(b);

  Self::from_int(source_info,i)
}


pub fn
from_int(source_info: SourceInfo, i: i64)-> Self
{
  let  op = Operation::LoadInt(i);

  Operand{source_info, kind: OperandKind::Value(Box::new(op))}
}


pub fn
make_load_global(source_info: SourceInfo, offset: usize)-> Self
{
  let  op = Operation::LoadInt(offset as i64);

  Operand{source_info, kind: OperandKind::Deref(Box::new(op),TyKind::I64)}
}


pub fn
make_load_local(source_info: SourceInfo, offset: isize)-> Self
{
  let  op = Operation::LoadPosFromFp(offset as i64);

  Operand{source_info, kind: OperandKind::Deref(Box::new(op),TyKind::I64)}
}


pub fn
make_load_fn(source_info: SourceInfo, offset: usize)-> Self
{
  let  o = Operand::make_load_global(source_info.clone(),offset);

  let  un = Operation::Unary(o,Opcode::Ld_i64);

  Operand{source_info, kind: OperandKind::Value(Box::new(un))}
}


pub fn
make_binary(source_info: SourceInfo, l: Self, r: Self, op: Opcode)-> Self
{
  let  bin = Operation::Binary(l,r,op);

  Operand{source_info, kind: OperandKind::Value(Box::new(bin))}
}


pub fn
make_unary(source_info: SourceInfo, o: Self, op: Opcode)-> Self
{
  let  un = Operation::Unary(o,op);

  Operand{source_info, kind: OperandKind::Value(Box::new(un))}
}


pub fn
try_get_const(&self)-> Result<i64,Error>
{
    match &self.kind
    {
  OperandKind::Undef(s)=>{Err(self.source_info.to_error(format!("{}",s)))}
  OperandKind::Symbol(s)=>{Err(self.source_info.to_error(format!("{}",s)))}
  OperandKind::Value(o)=>{o.try_get_const()}
  OperandKind::Deref(_,_)=>{Err(self.source_info.to_error(format!("")))}
    }
}


pub fn
clone_ty_kind(&self)-> TyKind
{
    if let OperandKind::Deref(_,k) = &self.kind
    {
      return k.clone();
    }


  TyKind::Void
}


fn
try_get_load_op(k: &TyKind)-> Result<Opcode,()>
{
    match k
    {
  TyKind::I8 =>{Ok(Opcode::Ld_i8 )}
  TyKind::I16=>{Ok(Opcode::Ld_i16)}
  TyKind::I32=>{Ok(Opcode::Ld_i32)}
  TyKind::I64=>{Ok(Opcode::Ld_i64)}
  TyKind::U8 =>{Ok(Opcode::Ld_u8 )}
  TyKind::U16=>{Ok(Opcode::Ld_u16)}
  TyKind::U32=>{Ok(Opcode::Ld_u32)}
  _=>{Err(())}
    }
}


pub fn
write_to(&self, loading: bool, txt: &mut AsmText)-> Result<(),Error>
{
    match &self.kind
    {
  OperandKind::Undef(s)=>{return Err(self.source_info.to_error(format!("write_to error: undef {}",s)));}
  OperandKind::Symbol(s)=>{return Err(self.source_info.to_error(format!("write_to error: symbol {}",s)));}
  OperandKind::Value(o)=>{o.write_to(txt)?}
  OperandKind::Deref(o,k)=>
    {
      o.write_to(txt)?;

        if loading
        {
            match Self::try_get_load_op(k)
            {
          Ok(op)=>{txt.push_opcode(op);}
          Err(())=>{return Err(self.source_info.to_error(format!("write_to error: deref")));}
            }
        }
    }
    }


  Ok(())
}


pub fn
print(&self)
{
    match &self.kind
    {
  OperandKind::Undef(s)=>{print!("undef {}",s);}
  OperandKind::Symbol(s)=>{print!("symbol {}",s);}
  OperandKind::Value(o)=>
    {
      print!("(");
      o.print();
      print!(")");

    }
  OperandKind::Deref(o,_)=>
    {
      print!("(");
      o.print();
      print!(")");
    }
    }
}


}




pub fn
evaluate_call(f: &Expr, args: &Vec<Expr>, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = f.get_source_info().clone();

  let  o = evaluate(f,set,scp_opt);

  let  mut buf = Vec::<Operand>::new();

    for a in args
    {
      buf.push(evaluate(a,set,scp_opt))
    }


  let  opr = 
    if let OperandKind::Symbol(s) = &o.kind
    {
      Operation::SymCall(source_info.clone(),s,buf)
    }

  else
    {
      Operation::Call(o,buf)
    }
  ;


  Operand{source_info, kind: OperandKind::Value(Box::new(opr))}
}


pub fn
evaluate_dot(e: &Expr, s: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = e.get_source_info().clone();

  let  o = evaluate(e,set,scp_opt);

    match o.kind
    {
  OperandKind::Undef(_)=>{o}
  OperandKind::Symbol(sym_s)=>
    {
        if sym_s == "sys"
        {
               if s == "spawn"{return Operand{source_info, kind: OperandKind::Symbol("spawn")};}
          else if s ==    "id"{return Operand{source_info, kind: OperandKind::Symbol("id")};}
          else if s ==    "fp"{return Operand{source_info, kind: OperandKind::Symbol("fp")};}
          else if s ==    "sp"{return Operand{source_info, kind: OperandKind::Symbol("sp")};}
          else if s ==    "pc"{return Operand{source_info, kind: OperandKind::Symbol("pc")};}
          else if s == "input"{return Operand{source_info, kind: OperandKind::Symbol("input")};}
          else if s == "timer"{return Operand{source_info, kind: OperandKind::Symbol("timer")};}
        }


      Operand::make_undef(source_info,format!("evaluate_dot case symbol: {} {}",sym_s,s))
    }
  OperandKind::Value(op)=>
    {
           if s ==  "i8ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I8 )}}
      else if s == "i16ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I16)}}
      else if s == "i32ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I32)}}
      else if s == "i64ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I64)}}
      else if s ==  "u8ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U8 )}}
      else if s == "u16ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U16)}}
      else if s == "u32ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U32)}}
      else
        {
          Operand::make_undef(source_info,format!("evaluate_dot case Value: {}",s))
        }
    }
  OperandKind::Deref(op,k)=>
    {
        if s == "ptr"{Operand{source_info, kind: OperandKind::Value(op)}}
      else
        {
               if s ==  "i8ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I8 )}}
          else if s == "i16ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I16)}}
          else if s == "i32ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I32)}}
          else if s == "i64ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::I64)}}
          else if s ==  "u8ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U8 )}}
          else if s == "u16ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U16)}}
          else if s == "u32ref"{Operand{source_info, kind: OperandKind::Deref(op,TyKind::U32)}}
          else
            {
              Operand::make_undef(source_info,format!("evaluate_dot case deref: {}",s))
            }
        }
    }
    }
}


pub fn
evaluate_subsc(ref_e: &Expr, idx_e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = ref_e.get_source_info().clone();

  let  ref_o = evaluate(ref_e,set,scp_opt);
  let  idx_o = evaluate(idx_e,set,scp_opt);

  let  k = ref_o.clone_ty_kind();

  let  subsc = Operation::Subsc(ref_o,idx_o);

  Operand{source_info, kind: OperandKind::Deref(Box::new(subsc),k)}
}


pub fn
evaluate_identifier(source_info: SourceInfo, name: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
    if name == "false"
    {
      return Operand::from_int(source_info,0);
    }

  else
    if name == "true"
    {
      return Operand::from_int(source_info,1);
    }

  else
    if name == "sys"
    {
      return Operand{source_info, kind: OperandKind::Symbol("sys")};
    }


    if let Some(scp) = scp_opt
    {
        if let Some(sym) = scp.find(name)
        {
          return match sym.get_kind()
            {
          SymbolKind::Const(i)=>
            {
              Operand::from_int(source_info,*i)
            }
          SymbolKind::Static(_)=>
            {
              Operand::make_load_global(source_info,sym.get_offset() as usize)
            }
          SymbolKind::Var(_)=>
            {
              Operand::make_load_local(source_info,sym.get_offset())
            }
          _=>{Operand::make_undef(source_info,format!("evaluate_identifier case local: {} is found in scope, but invalid",name))}
            };
        }
    }


    if let Some(decl) = set.search(name)
    {
        match decl.get_kind()
        {
      DeclKind::Const(_,i)=>
        {
          Operand::from_int(source_info,*i)
        }
      DeclKind::Static(inf)=>
        {
          Operand::make_load_global(source_info,decl.get_offset())
        }
      DeclKind::Var(inf)=>
        {
          panic!();
        }
      DeclKind::Fn(_)=>
        {
          Operand::make_load_fn(source_info,decl.get_offset())
        }
      _=>{Operand::make_undef(source_info,format!("evaluate_identifier case global: {} is found in global, but invalid",name))}
        }
    }

  else
    {
      Operand::make_undef(source_info,format!("evaluate_identifier error: {} is not found",name))
    }
}


pub fn
evaluate_unary(e: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = e.get_source_info().clone();

  let  o = evaluate(e,set,scp_opt);

    match op
    {
  (s) if s == "-" =>{Operand::make_unary(source_info,o,Opcode::Neg)}
  (s) if s == "!" =>{Operand::make_unary(source_info,o,Opcode::Lnot)}
  (s) if s == "~" =>{Operand::make_unary(source_info,o,Opcode::Not)}
  _=>{Operand::make_undef(source_info,format!("evaluate_unary: {} is invalid operator",op))}
    }
}


pub fn
evaluate_binary(l: &Expr, r: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = l.get_source_info().clone();

  let  lo = evaluate(l,set,scp_opt);
  let  ro = evaluate(r,set,scp_opt);

    match op
    {
  (s) if s == "+" =>{Operand::make_binary(source_info,lo,ro,Opcode::Add)}
  (s) if s == "-" =>{Operand::make_binary(source_info,lo,ro,Opcode::Sub)}
  (s) if s == "*" =>{Operand::make_binary(source_info,lo,ro,Opcode::Mul)}
  (s) if s == "/" =>{Operand::make_binary(source_info,lo,ro,Opcode::Div)}
  (s) if s == "%" =>{Operand::make_binary(source_info,lo,ro,Opcode::Rem)}
  (s) if s == "<<"=>{Operand::make_binary(source_info,lo,ro,Opcode::Shl)}
  (s) if s == ">>"=>{Operand::make_binary(source_info,lo,ro,Opcode::Shr)}
  (s) if s == "&" =>{Operand::make_binary(source_info,lo,ro,Opcode::And)}
  (s) if s == "|" =>{Operand::make_binary(source_info,lo,ro,Opcode::Or)}
  (s) if s == "^" =>{Operand::make_binary(source_info,lo,ro,Opcode::Xor)}
  (s) if s == "=="=>{Operand::make_binary(source_info,lo,ro,Opcode::Eq)}
  (s) if s == "!="=>{Operand::make_binary(source_info,lo,ro,Opcode::Neq)}
  (s) if s == "<" =>{Operand::make_binary(source_info,lo,ro,Opcode::Lt)}
  (s) if s == "<="=>{Operand::make_binary(source_info,lo,ro,Opcode::Lteq)}
  (s) if s == ">" =>{Operand::make_binary(source_info,lo,ro,Opcode::Gt)}
  (s) if s == ">="=>{Operand::make_binary(source_info,lo,ro,Opcode::Gteq)}
  (s) if s == "&&"=>{Operand::make_binary(source_info,lo,ro,Opcode::Land)}
  (s) if s == "||"=>{Operand::make_binary(source_info,lo,ro,Opcode::Lor)}
  _=>{Operand::make_undef(source_info,format!("evaluate_binary: {} is invalid operator",op))}
    }
}


pub fn
evaluate(e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> Operand
{
  let  source_info = e.get_source_info().clone();

    match e.get_kind()
    {
  ExprKind::Identifier(s)=>
    {
      evaluate_identifier(source_info,s,set,scp_opt)
    }
  ExprKind::Int(i)=>
    {
      Operand::from_int(source_info,*i)
    }
  ExprKind::String(_,name)=>
    {
        if let Some(decl) = set.get_root().find(&name)
        {
todo!();
        }

      else
        {
          Operand::make_undef(source_info,format!("evaluate case string"))
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
  ExprKind::DotOp(ins,s)=>
    {
      evaluate_dot(ins,s,set,scp_opt)
    }
  ExprKind::SubscOp(ref_o,idx_o)=>
    {
      evaluate_subsc(ref_o,idx_o,set,scp_opt)
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
  let  o = evaluate(e,set,scp_opt);

    match &o.kind
    {
  OperandKind::Value(_)=>
    {
        match o.try_get_const()
        {
      Ok(i)=>{Some(i)}
      Err(_)=>{None}
        }
    }
  _=>{None}
    }
}




