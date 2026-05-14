

use crate::node::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::scope::*;




#[derive(Clone)]
pub struct
ParameterDecl
{
  name: String,
  ty_node: TyNode,

}


impl
ParameterDecl
{


pub fn     get_name(&self)-> &String{&self.name}
pub fn  get_ty_node(&self)-> &TyNode{&self.ty_node}


pub fn
print(&self)
{
  print!("{}: {}",&self.name,&self.ty_node.to_string());
}


}




pub struct
FnDecl
{
  parameter_decl_list: Vec<ParameterDecl>,

  return_ty_node: TyNode,

  block: Block,

}


impl
FnDecl
{


pub fn  get_return_ty_node(&self)-> &TyNode{&self.return_ty_node}
pub fn  get_parameter_decl_list(&self)-> &Vec<ParameterDecl>{&self.parameter_decl_list}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
make_ty_node(&self)-> TyNode
{
  let  mut parameter_ty_nodes = Vec::<TyNode>::new();

    for pd in &self.parameter_decl_list
    {
      parameter_ty_nodes.push(pd.ty_node.clone());
    }


  let  return_ty_node = Box::new(self.return_ty_node.clone());

  TyNode::Function{parameter_ty_nodes, return_ty_node}
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


  print!(")-> {}",&self.return_ty_node.to_string());

  print!("\n");

  self.block.print();

  print!("\n");
}


}




pub enum
DeclKind
{
  Undef,

   Const(Option<TyNode>,Expr),
     Var(Option<TyNode>,Expr),
  Static(Option<TyNode>,Expr),

  Fn(FnDecl),

  Ty(TyNode),

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
  DeclKind::Const(tn_opt,e)=>
    {
      print!("const {}",name);

        if let Some(tn) = tn_opt
        {
          print!(": ");

          tn.print();
        }


      print!(" = ");

      e.print();
    }
  DeclKind::Var(tn_opt,e)=>
    {
      print!("var {}",name);

        if let Some(tn) = tn_opt
        {
          print!(": ");

          tn.print();
        }


      print!(" = ");

      e.print();
    }
  DeclKind::Static(tn_opt,e)=>
    {
      print!("static {}",name);

        if let Some(tn) = tn_opt
        {
          print!(": ");

          tn.print();
        }


      print!(" = ");

      e.print();
    }
  DeclKind::Fn(f)=>
    {
      print!("fn {}",name);

      f.print();
    }
  DeclKind::Ty(tn)=>
    {
      println!("type {} ",name);

      tn.print();
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


fn
collect_from_object_decl(tn_opt: &Option<TyNode>, e: &Expr, buf: &mut Vec<Collectible>)
{
    if let Some(tn) = tn_opt
    {
      tn.collect(buf);
    }


  e.collect(buf);
}


fn
collect_from_parameters(ls: &Vec<ParameterDecl>, buf: &mut Vec<Collectible>)
{
    for decl in ls
    {
      decl.ty_node.collect(buf);
    }
}


pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    match &self.kind
    {
  DeclKind::Undef=>{}
  DeclKind::Const(tn_opt,e)=>{Self::collect_from_object_decl(tn_opt,e,buf);}
  DeclKind::Var(tn_opt,e)  =>{Self::collect_from_object_decl(tn_opt,e,buf);}
  DeclKind::Fn(fd)=>{fd.get_block().collect(buf);}
  DeclKind::Ty(tn)=>{tn.collect(buf);}
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
read_parameter(start_nd: &Node)-> ParameterDecl
{
  let  mut cur = start_nd.cursor();

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(ty_nd) = cur.select_node("type")
            {
              let  ty_node = read_ty(ty_nd);

              return ParameterDecl{name,ty_node};
            }
        }
    }


  panic!();
}


pub fn
read_parameter_list(start_nd: &Node)-> Vec<ParameterDecl>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<ParameterDecl>::new();

  cur.advance(1);

    if let Some(first_nd) = cur.select_node("parameter")
    {
      let  first_p = read_parameter(first_nd);

      ls.push(first_p);

      cur.advance(1);

        while let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(p_nd) = cur.select_node("parameter")
            {
              let  p = read_parameter(p_nd);

              ls.push(p);

              cur.advance(1);
            }
        }
    }


  ls
}


