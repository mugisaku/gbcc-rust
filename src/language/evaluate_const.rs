

use std::rc::Rc;

use super::*;
use super::expr::*;
use super::ty::*;
use super::scope::*;
use super::symbol_table::*;




pub fn   to_i8(i: i64)-> i8{  i8::try_from(i).unwrap()}
pub fn  to_i16(i: i64)-> i16{i16::try_from(i).unwrap()}
pub fn  to_i32(i: i64)-> i32{i32::try_from(i).unwrap()}
pub fn  to_i64(u: u64)-> i64{i64::try_from(u).unwrap()}
pub fn   to_u8(u: u64)-> u8{  u8::try_from(u).unwrap()}
pub fn  to_u16(u: u64)-> u16{u16::try_from(u).unwrap()}
pub fn  to_u32(u: u64)-> u32{u32::try_from(u).unwrap()}
pub fn  to_f32(f: f64)-> f32{if f.abs() <= (f32::MAX as f64){f as f32} else{panic!();}}




#[derive(Clone)]
pub enum
EvalConstResult
{
  Type(String),

  NullPointer,

  Void,
  Bool(bool),

    Int(i64),
     I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
  ISize(isize),

   Uint(u64),
     U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
  USize(usize),

  Float(f64),
  F32(f32),
  F64(f64),

  String(String),

  Enumerator(String,String),

  Complex(String,Vec<Self>),

  Err,

}


impl
EvalConstResult
{


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


pub fn  to_i8_if_int(self)-> Self
{if let Self::Int(i) = self{Self::I8(to_i8(i))} else{self}}

pub fn  to_i16_if_int(self)-> Self
{if let Self::Int(i) = self{Self::I16(to_i16(i))} else{self}}

pub fn  to_i32_if_int(self)-> Self
{if let Self::Int(i) = self{Self::I32(to_i32(i))} else{self}}

pub fn  to_i64_if_int(self)-> Self
{if let Self::Int(i) = self{Self::I64(i)} else{self}}

pub fn  to_isize_if_int(self)-> Self
{if let Self::Int(i) = self{Self::ISize(i as isize)} else{self}}


pub fn  to_int_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::Int(i64::try_from(u).unwrap())} else{self}}

pub fn  to_u8_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::U8(to_u8(u))} else{self}}

pub fn  to_u16_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::U16(to_u16(u))} else{self}}

pub fn  to_u32_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::U32(to_u32(u))} else{self}}

pub fn  to_u64_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::U64(u)} else{self}}

pub fn  to_usize_if_uint(self)-> Self
{if let Self::Uint(u) = self{Self::USize(u as usize)} else{self}}

pub fn  to_f32_if_float(self)-> Self
{if let Self::Float(f) = self{Self::F32(to_f32(f))} else{self}}

pub fn  to_f64_if_float(self)-> Self
{if let Self::Float(f) = self{Self::F64(f)} else{self}}


