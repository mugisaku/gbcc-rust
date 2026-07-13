

use super::*;
use super::expr::*;
use super::scope::*;
use super::decl::*;
use super::asm::*;
use crate::source_file::{
  SourceInfo,
  Error,

};


use super::evaluate::EvalResult;




pub fn
evaluate_unary_int(v: i64, op: &str)-> Result<i64,String>
{
    match op
    {
  (s) if s == "!"=>{Ok(!v)}
  (s) if s == "-"=>{Ok(-v)}
  _=>{Err(format!("{} is invalid unary operator",op))}
    }
}


pub fn
evaluate_unary_const(e: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = e.get_source_info();

    match evaluate_const(e,set,scp_opt)
    {
  EvalResult::Const(v)=>
    {
        match evaluate_unary_int(v,op)
        {
      Ok(val) =>{EvalResult::Const(val)}
      Err(msg)=>{EvalResult::Err(srcinf.to_error(msg))}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef("")}
    }
}


pub fn
evaluate_binary_int(l: i64, r: i64, op: &str)-> Result<i64,String>
{
    match op
    {
  (s) if s ==  "+"=>{Ok(l+r)}
  (s) if s ==  "-"=>{Ok(l-r)}
  (s) if s ==  "*"=>{Ok(l*r)}
  (s) if s ==  "/"=>{Ok(l/r)}
  (s) if s ==  "%"=>{Ok(l%r)}
  (s) if s == "<<"=>{Ok(l<<r)}
  (s) if s == ">>"=>{Ok(l>>r)}
  (s) if s ==  "&"=>{Ok(l&r)}
  (s) if s ==  "|"=>{Ok(l|r)}
  (s) if s ==  "^"=>{Ok(l^r)}
  (s) if s == "=="=>{Ok(if l == r{1} else{0})}
  (s) if s == "!="=>{Ok(if l != r{1} else{0})}
  (s) if s ==  "<"=>{Ok(if l <  r{1} else{0})}
  (s) if s == "<="=>{Ok(if l <= r{1} else{0})}
  (s) if s ==  ">"=>{Ok(if l >  r{1} else{0})}
  (s) if s == ">="=>{Ok(if l >= r{1} else{0})}
  (s) if s == "&&"=>{Ok(if (l != 0) && (r != 0){1} else{0})}
  (s) if s == "||"=>{Ok(if (l != 0) || (r != 0){1} else{0})}
  _=>{Err(format!("{} is invalid binary operator",op))}
    }
}


pub fn
evaluate_binary_const(le: &Expr, re: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = le.get_source_info();

    match evaluate_const(le,set,scp_opt)
    {
  EvalResult::Const(lv)=>
    {
        match evaluate_const(re,set,scp_opt)
        {
      EvalResult::Const(rv)=>
        {
            match evaluate_binary_int(lv,rv,op)
            {
          Ok(val) =>{EvalResult::Const(val)}
          Err(msg)=>{EvalResult::Err(srcinf.to_error(msg))}
            }
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef("")}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef("")}
    }
}


pub fn
evaluate_const(e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = e.get_source_info();

    match e.get_kind()
    {
  ExprKind::Identifier(s)=>
    {
           if s == "false"{return EvalResult::Const(0);}
      else if s ==  "true"{return EvalResult::Const(1);}
      else if s ==   "sys"{return EvalResult::System;}


        if let Some(scp) = scp_opt
        {
            if let Some(lsym) = scp.find(s)
            {
                match lsym.get_kind()
                {
              LocalSymbolKind::Const=>{return EvalResult::Const(lsym.get_value());}
              _                     =>{return EvalResult::Undef("");}
                }
            }
        }


        if let Some(decl) = set.find(s)
        {
          return match decl.get_kind()
            {
          DeclKind::Const(_,i)=>{return EvalResult::Const(*i);}
          _                   =>{return EvalResult::Undef("");}
            };
        }


      EvalResult::Err(srcinf.to_error(format!("{} is not found",s)))
    }
  ExprKind::String(s)=>{EvalResult::String(s.clone())}
  ExprKind::Int(i)   =>{EvalResult::Const(*i)}
  ExprKind::CallOp(f,args)=>
    {
      let  mut buf = Vec::<EvalResult>::new();

        for arg_e in args
        {
          let  res = evaluate_const(arg_e,set,scp_opt);

            match res
            {
          EvalResult::Undef(_)=>{return EvalResult::Undef("");}
          EvalResult::Err(e)=>{return EvalResult::Err(e);}
          _=>{buf.push(res);}
            }
        }


        match evaluate_const(f,set,scp_opt)
        {
      EvalResult::Undef(_)=>{EvalResult::Undef("")}
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _                 =>{EvalResult::Undef("")}
        }
    }
  ExprKind::AccessOp(ins,s)=>
    {
        match evaluate_const(ins,set,scp_opt)
        {
      EvalResult::System=>
        {
               if s == "spawn"{EvalResult::SystemMember(s.clone())}
          else if s == "print"{EvalResult::SystemMember(s.clone())}
          else if s ==    "id"{EvalResult::SystemMember(s.clone())}
          else if s == "input"{EvalResult::SystemMember(s.clone())}
          else if s == "timer"{EvalResult::SystemMember(s.clone())}
          else                {EvalResult::Err(srcinf.to_error(format!("{} is not found in sys",s)))}
        }
      _=>{EvalResult::Undef("")}
        }
    }
  ExprKind::Expr(e)         =>{evaluate_const(e,set,scp_opt)}
  ExprKind::UnaryOp(o,op)   =>{evaluate_unary_const(o,op,set,scp_opt)}
  ExprKind::BinaryOp(l,r,op)=>{evaluate_binary_const(l,r,op,set,scp_opt)}
    }
}




