

use crate::node::*;
use super::ty::*;
use super::asm::*;




#[derive(Clone)]
pub enum
Expr
{
  Void,

  Identifier(String),

    Int(u64),
  Float(f64),

  String(String),

  AccessOp(Box<Expr>,String),
  SubscriptOp(Box<Expr>,Box<Expr>),
  CallOp(Box<Expr>,Vec<Expr>),

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
print(&self)
{
    match self
    {
  Self::Void=>{print!("void");}

  Self::Identifier(s)=>{print!("{}",s);}

    Self::Int(u)=>{print!("{}",*u);}
  Self::Float(f)=>{print!("{}",*f);}

  Self::String(s)=>{print!("\"{}\"",s);}

  Self::AccessOp(e,s)=>{  e.print(); print!(".{}",s);}
  Self::SubscriptOp(e,i_e)=>
    {
      e.print();
      print!("[");
      i_e.print();
      print!("]");
    }
  Self::CallOp(f,args)=>
    {
      f.print();

      print!("(");

        for e in args
        {
          e.print();

          print!(", ");
        }


      print!(")");
    }
  Self::Expr(e)=>
    {
      print!("(");
      e.print();
      print!(")");
    }
   Self::UnaryOp(o,op)=>
    {
      print!("{}",op);
      o.print();
    }
  Self::BinaryOp(l,r,op)=>
    {
      l.print();
      print!("{}",op);
      r.print();
    }
    }
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

    if let Some(nd) = cur.select_node("postfix_op")
    {
      let  name = nd.get_name();

           if name == "access"   {return read_access_op(nd,o);}
      else if name == "subscript"{return read_subscript_op(nd,o);}
      else if name == "call"     {return read_call_op(nd,o);}
    }


  panic!();
}


pub fn
read_access_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      return Expr::AccessOp(o,s.clone());
    }


  panic!();
}


pub fn
read_subscript_op(start_nd: &Node, o: Box<Expr>)-> Expr
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(e_nd) = cur.select_node("expression")
    {
      let  e = read_expr(e_nd);

      return Expr::SubscriptOp(o,Box::new(e));
    }


  panic!();
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
read_operand_core(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

    if let Some(v) = cur.current()
    {
        match v
        {
      Value::Identifier(s)=>{return Expr::Identifier(s.clone());}
      Value::Uint(u) =>{return Expr::Int(*u);}
      Value::Float(f)=>{return Expr::Float(*f);}
      Value::String(s)=>
        {
          return Expr::String(s.to_string());
        },
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