pub fn
read_initialize(start_nd: &Node)-> (Option<TyNode>,Expr)
{
  let  mut cur = start_nd.cursor();

  let  mut tynode_opt = Option::<TyNode>::None;

    if let Some(s) = cur.get_semi_string()
    {
        if s == ":"
        {
          cur.advance(1);

            if let Some(ty_nd) = cur.select_node("type")
            {
              let  ty = read_ty(ty_nd);

              tynode_opt = Some(ty);

              cur.advance(1);
            }

          else
            {panic!();}
        }


      cur.advance(1);

        if let Some(e_nd) = cur.select_node("expression")
        {
          let  expr = read_expr(e_nd);

          return (tynode_opt,expr);
        }
    }


  panic!();
}


pub fn
read_object_decl(start_nd: &Node)-> (String,Option<TyNode>,Expr)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      cur.advance(1);

        if let Some(init_nd) = cur.select_node("initialize")
        {
          let  (tynode_opt,expr) = read_initialize(init_nd);

          return (name,tynode_opt,expr);
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
          let  parameter_decl_list = read_parameter_list(parals_d);

          cur.advance(1);

          let  mut return_ty_node = TyNode::Root("void".to_string());

            if let Some(_) = cur.get_semi_string()
            {
              cur.advance(1);

                if let Some(ty_d) = cur.get_node()
                {
                  return_ty_node = read_ty(ty_d);

                  cur.advance(1);
                }
            }


            if let Some(blk_d) = cur.select_node("block")
            {
              let  block = read_block(blk_d);

              let  f = FnDecl{parameter_decl_list, return_ty_node, block};

              return (name,f);
            }
        }
    }


  panic!();
}




pub fn
read_struct(start_nd: &Node)-> (String,Vec<ParameterDecl>)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.select_node("field_list")
        {
          let  ls = read_parameter_list(ls_d);

          return (name,ls);
        }
    }


  panic!();
}


pub fn
read_enumerator(start_nd: &Node)-> (String,)
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_identifier()
    {
      return (s.clone(),);
    }


  panic!();
}


pub fn
read_enumerator_list(start_nd: &Node)-> Vec<(String,)>
{
  let  mut cur = start_nd.cursor();

  let  mut ls = Vec::<(String,)>::new();

  cur.advance(1);

    if let Some(first_nd) = cur.select_node("enumerator")
    {
      let  first_p = read_enumerator(first_nd);

      ls.push(first_p);

      cur.advance(1);

        while let Some(s) = cur.get_semi_string()
        {
          cur.advance(1);

            if let Some(e_nd) = cur.select_node("enumerator")
            {
              let  e = read_enumerator(e_nd);

              ls.push(e);

              cur.advance(1);
            }
        }
    }


  ls
}


pub fn
read_enum(start_nd: &Node)-> (String,Vec<(String,)>)
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(ls_d) = cur.select_node("enumerator_list")
        {
          let  ls = read_enumerator_list(ls_d);

          return (name,ls);
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

        if name == "struct"
        {
          let  (name,ls) = read_struct(nd);

          let  ty_node = TyNode::Struct(ls);

          return Decl{name, kind: DeclKind::Ty(ty_node)};
        }

      else
        if name == "union"
        {
          let  (name,ls) = read_struct(nd);

          let  ty_node = TyNode::Union(ls);

          return Decl{name, kind: DeclKind::Ty(ty_node)};
        }

      else
        if name == "enum"
        {
          let  (name,ls) = read_enum(nd);

          let  ty_node = TyNode::Enum(ls);

          return Decl{name, kind: DeclKind::Ty(ty_node)};
        }

      else
        if name == "fn"
        {
          let  (name,f) = read_fn_decl(nd);

          return Decl{name, kind: DeclKind::Fn(f)};
        }

      else
        if name == "var"
        {
          let  (name,tynode_opt,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Var(tynode_opt,expr)};
        }

      else
        if name == "const"
        {
          let  (name,tynode_opt,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Const(tynode_opt,expr)};
        }

      else
        if name == "static"
        {
          let  (name,tynode_opt,expr) = read_object_decl(nd);

          return Decl{name, kind: DeclKind::Static(tynode_opt,expr)};
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




