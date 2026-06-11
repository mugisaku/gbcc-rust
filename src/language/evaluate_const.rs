

use std::rc::Rc;

use super::*;
use super::expr::*;
use super::scope::*;
use super::symbol_table::*;




pub fn
evaluate_call_const(v: i64, args: Vec<i64>)-> Result<i64,()>
{
  Err(())
}


pub fn
evaluate_unary_const(v: i64, op: &str)-> Result<i64,()>
{
    match op
    {
  (s) if s == "!"=>{Ok(!v)}
  (s) if s == "-"=>{Ok(-v)}
  _=>{Err(())}
    }
}


pub fn
evaluate_binary_const(l: i64, r: i64, op: &str)-> Result<i64,()>
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
  _=>{Err(())}
    }
}


pub fn
evaluate_const(e: &Expr, symtbl: &SymbolTable, scp_opt: Option<&Scope>)-> Result<i64,()>
{
    match e
    {
  Expr::Identifier(s)=>
    {
        if let Some(scp) = scp_opt
        {
            if let Some(lsym) = scp.find(s)
            {
                if let LocalSymbolKind::Const = lsym.get_kind()
                {
                  return Ok(lsym.get_value());
                }


              return Err(());
            }
        }


        if let Some(sym) = symtbl.find_symbol(s)
        {
          return match sym.get_kind()
            {
          SymbolKind::Const(v)=>{Ok(*v)}
          _=>{Err(())}
            };
        }


      Err(())
    }
  Expr::Int(i)=>{Ok(*i)}
  Expr::CallOp(f,args)=>
    {
      let  mut buf = Vec::<i64>::new();

        for arg_e in args
        {
          let  res = evaluate_const(arg_e,symtbl,scp_opt);

            if res.is_err()
            {
              return Err(());
            }


          buf.push(res.unwrap());
        }


        if let Ok(f_i) = evaluate_const(f,symtbl,scp_opt)
        {
          evaluate_call_const(f_i,buf)
        }

      else
        {
          Err(())
        }
    }
  Expr::AccessOp(ins,s)=>
    {
      Err(())
    }
  Expr::Expr(e)=>{evaluate_const(e,symtbl,scp_opt)}
  Expr::UnaryOp(o,op)=>
    {
        if let Ok(i) = evaluate_const(o,symtbl,scp_opt)
        {
          evaluate_unary_const(i,op)
        }

      else
        {
          Err(())
        }
    }
  Expr::BinaryOp(l,r,op)=>
    {
      let  l_res = evaluate_const(l,symtbl,scp_opt);
      let  r_res = evaluate_const(r,symtbl,scp_opt);

        if let Ok(l) = l_res{
        if let Ok(r) = r_res{
          return evaluate_binary_const(l,r,op)
        }}


      Err(())
    }
    }
}




