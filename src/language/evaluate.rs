

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
  String(String),
  System,
  Spawn,
  Print,

  Undef,

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
to_text_from_const(i: i64)-> AsmEvalText
{
  let  mut text = AsmEvalText::new();

  text.push_i64(i);

  text
}


pub fn
try_to_text(self)-> Result<AsmEvalText,Error>
{
    match self
    {
  Self::Value(txt)=>{Ok(txt)}
  Self::Const(i)=>{Ok(Self::to_text_from_const(i))}
  _=>{Err(Error::new(format!("to_text is failed")))}
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
  Self::String(s)=>{print!("\"{}\"",s);}
  Self::System=>{print!("SYS");}
  Self::Spawn =>{print!("SPW");}
  Self::Print =>{print!("PRI");}
  Self::Undef =>{print!("UNDEF");}
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
  let  srcinf = ins.get_source_info();

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
          return EvalResult::Err(srcinf.to_error(format!("evalute_access error: unknown field {}",s)));
        }


      EvalResult::Value(txt)
    }
  Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef}
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
  let  srcinf = o.get_source_info();

    match evaluate(o,tbl,scp_opt)
    {
  EvalResult::Value(mut txt)=>
    {
      txt.push_unary(op);

      EvalResult::Value(txt)
    }
  EvalResult::Const(i)=>
    {
        match evaluate_unary_int(i,op)
        {
      Ok(val)=>{EvalResult::Const(val)}
      Err(msg)=>{EvalResult::Err(srcinf.to_error(msg))}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef}
    }
}


pub fn
evaluate_binary(l: &Expr, r: &Expr, op: &str, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  l_srcinf = l.get_source_info();
  let  r_srcinf = r.get_source_info();

    match evaluate(l,tbl,scp_opt)
    {
  EvalResult::Value(mut l_txt)=>
    {
        match evaluate(r,tbl,scp_opt)
        {
      EvalResult::Value(r_txt)=>
        {
          l_txt.push_binary(r_txt,op);

          EvalResult::Value(l_txt)
        }
      EvalResult::Const(r_val)=>
        {
          let  r_txt = EvalResult::to_text_from_const(r_val);

          l_txt.push_binary(r_txt,op);

          EvalResult::Value(l_txt)
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef}
        }
    }
  EvalResult::Const(l_val)=>
    {
        match evaluate(r,tbl,scp_opt)
        {
      EvalResult::Value(r_txt)=>
        {
          let  mut l_txt = EvalResult::to_text_from_const(l_val);

          l_txt.push_binary(r_txt,op);

          EvalResult::Value(l_txt)
        }
      EvalResult::Const(r_val)=>
        {
            match evaluate_binary_int(l_val,r_val,op)
            {
          Ok(val)=>{EvalResult::Const(val)}
          Err(msg)=>{EvalResult::Err(l_srcinf.to_error(msg))}
            }
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef}
    }
}


pub fn
evaluate(e: &Expr, tbl: &SymbolTable, scp_opt: Option<&Scope>)-> EvalResult
{
  let  res = evaluate_const(e,tbl,scp_opt);

    match res
    {
  EvalResult::Undef=>
    {
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
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{res}
    }
}




