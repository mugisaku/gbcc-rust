

use crate::node::*;
use crate::source_file::{
  SourceInfo,
  Message,

};

use super::asm::*;
use super::decl::*;




#[derive(Clone)]
pub enum
ExprKind
{
  Identifier(String),

  String(String,String),

  Int(i64),

   CallOp(Box<Expr>,Vec<Expr>),
    DotOp(Box<Expr>,String),
  SubscOp(Box<Expr>,Box<Expr>),

  Expr(Box<Expr>),

   UnaryOp(Box<Expr>,String),
  BinaryOp(Box<Expr>,Box<Expr>,String),

}




#[derive(Clone)]
pub struct
Expr
{
  source_info: SourceInfo,
  kind: ExprKind,

}


impl
Expr
{


pub fn
from_int(i: i64)-> Self
{
  Self{
    source_info: SourceInfo::new(),
    kind: ExprKind::Int(i),
  }
}


pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
get_kind(&self)-> &ExprKind
{
  &self.kind
}


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"expression")
    {
      return Ok(read_expr(&nd));
    }


  Err(())
}


pub fn
collect_identifier(&self, set: &DeclSet, ss: &mut StringSet)
{
    match &self.kind
    {
  ExprKind::Identifier(s)=>
    {
        if let Some(decl) = set.search(s)
        {
          ss.insert(s);
        }

      else
        {
          ss.record_fail(&self.source_info,s);
        }
    }
  ExprKind::String(_,_)=>{}
  ExprKind::CallOp(f,args)=>
    {
      f.collect_identifier(set,ss);

        for e in args
        {
          e.collect_identifier(set,ss);
        }
    }
  ExprKind::DotOp(ins,_)=>
    {
      ins.collect_identifier(set,ss);
    }
  ExprKind::SubscOp(ref_o,idx_o)=>
    {
      ref_o.collect_identifier(set,ss);
      idx_o.collect_identifier(set,ss);
    }
  ExprKind::Expr(e)=>{e.collect_identifier(set,ss);}
  ExprKind::UnaryOp(o,op)=>{o.collect_identifier(set,ss);}
  ExprKind::BinaryOp(l,r,op)=>
    {
      l.collect_identifier(set,ss);
      r.collect_identifier(set,ss);
    }
  _=>{}
    }
}


pub fn
collect_static(&mut self, ss: &mut StaticSet)
{
    match &mut self.kind
    {
  ExprKind::Identifier(_)=>
    {
    }
  ExprKind::String(s,name)=>
    {
      *name = ss.insert_string(&self.source_info,s);
    }
  ExprKind::CallOp(f,args)=>
    {
      f.collect_static(ss);

        for e in args
        {
          e.collect_static(ss);
        }
    }
  ExprKind::DotOp(ins,_)=>
    {
      ins.collect_static(ss);
    }
  ExprKind::SubscOp(ref_o,idx_o)=>
    {
      ref_o.collect_static(ss);
      idx_o.collect_static(ss);
    }
  ExprKind::Expr(e)=>{e.collect_static(ss);}
  ExprKind::UnaryOp(o,op)=>{o.collect_static(ss);}
  ExprKind::BinaryOp(l,r,op)=>
    {
      l.collect_static(ss);
      r.collect_static(ss);
    }
  _=>{}
    }
}


pub fn
to_string(&self)-> String
{
  let  mut buf = String::new();

  self.print_to(&mut buf);

  buf
}


pub fn
print_to(&self, buf: &mut String)
{
    match &self.kind
    {
  ExprKind::Identifier(s)=>
    {
      buf.push_str(s);
    }
  ExprKind::String(s,_)=>
    {
      buf.push('\"');
      buf.push_str(s);
      buf.push('\"');
    }
  ExprKind::Int(i)=>{buf.push_str(&format!("{}",*i));}
  ExprKind::CallOp(f,args)=>
    {
      f.print_to(buf);

      buf.push('(');

        for e in args
        {
          e.print_to(buf);

          buf.push(',');
        }


      buf.push(')');
    }
  ExprKind::DotOp(ins,s)=>
    {
      ins.print_to(buf);
      buf.push('.');
      buf.push_str(s);
    }
  ExprKind::SubscOp(ref_o,idx_o)=>
    {
      ref_o.print_to(buf);
      buf.push_str("[");
      idx_o.print_to(buf);
      buf.push_str("]");
    }
  ExprKind::Expr(e)=>
    {
      buf.push('(');
      e.print_to(buf);
      buf.push(')');
    }
   ExprKind::UnaryOp(o,op)=>
    {
      buf.push_str(op);
      o.print_to(buf);
    }
  ExprKind::BinaryOp(l,r,op)=>
    {
      l.print_to(buf);
      buf.push_str(op);
      r.print_to(buf);
    }
    }
}


pub fn
print(&self)
{
  let  s = self.to_string();

  print!("{}",&s);
}


}




