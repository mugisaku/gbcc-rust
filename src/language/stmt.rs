

use crate::node::*;
use super::expr::*;
use super::decl::*;
use super::ty::*;
use super::evaluate::*;
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
process(&self, tbl: &SymbolTable, ret_ty_str: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  mut blh = lid.make_br_label_holder();

  let  end_label = blh.make_end_label();

  let  res = evaluate(&self.condition,tbl,Some(scp));

    if let Ok(mut vp) = ValueProcess::try_from(res)
    {
        if vp.get_ty().is_bool()
        {
          output.push_table(vp.get_table_mut());
          output.push_brz(blh.get_label());

          self.block.process(tbl,ret_ty_str,lid,clh_opt,scp,output);

          output.push_jmp(&end_label);
        }

      else{panic!();}
    }

  else{panic!();}


    for elif in &self.elif_stmt_list
    {
      let  elif_res = evaluate(&elif.condition,tbl,Some(scp));

      output.push_label(blh.get_label());

      blh.increment();

        if let Ok(mut vp) = ValueProcess::try_from(elif_res)
        {
            if vp.get_ty().is_bool()
            {
              output.push_table(vp.get_table_mut());
              output.push_brz(blh.get_label());

              elif.block.process(tbl,ret_ty_str,lid,clh_opt,scp,output);

              output.push_jmp(&end_label);
            }

          else{panic!();}
        }

      else{panic!();}
    }


    if let Some(blk) = &self.else_block_opt
    {
      output.push_label(blh.get_label());

      blk.process(tbl,ret_ty_str,lid,clh_opt,scp,output);
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
process(&self, tbl: &SymbolTable, ret_ty_str: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  clh = lid.make_ctrl_label_holder();

  let  mut new_scp = Scope::new(scp);


  let  mut target_off = 0usize;

  let  target_res = evaluate(&self.expr,tbl,Some(scp));

    if let Ok(mut vp) = ValueProcess::try_from(target_res)
    {
        if vp.get_ty().is_int()
        {
          target_off = new_scp.add_var("<FOR_TARGET>",Ty::Int);

          output.push_li_local_addr(target_off);
          output.push_table(vp.get_table_mut());
          output.push_opcode(Opcode::St64);
        }

      else{panic!();}
    }

  else{panic!();}



  let  var_off = new_scp.add_var(&self.var_name,Ty::Int);

  output.push_li_local_addr(var_off);
  output.push_opcode(Opcode::Push0);
  output.push_opcode(Opcode::St64);


  output.push_label(&clh.on_continue);


  output.push_li_local_addr(var_off);
  output.push_opcode(Opcode::Dup);
  output.push_opcode(Opcode::Ld64);
  output.push_opcode(Opcode::Push1);
  output.push_opcode(Opcode::Addi);
  output.push_opcode(Opcode::St64);


  output.push_li_local_addr(var_off);
  output.push_opcode(Opcode::Ld64);
  output.push_li_local_addr(target_off);
  output.push_opcode(Opcode::Ld64);
  output.push_opcode(Opcode::Lti);
  output.push_brz(&clh.on_break);

  self.block.process(tbl,ret_ty_str,lid,Some(&clh),&new_scp,output);

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
process(&self, tbl: &SymbolTable, ret_ty_str: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmTable)
{
  let  mut new_scp = Scope::new(scp);

    for stmt in &self.stmt_list
    {
      stmt.process(tbl,ret_ty_str,lid,clh_opt,&mut new_scp,output);
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
  Decl(Decl,Option<usize>),
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
process(&self, tbl: &SymbolTable, ret_ty_str: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder> ,scp: &mut Scope, output: &mut AsmTable)
{
    match self
    {
  Self::Empty=>{}
  Self::Block(blk)=>{blk.process(tbl,ret_ty_str,lid,clh_opt,scp,output);}
  Self::Decl(decl,n_opt)=>
    {
        match decl.get_kind()
        {
      DeclKind::Const(e)=>
        {
          let  res = evaluate(e,tbl,Some(scp));

            match res
            {
          EvalResult::Void    =>{}
          EvalResult::Bool(b) =>{scp.add_const_bool( decl.get_name(),b);}
          EvalResult::Int(i)  =>{scp.add_const_int(  decl.get_name(),i);}
          EvalResult::Float(f)=>{scp.add_const_float(decl.get_name(),f);}
          _=>{panic!();}
            }
        }
      DeclKind::Var(e)=>
        {
          let  res = evaluate(e,tbl,Some(scp));

            if let Ok(mut vp) = ValueProcess::try_from(res)
            {
              let  off = scp.add_var(decl.get_name(),vp.get_ty().clone());

              output.push_li_local_addr(off);
              output.push_table(vp.get_table_mut());
              output.push_opcode(Opcode::St64);
            }

          else{panic!();}
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
      EvalResult::Value(mut vp)=>
        {
          output.push_table(vp.get_table_mut());
          output.push_opcode(Opcode::Pop);
        }
      EvalResult::Deref(mut ls,mc)=>
        {
          output.push_table(&mut ls);
          output.push_opcode(Opcode::Pop);
        }
      _=>{}
        }
    }
  Self::If(i)=>{i.process(tbl,ret_ty_str,lid,clh_opt,scp,output);}
  Self::Loop(blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      blk.process(tbl,ret_ty_str,lid,Some(&clh),scp,output);

      output.push_jmp(&clh.on_continue);

      output.push_label(&clh.on_break);
    }
  Self::While(e,blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      let  res = evaluate(e,tbl,Some(scp));

        if let Ok(mut vp) = ValueProcess::try_from(res)
        {
            if vp.get_ty().is_bool()
            {
              output.push_table(vp.get_table_mut());

              output.push_brz(&clh.on_break);

              blk.process(tbl,ret_ty_str,lid,Some(&clh),scp,output);

              output.push_jmp(&clh.on_continue);

              output.push_label(&clh.on_break);
            }

          else{panic!();}
        }

      else{panic!();}
    }
  Self::For(f)=>{f.process(tbl,ret_ty_str,lid,clh_opt,scp,output);}
  Self::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          let  res = evaluate(e,tbl,Some(scp));

            if let Ok(mut vp) = ValueProcess::try_from(res)
            {
              let  s = vp.get_ty().get_canonical_name();

                if &s == ret_ty_str
                {
                  output.push_table(vp.get_table_mut());
                }

              else{panic!("TYPE OF RETURN VALUE and TYPE OF EVALUATED VALUW are mismatched");}
            }

          else{panic!();}
        }

      else
        {
            if ret_ty_str == VOID_STR
            {
              output.push_opcode(Opcode::Push0);
            }

          else{panic!();}
        }


      output.push_opcode(Opcode::Ret);
    }
  Self::Assign(l,r,op)=>
    {
      let  l_res = evaluate(l,tbl,Some(scp));
      let  r_res = evaluate(r,tbl,Some(scp));

        if let EvalResult::Deref(mut l_stack,_) = l_res
        {
          output.push_table(&mut l_stack);
          output.push_opcode(Opcode::Dup);
          output.push_opcode(Opcode::Ld64);

            if let Ok(mut r_vp) = ValueProcess::try_from(r_res)
            {
              output.push_table(r_vp.get_table_mut());

                   if op ==  "+="{output.push_opcode(Opcode::Addi);}
              else if op ==  "-="{output.push_opcode(Opcode::Subi);}
              else if op ==  "*="{output.push_opcode(Opcode::Muli);}
              else if op ==  "/="{output.push_opcode(Opcode::Divi);}
              else if op ==  "%="{output.push_opcode(Opcode::Remi);}
              else if op == "<<="{output.push_opcode(Opcode::Shl );}
              else if op == ">>="{output.push_opcode(Opcode::Shr );}
              else if op ==  "&="{output.push_opcode(Opcode::And );}
              else if op ==  "|="{output.push_opcode(Opcode::Or  );}
              else if op ==  "^="{output.push_opcode(Opcode::Xor );}
              else if op ==   "="{                                 }
              else{panic!()}


              output.push_opcode(Opcode::St64);
            }
        }

      else{panic!();}
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
      let  res = evaluate(e,tbl,Some(scp));

        if let Ok(mut vp) = ValueProcess::try_from(res)
        {
          output.push_table(vp.get_table_mut());

          output.push_opcode(Opcode::Pri);
        }

      else{panic!();}
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
  Self::Decl(decl,_)=>{decl.print();}
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
          return Stmt::Decl(read_decl(d),None);
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




