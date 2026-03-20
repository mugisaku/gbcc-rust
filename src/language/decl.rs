

use crate::node::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;




pub struct
ParameterDecl
{
  name: String,
    ty: Ty,

}


impl
ParameterDecl
{


pub fn  get_name(&self)-> &String{&self.name}
pub fn  get_ty(&self)->   &Ty{&self.ty}


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.ty.print();
}


}




pub struct
FnDecl
{
  parameter_decl_list: Vec<ParameterDecl>,

  return_ty: Ty,

  block: Block,

}


impl
FnDecl
{


pub fn  get_return_ty(&self)-> &Ty{&self.return_ty}
pub fn  get_parameter_decl_list(&self)-> &Vec<ParameterDecl>{&self.parameter_decl_list}
pub fn  get_block(&self)-> &Block{&self.block}

pub fn
decompose(mut self)-> (Vec<String>,Ty,Block)
{
  let  mut name_list = Vec::<String>::new();
  let  mut   ty_list =     Vec::<Ty>::new();

    for decl in self.parameter_decl_list
    {
      name_list.push(decl.name);
        ty_list.push(decl.ty  );
    }


  let  ty = Ty::Function{parameter_ty_list: ty_list, return_ty: Box::new(self.return_ty)};

  (name_list,ty,self.block)
}


pub fn
print(&self)
{
  print!("(");

    for para in &self.parameter_decl_list
    {
      para.print();

      print!(", ");
    }


  print!(")-> ");

  self.return_ty.print();

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
  Static(Expr),

  Fn(FnDecl),

}


impl
DeclKind
{


pub fn
print(&self)
{
    match self
    {
  DeclKind::Undef   =>{print!("undef");}
  DeclKind::Const(o)=>
    {
      print!("const ");

      o.print();
    }
  DeclKind::Var(o)=>
    {
      print!("var ");

      o.print();
    }
  DeclKind::Static(o)=>
    {
      print!("static ");

      o.print();
    }
  DeclKind::Fn(f)=>
    {
      print!("fn");

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
release_name(&mut self)-> String
{
  let  mut s = String::new();

  std::mem::swap(&mut self.name,&mut s);

  s
}


pub fn
release_kind(&mut self)-> DeclKind
{
  let  mut k = DeclKind::Undef;

  std::mem::swap(&mut self.kind,&mut k);

  k
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

          ls.push(decl);

          cur.advance(1);
        }


      return Ok(ls);
    }


  Err(())
}


pub fn
print(&self)
{
  print!("{} ",&self.name);

  self.kind.print();

  print!("\n");
}


}




pub fn
read_parameter_decl(start_nd: &Node)-> ParameterDecl
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(ty_nd) = cur.select_node("type")
            {
              let  ty = read_ty(ty_nd);

              return ParameterDecl{name,ty};
            }
        }
    }


  panic!();
}


pub fn
read_parameter_decl_list(start_nd: &Node)-> Vec<ParameterDecl>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<ParameterDecl>::new();

  cur.advance(1);

    if let Some(first_nd) = cur.select_node("parameter")
    {
      let  first_p = read_parameter_decl(first_nd);

      ls.push(first_p);

      cur.advance(1);

        while let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(p_nd) = cur.select_node("parameter")
            {
              let  p = read_parameter_decl(p_nd);

              ls.push(p);

              cur.advance(1);
            }
        }
    }


  ls
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

        if let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(e_nd) = cur.select_node("expression")
            {
              let  expr = read_expr(e_nd);

              return (name,expr);
            }
        }
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
          let  parameter_decl_list = read_parameter_decl_list(parals_d);

          cur.advance(1);

          let  mut return_ty = Ty::Void;

            if let Some(_) = cur.get_semi_string()
            {
              cur.advance(1);

                if let Some(ty_d) = cur.get_node()
                {
                  return_ty = read_ty(ty_d);

                  cur.advance(1);
                }
            }


            if let Some(blk_d) = cur.select_node("block")
            {
              let  block = read_block(blk_d);

              let  f = FnDecl{parameter_decl_list, return_ty, block};

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
        if name == "static"
        {
          let  (name,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Static(expr)};
        }
    }

  else
    {
      return Decl{name: String::new(), kind: DeclKind::Undef};
    }


  panic!();
}




