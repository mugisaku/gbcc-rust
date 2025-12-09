

use crate::node::*;
use super::expr::*;
use super::decl::*;
use super::ty::*;




pub struct
ElifStmt
{
  condition: Expr,

  block: Block,

}


impl
ElifStmt
{


pub fn  get_condition(&self)-> &Expr{&self.condition}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
is_executable_as_const(&self)-> bool
{
  self.block.is_executable_as_const()
}


pub fn
print(&self)
{
  print!("else if ");

  self.condition.print();
  self.block.print();
}


}




pub struct
IfStmt
{
  condition: Expr,

  block: Block,

  elif_stmt_list: Vec<ElifStmt>,

  else_block_opt: Option<Block>,

}


impl
IfStmt
{


pub fn  get_condition(&self)-> &Expr{&self.condition}
pub fn  get_block(&self)-> &Block{&self.block}
pub fn  get_elif_stmt_list(&self)-> &Vec<ElifStmt>{&self.elif_stmt_list}
pub fn  get_else_block(&self)-> &Option<Block>{&self.else_block_opt}

pub fn
is_executable_as_const(&self)-> bool
{
    if !self.block.is_executable_as_const()
    {
      return false;
    }


    for elif in &self.elif_stmt_list
    {
        if elif.is_executable_as_const()
        {
          return false;
        }
    }


    if let Some(blk) = &self.else_block_opt
    {
        if !blk.is_executable_as_const()
        {
          return false;
        }
    }


  true
}

pub fn
print(&self)
{
  print!("if ");

  self.condition.print();

  self.block.print();

    for elif in &self.elif_stmt_list
    {
      elif.print();

      print!("\n");
    }


    if let Some(blk) = &self.else_block_opt
    {
      print!("else");

      blk.print();
    }
}


}




pub struct
ForStmt
{
  var_name: String, 

  expr: Expr,

  block: Block,

}


impl
ForStmt
{


pub fn  get_var_name(&self)-> &String{&self.var_name}
pub fn  get_expr(&self)-> &Expr{&self.expr}
pub fn  get_block(&self)-> &Block{&self.block}


pub fn
is_executable_as_const(&self)-> bool
{
  self.block.is_executable_as_const()
}


pub fn
print(&self)
{
  print!("for {} in ",&self.var_name);

  self.expr.print();

  self.block.print();
}


}




pub struct
Block
{
  stmt_list: Vec<Stmt>,

}


impl
Block
{


pub fn
new()-> Block
{
  Self{
    stmt_list: Vec::new(),
  }
}


pub fn
get_stmt_list(&self)-> &Vec<Stmt>
{
  &self.stmt_list
}


pub fn
is_executable_as_const(&self)-> bool
{
    for stmt in &self.stmt_list
    {
        if !stmt.is_executable_as_const()
        {
          return false;
        }
    }


  true
}


pub fn
print(&self)
{
  print!("{{\n");

    for stmt in &self.stmt_list
    {
      stmt.print();

      print!("\n");
    }


  print!("}}\n");
}


}




pub enum
Stmt
{
  Empty,

  Block(Block),

  Expr(Expr),
  Decl(Decl),
  Assign(Expr,Expr,String),
  If(IfStmt),
  Loop(Block),
  While(Expr,Block),
  For(ForStmt),
  Break,
  Continue,

  Return(Option<Expr>),

  Print(Expr),

}


impl
Stmt
{


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"statement",None)
    {
      return Ok(read_stmt(&nd));
    }


  Err(())
}


pub fn
is_executable_as_const(&self)-> bool
{
    match self
    {
  Self::Block(blk)=>{blk.is_executable_as_const()}

  Self::Expr(e)=>{true}
  Self::If(i)       =>{i.is_executable_as_const()}
  Self::Loop(blk)   =>{blk.is_executable_as_const()}
  Self::While(e,blk)=>{blk.is_executable_as_const()}
  Self::For(f)=>{f.is_executable_as_const()}
  Self::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
        }


      true
    }
  _=>{true}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Empty=>{print!(";");}

  Self::Block(blk)=>{blk.print();}

  Self::Expr(e)=>{e.print();}
  Self::Decl(decl)=>{decl.print();}
  Self::Assign(l,r,op)=>
    {
      l.print();
      print!("{}",op);
      r.print();
    }
  Self::If(i)=>{i.print();}
  Self::Loop(blk)=>{  print!("loop");  blk.print();}
  Self::While(e,blk)=>{  print!("while");  e.print();  blk.print();}
  Self::For(f)=>{f.print();}
  Self::Break=>{print!("break");}
  Self::Continue=>{print!("continue");}

  Self::Return(e_opt)=>
    {
      print!("return ");

        if let Some(e) = e_opt
        {
          e.print();
        }
    }
  Self::Print(e)=>
    {
      print!("print ");

      e.print();
    }
    }
}


}




