

use std::convert::{From, TryFrom};

use crate::source_file::{
  SourceInfo,
  Error,

};

use super::*;
use super::scope::*;
use super::symbol_table::*;
use super::expr::*;
use super::decl::*;
use super::asm::*;
use super::evaluate_const::*;




#[derive(Clone)]
pub enum
EvalResult
{
  Value(AsmEvalText),
  Const(i64),

  Err(Error),

}


impl
EvalResult
{


pub fn
is_ok(&self)-> bool
{
  !self.is_err()
}


pub fn
is_err(&self)-> bool
{
  if let Self::Err(_) = self{true} else{false}
}


pub fn
make_err(e: &Expr, msg: String)-> Self
{
  Self::Err(e.get_source_info().to_error(msg))
}


pub fn
try_to_text(self)-> Result<AsmEvalText,Error>
{
    match self
    {
  Self::Value(txt)=>{Ok(txt)}
  Self::Const(i)=>
    {
      let  mut text = AsmEvalText::new();

      text.push_i64(i);

      Ok(text)
    }
  Self::Err(e)=>{Err(e)}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Value(_)=>{print!("value");}
  Self::Const(i)=>{print!("const {}",*i);}

  Self::Err(e)=>{print!("ERR");}
    }
}


}




pub fn
evaluate_call(f: &Expr, args: &Vec<Expr>, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
    match evaluate(f,tbl,scp_opt).try_to_text()
    {
  Ok(mut txt)=>
    {
      let  mut buf = Vec::<AsmEvalText>::new();

        for a in args
        {
            match evaluate(a,tbl,scp_opt).try_to_text()
            {
          Ok(mut a_txt)=>
            {
              a_txt.push_load();

              buf.push(a_txt);
            }
          Err(e)=>{return EvalResult::Err(e);}
            }
        }


      txt.push_call(buf);

      EvalResult::Value(txt)
    }
  Err(e)=>{EvalResult::Err(e)}
    }
}


pub fn
evaluate_access(ins: &Expr, s: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
    match evaluate(ins,tbl,scp_opt).try_to_text()
    {
  Ok(mut txt)=>
    {
           if s == "ptr"{txt.push_to_ptr();}
      else if s ==  "i8"{txt.change_kind(AsmEvalKind::DerefI8 );}
      else if s == "i16"{txt.change_kind(AsmEvalKind::DerefI16);}
      else if s == "i32"{txt.change_kind(AsmEvalKind::DerefI32);}
      else if s == "i64"{txt.change_kind(AsmEvalKind::DerefI64);}
      else if s ==  "u8"{txt.change_kind(AsmEvalKind::DerefU8 );}
      else if s == "u16"{txt.change_kind(AsmEvalKind::DerefU16);}
      else if s == "u32"{txt.change_kind(AsmEvalKind::DerefU32);}
      else
        {
          return EvalResult::make_err(ins,format!("evalute_access error: unknown field {}",s));
        }


      EvalResult::Value(txt)
    }
  Err(e)=>{EvalResult::Err(e)}
    }
}


pub fn
evaluate_identifier(srcinf: &SourceInfo, s: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
    if let Some(scp) = scp_opt
    {
        if let Some(lsym) = scp.find(s)
        {
          return match lsym.get_kind()
            {
          LocalSymbolKind::Const=>
            {
              EvalResult::Const(lsym.get_value())
            }
          LocalSymbolKind::Var=>
            {
              let  mut txt = AsmEvalText::new();

              txt.push_local_var(lsym.get_offset());

              EvalResult::Value(txt)
            }
          _=>{EvalResult::Err(srcinf.to_error(format!("evaluate_identifier error: {} is invalid local symbol kind",s)))}
            };
        }
    }


    if let Some(sym) = tbl.find_symbol(s)
    {
      return match sym.get_kind()
        {
      SymbolKind::Const(i)=>
        {
          EvalResult::Const(*i)
        }
      SymbolKind::GlobalVar(_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_global_var(sym.get_offset());

          EvalResult::Value(txt)
        }
      SymbolKind::Io=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_global_var(sym.get_offset());

          EvalResult::Value(txt)
        }
      SymbolKind::Str(_,_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_i64(sym.get_offset() as i64);

          EvalResult::Value(txt)
        }
      SymbolKind::Field(_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_i64(sym.get_offset() as i64);

          EvalResult::Value(txt)
        }
      SymbolKind::Fn(_,_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_fn(sym.get_offset());

          EvalResult::Value(txt)
        }
      _=>{EvalResult::Err(srcinf.to_error(format!("evaluate_identifier error: {} is invalid symbol kind",s)))}
        };
    }


  EvalResult::Err(srcinf.to_error(format!("evaluate_identifier error: {} not found",s)))
}


pub fn
evaluate_unary(o: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  cres = evaluate_const(o,tbl,scp_opt);

    if let Ok(i) = cres
    {
      return EvalResult::Const(i);
    }


    match evaluate(o,tbl,scp_opt).try_to_text()
    {
  Ok(mut txt)=>
    {
      txt.push_unary(op);

      EvalResult::Value(txt)
    }
  Err(e)=>{EvalResult::Err(e)}
    }
}


pub fn
evaluate_binary(le: &Expr, re: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  lcres = evaluate_const(le,tbl,scp_opt);
  let  rcres = evaluate_const(re,tbl,scp_opt);

  let  mut lres = EvalResult::Const(0);
  let  mut rres = EvalResult::Const(0);

    if let Ok(l) = lcres
    {
        if let Ok(r) = rcres
        {
          return EvalResult::Const(evaluate_binary_const(l,r,op).unwrap());
        }

      else
        {
          lres = EvalResult::Const(l);
          rres = evaluate(re,tbl,scp_opt);
        }
    }

  else
    if let Ok(r) = rcres
    {
      lres = evaluate(le,tbl,scp_opt);
      rres = EvalResult::Const(r);
    }

  else
    {
      lres = evaluate(le,tbl,scp_opt);
      rres = evaluate(re,tbl,scp_opt);
    }


    match lres.try_to_text()
    {
  Ok(mut l_txt)=>
    {
        match rres.try_to_text()
        {
      Ok(r_txt)=>
        {
          l_txt.push_binary(r_txt,op);

          EvalResult::Value(l_txt)
        }
      Err(e)=>{EvalResult::Err(e)}
        }
    }
  Err(e)=>{EvalResult::Err(e)}
    }
}


pub fn
evaluate(e: &Expr, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  cres = evaluate_const(e,tbl,scp_opt);

    if let Ok(i) = cres
    {
      return EvalResult::Const(i);
    }


    match e.get_kind()
    {
  ExprKind::Identifier(s)=>
    {
      evaluate_identifier(e.get_source_info(),s,tbl,scp_opt)
    }
  ExprKind::Int(i)=>
    {
      EvalResult::Const(*i)
    }
  ExprKind::String(s)=>
    {
      let  sym = tbl.find_string_symbol(s).unwrap();

      let  mut txt = AsmEvalText::new();

      txt.push_i64(sym.get_offset() as i64);

      EvalResult::Value(txt)
    }
  ExprKind::CallOp(f,args)=>
    {
      evaluate_call(f,args,tbl,scp_opt)
    }
  ExprKind::AccessOp(ins,s)=>
    {
      evaluate_access(ins,s,tbl,scp_opt)
    }
  ExprKind::Expr(e)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  ExprKind::UnaryOp(o,op)=>
    {
      evaluate_unary(o,op,tbl,scp_opt)
    }
  ExprKind::BinaryOp(l,r,op)=>
    {
      evaluate_binary(l,r,op,tbl,scp_opt)
    }
    }
}




