

use super::*;
use super::expr::*;
use super::ty::*;
use super::scope::*;
use super::symbol_table::*;




#[derive(Clone)]
pub enum
EvalConstResult
{
  Type(String),

  Void,
  Bool(bool),
    Int(i64),
  Float(f64),

  String(String),
  Binary(Vec<u8>),

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


pub fn
print(&self)
{
    match self
    {
  Self::Type(ty_name)=>{print!("type {}",ty_name);}

  Self::Void=>{print!("void");}
  Self::Bool(b)=>{print!("bool");}
  Self::Int(i)  =>{print!("{}: literal   int",*i);}
  Self::Float(f)=>{print!("{}: literal float",*f);}

  Self::String(s)=>{print!("literal string");}
  Self::Binary(s)=>{print!("literal binary");}

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
    if let EvalConstResult::Type(ty) = res
    {
/*
        if let Some(ty) = find_ty(&ty_name)
        {
            match id
            {
          (s) if s ==  "name"=>{return EvalConstResult::String(ty.get_name().clone());}
          (s) if s ==  "size"=>{return EvalConstResult::Int(ty.get_size()  as i64);}
          (s) if s == "align"=>{return EvalConstResult::Int(ty.get_align() as i64);}
          _=>{return EvalConstResult::Err;}
            }
        }
*/
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
  EvalConstResult::Err
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
        match op
        {
      (s) if s == "!"=>{EvalConstResult::Int(!v)}
      (s) if s == "-"=>{EvalConstResult::Int(-v)}
      _=>{EvalConstResult::Err}
        }
    }

  else
    if let EvalConstResult::Float(v) = res
    {
        match op
        {
      (s) if s == "-"=>{EvalConstResult::Float(-v)}
      _=>{EvalConstResult::Err}
        }
    }

  else
    {EvalConstResult::Err}
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
    if let EvalConstResult::Int(l) = l_res{
    if let EvalConstResult::Int(r) = r_res{
      return match op
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
        };
    }}

  else
    if let EvalConstResult::Float(l) = l_res{
    if let EvalConstResult::Float(r) = r_res{
      return match op
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
        };
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
            if let SymbolKind::Const(_,cres) = sym.get_kind()
            {
              return cres.clone();
            }


          return EvalConstResult::Err;
        }


        if let Some(_) = find_ty(s)
        {
          return EvalConstResult::Type(s.clone());
        }


      EvalConstResult::Err
    }
  Expr::Int(u)  =>{EvalConstResult::Int(*u as i64)}
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




