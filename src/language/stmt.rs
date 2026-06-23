

use crate::node::*;
use crate::source_file::SourceInfo;
use super::expr::*;
use super::decl::*;
use super::evaluate::*;
use super::evaluate_const::*;
use super::asm::*;
use super::scope::*;
use super::symbol_table::*;




pub struct
IfStmt
{
  cond_block_list: Vec<(Expr,Block)>,

  else_block_opt: Option<Block>,

}


impl
IfStmt
{


pub fn  get_cond_block_list(&self)-> &Vec<(Expr,Block)>{&self.cond_block_list}
pub fn  get_else_block_opt(&self)-> &Option<Block>{&self.else_block_opt}

pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    for (e,blk) in &self.cond_block_list
    {
      e.collect_string(buf);
      blk.collect(buf);
    }


    if let Some(blk) = &self.else_block_opt
    {
      blk.collect(buf);
    }
}


pub fn
is_executable_as_const(&self)-> bool
{
    for (_,blk) in &self.cond_block_list
    {
        if blk.is_executable_as_const()
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
  let  mut iter = self.cond_block_list.iter();


    if let Some((e,blk)) = iter.next()
    {
      print!("if ");

      e.print();
      blk.print();

      print!("\n");
    }


    while let Some((e,blk)) = iter.next()
    {
      print!("else if ");

      e.print();
      blk.print();

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
collect(&self, buf: &mut Vec<Collectible>)
{
   self.expr.collect_string(buf);
  self.block.collect(buf);
}


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
collect(&self, buf: &mut Vec<Collectible>)
{
    for stmt in &self.stmt_list
    {
      stmt.collect(buf);
    }
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
StmtKind
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
  Halt,

  Return(Option<Expr>),

  Print(Expr),

}




pub struct
Stmt
{
  source_info: SourceInfo,
  kind: StmtKind,

}


impl
Stmt
{


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"statement")
    {
      return Ok(read_stmt(&nd));
    }


  Err(())
}




pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
get_kind(&self)-> &StmtKind
{
  &self.kind
}


pub fn
collect(&self, buf: &mut Vec<Collectible>)
{
    match &self.kind
    {
  StmtKind::Empty=>{}
  StmtKind::Block(blk)=>{blk.collect(buf);}
  StmtKind::Decl(decl)=>{decl.collect_string(buf);}
  StmtKind::Expr(e)=>{e.collect_string(buf);}
  StmtKind::If(i)=>{i.collect(buf);}
  StmtKind::Loop(blk)=>{blk.collect(buf);}
  StmtKind::While(e,blk)=>
    {
        e.collect_string(buf);
      blk.collect(buf);
    }
  StmtKind::For(f)=>{f.collect(buf);}
  StmtKind::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          e.collect_string(buf);
        }
    }
  StmtKind::Assign(l,r,_)=>
    {
      l.collect_string(buf);
      r.collect_string(buf);
    }
  StmtKind::Print(e)=>{e.collect_string(buf);}
  _=>{}
    }
}


pub fn
is_executable_as_const(&self)-> bool
{
    match &self.kind
    {
  StmtKind::Block(blk)=>{blk.is_executable_as_const()}

  StmtKind::Expr(e)=>{true}
  StmtKind::If(i)       =>{i.is_executable_as_const()}
  StmtKind::Loop(blk)   =>{blk.is_executable_as_const()}
  StmtKind::While(e,blk)=>{blk.is_executable_as_const()}
  StmtKind::For(f)=>{f.is_executable_as_const()}
  StmtKind::Return(e_opt)=>
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
    match &self.kind
    {
  StmtKind::Empty=>{print!(";");}

  StmtKind::Block(blk)=>{blk.print();}

  StmtKind::Expr(e)=>{e.print();}
  StmtKind::Decl(decl)=>{decl.print();}
  StmtKind::Assign(l,r,op)=>
    {
      l.print();
      print!("{}",op);
      r.print();
    }
  StmtKind::If(i)=>{i.print();}
  StmtKind::Loop(blk)=>{  print!("loop");  blk.print();}
  StmtKind::While(e,blk)=>{  print!("while");  e.print();  blk.print();}
  StmtKind::For(f)=>{f.print();}
  StmtKind::Break=>{print!("break");}
  StmtKind::Continue=>{print!("continue");}
  StmtKind::Halt=>{print!("halt");}

  StmtKind::Return(e_opt)=>
    {
      print!("return ");

        if let Some(e) = e_opt
        {
          e.print();
        }
    }
  StmtKind::Print(e)=>
    {
      print!("print ");

      e.print();
    }
    }
}


}




