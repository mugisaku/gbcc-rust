

use crate::syntax::print_indent;

use super::get_aligned_size;
use super::declaration::{
  Declaration,
  Storage,
  Component,

};

use super::operation::{
  Source,
  Destination,
  Operation,

};

use super::expression::{
  Expression,

};

use super::typesystem::{
  TypeInfo,

};

use super::compile::{
  RecordManager,

};


#[derive(Clone)]
pub enum
AssignOperator
{
  Nop,
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,

}


impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Nop=>{print!("=");},
  AssignOperator::Add=>{print!("+=");},
  AssignOperator::Sub=>{print!("-=");},
  AssignOperator::Mul=>{print!("*=");},
  AssignOperator::Div=>{print!("/=");},
  AssignOperator::Rem=>{print!("%=");},
  AssignOperator::Shl=>{print!("<<=");},
  AssignOperator::Shr=>{print!(">>=");},
  AssignOperator::And=>{print!("&=");},
  AssignOperator::Or=>{print!("|=");},
  AssignOperator::Xor=>{print!("^=");},
    }
}


}




pub enum
StorageKind
{
  Static,
  Const,
  Argument,
  Local,

}


pub struct
StorageInfo
{
  name: String,

  type_info: TypeInfo,

  position: usize,

  kind: StorageKind,

  expression_ptr: *const Expression,

}


pub struct
Scope
{
  parent_ptr: *const Scope,

  storage_info_list: Vec<StorageInfo>,

  begin: usize,
    end: usize,

}


impl
Scope
{


pub fn
new()-> Self
{
  Self{
    parent_ptr: std::ptr::null(),
    storage_info_list: Vec::new(),
    begin: 0,
      end: 0,
  }
}


fn
read(decl: &Declaration, pos: &mut usize, ls: &mut Vec<StorageInfo>)
{
    match &decl.component
    {
  Component::Var(sto)=>
        {
          let  mut type_info = sto.type_info.clone();

          let  position = *pos;

          *pos += type_info.get_size();

          let  si = StorageInfo{
            name: decl.name.clone(),
            type_info,
            position,
            kind: StorageKind::Local,
            expression_ptr: (&sto.expression) as *const Expression,
          };


          ls.push(si);
        }
  Component::Static(sto)=>
        {
          let  si = StorageInfo{
            name: decl.name.clone(),
            type_info: TypeInfo::new_void(),
            position: *pos,
            kind: StorageKind::Static,
            expression_ptr: std::ptr::null(),
          };


          ls.push(si);
        }
  Component::Const(sto)=>
        {
          let  si = StorageInfo{
            name: decl.name.clone(),
            type_info: TypeInfo::new_void(),
            position: *pos,
            kind: StorageKind::Const,
            expression_ptr: std::ptr::null(),
          };


          ls.push(si);
        }
  _=>{panic!();}
    }
}


pub fn
new_child(&self, ls: &Vec<Statement>)-> Self
{
  let  mut storage_info_list: Vec<StorageInfo> = Vec::new();

  let  mut end = self.end;

    for stmt in ls
    {
        if let Statement::Declaration(decl) = stmt
        {
          Self::read(&decl,&mut end,&mut storage_info_list);
        }
    }


  Self{
    parent_ptr: self as *const Scope,
    storage_info_list,
    begin: self.end,
      end,
  }
}


}




pub struct
ConditionalBlock
{
  pub(crate)      condition: Expression,
  pub(crate) statement_list: Vec<Statement>,

}


pub enum
Statement
{
  Empty,
  Declaration(Declaration),
  Block(Vec<Statement>),
  If(Vec<ConditionalBlock>),
  While(ConditionalBlock),
  Loop(Vec<Statement>),
  For(Vec<Statement>),
  Break,
  Continue,
  Return(Expression),
  Expression(Expression,Option<(AssignOperator,Expression)>),

}


