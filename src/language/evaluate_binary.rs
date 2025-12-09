

use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::opcode::*;
use super::evaluate::*;
use super::execute::*;




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
evaluate_int_and_uint(l: i64, r: u64, op: &str)-> EvalResult
{
    if let Ok(i) = i64::try_from(r)
    {
      return evaluate_int(l,i,op);
    }


  EvalResult::Err
}


pub fn
evaluate_uint(l: u64, r: u64, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::Uint(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::Uint(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::Uint(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::Uint(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::Uint(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::Uint(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::Uint(u);}}
  else if op == "&"{return EvalResult::Uint(l&r);}
  else if op == "|"{return EvalResult::Uint(l|r);}
  else if op == "^"{return EvalResult::Uint(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_uint_and_int(l: u64, r: i64, op: &str)-> EvalResult
{
    if let Ok(u) = u64::try_from(r)
    {
      return evaluate_uint(l,u,op);
    }


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


pub fn
evaluate_bool(l: bool, r: bool, op: &str)-> EvalResult
{
       if op == "&&"{return EvalResult::Bool(l && r);}
  else if op == "||"{return EvalResult::Bool(l || r);}


  EvalResult::Err
}


pub fn
evaluate_i8(l: i8, r: i8, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::I8(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::I8(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::I8(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::I8(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::I8(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::I8(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::I8(i);}}
  else if op == "&"{return EvalResult::I8(l&r);}
  else if op == "|"{return EvalResult::I8(l|r);}
  else if op == "^"{return EvalResult::I8(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_i16(l: i16, r: i16, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::I16(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::I16(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::I16(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::I16(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::I16(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::I16(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::I16(i);}}
  else if op == "&"{return EvalResult::I16(l&r);}
  else if op == "|"{return EvalResult::I16(l|r);}
  else if op == "^"{return EvalResult::I16(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_i32(l: i32, r: i32, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::I32(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::I32(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::I32(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::I32(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::I32(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::I32(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::I32(i);}}
  else if op == "&"{return EvalResult::I32(l&r);}
  else if op == "|"{return EvalResult::I32(l|r);}
  else if op == "^"{return EvalResult::I32(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_i64(l: i64, r: i64, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::I64(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::I64(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::I64(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::I64(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::I64(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::I64(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::I64(i);}}
  else if op == "&"{return EvalResult::I64(l&r);}
  else if op == "|"{return EvalResult::I64(l|r);}
  else if op == "^"{return EvalResult::I64(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_isize(l: isize, r: isize, op: &str)-> EvalResult
{
       if op == "+"{if let Some(i) = l.checked_add(r){return EvalResult::ISize(i);}}
  else if op == "-"{if let Some(i) = l.checked_sub(r){return EvalResult::ISize(i);}}
  else if op == "*"{if let Some(i) = l.checked_mul(r){return EvalResult::ISize(i);}}
  else if op == "/"{if let Some(i) = l.checked_div(r){return EvalResult::ISize(i);}}
  else if op == "%"{if let Some(i) = l.checked_rem(r){return EvalResult::ISize(i);}}
  else if op == "<<"{if let Some(i) = l.checked_shl(r as u32){return EvalResult::ISize(i);}}
  else if op == ">>"{if let Some(i) = l.checked_shr(r as u32){return EvalResult::ISize(i);}}
  else if op == "&"{return EvalResult::ISize(l&r);}
  else if op == "|"{return EvalResult::ISize(l|r);}
  else if op == "^"{return EvalResult::ISize(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_u8(l: u8, r: u8, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::U8(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::U8(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::U8(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::U8(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::U8(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::U8(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::U8(u);}}
  else if op == "&"{return EvalResult::U8(l&r);}
  else if op == "|"{return EvalResult::U8(l|r);}
  else if op == "^"{return EvalResult::U8(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_u16(l: u16, r: u16, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::U16(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::U16(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::U16(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::U16(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::U16(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::U16(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::U16(u);}}
  else if op == "&"{return EvalResult::U16(l&r);}
  else if op == "|"{return EvalResult::U16(l|r);}
  else if op == "^"{return EvalResult::U16(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_u32(l: u32, r: u32, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::U32(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::U32(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::U32(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::U32(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::U32(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::U32(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::U32(u);}}
  else if op == "&"{return EvalResult::U32(l&r);}
  else if op == "|"{return EvalResult::U32(l|r);}
  else if op == "^"{return EvalResult::U32(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_u64(l: u64, r: u64, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::U64(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::U64(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::U64(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::U64(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::U64(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::U64(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::U64(u);}}
  else if op == "&"{return EvalResult::U64(l&r);}
  else if op == "|"{return EvalResult::U64(l|r);}
  else if op == "^"{return EvalResult::U64(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_usize(l: usize, r: usize, op: &str)-> EvalResult
{
       if op == "+"{if let Some(u) = l.checked_add(r){return EvalResult::USize(u);}}
  else if op == "-"{if let Some(u) = l.checked_sub(r){return EvalResult::USize(u);}}
  else if op == "*"{if let Some(u) = l.checked_mul(r){return EvalResult::USize(u);}}
  else if op == "/"{if let Some(u) = l.checked_div(r){return EvalResult::USize(u);}}
  else if op == "%"{if let Some(u) = l.checked_rem(r){return EvalResult::USize(u);}}
  else if op == "<<"{if let Some(u) = l.checked_shl(r as u32){return EvalResult::USize(u);}}
  else if op == ">>"{if let Some(u) = l.checked_shr(r as u32){return EvalResult::USize(u);}}
  else if op == "&"{return EvalResult::USize(l&r);}
  else if op == "|"{return EvalResult::USize(l|r);}
  else if op == "^"{return EvalResult::USize(l^r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_f32(l: f32, r: f32, op: &str)-> EvalResult
{
       if op == "+"{return EvalResult::F32(l+r);}
  else if op == "-"{return EvalResult::F32(l-r);}
  else if op == "*"{return EvalResult::F32(l*r);}
  else if op == "/"{return EvalResult::F32(l/r);}
  else if op == "%"{return EvalResult::F32(l%r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}


pub fn
evaluate_f64(l: f64, r: f64, op: &str)-> EvalResult
{
       if op == "+"{return EvalResult::F64(l+r);}
  else if op == "-"{return EvalResult::F64(l-r);}
  else if op == "*"{return EvalResult::F64(l*r);}
  else if op == "/"{return EvalResult::F64(l/r);}
  else if op == "%"{return EvalResult::F64(l%r);}
  else if op == "=="{return EvalResult::Bool(l == r);}
  else if op == "!="{return EvalResult::Bool(l != r);}
  else if op == "<" {return EvalResult::Bool(l <  r);}
  else if op == "<="{return EvalResult::Bool(l <= r);}
  else if op == ">" {return EvalResult::Bool(l >  r);}
  else if op == ">="{return EvalResult::Bool(l >= r);}


  EvalResult::Err
}




fn
are_operatable(l: &SizedTy, r: &SizedTy)-> bool
{
    (l.is_bool() && r.is_bool())
  ||(l.is_i8()    && r.is_i8())
  ||(l.is_i16()   && r.is_i16())
  ||(l.is_i32()   && r.is_i32())
  ||(l.is_i64()   && r.is_i64())
  ||(l.is_isize() && r.is_isize())
  ||(l.is_u8()    && r.is_u8())
  ||(l.is_u16()   && r.is_u16())
  ||(l.is_u32()   && r.is_u32())
  ||(l.is_u64()   && r.is_u64())
  ||(l.is_usize() && r.is_usize())
  ||(l.is_f32()   && r.is_f32())
  ||(l.is_f64()   && r.is_f64())
}


pub fn
evaluate_value(mut lo: Opdata, mut ro: Opdata, op: &str)-> EvalResult
{
    if !are_operatable(lo.get_ty(),ro.get_ty())
    {
      return EvalResult::Err;
    }


  lo.get_opcode_list_mut().append(&mut ro.get_opcode_list_mut());

  let  ty = lo.get_ty();

  let  mut opco = Opcode::Nop;

    if op == "+"
    {
      opco = if ty.is_int()  {Opcode::Addi}
        else if ty.is_uint() {Opcode::Addu}
        else if ty.is_float(){Opcode::Addf}
        else{Opcode::Nop};
    }

  else
    if op == "-"
    {
      opco = if ty.is_int()  {Opcode::Subi}
        else if ty.is_uint() {Opcode::Subu}
        else if ty.is_float(){Opcode::Subf}
        else{Opcode::Nop};
    }

  else
    if op == "*"
    {
      opco = if ty.is_int()  {Opcode::Muli}
        else if ty.is_uint() {Opcode::Mulu}
        else if ty.is_float(){Opcode::Mulf}
        else{Opcode::Nop};
    }

  else
    if op == "/"
    {
      opco = if ty.is_int()  {Opcode::Divi}
        else if ty.is_uint() {Opcode::Divu}
        else if ty.is_float(){Opcode::Divf}
        else{Opcode::Nop};
    }

  else
    if op == "%"
    {
      opco = if ty.is_int()  {Opcode::Remi}
        else if ty.is_uint() {Opcode::Remu}
        else if ty.is_float(){Opcode::Remf}
        else{Opcode::Nop};
    }

  else
    if op == "<<"
    {
      opco = if ty.is_int() && ty.is_uint(){Opcode::Shl}
        else{Opcode::Nop};
    }

  else
    if op == ">>"
    {
      opco = if ty.is_int() && ty.is_uint(){Opcode::Shr}
        else{Opcode::Nop};
    }

  else
    if op == "&"
    {
      opco = if ty.is_int() && ty.is_uint(){Opcode::And}
        else{Opcode::Nop};
    }

  else
    if op == "|"
    {
      opco = if ty.is_int() && ty.is_uint(){Opcode::Or}
        else{Opcode::Nop};
    }

  else
    if op == "^"
    {
      opco = if ty.is_int() && ty.is_uint(){Opcode::Xor}
        else{Opcode::Nop};
    }

  else
    if op == "=="
    {
      opco = if ty.is_int()  {Opcode::Eq}
        else if ty.is_uint() {Opcode::Eq}
        else if ty.is_float(){Opcode::Eqf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == "!="
    {
      opco = if ty.is_int()  {Opcode::Neq}
        else if ty.is_uint() {Opcode::Neq}
        else if ty.is_float(){Opcode::Neqf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == "<"
    {
      opco = if ty.is_int()  {Opcode::Lti}
        else if ty.is_uint() {Opcode::Ltu}
        else if ty.is_float(){Opcode::Ltf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == "<="
    {
      opco = if ty.is_int()  {Opcode::Lteqi}
        else if ty.is_uint() {Opcode::Ltequ}
        else if ty.is_float(){Opcode::Lteqf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == ">"
    {
      opco = if ty.is_int()  {Opcode::Gti}
        else if ty.is_uint() {Opcode::Gtu}
        else if ty.is_float(){Opcode::Gtf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == ">="
    {
      opco = if ty.is_int()  {Opcode::Gteqi}
        else if ty.is_uint() {Opcode::Gtequ}
        else if ty.is_float(){Opcode::Gteqf}
        else{Opcode::Nop};

      lo.set_ty(SizedTy::Bool);
    }

  else
    if op == "&&"
    {
      opco = if ty.is_bool(){Opcode::Andl}
        else{Opcode::Nop};
    }

  else
    if op == "||"
    {
      opco = if ty.is_bool(){Opcode::Orl}
        else{Opcode::Nop};
    }


    if let Opcode::Nop = opco
    {
      return EvalResult::Err;
    }


  lo.get_opcode_list_mut().push(opco);

  EvalResult::Value(lo)
}




pub fn
evaluate_int_and_value(li: i64, rv: Opdata, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  SizedTy::I8=>
    {
        if let Ok(i) = i8::try_from(li){evaluate_value(Opdata::from(i),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::I16=>
    {
        if let Ok(i) = i16::try_from(li){evaluate_value(Opdata::from(i),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::I32=>
    {
        if let Ok(i) = i32::try_from(li){evaluate_value(Opdata::from(i),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::I64=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  SizedTy::ISize=>
    {
      evaluate_value(Opdata::from(li as isize),rv,op)
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
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_int_and_value(li,rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
      evaluate_int(li,ri,op)
    }
  EvalResult::Uint(ru)=>
    {
        if li < 0
        {
            if let Ok(i) = i64::try_from(ru){evaluate_int(li,i,op)}
          else{EvalResult::Err}
        }

      else
        {
          evaluate_int(li,ru as i64,op)
        }
    }
  EvalResult::I8(ri)=>
    {
        if let Ok(i) = i8::try_from(li){evaluate_i8(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I16(ri)=>
    {
        if let Ok(i) = i16::try_from(li){evaluate_i16(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I32(ri)=>
    {
        if let Ok(i) = i32::try_from(li){evaluate_i32(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I64(ri)=>
    {
      evaluate_i64(li,ri,op)
    }
  EvalResult::ISize(ri)=>
    {
      evaluate_isize(li as isize,ri,op)
    }
  EvalResult::U8(ru)=>
    {
        if let Ok(u) = u8::try_from(li){evaluate_u8(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U16(ru)=>
    {
        if let Ok(u) = u16::try_from(li){evaluate_u16(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U32(ru)=>
    {
        if let Ok(u) = u32::try_from(li){evaluate_u32(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U64(ru)=>
    {
        if let Ok(u) = u64::try_from(li){evaluate_u64(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::USize(ru)=>
    {
        if let Ok(u) = usize::try_from(li){evaluate_usize(u,ru,op)}
      else{EvalResult::Err}
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_uint_and_value(lu: u64, rv: Opdata, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  SizedTy::U8=>
    {
        if let Ok(u) = u8::try_from(lu){evaluate_value(Opdata::from(u),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::U16=>
    {
        if let Ok(u) = u16::try_from(lu){evaluate_value(Opdata::from(u),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::U32=>
    {
        if let Ok(u) = u32::try_from(lu){evaluate_value(Opdata::from(u),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::U64=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  SizedTy::USize=>
    {
      evaluate_value(Opdata::from(lu as usize),rv,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_uint_and(lu: u64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_uint_and_value(lu,rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_uint_and_value(lu,rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if ri < 0
        {
            if let Ok(i) = i64::try_from(lu){evaluate_int(i,ri,op)}
          else{EvalResult::Err}
        }

      else
        {
          evaluate_uint(lu,ri as u64,op)
        }
    }
  EvalResult::Uint(ru)=>
    {
      evaluate_uint(lu,ru,op)
    }
  EvalResult::I8(ri)=>
    {
        if let Ok(i) = i8::try_from(lu){evaluate_i8(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I16(ri)=>
    {
        if let Ok(i) = i16::try_from(lu){evaluate_i16(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I32(ri)=>
    {
        if let Ok(i) = i32::try_from(lu){evaluate_i32(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::I64(ri)=>
    {
        if let Ok(i) = i64::try_from(lu){evaluate_i64(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::ISize(ri)=>
    {
        if let Ok(i) = isize::try_from(lu){evaluate_isize(i,ri,op)}
      else{EvalResult::Err}
    }
  EvalResult::U8(ru)=>
    {
        if let Ok(u) = u8::try_from(lu){evaluate_u8(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U16(ru)=>
    {
        if let Ok(u) = u16::try_from(lu){evaluate_u16(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U32(ru)=>
    {
        if let Ok(u) = u32::try_from(lu){evaluate_u32(u,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::U64(ru)=>
    {
      evaluate_u64(lu,ru,op)
    }
  EvalResult::USize(ru)=>
    {
      evaluate_usize(lu as usize,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_float_and_value(lf: f64, rv: Opdata, op: &str)-> EvalResult
{
    match rv.get_ty()
    {
  SizedTy::F32=>
    {
        if let Ok(f) = to_f32_from_f64(lf){evaluate_value(Opdata::from(f),rv,op)}
      else{EvalResult::Err}
    }
  SizedTy::F64=>
    {
      evaluate_value(Opdata::from(lf),rv,op)
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
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_float_and_value(lf,rv,op)
    }
  EvalResult::Float(rf)=>
    {
      evaluate_float(lf,rf,op)
    }
  EvalResult::F32(rf)=>
    {
        if let Ok(f) = to_f32_from_f64(lf){evaluate_f32(f,rf,op)}
      else{EvalResult::Err}
    }
  EvalResult::F64(rf)=>
    {
      evaluate_f64(lf,rf,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_i8_and(li: i8, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(i) = i8::try_from(ri){evaluate_i8(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(i) = i8::try_from(ru){evaluate_i8(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::I8(ri)=>
    {
      evaluate_i8(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_i16_and(li: i16, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(i) = i16::try_from(ri){evaluate_i16(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(i) = i16::try_from(ru){evaluate_i16(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::I16(ri)=>
    {
      evaluate_i16(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_i32_and(li: i32, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(i) = i32::try_from(ri){evaluate_i32(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(i) = i32::try_from(ru){evaluate_i32(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::I32(ri)=>
    {
      evaluate_i32(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_i64_and(li: i64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
      evaluate_i64(li,ri,op)
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(i) = i64::try_from(ru){evaluate_i64(li,i,op)}
      else{EvalResult::Err}
    }
  EvalResult::I64(ri)=>
    {
      evaluate_i64(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_isize_and(li: isize, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(li),rv,op)
    }
  EvalResult::Int(ri)=>
    {
      evaluate_isize(li,ri as isize,op)
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(i) = i64::try_from(ru){evaluate_isize(li,i as isize,op)}
      else{EvalResult::Err}
    }
  EvalResult::ISize(ri)=>
    {
      evaluate_isize(li,ri,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_u8_and(lu: u8, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(u) = u8::try_from(ri){evaluate_u8(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(u) = u8::try_from(ru){evaluate_u8(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::U8(ru)=>
    {
      evaluate_u8(lu,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_u16_and(lu: u16, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(u) = u16::try_from(ri){evaluate_u16(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(u) = u16::try_from(ru){evaluate_u16(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::U16(ru)=>
    {
      evaluate_u16(lu,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_u32_and(lu: u32, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(u) = u32::try_from(ri){evaluate_u32(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
        if let Ok(u) = u32::try_from(ru){evaluate_u32(lu,u,op)}
      else{EvalResult::Err}
    }
  EvalResult::U32(ru)=>
    {
      evaluate_u32(lu,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_u64_and(lu: u64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(ru) = u64::try_from(ri){evaluate_u64(lu,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
      evaluate_u64(lu,ru as u64,op)
    }
  EvalResult::U64(ru)=>
    {
      evaluate_u64(lu,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_usize_and(lu: usize, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lu),rv,op)
    }
  EvalResult::Int(ri)=>
    {
        if let Ok(ru) = usize::try_from(ri){evaluate_usize(lu,ru,op)}
      else{EvalResult::Err}
    }
  EvalResult::Uint(ru)=>
    {
      evaluate_usize(lu,ru as usize,op)
    }
  EvalResult::USize(ru)=>
    {
      evaluate_usize(lu,ru,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_f32_and(lf: f32, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lf),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lf),rv,op)
    }
  EvalResult::Float(rf)=>
    {
        if let Ok(f) = to_f32_from_f64(rf){evaluate_f32(lf,f,op)}
      else{EvalResult::Err}
    }
  EvalResult::F32(rf)=>
    {
      evaluate_f32(lf,rf,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_f64_and(lf: f64, r: EvalResult, op: &str)-> EvalResult
{
    match r
    {
  EvalResult::Value(rv)=>
    {
      evaluate_value(Opdata::from(lf),rv,op)
    }
  EvalResult::Dereference(mut rv)=>
    {
      rv.add_ld();

      evaluate_value(Opdata::from(lf),rv,op)
    }
  EvalResult::Float(rf)=>
    {
      evaluate_f64(lf,rf,op)
    }
  EvalResult::F64(rf)=>
    {
      evaluate_f64(lf,rf,op)
    }
  _=>{EvalResult::Err}
    }
}


pub fn
evaluate_binary(le: &Expr, re: &Expr, op: &str, ctx: &ExecContext)-> EvalResult
{
  let  l = evaluate(le,ctx);
  let  r = evaluate(re,ctx);

    match l
    {
  EvalResult::Value(lv)=>
    {
        if let Ok(rv) = Opdata::try_from(r){evaluate_value(lv,rv,op)}
      else{EvalResult::Err}
    }
  EvalResult::Dereference(mut lv)=>
    {
      lv.add_ld();

        if let Ok(rv) = Opdata::try_from(r){evaluate_value(lv,rv,op)}
      else{EvalResult::Err}
    }
  EvalResult::Int(li)  =>{evaluate_int_and(li,r,op)}
  EvalResult::Uint(lu) =>{evaluate_uint_and(lu,r,op)}
  EvalResult::Float(lf)=>{evaluate_float_and(lf,r,op)}
  EvalResult::Bool(lb)=>
    {
        match r
        {
      EvalResult::Value(rv)=>
        {
          EvalResult::Err
        }
      EvalResult::Bool(rb)=>
        {
          evaluate_bool(lb,rb,op)
        }
      _=>{EvalResult::Err}
        }
    }
  EvalResult::I8(li)   =>{evaluate_i8_and(li,r,op)}
  EvalResult::I16(li)  =>{evaluate_i16_and(li,r,op)}
  EvalResult::I32(li)  =>{evaluate_i32_and(li,r,op)}
  EvalResult::I64(li)  =>{evaluate_i64_and(li,r,op)}
  EvalResult::ISize(li)=>{evaluate_isize_and(li,r,op)}
  EvalResult::U8(lu)   =>{evaluate_u8_and(lu,r,op)}
  EvalResult::U16(lu)  =>{evaluate_u16_and(lu,r,op)}
  EvalResult::U32(lu)  =>{evaluate_u32_and(lu,r,op)}
  EvalResult::U64(lu)  =>{evaluate_u64_and(lu,r,op)}
  EvalResult::USize(lu)=>{evaluate_usize_and(lu,r,op)}
  EvalResult::F32(lf)=>{evaluate_f32_and(lf,r,op)}
  EvalResult::F64(lf)=>{evaluate_f64_and(lf,r,op)}
  _=>{EvalResult::Err}
    }
}