pub fn
read_assign(start_nd: &Node)-> StmtKind
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

              return StmtKind::Assign(l,r,op);
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
read_return(start_nd: &Node)-> StmtKind
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      let  e = read_expr(nd);

      return StmtKind::Return(Some(e));
    }


  StmtKind::Return(None)
}


pub fn
read_print(start_nd: &Node)-> StmtKind
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(nd) = cur.select_node("expression")
    {
      let  e = read_expr(nd);

      return StmtKind::Print(e);
    }


  panic!();
}


pub fn
read_if_block(start_nd: &Node)-> (Expr,Block)
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

          return (condition,block);
        }
    }


  panic!();
}


pub fn
read_if_stmt(start_nd: &Node)-> IfStmt
{
  let  mut cur = start_nd.cursor();

  let  mut cond_block_list = Vec::<(Expr,Block)>::new();

    if let Some(first_d) = cur.select_node("if_block")
    {
      let  mut else_block_opt = None;

      cond_block_list.push(read_if_block(first_d));

      cur.advance(1);

        while cur.is_keyword()
        {
          cur.advance(1);

            if let Some(ifblk_d) = cur.select_node("if_block")
            {
              cond_block_list.push(read_if_block(ifblk_d));

              cur.advance(1);
            }

          else
            if let Some(blk_d) = cur.select_node("block")
            {
              else_block_opt = Some(read_block(blk_d));

              break;
            }

          else
            {panic!();}
        }


      return IfStmt{cond_block_list, else_block_opt};
    }


  panic!();
}


pub fn
read_while(start_nd: &Node)-> StmtKind
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

          return StmtKind::While(e,blk);
        }
    }


  panic!();
}


pub fn
read_loop(start_nd: &Node)-> StmtKind
{
  let  mut cur = start_nd.cursor();

  cur.advance(1);

    if let Some(ls_d) = cur.select_node("block")
    {
      return StmtKind::Loop(read_block(ls_d));
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
      let  stmt = read_stmt(d);

        if let StmtKind::Empty = &stmt.kind
        {
        }

      else
        {
          blk.stmt_list.push(stmt);
        }


      cur.advance(1);
    }


  blk
}


pub fn
read_stmt(start_nd: &Node)-> Stmt
{
  let  source_info = start_nd.get_source_info().clone();

  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
        if s == ";"
        {
          return Stmt{source_info, kind: StmtKind::Empty};
        }
    }

  else
    if let Some(d) = cur.get_node()
    {
      let  d_name = d.get_name();

        if d_name == "block"
        {
          return Stmt{source_info, kind: StmtKind::Block(read_block(d))}
        }

      else
        if d_name == "if"
        {
          return Stmt{source_info, kind: StmtKind::If(read_if_stmt(d))}
        }

      else
        if d_name == "for"
        {
          return Stmt{source_info, kind: StmtKind::For(read_for_stmt(d))}
        }

      else
        if d_name == "while"
        {
          return Stmt{source_info, kind: read_while(d)}
        }

      else
        if d_name == "loop"
        {
          return Stmt{source_info, kind: read_loop(d)}
        }

      else
        if d_name == "break"
        {
          return Stmt{source_info, kind: StmtKind::Break}
        }

      else
        if d_name == "continue"
        {
          return Stmt{source_info, kind: StmtKind::Continue}
        }

      else
        if d_name == "halt"
        {
          return Stmt{source_info, kind: StmtKind::Halt}
        }

      else
        if d_name == "return"
        {
          return Stmt{source_info, kind: read_return(d)}
        }

      else
        if d_name == "declaration"
        {
          return Stmt{source_info, kind: StmtKind::Decl(read_decl(d))}
        }

      else
        if d_name == "expression"
        {
          let  e = read_expr(d);

          return Stmt{source_info, kind: StmtKind::Expr(e)}
        }

      else
        if d_name == "assign"
        {
          return Stmt{source_info, kind: read_assign(d)}
        }

      else
        if d_name == "print"
        {
          return Stmt{source_info, kind: read_print(d)}
        }
    }


  panic!();
}




