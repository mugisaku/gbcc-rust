

use super::scope::*;
use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::asm::*;
use super::evaluate::*;




pub fn
evaluate_bool(l: bool, r: bool, op: &str)-> EvalResult
{
       if op == "&&"{return EvalResult::Bool(l && r);}
  else if op == "||"{return EvalResult::Bool(l || r);}


  EvalResult::Err
}


pub fn
evaluate_int(l: i64, r: i64, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::Int(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::Int(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::Int(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::Int(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::Int(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::Int(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::Int(i);}}
  else if op == "&"{return EvalResult::Int(l&r);}
  else if op == "|"{return EvalResult::Int(l|r);}
  else if op == "^"{return EvalResult::Int(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_float(l: f64, r: f64, op: &str)-> EvalResult
{
       if op == "+"{return EvalResult::Float(l+r);}
  else if op == "-"{return EvalResult::Float(l-r);}
  else if op == "*"{return EvalResult::Float(l*r);}
  else if op == "/"{return EvalResult::Float(l/r);}
  else if op == "%"{return EvalResult::Float(l%r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


fn
are_operatable(l: &Ty, r: &Ty)-> bool
{
    (l.is_bool()  && r.is_bool())
  ||(l.is_int()   && r.is_int())
  ||(l.is_float() && r.is_float())
}


pub fn
evaluate_value(mut lo: ValueProcess, mut ro: ValueProcess, op: &str)-> EvalResult
{
    if !are_operatable(lo.get_ty(),ro.get_ty())
    {
      return EvalResult::Err;
    }


  lo.get_table_mut().push_table(ro.get_table_mut());

  let  ty = lo.get_ty();

  let  mut opco = Opcode::Nop;

    if op == "+"
    {
      opco = if ty.is_int()  {Opcode::Addi}
//        else if ty.is_uint() {Opcode::Addu}
        else if ty.is_float(){Opcode::Addf}
        else{Opcode::Nop};
    }

  else
    if op == "-"
    {
      opco = if ty.is_int()  {Opcode::Subi}
//        else if ty.is_uint() {Opcode::Subu}
        else if ty.is_float(){Opcode::Subf}
        else{Opcode::Nop};
    }

  else
    if op == "*"
    {
      opco = if ty.is_int()  {Opcode::Muli}
//        else if ty.is_uint() {Opcode::Mulu}
        else if ty.is_float(){Opcode::Mulf}
        else{Opcode::Nop};
    }

  else
    if op == "/"
    {
      opco = if ty.is_int()  {Opcode::Divi}
//        else if ty.is_uint() {Opcode::Divu}
        else if ty.is_float(){Opcode::Divf}
        else{Opcode::Nop};
    }

  else
    if op == "%"
    {
      opco = if ty.is_int()  {Opcode::Remi}
//        else if ty.is_uint() {Opcode::Remu}
        else if ty.is_float(){Opcode::Remf}
        else{Opcode::Nop};
    }

  else
    if op == "<<"
    {
      opco = if ty.is_int(){Opcode::Shl}
        else{Opcode::Nop};
    }

  else
    if op == ">>"
    {
      opco = if ty.is_int(){Opcode::Shr}
        else{Opcode::Nop};
    }

  else
    if op == "&"
    {
      opco = if ty.is_int(){Opcode::And}
        else{Opcode::Nop};
    }

  else
    if op == "|"
    {
      opco = if ty.is_int(){Opcode::Or}
        else{Opcode::Nop};
    }

  else
    if op == "^"
    {
      opco = if ty.is_int(){Opcode::Xor}
        else{Opcode::Nop};
    }

  else
    if op == "=="
    {
      opco = if ty.is_int()  {Opcode::Eq}
//        else if ty.is_uint() {Opcode::Eq}
        else if ty.is_float(){Opcode::Eqf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == "!="
    {
      opco = if ty.is_int()  {Opcode::Neq}
//        else if ty.is_uint() {Opcode::Neq}
        else if ty.is_float(){Opcode::Neqf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == "<"
    {
      opco = if ty.is_int()  {Opcode::Lti}
//        else if ty.is_uint() {Opcode::Ltu}
        else if ty.is_float(){Opcode::Ltf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == "<="
    {
      opco = if ty.is_int()  {Opcode::Lteqi}
//        else if ty.is_uint() {Opcode::Ltequ}
        else if ty.is_float(){Opcode::Lteqf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == ">"
    {
      opco = if ty.is_int()  {Opcode::Gti}
//        else if ty.is_uint() {Opcode::Gtu}
        else if ty.is_float(){Opcode::Gtf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == ">="
    {
      opco = if ty.is_int()  {Opcode::Gteqi}
//        else if ty.is_uint() {Opcode::Gtequ}
        else if ty.is_float(){Opcode::Gteqf}
        else{Opcode::Nop};

      lo.set_ty(Ty::Bool);
    }

  else
    if op == "&&"
    {
      opco = Opcode::Andl;
    }

  else
    if op == "||"
    {
      opco = Opcode::Orl;
    }


    if let Opcode::Nop = opco
    {
      return EvalResult::Err;
    }


  lo.get_table_mut().push_opcode(opco);

  EvalResult::Value(lo)
}




pub fn
evaluate_bool_and_value(lb: bool, rv: ValueProcess, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  Ty::Bool=>
    {
      evaluate_value(ValueProcess::from(lb),rv,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_bool_and(lb: bool, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_bool_and_value(lb,rv,op)
    }
  EvalResult::Deref(_,_)=>
    {
      let  rv = ValueProcess::try_from(r).unwrap();

      evaluate_value(ValueProcess::from(lb),rv,op)
    }
  EvalResult::Bool(rb)=>
    {
      evaluate_bool(lb,rb,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_int_and_value(li: i64, rv: ValueProcess, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  Ty::Int=>
    {
      evaluate_value(ValueProcess::from(li),rv,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_int_and(li: i64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_int_and_value(li,rv,op)
    }
  EvalResult::Deref(_,_)=>
    {
      let  rv = ValueProcess::try_from(r).unwrap();

      evaluate_value(ValueProcess::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
      evaluate_int(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_float_and_value(lf: f64, rv: ValueProcess, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  Ty::Float=>
    {
      evaluate_value(ValueProcess::from(lf),rv,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_float_and(lf: f64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_float_and_value(lf,rv,op)
    }
  EvalResult::Deref(_,_)=>
    {
      let  rv = ValueProcess::try_from(r).unwrap();

      evaluate_float_and_value(lf,rv,op)
    }
  EvalResult::Float(rf)=>
    {
      evaluate_float(lf,rf,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_binary(le: &Expr, re: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  l = evaluate(le,tbl,scp_opt);
  let  r = evaluate(re,tbl,scp_opt);

    match l
    {
  EvalResult::Value(lv)=>
    {
        if let Ok(rv) = ValueProcess::try_from(r){evaluate_value(lv,rv,op)}
      else{EvalResult::Err}
    }
  EvalResult::Deref(_,_)=>
    {
      let  lv = ValueProcess::try_from(l).unwrap();

        if let Ok(rv) = ValueProcess::try_from(r){evaluate_value(lv,rv,op)}
      else{EvalResult::Err}
    }
  EvalResult::Bool(lb) =>{evaluate_bool_and( lb,r,op)}
  EvalResult::Int(li)  =>{evaluate_int_and(  li,r,op)}
  EvalResult::Float(lf)=>{evaluate_float_and(lf,r,op)}
  _=>{EvalResult::Err}
    }
}




