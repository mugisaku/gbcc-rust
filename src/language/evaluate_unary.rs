

use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::opcode::*;
use super::evaluate::*;
use super::execute::*;


pub fn
evaluate_value(mut val: Opdata, op: &str)-> EvalResult
{
  let  mut opco = Opcode::Nop;
  let  ty = val.get_ty();

    if op == "-"
    {
      opco = if ty.is_int()  {Opcode::Neg}
        else if ty.is_uint() {Opcode::Neg}
        else if ty.is_float(){Opcode::Negf}
        else{Opcode::Nop};
    }

  else
    if op == "!"
    {
      opco = if ty.is_int() {Opcode::Not}
        else if ty.is_uint(){Opcode::Not}
        else if ty.is_bool(){Opcode::Notl}
        else{Opcode::Nop};
    }

  else
    if op == "*"
    {
      return if ty.is_reference(){EvalResult::Dereference(val)}
        else if ty.is_pointer()  {EvalResult::Dereference(val)}
        else{EvalResult::Err};
    }


    if let Opcode::Nop = opco
    {
      return EvalResult::Err;
    }


  val.get_opcode_list_mut().push(opco);

  EvalResult::Value(val)
}


pub fn
evaluate_int(i: i64, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::Int(-i)}
  else if op == "!"{EvalResult::Int(!i)}
  else{EvalResult::Err}
}


pub fn
evaluate_uint(u: u64, op: &str)-> EvalResult
{
    if op == "-"
    {
        if let Ok(i) = i64::try_from(u){EvalResult::Int(-i)}
      else{EvalResult::Err}
    }

  else
    if op == "!"
    {
      EvalResult::Uint(!u)
    }

  else
    {
      EvalResult::Err
    }
}


pub fn
evaluate_float(f: f64, op: &str)-> EvalResult
{
    if op == "-"{EvalResult::Float(-f)}
  else{EvalResult::Err}
}


pub fn
evaluate_bool(v: bool, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::Bool(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_i8(v: i8, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::I8(-v)}
  else if op == "!"{EvalResult::I8(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_i16(v: i16, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::I16(-v)}
  else if op == "!"{EvalResult::I16(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_i32(v: i32, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::I32(-v)}
  else if op == "!"{EvalResult::I32(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_i64(v: i64, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::I64(-v)}
  else if op == "!"{EvalResult::I64(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_isize(v: isize, op: &str)-> EvalResult
{
       if op == "-"{EvalResult::ISize(-v)}
  else if op == "!"{EvalResult::ISize(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_u8(v: u8, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::U8(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_u16(v: u16, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::U16(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_u32(v: u32, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::U32(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_u64(v: u64, op: &str)-> EvalResult
{
    if op == "!"{EvalResult::U64(!v)}
  else{EvalResult::Err}
}


pub fn
evaluate_usize(v: usize, op: &str)-> EvalResult
{
   if op == "!"{EvalResult::USize(!v)}
  else{EvalResult::Err}
}




pub fn
evaluate_f32(v: f32, op: &str)-> EvalResult
{
    if op == "-"{return EvalResult::F32(-v)}
  else{EvalResult::Err}
}


pub fn
evaluate_f64(v: f64, op: &str)-> EvalResult
{
    if op == "-"{return EvalResult::F64(-v)}
  else{EvalResult::Err}
}


pub fn
evaluate_unary(o: &Expr, op: &str, ctx: &ExecContext)-> EvalResult
{
  let  res = evaluate(o,ctx);

    if op == "&"
    {
      return if let EvalResult::Dereference(v) = res{EvalResult::Value(v.to_reference())}
        else{EvalResult::Err}
    }


    match res
    {
  EvalResult::Value(v)          =>{               evaluate_value(v,op)}
  EvalResult::Dereference(mut v)=>{  v.add_ld();  evaluate_value(v,op)}
  EvalResult::Int(i)  =>{evaluate_int(  i,op)}
  EvalResult::Uint(u) =>{evaluate_uint( u,op)}
  EvalResult::Float(f)=>{evaluate_float(f,op)}
  EvalResult::I8(i)   =>{evaluate_i8(i,op)}
  EvalResult::I16(i)  =>{evaluate_i16(i,op)}
  EvalResult::I32(i)  =>{evaluate_i32(i,op)}
  EvalResult::I64(i)  =>{evaluate_i64(i,op)}
  EvalResult::ISize(i)=>{evaluate_isize(i,op)}
  EvalResult::U8(u)   =>{evaluate_u8(u,op)}
  EvalResult::U16(u)  =>{evaluate_u16(u,op)}
  EvalResult::U32(u)  =>{evaluate_u32(u,op)}
  EvalResult::U64(u)  =>{evaluate_u64(u,op)}
  EvalResult::USize(u)=>{evaluate_usize(u,op)}
  EvalResult::F32(f)=>{evaluate_f32(f,op)}
  EvalResult::F64(f)=>{evaluate_f64(f,op)}
  _=>{EvalResult::Err}
    }
}




