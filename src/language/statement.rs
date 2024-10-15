

use super::expression::{
  Expression,
  AssignOperator,

};


use super::type_info::{
  Parameter,
  TypeInfo,
  StorageInfo,

};


pub struct
VariableInfo
{
  pub(crate) previous_ptr: *const VariableInfo,

  pub(crate)           name: String,
  pub(crate) expression_opt: Option<Expression>,

  pub(crate) storage_info: StorageInfo,

}


impl
VariableInfo
{


pub fn
new(name: String, expression_opt: Option<Expression>)-> Self
{
  Self{
    previous_ptr: std::ptr::null(),
    name,
    expression_opt,
    storage_info: StorageInfo::new(),
  }
}


pub fn
find(&self, name: &str)-> Option<&VariableInfo>
{
    if &self.name == name
    {
      return Some(self);
    }


    if self.previous_ptr != std::ptr::null()
    {
      return unsafe{&*self.previous_ptr}.find(name);
    }


  None
}


pub fn
print(&self)
{
  print!("{}({})",&self.name,self.storage_info.index);
}


}




pub enum
Statement
{
  Empty,
  Let(VariableInfo),
  Const(VariableInfo),
  Expression(Expression,Option<(AssignOperator,Expression)>),
  If(Vec<(Expression,Block)>,Option<Block>),
  While(Expression,Block),
  For(For),
  Loop(Block),
  Block(Block),
  Return(Option<Expression>),
  Break,
  Continue,
  PrintS(String),
  PrintV(String),

}


impl
Statement
{


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");}
  Statement::Let(vi)=>
        {
          print!("let  {}",&vi.name);

            if let Some(e) = &vi.expression_opt
            {
              print!(": ");

              e.print();
            }
        }
  Statement::Const(vi)=>
        {
          print!("const  {}",&vi.name);

            if let Some(e) = &vi.expression_opt
            {
              print!(": ");

              e.print();
            }
        }
  Statement::Expression(e,ass_opt)=>
        {
          e.print();

            if let Some((ass_op,r)) = ass_opt
            {
              ass_op.print();

              r.print();
            }
        }
  Statement::If(ls,blk_opt)=>
       {
            if let Some((first_e,first_blk)) = ls.first()
            {
              print!("if ");

              first_e.print();

              first_blk.print();

              print!("\n");

                for i in 1..ls.len()
                {
                  let  (e,blk) = &ls[i];

                  print!("else if ");

                  e.print();

                  blk.print();

                  print!("\n");
                }
            }


            if let Some(blk) = blk_opt
            {
              print!("else");

              blk.print();
            }
        }
  Statement::For(fo)=>
        {
          print!("for {} in ",&fo.current_vi.name);

          fo.get_end_expression().print();

          fo.block.print();
        }
  Statement::While(e,blk)=>
        {
          print!("while ");

          e.print();

          blk.print();
        }
  Statement::Loop(blk)=>
        {
          print!("loop");

          blk.print();
        }
  Statement::Block(blk)=>
        {
          print!("//plain block");

          blk.print();
        }
  Statement::Return(e_opt)=>
        {
          print!("return ");

            if let Some(e) = e_opt
            {
              e.print();
            }
        }
  Statement::Break=>{print!("break");}
  Statement::Continue=>{print!("continue");}
  Statement::PrintS(s)=>{print!("print \"{}\"",s);}
  Statement::PrintV(s)=>{print!("print {}",s);}
    }
}


}




pub struct
Block
{
  pub(crate) stack_allocation_count: usize,
  pub(crate) statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
new(statement_list: Vec<Statement>)-> Self
{
  Self{
    stack_allocation_count: 0,
    statement_list,
  }
}


pub fn
scan(&mut self)
{
  self.scan_internal(std::ptr::null(),0);
}


fn
link(cur: &mut VariableInfo, prev_ptr: *const VariableInfo)-> *const VariableInfo
{
  cur.previous_ptr = prev_ptr;

  cur as *const VariableInfo
}


fn
scan_internal(&mut self, mut last_vi_ptr: *const VariableInfo, base: usize)
{
  let  mut count           = 0;
  let  mut child_count_max = 0;

    for stmt in &mut self.statement_list
    {
        match stmt
        {
      Statement::Let(vi)=>
            {
              last_vi_ptr = Self::link(vi,last_vi_ptr);

              vi.storage_info.index = base+count;

              count += 1;
            }
      Statement::Const(vi)=>
            {
              last_vi_ptr = Self::link(vi,last_vi_ptr);

              vi.storage_info.index = base+count;

              count += 1;
            }
      Statement::Block(blk)=>
            {
              blk.scan_internal(last_vi_ptr,base+count);

              child_count_max = std::cmp::max(child_count_max,blk.stack_allocation_count);
            }
      Statement::For(fo)=>
            {
              let  ptr = Self::link(&mut fo.current_vi,last_vi_ptr);

              fo.current_vi.storage_info.index = base+count  ;
              fo.end_vi.storage_info.index     = base+count+1;

              fo.block.scan_internal(ptr,count+2);

              child_count_max = std::cmp::max(child_count_max,2+fo.block.stack_allocation_count);
            }
      Statement::While(e,blk)=>
            {
              blk.scan_internal(last_vi_ptr,base+count);

              child_count_max = std::cmp::max(child_count_max,blk.stack_allocation_count);
            }
      Statement::Loop(blk)=>
            {
              blk.scan_internal(last_vi_ptr,base+count);

              child_count_max = std::cmp::max(child_count_max,blk.stack_allocation_count);
            }
      _=>{}
        }
    }


  self.stack_allocation_count = count+child_count_max;
}


pub fn
print(&self)
{
  print!("{{\n");

    for stmt in &self.statement_list
    {
      stmt.print();

      print!("\n");
    }


  print!("}}\n");
}


}




pub struct
For
{
  pub(crate) current_vi: VariableInfo,
  pub(crate)     end_vi: VariableInfo,

  pub(crate) block: Block,

}


impl
For
{


pub fn
new(name: String, e: Expression, block: Block)-> Self
{
  Self{
    current_vi: VariableInfo::new(name,None),
        end_vi: VariableInfo::new("**FOR_END_VAR**".to_string(),Some(e)),
    block,
  }
}


pub fn
get_end_expression(&self)-> &Expression
{
    if let Some(e) = &self.end_vi.expression_opt
    {
      return e;
    }


  panic!();
}


}




