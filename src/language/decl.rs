

use crate::node::*;
use super::expr::*;
use super::stmt::*;
use super::scope::*;




pub struct
FnDecl
{
  parameter_names: Vec<String>,

  block: Block,

}


impl
FnDecl
{


pub fn  get_parameter_names(&self)-> &Vec<String>{&self.parameter_names}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
print(&self)
{
  print!("(");

    for name in &self.parameter_names
    {
      print!("{}, ",name);
    }


  print!(")");

  print!("\n");

  self.block.print();

  print!("\n");
}


}




pub enum
DeclKind
{
  Undef,

   Const(Expr),
     Var(Expr),
      Io,

  Fn(FnDecl),

}


impl
DeclKind
{


pub fn
print(&self, name: &str)
{
    match self
    {
  DeclKind::Undef=>{print!("undef {}",name);}
  DeclKind::Const(e)=>
    {
      print!("const {}",name);

      print!(" = ");

      e.print();
    }
  DeclKind::Var(e)=>
    {
      print!("var {}",name);

      print!(" = ");

      e.print();
    }
  DeclKind::Io=>
    {
      print!("io {}",name);
    }
  DeclKind::Fn(f)=>
    {
      print!("fn {}",name);

      f.print();
    }
    }
}


}




pub struct
Decl
{
  name: String,
  kind: DeclKind,

}


impl
Decl
{


pub fn
new()-> Self
{
  Self{
    name: String::new(),
    kind: DeclKind::Undef,
  }
}


pub fn
expire(self)-> (String,DeclKind)
{
  (self.name,self.kind)
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_kind(&self)-> &DeclKind
{
  &self.kind
}


pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    match &self.kind
    {
  DeclKind::Undef=>{}
  DeclKind::Const(e)=>{e.collect(buf);}
  DeclKind::Var(e)  =>{e.collect(buf);}
  DeclKind::Io      =>{}
  DeclKind::Fn(fd)=>{/*fd.get_block().collect(buf);*/}
  _=>{panic!();}
    }
}


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"declaration",None)
    {
      let  mut cur = nd.cursor();

        if let Some(decl_nd) = cur.select_node("declaration")
        {
          return Ok(read_decl(decl_nd));
        }
    }


  Err(())
}


pub fn
read_as_root(s: &str)-> Result<Vec<Self>,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"declaration",None)
    {
      let  mut cur = nd.cursor();

      let  mut ls = Vec::<Decl>::new();

        while let Some(decl_nd) = cur.select_node("declaration")
        {
          let  decl = read_decl(decl_nd);

            if let DeclKind::Undef = &decl.kind
            {
            }

          else
            {ls.push(decl);}


          cur.advance(1);
        }


      return Ok(ls);
    }


  Err(())
}


pub fn
print(&self)
{
  self.kind.print(&self.name);
}


}




pub fn
read_parameter_list(start_nd: &Node)-> Vec<String>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<String>::new();

  cur.advance(1);

    if let Some(first_id) = cur.get_identifier()
    {
      ls.push(first_id.clone());

      cur.advance(1);

        while let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(p_id) = cur.get_identifier()
            {
              ls.push(p_id.clone());

              cur.advance(1);
            }
        }
    }


  ls
}


pub fn
read_initialize(start_nd: &Node)-> Expr
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      cur.advance(1);

        if let Some(e_nd) = cur.select_node("expression")
        {
          let  expr = read_expr(e_nd);

          return expr;
        }
    }


  panic!();
}


pub fn
read_object_decl(start_nd: &Node)-> (String,Expr)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(init_nd) = cur.select_node("initialize")
        {
          let  expr = read_initialize(init_nd);

          return (name,expr);
        }
    }


  panic!();
}


pub fn
read_io_decl(start_nd: &Node)-> String
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

//      cur.advance(2);

//      let  expr = read_expr(cur.select_node("expression").unwrap());

      return name;
    }


  panic!();
}


pub fn
read_fn_decl(start_nd: &Node)-> (String,FnDecl)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.select_node("parameter_list")
        {
          let  parameter_names = read_parameter_list(parals_d);

          cur.advance(1);

            if let Some(blk_d) = cur.select_node("block")
            {
              let  block = read_block(blk_d);

              let  f = FnDecl{parameter_names, block};

              return (name,f);
            }
        }
    }


  panic!();
}




pub fn
read_decl(start_nd: &Node)-> Decl
{
  let  mut cur = start_nd.cursor();

    if let Some(nd) = cur.get_node()
    {
      let  name = nd.get_name();

        if name == "fn"
        {
          let  (name,f) = read_fn_decl(nd);

          return Decl{name, kind: DeclKind::Fn(f)};
        }

      else
        if name == "io"
        {
          let  name = read_io_decl(nd);

          return Decl{name, kind: DeclKind::Io};
        }

      else
        if name == "var"
        {
          let  (name,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Var(expr)};
        }

      else
        if name == "const"
        {
          let  (name,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Const(expr)};
        }

      else
        {
          println!("{} is unknown decl",name);
        }
    }

  else
    {
      return Decl::new();
    }


  panic!();
}




