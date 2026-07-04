

use super::*;
use super::expr::*;
use super::scope::*;
use super::symbol_table::*;
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
evaluate_unary_const(e: &Expr, op: &str, symtbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = e.get_source_info();

    match evaluate_const(e,symtbl,scp_opt)
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
evaluate_binary_const(le: &Expr, re: &Expr, op: &str, symtbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = le.get_source_info();

    match evaluate_const(le,symtbl,scp_opt)
    {
  EvalResult::Const(lv)=>
    {
        match evaluate_const(re,symtbl,scp_opt)
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
evaluate_const(e: &Expr, symtbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
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


        if let Some(sym) = symtbl.find_symbol(s)
        {
          return match sym.get_kind()
            {
          SymbolKind::Const(v)=>{return EvalResult::Const(*v);}
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
          let  res = evaluate_const(arg_e,symtbl,scp_opt);

            match res
            {
          EvalResult::Undef(_)=>{return EvalResult::Undef("");}
          EvalResult::Err(e)=>{return EvalResult::Err(e);}
          _=>{buf.push(res);}
            }
        }


        match evaluate_const(f,symtbl,scp_opt)
        {
      EvalResult::Undef(_)=>{EvalResult::Undef("")}
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _                 =>{EvalResult::Undef("")}
        }
    }
  ExprKind::AccessOp(ins,s)=>
    {
        match evaluate_const(ins,symtbl,scp_opt)
        {
      EvalResult::System=>
        {
               if s == "spawn"{EvalResult::Spawn}
          else if s == "print"{EvalResult::Print}
          else                {EvalResult::Err(srcinf.to_error(format!("{} is not found in sys",s)))}
        }
      _=>{EvalResult::Undef("")}
        }
    }
  ExprKind::Expr(e)         =>{evaluate_const(e,symtbl,scp_opt)}
  ExprKind::UnaryOp(o,op)   =>{evaluate_unary_const(o,op,symtbl,scp_opt)}
  ExprKind::BinaryOp(l,r,op)=>{evaluate_binary_const(l,r,op,symtbl,scp_opt)}
    }
}




