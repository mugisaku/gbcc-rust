

use std::convert::{From, TryFrom};

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

  Err,

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
  if let Self::Err = self{true} else{false}
}




pub fn
to_text(self)-> AsmEvalText
{
    match self
    {
  Self::Value(txt)=>{txt}
  Self::Const(i)=>
    {
      let  mut text = AsmEvalText::new();

      text.push_i64(i);

      text
    }
  Self::Err=>{panic!();}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Value(_)=>{print!("value");}
  Self::Const(i)=>{print!("const {}",*i);}

  Self::Err=>{print!("ERR");}
    }
}


}




pub fn
evaluate_call(f: &Expr, args: &Vec<Expr>, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  res = evaluate(f,tbl,scp_opt);

  let  mut txt = res.to_text();

  let  mut buf = Vec::<AsmEvalText>::new();

    for a in args
    {
      let  mut a_txt = evaluate(a,tbl,scp_opt).to_text();

        if a_txt.is_deref()
        {
          a_txt.push_load();
        }


      buf.push(a_txt);
    }


  txt.push_call(buf);

  EvalResult::Value(txt)
}


pub fn
evaluate_access(ins: &Expr, s: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  res = evaluate(ins,tbl,scp_opt);

  let  mut txt = res.to_text();

       if s == "ptr"{txt.push_to_ptr();}
  else if s ==  "i8"{txt.change_kind(AsmEvalKind::DerefI8 );}
  else if s == "i16"{txt.change_kind(AsmEvalKind::DerefI16);}
  else if s == "i32"{txt.change_kind(AsmEvalKind::DerefI32);}
  else if s == "i64"{txt.change_kind(AsmEvalKind::DerefI64);}
  else if s ==  "u8"{txt.change_kind(AsmEvalKind::DerefU8 );}
  else if s == "u16"{txt.change_kind(AsmEvalKind::DerefU16);}
  else if s == "u32"{txt.change_kind(AsmEvalKind::DerefU32);}
  else{panic!("evalute_access error: unknown field {}",s);}


  EvalResult::Value(txt)
}


pub fn
evaluate_identifier(s: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
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
          _=>{EvalResult::Err}
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
      SymbolKind::Fn(_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_fn(sym.get_offset());

          EvalResult::Value(txt)
        }
      _=>{panic!("evaluate_identifier error: {} is invalid symbol kind",s);}
        };
    }


  panic!("evaluate_identifier error: {} not found",s);
}


pub fn
evaluate_unary(o: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  cres = evaluate_const(o,tbl,scp_opt);

    if let Ok(i) = cres
    {
      return EvalResult::Const(i);
    }


  let  res = evaluate(o,tbl,scp_opt);

  let  mut txt = res.to_text();

  txt.push_unary(op);

  EvalResult::Value(txt)
}


pub fn
evaluate_binary(le: &Expr, re: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  lcres = evaluate_const(le,tbl,scp_opt);
  let  rcres = evaluate_const(re,tbl,scp_opt);

  let  mut lres = EvalResult::Err;
  let  mut rres = EvalResult::Err;

    if let Ok(l) = lcres
    {
        if let Ok(r) = rcres
        {
            if let Ok(i) = evaluate_binary_const(l,r,op)
            {
              return EvalResult::Const(i);
            }

          else
            {return EvalResult::Err;}
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


  let  mut l_txt = lres.to_text();
  let      r_txt = rres.to_text();

  l_txt.push_binary(r_txt,op);

  EvalResult::Value(l_txt)
}


pub fn
evaluate(e: &Expr, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  cres = evaluate_const(e,tbl,scp_opt);

    if let Ok(i) = cres
    {
      return EvalResult::Const(i);
    }


    match e
    {
  Expr::Identifier(s)=>
    {
      evaluate_identifier(s,tbl,scp_opt)
    }
  Expr::String(s)=>
    {
      let  sym = tbl.find_string_symbol(s).unwrap();

      let  mut txt = AsmEvalText::new();

      txt.push_i64(sym.get_offset() as i64);

      EvalResult::Value(txt)
    }
  Expr::CallOp(f,args)=>
    {
      evaluate_call(f,args,tbl,scp_opt)
    }
  Expr::AccessOp(ins,s)=>
    {
      evaluate_access(ins,s,tbl,scp_opt)
    }
  Expr::Expr(e)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::UnaryOp(o,op)=>
    {
      evaluate_unary(o,op,tbl,scp_opt)
    }
  Expr::BinaryOp(l,r,op)=>
    {
      evaluate_binary(l,r,op,tbl,scp_opt)
    }
  _=>{EvalResult::Err}
    }
}




