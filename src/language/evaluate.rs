

use std::convert::{From, TryFrom};

use crate::source_file::{
  SourceInfo,
  Error,

};

use super::*;
use super::scope::*;
use super::expr::*;
use super::decl::*;
use super::asm::*;
use super::evaluate_const::*;




#[derive(Clone)]
pub enum
EvalResult
{
  Value(AsmEvalText),
  Values(Vec<AsmEvalText>),
  Asm(Vec<Opcode>),
  AsmWithArgc(Vec<Opcode>),
  Const(i64),
  String(String,String),
  Mod(std::ptr::NonNull<DeclSet>),

  Undef(&'static str),

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
try_to_text(self, srcinf: &SourceInfo)-> Result<AsmEvalText,Error>
{
    match self
    {
  Self::Value(txt)=>{Ok(txt)}
  Self::Values(txts)=>{Err(srcinf.to_error(format!("to_text is failed. from values")))}
  Self::Asm(_)        =>{Err(srcinf.to_error(format!("to_text is failed. from asm")))}
  Self::AsmWithArgc(_)=>{Err(srcinf.to_error(format!("to_text is failed. from asm with argc")))}
  Self::Const(i)=>{Ok(Self::to_text_from_const(i))}
  Self::String(_,_)=>{Err(srcinf.to_error(format!("to_text is failed. from str")))}
  Self::Mod(_) =>{Err(srcinf.to_error(format!("to_text is failed. from mod")))}
  Self::Undef(s)=>{Err(srcinf.to_error(format!("to_text is failed. from undef: {}",s)))}
  Self::Err(e)=>{Err(e)}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Value(_)=>{print!("value");}
  Self::Values(_)=>{print!("values");}
  Self::Asm(_)=>{print!("asm");}
  Self::AsmWithArgc(_)=>{print!("asm with argc");}
  Self::Const(i)=>{print!("const {}",*i);}
  Self::String(s,_)=>{print!("\"{}\"",s);}
  Self::Mod(ptr)=>{print!("MOD {}",&unsafe{ptr.as_ref()}.as_decl().get_qualified_name());}
  Self::Undef(s)=>{print!("UNDEF {}",s);}
  Self::Err(e)=>{print!("ERR");}
    }
}


}




pub fn
evaluate_args(args: &Vec<Expr>, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  mut buf = Vec::<AsmEvalText>::new();

    for a in args
    {
        match evaluate(a,set,scp_opt)
        {
      EvalResult::Value(mut a_txt)=>
        {
          a_txt.push_load();

          buf.push(a_txt);
        }
      EvalResult::Const(a_val)=>
        {
          buf.push(EvalResult::to_text_from_const(a_val));
        }
      EvalResult::Err(e)=>{return EvalResult::Err(e);}
      _=>{return EvalResult::Undef("call value default");}
        }
    }


  EvalResult::Values(buf)
}


pub fn
evaluate_call(f: &Expr, args: &Vec<Expr>, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = f.get_source_info();

    match evaluate(f,set,scp_opt)
    {
  EvalResult::Value(mut txt)=>
    {
        match evaluate_args(args,set,scp_opt)
        {
      EvalResult::Values(txts)=>
        {
          txt.push_call(txts);

          EvalResult::Value(txt)
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef("call value")}
        }
    }
  EvalResult::Asm(ops)=>
    {
        match evaluate_args(args,set,scp_opt)
        {
      EvalResult::Values(txts)=>
        {
          let  txt = AsmEvalText::concatenate(txts,ops,false);

          EvalResult::Value(txt)
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef("call asm")}
        }
    }
  EvalResult::AsmWithArgc(ops)=>
    {
        match evaluate_args(args,set,scp_opt)
        {
      EvalResult::Values(txts)=>
        {
          let  txt = AsmEvalText::concatenate(txts,ops,true);

          EvalResult::Value(txt)
        }
      EvalResult::Err(e)=>{EvalResult::Err(e)}
      _=>{EvalResult::Undef("call asm with argc")}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef("call defalut")}
    }
}


pub fn
evaluate_access(ins: &Expr, s: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = ins.get_source_info();

    match evaluate(ins,set,scp_opt)
    {
  EvalResult::Value(mut txt)=>
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
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef("access default")}
    }
}


pub fn
evaluate_decl(decl: &Decl)-> EvalResult
{
    match decl.get_kind()
    {
  DeclKind::Const(_,i)=>
    {
      EvalResult::Const(*i)
    }
  DeclKind::Static(_)=>
    {
      let  mut txt = AsmEvalText::new();

      txt.push_global_var(decl.get_offset());

      EvalResult::Value(txt)
    }
  DeclKind::Var(_)=>
    {
      let  mut txt = AsmEvalText::new();

      txt.push_global_var(decl.get_offset());

      EvalResult::Value(txt)
    }
  DeclKind::Fn(_)=>
    {
      let  mut txt = AsmEvalText::new();

      txt.push_fn(decl.get_offset());

      EvalResult::Value(txt)
    }
  DeclKind::Asm(txt)=>
    {
      EvalResult::Asm(txt.clone())
    }
  DeclKind::AsmWithArgc(txt)=>
    {
      EvalResult::AsmWithArgc(txt.clone())
    }
  DeclKind::Mod(set)=>{EvalResult::Mod(std::ptr::NonNull::from_ref(set))}
  _=>{EvalResult::Err(decl.get_source_info().to_error(format!("evaluate_identifier error: {} is invalid symbol kind",&decl.get_qualified_name())))}
    }
}


pub fn
evaluate_identifier(srcinf: &SourceInfo, name: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
    if let Some(scp) = scp_opt
    {
        if let Some(sym) = scp.find(name)
        {
          return match sym.get_kind()
            {
          SymbolKind::Const(i)=>
            {
              EvalResult::Const(*i)
            }
          SymbolKind::Static=>
            {
              let  mut txt = AsmEvalText::new();

              txt.push_global_var(sym.get_offset() as usize);

              EvalResult::Value(txt)
            }
          SymbolKind::Var=>
            {
              let  mut txt = AsmEvalText::new();

              txt.push_local_var(sym.get_offset());

              EvalResult::Value(txt)
            }
          _=>{EvalResult::Err(srcinf.to_error(format!("evaluate_identifier error: {} is invalid local symbol kind",name)))}
            };
        }
    }


    if let Some(decl) = set.search(name)
    {
      evaluate_decl(decl)
    }

  else
    {
      EvalResult::Err(srcinf.to_error(format!("evaluate_identifier error: {} not found",name)))
    }
}


pub fn
evaluate_unary(o: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  srcinf = o.get_source_info();

    match evaluate(o,set,scp_opt)
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
  _=>{EvalResult::Undef("unary default")}
    }
}


