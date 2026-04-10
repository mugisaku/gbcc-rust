

use crate::node::*;
use super::expr::*;
use super::decl::*;
use super::ty::*;
use super::evaluate::*;
use super::evaluate_const::*;
use super::asm::*;
use super::scope::*;
use super::symbol_table::*;




pub struct
LabelID
{
  value: usize,
}


impl
LabelID
{


pub fn
new()-> Self
{
  Self{value: 0}
}


pub fn
make_br_label_holder(&mut self)-> BrLabelHolder
{
  let  blh = BrLabelHolder::new(self.value);

  self.value += 1;

  blh
}


pub fn
make_ctrl_label_holder(&mut self)-> CtrlLabelHolder
{
  let  clh = CtrlLabelHolder::new(self.value);

  self.value += 1;

  clh
}


}




pub struct
BrLabelHolder
{
  base: String,
  number: usize,
  label: String,

}


impl
BrLabelHolder
{


pub fn
new(id: usize)-> Self
{
  let   base = format!("L{}_",id);
  let  label = format!("L{}_1",&base);

  Self{base, number: 1, label}
}


pub fn
get_label(&self)-> &String
{
  &self.label
}


pub fn
make_end_label(&self)-> String
{
  format!("{}_END",&self.base)
}


pub fn
increment(&mut self)
{
  self.number += 1;

  self.label = format!("{}{}",&self.base,self.number);
}


}




pub struct
CtrlLabelHolder
{
     on_break: String,
  on_continue: String,

}


impl
CtrlLabelHolder
{


pub fn
new(id: usize)-> Self
{
  Self{
       on_break: format!("L{}_END",id),
    on_continue: format!("L{}_RESTART",id),
  }
}


}


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
collect(&self, buf: &mut Vec<Collectible>)
{
  self.condition.collect(buf);
  self.block.collect(buf);

    for elif in &self.elif_stmt_list
    {
      elif.condition.collect(buf);
      elif.block.collect(buf);
    }


    if let Some(blk) = &self.else_block_opt
    {
      blk.collect(buf);
    }
}


pub fn
process(&self, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  mut blh = lid.make_br_label_holder();

  let  end_label = blh.make_end_label();

  let  txt = evaluate(&self.condition,tbl,Some(scp)).to_text();

    if txt.get_ty_name() == "bool"
    {
      output.push_eval_text(txt);

      output.push_brz(blh.get_label());

      self.block.process(tbl,ret_ty_name,lid,clh_opt,scp,output);

      output.push_jmp(&end_label);
    }

  else
    {panic!();}


    for elif in &self.elif_stmt_list
    {
      let  elif_txt = evaluate(&elif.condition,tbl,Some(scp)).to_text();

      output.push_label(blh.get_label());

      blh.increment();

        if elif_txt.get_ty_name() == "bool"
        {
          output.push_eval_text(elif_txt);

          output.push_brz(blh.get_label());

          elif.block.process(tbl,ret_ty_name,lid,clh_opt,scp,output);

          output.push_jmp(&end_label);
        }

      else
        {panic!();}
    }


    if let Some(blk) = &self.else_block_opt
    {
      output.push_label(blh.get_label());

      blk.process(tbl,ret_ty_name,lid,clh_opt,scp,output);
    }


  output.push_label(&end_label);
}


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
collect(&self, buf: &mut Vec<Collectible>)
{
   self.expr.collect(buf);
  self.block.collect(buf);
}


