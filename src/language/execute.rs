

use super::expr::*;
use super::decl::*;
use super::ty::*;
use super::symbol_table::*;
use super::evaluate::*;
use super::stmt::*;




pub enum
ExecResult
{
  Ok,

  Break,
  Continue,
  Return(Option<EvalResult>),

  Err,

}


pub struct
Scope
{
  symbol_list: Vec<Symbol>,

  next_offset: usize,

}


impl
Scope
{


pub fn
new()-> Self
{
  Self{
    symbol_list: Vec::new(),
    next_offset: 0,
  }
}


pub fn
find(&self, name: &str)-> Option<&Symbol>
{
    for sym in self.symbol_list.iter().rev()
    {
        if sym.get_name() == name
        {
          return Some(sym);
        }
    }


  None
}


pub fn
find_mut(&mut self, name: &str)-> Option<&mut Symbol>
{
    for sym in self.symbol_list.iter_mut().rev()
    {
        if sym.get_name() == name
        {
          return Some(sym);
        }
    }


  None
}


}




pub struct
ExecContext<'a>
{
  depth: usize,

  argument_list: Vec<Symbol>,

  symbol_table_opt: Option<&'a SymbolTable>,

  scope_stack: Vec<Scope>,

}


impl<'a>
ExecContext<'a>
{


pub fn
new()-> Self
{
  Self{
    depth: 0,
    argument_list: Vec::new(),
    symbol_table_opt: None,
    scope_stack: Vec::new(),
  }
}


pub fn
new_with_symbol_table(symbol_table: &'a SymbolTable)-> Self
{
  Self{
    depth: 0,
    argument_list: Vec::new(),
    symbol_table_opt: Some(symbol_table),
    scope_stack: Vec::new(),
  }
}


pub fn
new_with_parent(parent: &Self)-> Self
{
  Self{
    depth: parent.depth+1,
    argument_list: Vec::new(),
    symbol_table_opt: parent.symbol_table_opt,
    scope_stack: Vec::new(),
  }
}


pub fn
enter_scope(&mut self)
{
  self.scope_stack.push(Scope::new());
}


pub fn
add_argument_as_const_var(&mut self, name: &str, eval_result: EvalResult)-> &mut Self
{
  let  sym = Symbol::new(name,SymbolKind::ConstVar(eval_result));

  self.argument_list.push(sym);

  self
}


pub fn
add_argument_as_temp_var(&mut self, name: &str, ty: SizedTy)-> &mut Self
{
  let  sym = Symbol::new(name,SymbolKind::TempVar(TempVar::new_with_ty(ty)));

  self.argument_list.push(sym);

  self
}


pub fn
add_const_var(&mut self, name: &str, res: EvalResult)
{
    if let Some(scp) = self.scope_stack.last_mut()
    {
      scp.symbol_list.push(Symbol::new(name,SymbolKind::ConstVar(res)));
    }
}


pub fn
add_temp_var(&mut self, name: &str, ty: SizedTy)
{
    if let Some(scp) = self.scope_stack.last_mut()
    {
      scp.symbol_list.push(Symbol::new(name,SymbolKind::TempVar(TempVar::new_with_ty(ty))));
    }
}


pub fn
find_symbol(&self, name: &str)-> Option<&Symbol>
{
    for scp in self.scope_stack.iter().rev()
    {
        if let Some(sym) = scp.find(name)
        {
          return Some(sym);
        }
    }


    for sym in self.argument_list.iter()
    {
        if sym.get_name() == name
        {
          return Some(sym);
        }
    }


    if let Some(tbl) = &self.symbol_table_opt
    {
      return tbl.find(name);
    }


  None
}


pub fn
find_symbol_mut(&mut self, name: &str)-> Option<&mut Symbol>
{
    for scp in self.scope_stack.iter_mut().rev()
    {
        if let Some(sym) = scp.find_mut(name)
        {
          return Some(sym);
        }
    }


    for sym in self.argument_list.iter_mut()
    {
        if sym.get_name() == name
        {
          return Some(sym);
        }
    }


  None
}


pub fn
get_eval_result(&self, name: &str)-> Option<EvalResult>
{
  Some(EvalResult::Void)
}


pub fn
exit_scope(&mut self)
{
  let  _ = self.scope_stack.pop();
}


}




pub fn
execute_block_as_const(blk: &Block, ctx: &mut ExecContext)-> ExecResult
{
    for stmt in blk.get_stmt_list()
    {
        match execute_stmt_as_const(stmt,ctx)
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
execute_if_stmt_as_const(if_stmt: &IfStmt, ctx: &mut ExecContext)-> ExecResult
{
    if let EvalResult::Bool(first_b) = evaluate(if_stmt.get_condition(),ctx)
    {
        if first_b
        {
          return execute_block_as_const(if_stmt.get_block(),ctx);
        }


        for elif in if_stmt.get_elif_stmt_list()
        {
            if let EvalResult::Bool(b) = evaluate(elif.get_condition(),ctx)
            {
                if b
                {
                  return execute_block_as_const(elif.get_block(),ctx);
                }
            }
        }


        if let Some(blk) = if_stmt.get_else_block()
        {
          return execute_block_as_const(blk,ctx);
        }
    }


  ExecResult::Ok
}


pub fn
execute_for_stmt_as_const(for_stmt: &ForStmt, ctx: &mut ExecContext)-> ExecResult
{
  let  res = evaluate(for_stmt.get_expr(),ctx);

    if let Result::<usize,()>::Ok(n) = res.try_into()
    {
      ctx.enter_scope();

      ctx.add_const_var(for_stmt.get_var_name(),EvalResult::USize(0));

        loop
        {
            match execute_block_as_const(for_stmt.get_block(),ctx)
            {
          ExecResult::Ok         =>{}
          ExecResult::Break      =>{break;}
          ExecResult::Continue   =>{}
          ExecResult::Return(opt)=>{return ExecResult::Return(opt);}
          ExecResult::Err        =>{return ExecResult::Err;}
            }


          let  sym = ctx.find_symbol_mut(for_stmt.get_var_name()).unwrap();

            if let SymbolKind::ConstVar(res) = sym.get_kind_mut()
            {
                if let EvalResult::USize(i) = res
                {
                  *i += 1;

                    if *i == n
                    {
                      break;
                    }
                }
            }
        }


      ctx.exit_scope();
    }

  else
    {
      return ExecResult::Err;
    }


  ExecResult::Ok
}


pub fn
execute_loop_as_const(blk: &Block, ctx: &mut ExecContext)-> ExecResult
{
    loop
    {
        match execute_block_as_const(blk,ctx)
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
execute_while_as_const(e: &Expr, blk: &Block, ctx: &mut ExecContext)-> ExecResult
{
    loop
    {
        if let EvalResult::Bool(b) = evaluate(e,ctx)
        {
            if b
            {
                match execute_block_as_const(blk,ctx)
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
execute_stmt_as_const(stmt: &Stmt, ctx: &mut ExecContext)-> ExecResult
{
    match stmt
    {
  Stmt::Empty=>{ExecResult::Ok}
  Stmt::Block(blk)=>{execute_block_as_const(blk,ctx)}
  Stmt::Expr(e)=>
    {
      evaluate(e,ctx);

      ExecResult::Ok
    }
  Stmt::Decl(_)      =>{  println!("can not   decl in execute as const.");  ExecResult::Err}
  Stmt::Assign(_,_,_)=>{  println!("can not assign in execute as const.");  ExecResult::Err}
  Stmt::If(i)       =>{execute_if_stmt_as_const(i,ctx)}
  Stmt::Loop(blk)   =>{execute_loop_as_const(blk,ctx)}
  Stmt::While(e,blk)=>{execute_while_as_const(e,blk,ctx)}
  Stmt::For(f)      =>{execute_for_stmt_as_const(f,ctx)}
  Stmt::Break       =>{ExecResult::Break}
  Stmt::Continue    =>{ExecResult::Continue}
  Stmt::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          ExecResult::Return(Some(evaluate(e,ctx)))
        }

      else
        {
          ExecResult::Return(None)
        }
    }
  Stmt::Print(e)=>
    {
      print!("**PRINT**: ");

      evaluate(e,ctx).print();

      print!("\n");

      ExecResult::Ok
    }
    }
}



 