pub fn
get_ty_name(&self)-> Option<String>
{
    match self
    {
  Self::Void    =>{Some("void".to_string())}
  Self::Bool(_) =>{Some("bool".to_string())}
  Self::Int(_)  =>{Some("i64".to_string())}
  Self::I8(_)   =>{Some("i8".to_string())}
  Self::I16(_)  =>{Some("i16".to_string())}
  Self::I32(_)  =>{Some("i32".to_string())}
  Self::I64(_)  =>{Some("i64".to_string())}
  Self::ISize(_)=>{Some("isize".to_string())}
  Self::Uint(_) =>{Some("u64".to_string())}
  Self::U8(_)   =>{Some("u8".to_string())}
  Self::U16(_)  =>{Some("u16".to_string())}
  Self::U32(_)  =>{Some("u32".to_string())}
  Self::U64(_)  =>{Some("u64".to_string())}
  Self::USize(_)=>{Some("usize".to_string())}
  Self::Float(_)=>{Some("f64".to_string())}
  Self::F32(_)  =>{Some("f32".to_string())}
  Self::F64(_)  =>{Some("f64".to_string())}
  Self::Enumerator(ty_name,_)=>{Some(ty_name.clone())}
  Self::Complex(ty_name,_)=>{Some(ty_name.clone())}
  _=>{None}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Type(ty_name)=>{print!("type {}",ty_name);}

  Self::NullPointer=>{print!("null_pointer");}

  Self::Void=>{print!("void");}
  Self::Bool(b)=>{print!("bool");}

  Self::Int(i)  =>{print!("{}: literal   int",*i);}
  Self::I8(i)   =>{print!("{}: i8",*i);}
  Self::I16(i)  =>{print!("{}: i16",*i);}
  Self::I32(i)  =>{print!("{}: i32",*i);}
  Self::I64(i)  =>{print!("{}: i64",*i);}
  Self::ISize(i)=>{print!("{}: isize",*i);}

  Self::Uint(u) =>{print!("{}: literal  uint",*u);}
  Self::U8(u)   =>{print!("{}: u8",*u);}
  Self::U16(u)  =>{print!("{}: u16",*u);}
  Self::U32(u)  =>{print!("{}: u32",*u);}
  Self::U64(u)  =>{print!("{}: u64",*u);}
  Self::USize(u)=>{print!("{}: usize",*u);}

  Self::Float(f)=>{print!("{}: literal float",*f);}
  Self::F32(f)=>{print!("{}: f32",*f);}
  Self::F64(f)=>{print!("{}: f64",*f);}

  Self::String(s)=>{print!("literal string");}
  Self::Enumerator(ty_name,id)=>
    {
      print!("{}::{}",ty_name,id);
    }
  Self::Complex(ty_name,_)=>
    {
      print!("{}",ty_name);
    }
  Self::Err=>{print!("ERR");}
    }
}


}




pub fn
evaluate_self_access_const(res: EvalConstResult, s: &str)-> EvalConstResult
{
  EvalConstResult::Err
}


pub fn
evaluate_type_access_const(res: EvalConstResult, id: &str)-> EvalConstResult
{
    if let EvalConstResult::Type(ty_name) = res
    {
        match id
        {
      (s) if s ==  "name"=>{return EvalConstResult::String(ty_name);}
      (s) if s ==  "size"=>{return EvalConstResult::Int(find_ty(&ty_name).unwrap().get_size()  as i64);}
      (s) if s == "align"=>{return EvalConstResult::Int(find_ty(&ty_name).unwrap().get_align() as i64);}
      (s) if s == "default"=>{return find_ty(&ty_name).unwrap().get_default().clone();}
      _=>{return EvalConstResult::Err;}
        }
    }


  EvalConstResult::Err
}


pub fn
evaluate_subscript_const(res: EvalConstResult, i: EvalConstResult)-> EvalConstResult
{
  EvalConstResult::Err
}