pub fn
read_expr(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

    if let Some(o_nd) = cur.select_node("operand")
    {
      let  mut o = read_operand(o_nd);

      cur.advance(1);

        while let Some(b_nd) = cur.select_node("binary_operator")
        {
          let  (source_info,bo) = read_binary_operator(b_nd);

          cur.advance(1);

            if let Some(next_o_nd) = cur.select_node("operand")
            {
              let  next_o = read_operand(next_o_nd);

              o = Expr{source_info, kind: ExprKind::BinaryOp(Box::new(o),Box::new(next_o),bo)};

              cur.advance(1);
            }
        }


      return o;
    }


  panic!();
}




pub fn
read_expr_list(start_nd: &Node)-> Vec<Expr>
{
  let  mut cur = start_nd.cursor();

  let  mut es = Vec::<Expr>::new();

  cur.advance(1);

    if let Some(first_e_nd) = cur.select_node("expression")
    {
      es.push(read_expr(first_e_nd));

      cur.advance(2);

        while let Some(e_nd) = cur.select_node("expression")
        {
          es.push(read_expr(e_nd));

          cur.advance(2);
        }
    }


  es
}


pub fn
read_unary_operator(start_nd: &Node)-> (SourceInfo,String)
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      return (source_info,s.clone());
    }


  panic!();
}


pub fn
read_binary_operator(start_nd: &Node)-> (SourceInfo,String)
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      return (source_info,s.clone());
    }


  panic!();
}


pub fn
read_postfix_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  mut cur = start_nd.cursor();

  let  nd = cur.get_node().unwrap();
  let  name = nd.get_name();

       if name ==  "call"{return read_call_op(nd,o);}
  else if name ==   "dot"{return read_dot_op(nd,o);}
  else if name == "subsc"{return read_subsc_op(nd,o);}
  else{panic!();}
}


pub fn
read_call_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

  let  mut args = Vec::<Expr>::new();

  cur.advance(1);

    if let Some(first_e_nd) = cur.select_node("expression")
    {
      args.push(read_expr(first_e_nd));

      cur.advance(2);

        while let Some(e_nd) = cur.select_node("expression")
        {
          args.push(read_expr(e_nd));

          cur.advance(2);
        }
    }


  Expr{source_info, kind: ExprKind::CallOp(o,args)}
}


pub fn
read_dot_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      return Expr{source_info, kind: ExprKind::DotOp(o,id.clone())};
    }


  panic!();
}


pub fn
read_subsc_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.get_node()
    {
      let  e = read_expr(nd);

      return Expr{source_info, kind: ExprKind::SubscOp(o,Box::new(e))};
    }


  panic!();
}


pub fn
read_qualified_identifier(start_nd: &Node)-> String
{
  let  mut cur = start_nd.cursor();

    if let Some(first_s) = cur.get_identifier()
    {
      let  mut buf = String::new();

      buf.push_str(first_s);

      cur.advance(1);

        while cur.is_semi_string()
        {
          cur.advance(1);

            if let Some(s) = cur.get_identifier()
            {
              buf.push_str("::");

              buf.push_str(s);

              cur.advance(1);
            }

          else
            {panic!();}
        }


      return buf;
    }


  panic!();
}


pub fn
read_operand_core(start_nd: &Node)-> Expr
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

    if let Some(v) = cur.current()
    {
        match v.get_kind()
        {
      ValueKind::Node(nd)=>
        {
            if nd.get_name() == "qualified_identifier"
            {
              let  buf = read_qualified_identifier(&*nd);

              return Expr{source_info, kind: ExprKind::Identifier(buf)};
            }


          panic!();
        }
      ValueKind::String(s)=>{return Expr{source_info, kind: ExprKind::String(s.clone(),String::new())};}
      ValueKind::Uint(u) =>{return Expr{source_info, kind: ExprKind::Int(*u as i64)};}
      ValueKind::Char(c) =>{return Expr{source_info, kind: ExprKind::Int(*c as i64)};}
      ValueKind::Float(_) =>{panic!("do not use floating point number");}
      ValueKind::SemiString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_nd) = cur.select_node("expression")
                  {
                    return Expr{source_info, kind: ExprKind::Expr(Box::new(read_expr(e_nd)))};
                  }
              }
          },
      _=>{println!("unknown value of node");},
        }
    }


  panic!();
}


pub fn
read_operand(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

  let  mut unop_buf = Vec::<(SourceInfo,String)>::new();

    while let Some(un_nd) = cur.select_node("unary_operator")
    {
      unop_buf.push(read_unary_operator(un_nd));

      cur.advance(1);
    }


    if let Some(core_nd) = cur.select_node("operand_core")
    {
      let  mut e = read_operand_core(core_nd);

      cur.advance(1);

        while let Some(post_nd) = cur.select_node("postfix_op")
        {
          e = read_postfix_op(post_nd,Box::new(e));

          cur.advance(1);
        }


        while let Some((source_info,unop)) = unop_buf.pop()
        {
          e = Expr{source_info, kind: ExprKind::UnaryOp(Box::new(e),unop)};
        }


      return e;
    }


  panic!();
}




