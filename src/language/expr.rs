

use crate::node::*;
use super::asm::*;




pub enum
Collectible
{
  Identifier(String),

}




#[derive(Clone,PartialEq)]
pub enum
Expr
{
  Identifier(String),

  Int(i64),

  CallOp(Box<Expr>,Vec<Expr>),
  AccessOp(Box<Expr>,String),

  Expr(Box<Expr>),

   UnaryOp(Box<Expr>,String),
  BinaryOp(Box<Expr>,Box<Expr>,String),

}


impl
Expr
{


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"expression",None)
    {
      return Ok(read_expr(&nd));
    }


  Err(())
}


pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    match self
    {
  Self::Identifier(s)=>{buf.push(Collectible::Identifier(s.clone()));}
  Self::CallOp(f,args)=>
    {
      f.collect(buf);

        for e in args
        {
          e.collect(buf);
        }
    }
  Self::AccessOp(ins,_)=>
    {
      ins.collect(buf);
    }
  Self::Expr(e)=>{e.collect(buf);}
  Self::UnaryOp(o,op)=>{o.collect(buf);}
  Self::BinaryOp(l,r,op)=>
    {
      l.collect(buf);
      r.collect(buf);
    }
  _=>{}
    }
}


pub fn
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Identifier(s)=>{buf.push_str(s);}

  Self::Int(i)=>{buf.push_str(&format!("{}",*i));}
  Self::CallOp(f,args)=>
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
  Self::AccessOp(ins,s)=>
    {
      ins.print_to(buf);
      buf.push('.');
      buf.push_str(s);
    }
  Self::Expr(e)=>
    {
      buf.push('(');
      e.print_to(buf);
      buf.push(')');
    }
   Self::UnaryOp(o,op)=>
    {
      buf.push_str(op);
      o.print_to(buf);
    }
  Self::BinaryOp(l,r,op)=>
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
          let  bo = read_binary_operator(b_nd);

          cur.advance(1);

            if let Some(next_o_nd) = cur.select_node("operand")
            {
              let  next_o = read_operand(next_o_nd);

              o = Expr::BinaryOp(Box::new(o),Box::new(next_o),bo);

              cur.advance(1);
            }
        }


      return o;
    }


  panic!();
}




pub fn
read_unary_operator(start_nd: &Node)-> String
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      return s.clone();
    }


  panic!();
}


pub fn
read_binary_operator(start_nd: &Node)-> String
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      return s.clone();
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


      return Expr::CallOp(o,args);
    }


  panic!();
}


pub fn
read_access_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      return Expr::AccessOp(o,id.clone());
    }


  panic!();
}


pub fn
read_operand_core(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

    if let Some(v) = cur.current()
    {
        match v
        {
      Value::Identifier(s)=>{return Expr::Identifier(s.clone());}
      Value::Uint(u) =>{return Expr::Int(*u as i64);}
      Value::Float(_) =>{panic!("do not use floating point number");}
      Value::SemiString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_nd) = cur.select_node("expression")
                  {
                    return Expr::Expr(Box::new(read_expr(e_nd)));
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

  let  mut unop_buf = Vec::<String>::new();

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


        while let Some(unop) = unop_buf.pop()
        {
          e = Expr::UnaryOp(Box::new(e),unop);
        }


      return e;
    }


  panic!();
}