impl
Statement
{


pub fn
print_statement_list(ls: &Vec<Statement>, indent: usize)
{
  print!("\n");

  print_indent(indent);

  print!("{{\n");

    for stmt in ls
    {
      stmt.print(indent+1);

      print!(";\n");
    }


  print_indent(indent);

  print!("}}\n");
}


pub fn
print_conditional_block(cond_blk: &ConditionalBlock, indent: usize)
{
  cond_blk.condition.print();

  print!("\n");

  print_indent(indent);

  print!("{{\n");

  Self::print_statement_list(&cond_blk.statement_list,indent+1);

  print_indent(indent);

  print!("}}\n");
}


pub fn
print(&self, indent: usize)
{
  print_indent(indent);

    match self
    {
  Statement::Empty=>{},
  Statement::Declaration(d)=>{d.print();},
  Statement::Block(ls)=>
        {
          Self::print_statement_list(ls,indent);
        },
  Statement::If(ls)=>
        {
          print!("if ");

            if let Some(first) = ls.first()
            {
              Self::print_conditional_block(first,indent);

                for i in 1..ls.len()
                {
                  print_indent(indent);

                  print!("else");

                  let  cond_blk = &ls[i];

                    if let Expression::None = &cond_blk.condition
                    {
                    }

                  else
                    {
                      print!(" if ");
                    }


                  Self::print_conditional_block(cond_blk,indent);
                }
            }
        },
  Statement::For(ls)=>{},
  Statement::While(cond_blk)=>
        {
          print!("while ");

          Self::print_conditional_block(cond_blk,indent);
        },
  Statement::Loop(ls)=>
        {
          print!("loop\n");

          Self::print_statement_list(ls,indent);
        },
  Statement::Break=>{print!("break");},
  Statement::Continue=>{print!("continue");},
  Statement::Return(e)=>
        {
          print!("return ");

          e.print();
        },
  Statement::Expression(e,ass_opt)=>
        {
          e.print();

            if let Some((o,r)) = ass_opt
            {
              o.print();

              r.print();
            }
        },
    }
}


}




pub struct
Compiler
{
  number: usize,
  pub(crate) operation_list: Vec<Operation>,

}