pub fn
evaluate_binary(l: &Expr, r: &Expr, op: &str, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  l_srcinf = l.get_source_info();
  let  r_srcinf = r.get_source_info();

    match evaluate(l,set,scp_opt)
    {
  EvalResult::Value(mut l_txt)=>
    {
        match evaluate(r,set,scp_opt)
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
      _=>{EvalResult::Undef("binary value default")}
        }
    }
  EvalResult::Const(l_val)=>
    {
        match evaluate(r,set,scp_opt)
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
      _=>{EvalResult::Undef("binary const default")}
        }
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{EvalResult::Undef("binary default")}
    }
}


pub fn
get_string(srcinf: &SourceInfo, name: &str, set: &DeclSet)-> EvalResult
{
    if let Some(decl) = set.get_root().find(name)
    {
        if let DeclKind::Static(k) = decl.get_kind()
        {
            if let StorageKind::String(_,_) = k
            {
              let  mut txt = AsmEvalText::new();

              txt.push_i64(decl.get_offset() as i64);

              return EvalResult::Value(txt);
            }
        }
    }


  EvalResult::Err(srcinf.to_error(format!("{} is not found or string",name)))
}


pub fn
evaluate(e: &Expr, set: &DeclSet, scp_opt: Option<&Scope>)-> EvalResult
{
  let  res = evaluate_const(e,set,scp_opt);

    match res
    {
  EvalResult::Undef(_)=>
    {
        match e.get_kind()
        {
      ExprKind::Identifier(s)=>
        {
          evaluate_identifier(e.get_source_info(),s,set,scp_opt)
        }
      ExprKind::Int(i)=>
        {
          EvalResult::Const(*i)
        }
      ExprKind::String(_,name)=>
        {
          get_string(e.get_source_info(),name,set)
        }
      ExprKind::CallOp(f,args)=>
        {
          evaluate_call(f,args,set,scp_opt)
        }
      ExprKind::AccessOp(ins,s)=>
        {
          evaluate_access(ins,s,set,scp_opt)
        }
      ExprKind::Expr(e)=>
        {
          evaluate(e,set,scp_opt)
        }
      ExprKind::UnaryOp(o,op)=>
        {
          evaluate_unary(o,op,set,scp_opt)
        }
      ExprKind::BinaryOp(l,r,op)=>
        {
          evaluate_binary(l,r,op,set,scp_opt)
        }
        }
    }
  EvalResult::String(_,name)=>
    {
      get_string(e.get_source_info(),&name,set)
    }
  EvalResult::Err(e)=>{EvalResult::Err(e)}
  _=>{res}
    }
}




