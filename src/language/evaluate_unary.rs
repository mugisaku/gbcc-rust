

use super::scope::*;
use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::asm::*;
use super::evaluate::*;


pub fn
evaluate_value(mut val: ValueProcess, op: &str)-> EvalResult
{
  let  mut opco = Opcode::Nop;
  let  ty = val.get_ty();

    if op == "-"
    {
      opco = if ty.is_int()  {Opcode::Neg}
//        else if ty.is_uint() {Opcode::Neg}
        else if ty.is_float(){Opcode::Negf}
        else{Opcode::Nop};
    }

  else
    if op == "!"
    {
      opco = if ty.is_int() {Opcode::Not}
//        else if ty.is_uint(){Opcode::Not}
        else if ty.is_bool(){Opcode::Notl}
        else{Opcode::Nop};
    }
/*
  else
    if op == "*"
    {
      return if ty.is_reference(){EvalResult::Dereference(val)}
        else if ty.is_pointer()  {EvalResult::Dereference(val)}
        else{EvalResult::Err};
    }
*/


    if let Opcode::Nop = opco
    {
      return EvalResult::Err;
    }


  val.get_table_mut().push_opcode(opco);

  EvalResult::Value(val)
}




pub fn
evaluate_bool(v: bool, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::Bool(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_int(i: i64, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::Int(-i)}
  else if op == "!"{EvalResult::Int(!i)}
  else{EvalResult::Err}
}


pub fn
evaluate_float(f: f64, op: &str)-> EvalResult
{
    if op == "-"{EvalResult::Float(-f)}
  else{EvalResult::Err}
}


pub fn
evaluate_unary(o: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  res = evaluate(o,tbl,scp_opt);

    if op == "&"
    {
      return /*if let EvalResult::Deref(v) = res{EvalResult::Value(v.to_reference())}
        else{*/EvalResult::Err;//}
    }


    match res
    {
  EvalResult::Value(v)          =>{               evaluate_value(v,op)}
  EvalResult::Deref(_,_)=>{if let Ok(v) = ValueProcess::try_from(res){evaluate_value(v,op)} else{EvalResult::Err}}
  EvalResult::Bool(b) =>{evaluate_bool( b,op)}
  EvalResult::Int(i)  =>{evaluate_int(  i,op)}
  EvalResult::Float(f)=>{evaluate_float(f,op)}
  _=>{EvalResult::Err}
    }
}