impl
Compiler
{


pub fn
new()-> Self
{
  Self{
    number: 0,
    operation_list: Vec::new(),
  }
}


fn
generate_name(&mut self, s: &str)-> String
{
  let  n = self.number;

  self.number += 1;

  format!("ST{}{}",s,n)
}


fn
push(&mut self, op: Operation)
{
  self.operation_list.push(op);
}


fn
push_jump(&mut self, base: &str, tail: &str)
{
  let  dst = Destination{name: format!("{}_{}",base,tail)};

  self.operation_list.push(Operation::Jump(dst));
}


fn
push_label(&mut self, base: &str, tail: &str)
{
  self.operation_list.push(Operation::Label(format!("{}_{}",base,tail)));
}


fn
compile_if(&mut self, ls: &Vec<ConditionalBlock>, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
  let  base_name = self.generate_name("if");

  let  len = ls.len();

  let  mut name_ls: Vec<String> = Vec::new();

/*
    for i in 0..len
    {
      let  expr_name = format!("{}Expr{}",&base_name,i);

      let  (cond_ptr,_) = &ls[i];

      unsafe{&**cond_ptr}.compile(&expr_name,scope,&mut self.operation_list);

      let  label = format!("{}{}_START",&base_name,i);

      let  dst = Destination{name: label.clone()};
      let  src = Source{name: expr_name};

      self.push(Operation::BranchIfNonZero(dst,src));

      name_ls.push(label);
    }


    for i in 0..len
    {
      let  (_,stmt_ls_ptr) = &e_ls[i];

      self.push_label(&name_ls[i],"");

      self.compile_statement_list(unsafe{&**stmt_ls_ptr},ctrl_name_opt,scope);

      self.push_jump(&base_name,"_END");
    }


  self.push_label(&base_name,"_END");
*/

  Ok(())
}


fn
compile_while(&mut self, cond_blk: &ConditionalBlock, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
  let  while_name = self.generate_name("while");
  let   cond_name = self.generate_name("expr");

  self.push_label(&while_name,"_RESTART");

    if let Ok(ti) = cond_blk.condition.compile(&cond_name,scope,&mut self.operation_list)
    {
    }

  else
    {
      println!("compile_block error: while condition compile is failed");

      return Err(());
    }


  let  src = Source{name: cond_name};
  let  dst = Destination{name: while_name.clone()};

  self.push(Operation::BranchIfZero(dst,src));

    if self.compile_statement_list(&cond_blk.statement_list,Some(&while_name),scope).is_err()
    {
      println!("compile_block error");

      return Err(());
    }


  self.push_label(&while_name,"END");

  Ok(())
}


fn
compile_loop(&mut self, ls: &Vec<Statement>, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
  let  loop_name = self.generate_name("loop");

  self.push_label(&loop_name,"_RESTART");

  self.compile_statement_list(&ls,Some(&loop_name),scope);

  self.push_label(&loop_name,"_END");

  Ok(())
}


fn
compile_return(&mut self, e: &Expression, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
    if e.is_empty()
    {
      self.push(Operation::ReturnVoid);
    }

  else
    {
      let  name = self.generate_name("expr");

      e.compile(&name,scope,&mut self.operation_list);

      let  src = Source{name};

      self.push(Operation::ReturnNonVoid(src,0));
    }


  Ok(())
}


fn
compile_statement(&mut self, stmt: &Statement, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
    match stmt
    {
  Statement::Empty=>{},
  Statement::Declaration(d)=>
        {
            if let Some(st) = d.get_storage()
            {
                if let Ok(ti) = st.expression.compile(&d.name,scope,&mut self.operation_list)
                {
                }

              else
                {
                  println!("compile_statement error");

                  return Err(());
                }
            }
        },
  Statement::Block(ls)=>
        {
          return self.compile_statement_list(ls,ctrl_name_opt,scope);
        },
  Statement::If(ls)=>
        {
          return self.compile_if(ls,ctrl_name_opt,scope);
        },
  Statement::For(ls)=>
        {
        },
  Statement::While(cond_blk)=>
        {
          return self.compile_while(cond_blk,ctrl_name_opt,scope);
        },
  Statement::Loop(ls)=>
        {
          return self.compile_loop(ls,ctrl_name_opt,scope);
        },
  Statement::Break=>
        {
            if let Some(ctrl_name) = ctrl_name_opt
            {
              self.push_jump(&ctrl_name,"_END");
            }

          else
            {
              println!("compile_statement error");

              panic!();
            }
        },
  Statement::Continue=>
        {
            if let Some(ctrl_name) = ctrl_name_opt
            {
              self.push_jump(&ctrl_name,"_RESTART");
            }

          else
            {
              panic!();
            }
        },
  Statement::Return(e)=>
        {
          return self.compile_return(e,ctrl_name_opt,scope);
        },
  Statement::Expression(e,ass_opt)=>
        {
          let  l_name = self.generate_name("expr");

            if let Ok(l_ti) = e.compile(&l_name,scope,&mut self.operation_list)
            {
                if let Some((o,r)) = ass_opt
                {
                  let  r_name = self.generate_name("expr");

                    if let Ok(r_ti) = r.compile(&r_name,scope,&mut self.operation_list)
                    {
                    }

                  else
                    {
                      println!("compile_statement error");

                      return Err(());
                    }
                }
            }

          else
            {
              println!("compile_statement error");

              return Err(());
            }
        },
    }


  Ok(())
}


fn
compile_statement_list(&mut self, ls: &Vec<Statement>, ctrl_name_opt: Option<&str>, scope: &Scope)-> Result<(),()>
{
    for stmt in ls
    {
        if self.compile_statement(stmt,ctrl_name_opt,scope).is_err()
        {
          println!("compile_statement_list error");

          return Err(());
        }
    }


  Ok(())
}


}




pub fn
compile_statement_list(ls: &Vec<Statement>, scope: &Scope)-> Result<Vec<Operation>,()>
{
  let  mut cmplr = Compiler::new();

    if cmplr.compile_statement_list(ls,None,scope).is_ok()
    {
      return Ok(cmplr.operation_list);
    }


  Err(())
}