pub fn
process(&self, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  clh = lid.make_ctrl_label_holder();

  let  mut new_scp = Scope::new(scp);


  let  mut count_max_off = 0usize;

  let  mut count_max_txt = evaluate(&self.expr,tbl,Some(scp)).to_text();

    if count_max_txt.get_ty_name() == "i64"
    {
      count_max_off = new_scp.add_var("<FOR_COUNT_MAX>","i64");

      let  mut var_txt = AsmEvalText::new();

      var_txt.push_local_var(count_max_off,"i64");

      output.push_assign(var_txt,count_max_txt,"=");
    }

  else
    {panic!();}



  let  count_cur_off = new_scp.add_var(&self.var_name,"i64");

  let  mut init_txt = AsmEvalText::new();

  init_txt.push_local_var(count_cur_off,"i64");
  init_txt.push_opcode(Opcode::Push0);
  init_txt.push_opcode(Opcode::St64);

  output.push_eval_text(init_txt);

  output.push_label(&clh.on_continue);


  let  mut inc_txt = AsmEvalText::new();

  inc_txt.push_local_var(count_cur_off,"i64");
  inc_txt.push_opcode(Opcode::Dup);
  inc_txt.push_opcode(Opcode::Ld64);
  inc_txt.push_opcode(Opcode::Push1);
  inc_txt.push_opcode(Opcode::Addi);
  inc_txt.push_opcode(Opcode::St64);

  output.push_eval_text(inc_txt);


  let  mut cmp_txt = AsmEvalText::new();

  cmp_txt.push_local_var(count_cur_off,"i64");
  cmp_txt.push_opcode(Opcode::Ld64);
  cmp_txt.push_local_var(count_max_off,"i64");
  cmp_txt.push_opcode(Opcode::Ld64);
  cmp_txt.push_opcode(Opcode::Lti);

  output.push_eval_text(cmp_txt);

  output.push_brz(&clh.on_break);

  self.block.process(tbl,ret_ty_name,lid,Some(&clh),&new_scp,output);

  output.push_jmp(&clh.on_continue);

  output.push_label(&clh.on_break);
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
process(&self, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  mut new_scp = Scope::new(scp);

    for stmt in &self.stmt_list
    {
      stmt.process(tbl,ret_ty_name,lid,clh_opt,&mut new_scp,output);
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
collect(&self, buf: &mut Vec<Collectible>)
{
    match self
    {
  Self::Empty=>{}
  Self::Block(blk)=>{blk.collect(buf);}
  Self::Decl(decl)=>
    {
        match decl.get_kind()
        {
      DeclKind::Const(e)=>{e.collect(buf);}
      DeclKind::Var(e)=>  {e.collect(buf);}
      DeclKind::Static(_)=>{}
      _=>{panic!();}
        }
    }
  Self::Expr(e)=>{e.collect(buf);}
  Self::If(i)=>{i.collect(buf);;}
  Self::Loop(blk)=>{blk.collect(buf);}
  Self::While(e,blk)=>
    {
        e.collect(buf);
      blk.collect(buf);
    }
  Self::For(f)=>{f.collect(buf);}
  Self::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          e.collect(buf);
        }
    }
  Self::Assign(l,r,_)=>
    {
      l.collect(buf);
      r.collect(buf);
    }
  Self::Print(e)=>{e.collect(buf);}
  _=>{}
    }
}


pub fn
process(&self, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder> ,scp: &mut Scope, output: &mut AsmTable)
{
    match self
    {
  Self::Empty=>{}
  Self::Block(blk)=>{blk.process(tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Self::Decl(decl)=>
    {
        match decl.get_kind()
        {
      DeclKind::Const(e)=>
        {
          let  cres = evaluate_const(e,tbl,Some(scp));

            match cres
            {
          EvalConstResult::Void    =>{}
          EvalConstResult::Bool(b) =>{scp.add_const_bool( decl.get_name(),b);}
          EvalConstResult::Int(i)  =>{scp.add_const_int(  decl.get_name(),i);}
          EvalConstResult::Float(f)=>{scp.add_const_float(decl.get_name(),f);}
          _=>{panic!();}
            }
        }
      DeclKind::Var(e)=>
        {
          let  r_txt = evaluate(e,tbl,Some(scp)).to_text();

          let  off = scp.add_var(decl.get_name(),r_txt.get_ty_name());

          let  mut l_txt = AsmEvalText::new();

          l_txt.push_local_var(off,r_txt.get_ty_name());

          output.push_assign(l_txt,r_txt,"=");
        }
      DeclKind::Static(_)=>{}
      _=>{panic!();}
        }
    }
  Self::Expr(e)=>
    {
      let  res = evaluate(e,tbl,Some(scp));

        match res
        {
      EvalResult::Value(txt)=>
        {
          output.push_eval_text(txt);

          output.push_opcode(Opcode::Pop);
        }
      _=>{}
        }
    }
  Self::If(i)=>{i.process(tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Self::Loop(blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      blk.process(tbl,ret_ty_name,lid,Some(&clh),scp,output);

      output.push_jmp(&clh.on_continue);

      output.push_label(&clh.on_break);
    }
  Self::While(e,blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      let  txt = evaluate(e,tbl,Some(scp)).to_text();

        if txt.get_ty_name() == "bool"
        {
          output.push_eval_text(txt);

          output.push_brz(&clh.on_break);

          blk.process(tbl,ret_ty_name,lid,Some(&clh),scp,output);

          output.push_jmp(&clh.on_continue);

          output.push_label(&clh.on_break);
        }

      else
        {panic!();}
    }
  Self::For(f)=>{f.process(tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Self::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          let  mut txt = evaluate(e,tbl,Some(scp)).to_text();

            if txt.get_ty_name() == ret_ty_name
            {
                if txt.is_deref()
                {
                  txt.push_load();
                }


              output.push_eval_text(txt);
            }

          else
            {panic!("TYPE OF RETURN VALUE and TYPE OF EVALUATED VALUW are mismatched");}
        }

      else
        {
            if ret_ty_name == "void"
            {
              output.push_opcode(Opcode::Push0);
            }

          else{panic!();}
        }


      output.push_opcode(Opcode::Ret);
    }
  Self::Assign(l,r,op)=>
    {
      let  mut l_asm = evaluate(l,tbl,Some(scp)).to_text();
      let  mut r_asm = evaluate(r,tbl,Some(scp)).to_text();

todo!();
      output.push_opcode(Opcode::St64);
    }
  Self::Break=>
    {
      output.push_jmp(&clh_opt.unwrap().on_break);
    }
  Self::Continue=>
    {
      output.push_jmp(&clh_opt.unwrap().on_continue);
    }
  Self::Print(e)=>
    {
      let  mut txt = evaluate(e,tbl,Some(scp)).to_text();

      txt.push_opcode(Opcode::Pri);

      output.push_eval_text(txt);
    }
    }
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
              else_block_opt = Some(read_else(el_d));
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
      let  stmt = read_stmt(d);

        if let Stmt::Empty = stmt
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




