

use super::*;
use super::expr::*;
use super::decl::*;
use super::ty::*;
use super::symbol_table::*;
use super::scope::*;
use super::evaluate::*;
use super::stmt::*;
use super::asm::*;




pub enum
ExecResult
{
  Ok,

  Break,
  Continue,
  Return(Option<EvalResult>),

  Err,

}




pub fn
execute_block_as_const(blk: &Block, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
    for stmt in blk.get_stmt_list()
    {
        match execute_stmt_as_const(stmt,tbl,scp)
        {
      ExecResult::Ok         =>{}
      ExecResult::Break      =>{return ExecResult::Break;}
      ExecResult::Continue   =>{return ExecResult::Continue;}
      ExecResult::Return(opt)=>{return ExecResult::Return(opt);}
      ExecResult::Err        =>{return ExecResult::Err;}
        }
    }


  ExecResult::Ok
}


pub fn
execute_if_stmt_as_const(if_stmt: &IfStmt, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
    if let EvalResult::Bool(first_b) = evaluate(if_stmt.get_condition(),tbl,Some(scp))
    {
        if first_b
        {
          return execute_block_as_const(if_stmt.get_block(),tbl,scp);
        }


        for elif in if_stmt.get_elif_stmt_list()
        {
            if let EvalResult::Bool(b) = evaluate(elif.get_condition(),tbl,Some(scp))
            {
                if b
                {
                  return execute_block_as_const(elif.get_block(),tbl,scp);
                }
            }

          else
            {
              println!("value type is not bool ");

              return ExecResult::Err;
            }
        }


        if let Some(blk) = if_stmt.get_else_block()
        {
          return execute_block_as_const(blk,tbl,scp);
        }
    }

  else
    {
      println!("value type is not bool ");

      return ExecResult::Err;
    }


  ExecResult::Ok
}


pub fn
execute_for_stmt_as_const(for_stmt: &ForStmt, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
  let  res = evaluate(for_stmt.get_expr(),tbl,Some(scp));

    if let Result::<usize,()>::Ok(n) = res.try_into()
    {
      let  mut new_scp = Scope::new(scp);

      new_scp.add_const_int(for_stmt.get_var_name(),0);

        loop
        {
            match execute_block_as_const(for_stmt.get_block(),tbl,&new_scp)
            {
          ExecResult::Ok         =>{}
          ExecResult::Break      =>{break;}
          ExecResult::Continue   =>{}
          ExecResult::Return(opt)=>{return ExecResult::Return(opt);}
          ExecResult::Err        =>{return ExecResult::Err;}
            }


          let  sym = new_scp.find(for_stmt.get_var_name()).unwrap();

          let  val = sym.get_value();

            if let SymbolValue::IntV(i) = val
            {
              i.set(i.get()+1);

                if i.get() == n as i64
                {
                  break;
                }
            }
        }
    }

  else
    {
      return ExecResult::Err;
    }


  ExecResult::Ok
}


pub fn
execute_loop_as_const(blk: &Block, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
    loop
    {
        match execute_block_as_const(blk,tbl,scp)
        {
      ExecResult::Ok         =>{}
      ExecResult::Break      =>{break;}
      ExecResult::Continue   =>{}
      ExecResult::Return(opt)=>{return ExecResult::Return(opt);}
      ExecResult::Err        =>{return ExecResult::Err;}
        }
    }


  ExecResult::Ok
}


pub fn
execute_while_as_const(e: &Expr, blk: &Block, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
    loop
    {
        if let EvalResult::Bool(b) = evaluate(e,tbl,Some(scp))
        {
            if b
            {
                match execute_block_as_const(blk,tbl,scp)
                {
              ExecResult::Ok         =>{}
              ExecResult::Break      =>{break;}
              ExecResult::Continue   =>{}
              ExecResult::Return(opt)=>{return ExecResult::Return(opt);}
              ExecResult::Err        =>{return ExecResult::Err;}
                }
            }

          else
            {
              break;
            }
        }

      else
        {
          return ExecResult::Err;
        }
    }


  ExecResult::Ok
}


pub fn
execute_stmt_as_const(stmt: &Stmt, tbl: &SymbolTable, scp: &Scope)-> ExecResult
{
    match stmt
    {
  Stmt::Empty=>{ExecResult::Ok}
  Stmt::Block(blk)=>{execute_block_as_const(blk,tbl,scp)}
  Stmt::Expr(e)=>
    {
      evaluate(e,tbl,Some(scp));

      ExecResult::Ok
    }
  Stmt::Decl(_,_)    =>{  println!("can not   decl in execute as const.");  ExecResult::Err}
  Stmt::Assign(_,_,_)=>{  println!("can not assign in execute as const.");  ExecResult::Err}
  Stmt::If(i)       =>{execute_if_stmt_as_const(i,tbl,scp)}
  Stmt::Loop(blk)   =>{execute_loop_as_const(blk,tbl,scp)}
  Stmt::While(e,blk)=>{execute_while_as_const(e,blk,tbl,scp)}
  Stmt::For(f)      =>{execute_for_stmt_as_const(f,tbl,scp)}
  Stmt::Break       =>{ExecResult::Break}
  Stmt::Continue    =>{ExecResult::Continue}
  Stmt::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          ExecResult::Return(Some(evaluate(e,tbl,Some(scp))))
        }

      else
        {
          ExecResult::Return(None)
        }
    }
  Stmt::Print(e)=>
    {
      print!("**PRINT**: ");

      evaluate(e,tbl,Some(scp)).print();

      print!("\n");

      ExecResult::Ok
    }
    }
}



 
