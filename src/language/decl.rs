

use crate::node::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;




pub struct
Space
{
  decl_list: Vec<Decl>,

}


impl
Space
{


pub fn
new()-> Self
{
  Self{
    decl_list: Vec::new(),
  }
}


pub fn
get_decl_list(&self)-> &Vec<Decl>
{
  &self.decl_list
}


pub fn
get_decl_list_mut(&mut self)-> &mut Vec<Decl>
{
  &mut self.decl_list
}


pub fn
add(&mut self, decl: Decl)
{
  self.decl_list.push(decl)
}


pub fn
find(&self, name: &str)-> Option<&Decl>
{
    for decl in &self.decl_list
    {
        if &decl.name == name
        {
          return Some(decl);
        }
    }


  None
}


pub fn
find_deep(&self, name: &str)-> Option<&Decl>
{
    for decl in &self.decl_list
    {
        if &decl.name == name
        {
          return Some(decl);
        }


        if let DeclKind::Space(sp) = &decl.kind
        {
            if let Some(decl) = sp.find_deep(name)
            {
              return Some(decl);
            }
        }
    }


  None
}


pub fn
print(&self)
{
  print!("{{\n");

    for decl in &self.decl_list
    {
      decl.print();

      print!("\n");
    }


  print!("}}\n");
}


}




pub struct
Object
{
  ty: Ty,

  expr: Expr,

}


impl
Object
{


pub fn  get_ty(&self)->     &Ty{&self.ty}
pub fn  get_expr(&self)-> &Expr{&self.expr}


pub fn
print(&self)
{
  self.ty.print();

  print!(" ");

  self.expr.print();
}


}




pub struct
Function
{
  parameter_list: Vec<Field>,

  return_ty: Ty,

  block: Block,

}


impl
Function
{


pub fn  get_return_ty(&self)-> &Ty{&self.return_ty}
pub fn  get_parameter_list(&self)-> &Vec<Field>{&self.parameter_list}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
print(&self)
{
  print!("(");

    for para in &self.parameter_list
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
  Space(Space),

  Type(Ty),

     Var(Object),
   Const(Object),
  Static(Object),

  Function(Function),

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
  DeclKind::Space(sp)=>
    {
      print!("space");

      sp.print();
    }
  DeclKind::Type(ty)=>
    {
      print!("type ");

      ty.print();
    }
  DeclKind::Var(o)=>
    {
      print!("var ");

      o.print();
    }
  DeclKind::Const(o)=>
    {
      print!("const ");

      o.print();
    }
  DeclKind::Static(o)=>
    {
      print!("static ");

      o.print();
    }
  DeclKind::Function(f)=>
    {
      print!("function");

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
new_primitive_type(name: &str, ty: SizedTy)-> Self
{
  Self{
    name: name.to_string(),
    kind: DeclKind::Type(Ty::Sized(Box::new(ty))),
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
get_kind_mut(&mut self)-> &mut DeclKind
{
  &mut self.kind
}


pub fn
decompose(self)-> (String,DeclKind)
{
  (self.name,self.kind)
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
read_as_root(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"declaration",None)
    {
      let  mut cur = nd.cursor();

      let  mut sp = Space::new();

        if let Some(decl_nd) = cur.select_node("declaration")
        {
          sp.add(read_decl(decl_nd));
        }


      let  decl = Decl{name: String::new(), kind: DeclKind::Space(sp)};

      return Ok(decl);
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
read_object_decl(start_nd: &Node)-> (String,Ty,Expr)
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

          let  mut ty = Ty::Sized(Box::new(SizedTy::Void));

            if let Some(ty_nd) = cur.select_node("type")
            {
              ty = read_ty(ty_nd);

              cur.advance(1);
            }


            if let Some(e_nd) = cur.select_node("expression")
            {
              let  expr = read_expr(e_nd);

              return (name,ty,expr);
            }
        }
    }


  panic!();
}


pub fn
read_function_decl(start_nd: &Node)-> (String,Function)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.select_node("parameter_list")
        {
          let  parameter_list = read_field_list(parals_d);

          cur.advance(1);

            if let Some(_) = cur.get_semi_string()
            {
              cur.advance(1);

              let  mut return_ty = Ty::Sized(Box::new(SizedTy::Void));

                if let Some(ty_d) = cur.get_node()
                {
                  return_ty = read_ty(ty_d);

                  cur.advance(1);
                }


                if let Some(blk_d) = cur.select_node("block")
                {
                  let  block = read_block(blk_d);

                  let  f = Function{parameter_list, return_ty, block};

                  return (name,f);
                }
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

        if name == "function"
        {
          let  (name,f) = read_function_decl(nd);

          return Decl{name, kind: DeclKind::Function(f)};
        }

      else
        if name == "let"
        {
          let  (name,ty,expr) = read_object_decl(nd);

          let  o = Object{ty, expr};

          return Decl{name, kind: DeclKind::Var(o)};
        }

      else
        if name == "const"
        {
          let  (name,ty,expr) = read_object_decl(nd);

          let  o = Object{ty, expr};

          return Decl{name, kind: DeclKind::Const(o)};
        }

      else
        if name == "static"
        {
          let  (name,ty,expr) = read_object_decl(nd);

          let  o = Object{ty, expr};

          return Decl{name, kind: DeclKind::Static(o)};
        }
    }


  panic!();
}