pub fn
evaluate_call_const(res: EvalConstResult, args: Vec<EvalConstResult>)-> EvalConstResult
{
    match res
    {
  EvalConstResult::Type(ty_name)=>
    {
      let  ty = find_ty(&ty_name).unwrap();

      ty.construct(args)
    }
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_unary_const_i(v: i64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s == "!"=>{EvalConstResult::Int(!v)}
  (s) if s == "-"=>{EvalConstResult::Int(-v)}
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_unary_const_u(v: u64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s == "!"=>{EvalConstResult::Uint(!v)}
  (s) if s == "-"=>
    {
        if v <= (i64::MAX as u64){EvalConstResult::Int(-(v as i64))}
      else{panic!();}
    }
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_unary_const_f(v: f64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s == "-"=>{EvalConstResult::Float(-v)}
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_unary_const(res: EvalConstResult, op: &str)-> EvalConstResult
{
    if let EvalConstResult::Bool(v) = res
    {
        match op
        {
      (s) if s == "!"=>{EvalConstResult::Bool(!v)}
      _=>{EvalConstResult::Err}
        }
    }

  else
    if let EvalConstResult::Int(v) = res
    {
      evaluate_unary_const_i(v,op)
    }

  else
    if let EvalConstResult::I8(v) = res
    {
      evaluate_unary_const_i(v as i64,op).to_i8_if_int()
    }

  else
    if let EvalConstResult::I16(v) = res
    {
      evaluate_unary_const_i(v as i64,op).to_i16_if_int()
    }

  else
    if let EvalConstResult::I32(v) = res
    {
      evaluate_unary_const_i(v as i64,op).to_i32_if_int()
    }

  else
    if let EvalConstResult::I64(v) = res
    {
      evaluate_unary_const_i(v as i64,op).to_i64_if_int()
    }

  else
    if let EvalConstResult::ISize(v) = res
    {
      evaluate_unary_const_i(v as i64,op).to_isize_if_int()
    }

  else
    if let EvalConstResult::Uint(v) = res
    {
      evaluate_unary_const_u(v,op)
    }

  else
    if let EvalConstResult::U8(v) = res
    {
      evaluate_unary_const_u(v as u64,op).to_u8_if_uint()
    }

  else
    if let EvalConstResult::U16(v) = res
    {
      evaluate_unary_const_u(v as u64,op).to_u16_if_uint()
    }

  else
    if let EvalConstResult::U32(v) = res
    {
      evaluate_unary_const_u(v as u64,op).to_u32_if_uint()
    }

  else
    if let EvalConstResult::U64(v) = res
    {
      evaluate_unary_const_u(v as u64,op).to_u64_if_uint()
    }

  else
    if let EvalConstResult::USize(v) = res
    {
      evaluate_unary_const_u(v as u64,op).to_usize_if_uint()
    }

  else
    if let EvalConstResult::Float(v) = res
    {
      evaluate_unary_const_f(v,op)
    }

  else
    if let EvalConstResult::F32(v) = res
    {
      evaluate_unary_const_f(v as f64,op).to_f32_if_float()
    }

  else
    if let EvalConstResult::F64(v) = res
    {
      evaluate_unary_const_f(v,op).to_f64_if_float()
    }

  else
    {EvalConstResult::Err}
}


pub fn
evaluate_binary_const_i(l: i64, r: i64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s ==  "+"=>{EvalConstResult::Int(l+r)}
  (s) if s ==  "-"=>{EvalConstResult::Int(l-r)}
  (s) if s ==  "*"=>{EvalConstResult::Int(l*r)}
  (s) if s ==  "/"=>{EvalConstResult::Int(l/r)}
  (s) if s ==  "%"=>{EvalConstResult::Int(l%r)}
  (s) if s == "<<"=>{EvalConstResult::Int(l<<r)}
  (s) if s == ">>"=>{EvalConstResult::Int(l>>r)}
  (s) if s ==  "&"=>{EvalConstResult::Int(l&r)}
  (s) if s ==  "|"=>{EvalConstResult::Int(l|r)}
  (s) if s ==  "^"=>{EvalConstResult::Int(l^r)}
  (s) if s == "=="=>{EvalConstResult::Bool(l == r)}
  (s) if s == "!="=>{EvalConstResult::Bool(l != r)}
  (s) if s ==  "<"=>{EvalConstResult::Bool(l <  r)}
  (s) if s == "<="=>{EvalConstResult::Bool(l <= r)}
  (s) if s ==  ">"=>{EvalConstResult::Bool(l >  r)}
  (s) if s == ">="=>{EvalConstResult::Bool(l >= r)}
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_binary_const_u(l: u64, r: u64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s ==  "+"=>{EvalConstResult::Uint(l+r)}
  (s) if s ==  "-"=>
    {
        if l < r
        {
            if (r-l) < (i64::MAX as u64)
            {
              let  li = l as i64;
              let  ri = r as i64;

              EvalConstResult::Int(li-ri)
            }

          else
            {panic!();}
        }

      else
        {EvalConstResult::Uint(l-r)}
    }
  (s) if s ==  "*"=>{EvalConstResult::Uint(l*r)}
  (s) if s ==  "/"=>{EvalConstResult::Uint(l/r)}
  (s) if s ==  "%"=>{EvalConstResult::Uint(l%r)}
  (s) if s == "<<"=>{EvalConstResult::Uint(l<<r)}
  (s) if s == ">>"=>{EvalConstResult::Uint(l>>r)}
  (s) if s ==  "&"=>{EvalConstResult::Uint(l&r)}
  (s) if s ==  "|"=>{EvalConstResult::Uint(l|r)}
  (s) if s ==  "^"=>{EvalConstResult::Uint(l^r)}
  (s) if s == "=="=>{EvalConstResult::Bool(l == r)}
  (s) if s == "!="=>{EvalConstResult::Bool(l != r)}
  (s) if s ==  "<"=>{EvalConstResult::Bool(l <  r)}
  (s) if s == "<="=>{EvalConstResult::Bool(l <= r)}
  (s) if s ==  ">"=>{EvalConstResult::Bool(l >  r)}
  (s) if s == ">="=>{EvalConstResult::Bool(l >= r)}
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_binary_const_f(l: f64, r: f64, op: &str)-> EvalConstResult
{
    match op
    {
  (s) if s ==  "+"=>{EvalConstResult::Float(l+r)}
  (s) if s ==  "-"=>{EvalConstResult::Float(l-r)}
  (s) if s ==  "*"=>{EvalConstResult::Float(l*r)}
  (s) if s ==  "/"=>{EvalConstResult::Float(l/r)}
  (s) if s ==  "%"=>{EvalConstResult::Float(l%r)}
  (s) if s == "=="=>{EvalConstResult::Bool(l == r)}
  (s) if s == "!="=>{EvalConstResult::Bool(l != r)}
  (s) if s ==  "<"=>{EvalConstResult::Bool(l <  r)}
  (s) if s == "<="=>{EvalConstResult::Bool(l <= r)}
  (s) if s ==  ">"=>{EvalConstResult::Bool(l >  r)}
  (s) if s == ">="=>{EvalConstResult::Bool(l >= r)}
  _=>{EvalConstResult::Err}
    }
}


pub fn
evaluate_binary_const(l_res: EvalConstResult, r_res: EvalConstResult, op: &str)-> EvalConstResult
{
    if let EvalConstResult::Bool(l) = l_res{
    if let EvalConstResult::Bool(r) = r_res{
      return match op
        {
      (s) if s == "&&"=>{EvalConstResult::Bool(l && r)}
      (s) if s == "||"=>{EvalConstResult::Bool(l || r)}
      _=>{EvalConstResult::Err}
        };
    }}

  else
    if let EvalConstResult::Int(l) = l_res
    {
        match r_res
        {
      EvalConstResult::Int(r) =>{return evaluate_binary_const_i(l,        r,op);}
      EvalConstResult::Uint(r)=>{return evaluate_binary_const_i(l,to_i64(r),op);}
      _=>{panic!();}
        }
    }

  else
    if let EvalConstResult::I8(l) = l_res{
    if let EvalConstResult::I8(r) = r_res{
      return evaluate_binary_const_i(l as i64,r as i64,op).to_i8_if_int();
    }}

  else
    if let EvalConstResult::I16(l) = l_res{
    if let EvalConstResult::I16(r) = r_res{
      return evaluate_binary_const_i(l as i64,r as i64,op).to_i16_if_int();
    }}

  else
    if let EvalConstResult::I32(l) = l_res{
    if let EvalConstResult::I32(r) = r_res{
      return evaluate_binary_const_i(l as i64,r as i64,op).to_i32_if_int();
    }}

  else
    if let EvalConstResult::I64(l) = l_res{
    if let EvalConstResult::I64(r) = r_res{
      return evaluate_binary_const_i(l as i64,r as i64,op).to_i64_if_int();
    }}

  else
    if let EvalConstResult::ISize(l) = l_res{
    if let EvalConstResult::ISize(r) = r_res{
      return evaluate_binary_const_i(l as i64,r as i64,op).to_isize_if_int();
    }}

  else
    if let EvalConstResult::Uint(l) = l_res{
    if let EvalConstResult::Uint(r) = r_res{
      return evaluate_binary_const_u(l,r,op);
    }}

  else
    if let EvalConstResult::U8(l) = l_res{
    if let EvalConstResult::U8(r) = r_res{
      return evaluate_binary_const_u(l as u64,r as u64,op).to_u8_if_uint();
    }}

  else
    if let EvalConstResult::U16(l) = l_res{
    if let EvalConstResult::U16(r) = r_res{
      return evaluate_binary_const_u(l as u64,r as u64,op).to_u16_if_uint();
    }}

  else
    if let EvalConstResult::U32(l) = l_res{
    if let EvalConstResult::U32(r) = r_res{
      return evaluate_binary_const_u(l as u64,r as u64,op).to_u32_if_uint();
    }}

  else
    if let EvalConstResult::U64(l) = l_res{
    if let EvalConstResult::U64(r) = r_res{
      return evaluate_binary_const_u(l as u64,r as u64,op).to_u64_if_uint();
    }}

  else
    if let EvalConstResult::USize(l) = l_res{
    if let EvalConstResult::USize(r) = r_res{
      return evaluate_binary_const_u(l as u64,r as u64,op).to_usize_if_uint();
    }}

  else
    if let EvalConstResult::Float(l) = l_res{
    if let EvalConstResult::Float(r) = r_res{
      return evaluate_binary_const_f(l,r,op);
    }}

  else
    if let EvalConstResult::F32(l) = l_res{
    if let EvalConstResult::F32(r) = r_res{
      return evaluate_binary_const_f(l as f64,r as f64,op).to_f32_if_float();
    }}

  else
    if let EvalConstResult::F64(l) = l_res{
    if let EvalConstResult::F64(r) = r_res{
      return evaluate_binary_const_f(l,r,op).to_f64_if_float();
    }}


  EvalConstResult::Err
}


pub fn
evaluate_const(e: &Expr, symtbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalConstResult
{
    match e
    {
  Expr::Void=>{EvalConstResult::Void}
  Expr::Identifier(s)=>
    {
           if s ==  "void"{return EvalConstResult::Void;}
      else if s == "false"{return EvalConstResult::Bool(false);}
      else if s ==  "true"{return EvalConstResult::Bool(true);}
      else if let Some(scp) = scp_opt
        {
            if let Some(lsym) = scp.find(s)
            {
                if let LocalSymbolKind::Const = lsym.get_kind()
                {
                  return lsym.get_value().clone();
                }


              return EvalConstResult::Err;
            }
        }


        if let Some(sym) = symtbl.find_symbol(s)
        {
          return match sym.get_kind()
            {
          SymbolKind::Ty=>
            {
              let  ty = find_ty(sym.get_ty_name()).unwrap();

              EvalConstResult::Type(ty.get_name().clone())
            }
          SymbolKind::Const(cres)=>
            {
              cres.clone()
            }
          _=>{EvalConstResult::Err}
            };
        }


        if find_ty(s).is_some()
        {
          return EvalConstResult::Type(s.clone());
        }


      EvalConstResult::Err
    }
  Expr::Int(u)  =>{EvalConstResult::Uint(*u)}
  Expr::Float(f)=>{EvalConstResult::Float(*f)}
  Expr::String(s)=>{EvalConstResult::String(s.clone())}
  Expr::SelfAccessOp(e,s)=>{evaluate_self_access_const(evaluate_const(e,symtbl,scp_opt),s)}
  Expr::TypeAccessOp(e,s)=>{evaluate_type_access_const(evaluate_const(e,symtbl,scp_opt),s)}
  Expr::SubscriptOp(e,i_e)=>
    {
      let  res = evaluate_const(i_e,symtbl,scp_opt);

      evaluate_subscript_const(evaluate_const(e,symtbl,scp_opt),res)
    }
  Expr::CallOp(f,args)=>
    {
      let  mut buf = Vec::<EvalConstResult>::new();

        for arg_e in args
        {
          let  res = evaluate_const(arg_e,symtbl,scp_opt);

            if res.is_err()
            {
              return EvalConstResult::Err;
            }


          buf.push(res);
        }


      evaluate_call_const(evaluate_const(f,symtbl,scp_opt),buf)
    }
  Expr::Expr(e)=>{evaluate_const(e,symtbl,scp_opt)}
  Expr::UnaryOp(o,op)=>{evaluate_unary_const(evaluate_const(o,symtbl,scp_opt),op)}
  Expr::BinaryOp(l,r,op)=>
    {
      let  l_res = evaluate_const(l,symtbl,scp_opt);
      let  r_res = evaluate_const(r,symtbl,scp_opt);

      evaluate_binary_const(l_res,r_res,op)
    }
    }
}