pub fn
read_assign(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

    if let Some(l_nd) = cur.select_node("expression")
    {
      let  l = read_expr(l_nd);

      cur.advance(1);

        if let Some(o_nd) = cur.select_node("assign_operator")
        {
          let  op = read_assign_operator(o_nd);

          cur.advance(1);

            if let Some(r_nd) = cur.select_node("expression")
            {
              let  r = read_expr(r_nd);

              return Stmt::Assign(l,r,op);
            }
        }
    }


  panic!();
}


pub fn
read_assign_operator(start_nd: &Node)-> String
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      return s.clone();
    }


  panic!();
}


pub fn
read_return(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      let  e = read_expr(nd);

      return Stmt::Return(Some(e));
    }


  Stmt::Return(None)
}


pub fn
read_print(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      let  e = read_expr(nd);

      return Stmt::Print(e);
    }


  panic!();
}


pub fn
read_elif_stmt(start_nd: &Node)-> ElifStmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(2);

    if let Some(expr_d) = cur.select_node("expression")
    {
      let  condition = read_expr(expr_d);

      cur.advance(1);

        if let Some(blk_d) = cur.select_node("block")
        {
          let  block = read_block(blk_d);

          return ElifStmt{condition, block};
        }
    }


  panic!();
}


pub fn
read_else(start_nd: &Node)-> Block
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(blk_d) = cur.select_node("block")
    {
      return read_block(blk_d);
    }


  panic!();
}


pub fn
read_if_stmt(start_nd: &Node)-> IfStmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(expr_d) = cur.select_node("expression")
    {
      let  condition = read_expr(expr_d);

      cur.advance(1);

        if let Some(blk_d) = cur.select_node("block")
        {
          let  block = read_block(blk_d);

          let  mut elif_stmt_list = Vec::<ElifStmt>::new();

          cur.advance(1);

            while let Some(elif_d) = cur.select_node("else_if")
            {
              elif_stmt_list.push(read_elif_stmt(elif_d));

              cur.advance(1);
            }


          let  mut else_block_opt = None;

            if let Some(el_d) = cur.select_node("else")
            {
              else_block_opt = Some(read_block(el_d));
            }


          return IfStmt{condition, block, elif_stmt_list, else_block_opt};
        }
    }


  panic!();
}


pub fn
read_while(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(expr_d) = cur.select_node("expression")
    {
      let  e = read_expr(expr_d);

      cur.advance(1);

        if let Some(ls_d) = cur.select_node("block")
        {
          let  blk = read_block(ls_d);

          return Stmt::While(e,blk);
        }
    }


  panic!();
}


pub fn
read_loop(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(ls_d) = cur.select_node("block")
    {
      return Stmt::Loop(read_block(ls_d));
    }


  panic!();
}


pub fn
read_for_stmt(start_nd: &Node)-> ForStmt
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      let  var_name = s.clone();

      cur.advance(2);

        if let Some(expr_d) = cur.select_node("expression")
        {
          let  expr = read_expr(expr_d);

          cur.advance(1);

            if let Some(blk_d) = cur.select_node("block")
            {
              let  block = read_block(blk_d);

              return ForStmt{var_name, expr, block};
            }
        }
    }


  panic!();
}


pub fn
read_block(start_nd: &Node)-> Block
{
  let  mut cur = start_nd.cursor();

  let  mut blk = Block::new();

  cur.advance(1);

    while let Some(d) = cur.select_node("statement")
    {
      blk.stmt_list.push(read_stmt(d));

      cur.advance(1);
    }


  blk
}


pub fn
read_stmt(start_nd: &Node)-> Stmt
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
        if s == ";"
        {
          return Stmt::Empty;
        }
    }

  else
    if let Some(d) = cur.get_node()
    {
      let  d_name = d.get_name();

        if d_name == "block"
        {
          return Stmt::Block(read_block(d));
        }

      else
        if d_name == "if"
        {
          return Stmt::If(read_if_stmt(d));
        }

      else
        if d_name == "for"
        {
          return Stmt::For(read_for_stmt(d));
        }

      else
        if d_name == "while"
        {
          return read_while(d);
        }

      else
        if d_name == "loop"
        {
          return read_loop(d);
        }

      else
        if d_name == "break"
        {
          return Stmt::Break;
        }

      else
        if d_name == "continue"
        {
          return Stmt::Continue;
        }

      else
        if d_name == "return"
        {
          return read_return(d);
        }

      else
        if d_name == "declaration"
        {
          return Stmt::Decl(read_decl(d));
        }

      else
        if d_name == "expression"
        {
          let  e = read_expr(d);

          return Stmt::Expr(e);
        }

      else
        if d_name == "assign"
        {
          return read_assign(d);
        }

      else
        if d_name == "print"
        {
          return read_print(d);
        }
    }


  panic!();
}




