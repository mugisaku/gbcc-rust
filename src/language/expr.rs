

use crate::node::*;
use crate::source_file::{
  SourceInfo,
  Error,

};

use super::asm::*;




pub enum
CollectibleKind
{
  Identifier,
      String,

}


pub struct
Collectible
{
  source_info: SourceInfo,
  content: String,
  kind: CollectibleKind,
  
}


impl
Collectible
{


pub fn
destruct(self)-> (SourceInfo,String,CollectibleKind)
{
  (self.source_info,self.content,self.kind)
}


pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
get_content(&self)-> &String
{
  &self.content
}


pub fn
get_kind(&self)-> &CollectibleKind
{
  &self.kind
}


}




#[derive(Clone)]
pub enum
ExprKind
{
  Identifier(String),
      String(String),

  Int(i64),

  CallOp(Box<Expr>,Vec<Expr>),
  AccessOp(Box<Expr>,String),

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
collect(&self, buf: &mut Vec<Collectible>)
{
    match &self.kind
    {
  ExprKind::Identifier(s)=>
    {
      let  source_info = self.source_info.clone();

      let  content = s.clone();

      let  kind = CollectibleKind::Identifier;

      buf.push(Collectible{source_info, content, kind});
    }
  ExprKind::String(s)=>
    {
      let  source_info = self.source_info.clone();

      let  content = s.clone();

      let  kind = CollectibleKind::String;

      buf.push(Collectible{source_info, content, kind});
    }
  ExprKind::CallOp(f,args)=>
    {
      f.collect(buf);

        for e in args
        {
          e.collect(buf);
        }
    }
  ExprKind::AccessOp(ins,_)=>
    {
      ins.collect(buf);
    }
  ExprKind::Expr(e)=>{e.collect(buf);}
  ExprKind::UnaryOp(o,op)=>{o.collect(buf);}
  ExprKind::BinaryOp(l,r,op)=>
    {
      l.collect(buf);
      r.collect(buf);
    }
  _=>{}
    }
}


pub fn
collect_string(&self, buf: &mut Vec<Collectible>)
{
    match &self.kind
    {
  ExprKind::String(s)=>
    {
      let  source_info = self.source_info.clone();

      let  content = s.clone();

      let  kind = CollectibleKind::String;

      buf.push(Collectible{source_info, content, kind});
    }
  ExprKind::CallOp(f,args)=>
    {
      f.collect_string(buf);

        for e in args
        {
          e.collect_string(buf);
        }
    }
  ExprKind::AccessOp(ins,_)=>
    {
      ins.collect_string(buf);
    }
  ExprKind::Expr(e)=>{e.collect_string(buf);}
  ExprKind::UnaryOp(o,op)=>{o.collect_string(buf);}
  ExprKind::BinaryOp(l,r,op)=>
    {
      l.collect_string(buf);
      r.collect_string(buf);
    }
  _=>{}
    }
}


pub fn
print_to(&self, buf: &mut String)
{
    match &self.kind
    {
  ExprKind::Identifier(s)=>{buf.push_str(s);}
  ExprKind::String(s)=>
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
  ExprKind::AccessOp(ins,s)=>
    {
      ins.print_to(buf);
      buf.push('.');
      buf.push_str(s);
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
  let  mut buf = String::new();

  self.print_to(&mut buf);

  print!("{}",&buf);
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

       if name ==   "call"{return read_call_op(nd,o);}
  else if name == "access"{return read_access_op(nd,o);}
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
read_access_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      return Expr{source_info, kind: ExprKind::AccessOp(o,id.clone())};
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
      ValueKind::Identifier(s)=>{return Expr{source_info, kind: ExprKind::Identifier(s.clone())};}
      ValueKind::String(s)=>{return Expr{source_info, kind: ExprKind::String(s.clone())};}
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




