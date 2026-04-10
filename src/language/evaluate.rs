

use std::convert::{From, TryFrom};

use super::*;
use super::scope::*;
use super::symbol_table::*;
use super::expr::*;
use super::ty::*;
use super::decl::*;
use super::asm::*;
use super::evaluate_const::*;




#[derive(Clone)]
pub enum
EvalResult
{
  Value(AsmEvalText),
  Const(EvalConstResult),

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
  Self::Const(cres)=>
    {
        match cres
        {
      EvalConstResult::Type(ty)=>{panic!();}
      EvalConstResult::Void=>
        {
          let  mut text = AsmEvalText::new();

          text.push_opcode(Opcode::Push0);

          text.set_ty_name("void");

          text
        }
      EvalConstResult::Bool(b)=>
        {
          let  mut text = AsmEvalText::new();

          text.push_li_bool(b);

          text
        }
      EvalConstResult::Int(i)=>
        {
          let  mut text = AsmEvalText::new();

          text.push_li_int(i);

          text
        }
      EvalConstResult::Float(f)=>
        {
          let  mut text = AsmEvalText::new();

          text.push_li_float(f);

          text
        }
      EvalConstResult::String(_)=>{todo!();}
      EvalConstResult::Binary(_)=>{todo!();}
      EvalConstResult::Err=>{panic!();}
        }
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
  Self::Const(res)=>{  print!("const ");  res.print();}

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
      let  a_res = evaluate(a,tbl,scp_opt);

      buf.push(a_res.to_text());
    }


  txt.push_call(buf);

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
              EvalResult::Const(lsym.get_value().clone())
            }
          LocalSymbolKind::Var=>
            {
              let  mut txt = AsmEvalText::new();

              txt.push_local_var(lsym.get_offset(),lsym.get_ty_name());

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
      SymbolKind::Const(_,res)=>
        {
          EvalResult::Const(res.clone())
        }
      SymbolKind::GlobalVar(_,_)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_global_var(sym.get_offset(),sym.get_ty_name());

          EvalResult::Value(txt)
        }
      SymbolKind::Fn(fd)=>
        {
          let  mut txt = AsmEvalText::new();

          txt.push_fn(sym.get_offset(),sym.get_ty_name());

          EvalResult::Value(txt)
        }
      _=>{EvalResult::Err}
        };
    }


  EvalResult::Err
}


pub fn
evaluate_unary(o: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  cres = evaluate_const(o,tbl,scp_opt);

    if cres.is_ok()
    {
      return EvalResult::Const(cres);
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

    if lcres.is_ok()
    {
        if rcres.is_ok()
        {
          let  cres = evaluate_binary_const(lcres,rcres,op);

          return EvalResult::Const(cres);
        }

      else
        {
          lres = EvalResult::Const(lcres);
          rres = evaluate(re,tbl,scp_opt);
        }
    }

  else
    if rcres.is_ok()
    {
      lres = evaluate(le,tbl,scp_opt);
      rres = EvalResult::Const(rcres);
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

    if cres.is_ok()
    {
      return EvalResult::Const(cres);
    }


    match e
    {
  Expr::Identifier(s)=>
    {
      evaluate_identifier(s,tbl,scp_opt)
    }
  Expr::String(s)=>
    {
//      EvalResult::String(s.clone())
todo!();
    }
  Expr::SelfAccessOp(e,s)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::TypeAccessOp(e,s)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::SubscriptOp(e,i_e)=>
    {
      evaluate(e,tbl,scp_opt)
    }
  Expr::CallOp(f,args)=>
    {
      evaluate_call(f,args,tbl,scp_opt)
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




