

use super::memory::{
  Word,
  WORD_SIZE,
};

use super::allocation::{
  Allocation,
  AllocationLink,
};

use super::allocating_operation::{
  AllocatingOperation,
  Operand,
  PhiOperand,

};

use super::line::{
  Line,

};


use super::function::{
  Function,

};

use super::collection::{
  Collection,

};




#[derive(Clone)]
pub enum
BlockLink
{
  Unresolved(String),
     Resolved(usize),

}


impl
BlockLink
{


pub fn
new(name: &str)-> BlockLink
{
  BlockLink::Unresolved(String::from(name))
}


pub fn
resolve(&mut self, name_ls: &Vec<String>)-> Result<(),()>
{
    if let BlockLink::Unresolved(name) = self
    {
        for i in 0..name_ls.len()
        {
            if name_ls[i] == name.as_str()
            {
              *self = BlockLink::Resolved(i);

              return Ok(());
            }
        }


      println!("BlockLink::resolve error: block <{}> is not found",name);
    }


  Err(())
}


pub fn
print(&self, coll: &Collection, f: &Function)
{
    match self
    {
  BlockLink::Unresolved(name)=>{print!("{}(UNRESOLVED)",name);},
  BlockLink::Resolved(i)=>
        {
            if let Some(blk) = f.get_block(*i)
            {
              print!("{}",&blk.name);
            }
        },
    }
}


}




pub struct
BranchInfo
{
  pub(crate) condition: AllocationLink,

  pub(crate) on_true:  BlockLink,
  pub(crate) on_false: BlockLink,

}


impl
BranchInfo
{


pub fn
new(alo_name: &str, on_true: &str, on_false: &str)-> BranchInfo
{
  BranchInfo{
    condition: AllocationLink::Unresolved(String::from(alo_name)),
     on_true: BlockLink::Unresolved(String::from(on_true )),
    on_false: BlockLink::Unresolved(String::from(on_false)),
  }
}


pub fn
print(&self, coll: &Collection, f: &Function)
{
  self.condition.print(coll,0);

  print!(" ");

  self.on_true.print(coll,f);

  print!(" ");

  self.on_false.print(coll,f);
}


}




pub enum
Terminator
{
  Jump(BlockLink),
  Branch(BranchInfo),
  Return(Option<Operand>),

}


impl
Terminator
{


pub fn
resolve_block_link(&mut self, name_ls: &Vec<String>)-> Result<(),()>
{
    match self
    {
  Terminator::Jump(dst)=>
        {
          dst.resolve(name_ls)
        },
  Terminator::Branch(bi)=>
        {
            if  bi.on_true.resolve(name_ls).is_ok()
            && bi.on_false.resolve(name_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Terminator::Return(o_opt)=>
        {
          Ok(())
        },
    }
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>)-> Result<(),()>
{
    match self
    {
  Terminator::Jump(dst)=>{Ok(())},
  Terminator::Branch(bi)=>
        {
          bi.condition.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
        },
  Terminator::Return(o_opt)=>
        {
            if let Some(o) = o_opt
            {
              o.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
            }

          else
            {
              Ok(())
            }
        },
    }
}


pub fn
print(&self, coll: &Collection, f: &Function)
{
    match self
    {
  Terminator::Jump(dst)=>
        {
          print!("jmp ");

          dst.print(coll,f);
        },
  Terminator::Branch(bi)=>
        {
          print!("br ");
          bi.print(coll,f);
        },
  Terminator::Return(o_opt)=>
        {
          print!("ret ");

            if let Some(o) = o_opt
            {
              o.print(coll);
            }
        },
    }
}




pub fn
jmp(name: &str)-> Terminator
{
  Terminator::Jump(BlockLink::new(name))
}


pub fn
br(cond: &str, on_true: &str, on_false: &str)-> Terminator
{
  Terminator::Branch(BranchInfo::new(cond,on_true,on_false))
}


pub fn
ret(op_opt: Option<Operand>)-> Terminator
{
  Terminator::Return(op_opt)
}


}




pub struct
Block
{
  pub(crate) name: String,

  pub(crate) line_list: Vec<Line>,

  pub(crate) terminator: Terminator,

}


impl
Block
{


pub fn
new(name: &str)-> Block
{
  Block{
    name: String::from(name),
    line_list: Vec::new(),
    terminator: Terminator::Return(None),
  }
}


pub fn
add_line(&mut self, ln: Line)
{
  self.line_list.push(ln);
}


pub fn
set_terminator(&mut self, term: Terminator)
{
  self.terminator = term;
}


fn
resolve_block_link_in_phi(op_ls: &mut Vec<PhiOperand>, name_ls: &Vec<String>)-> Result<(),()>
{
    for op in op_ls
    {
        if op.from.resolve(name_ls).is_err()
        {
          return Err(());
        }
    }


  Ok(())
}


pub fn
resolve_block_link(&mut self, name_ls: &Vec<String>)-> Result<(),()>
{
    for ln in &mut self.line_list
    {
        if let Line::AllocatingOperation(_,_,o) = ln
        {
            if let AllocatingOperation::Phi(op_ls) = o
            {
                if Self::resolve_block_link_in_phi(op_ls,name_ls).is_err()
                {
                  return Err(());
                }
            }
        }
    }


  self.terminator.resolve_block_link(name_ls)
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
    for ln in &mut self.line_list
    {
        if ln.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls,fname_ls).is_err()
        {
          println!("Block::resolve error: line resolve is failed.");

          return Err(());
        }
    }


  self.terminator.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
}


pub fn
print(&self, coll: &Collection, f: &Function)
{
  print!(":{}\n",&self.name);

    for l in &self.line_list
    {
      print!("  ");

      l.print(coll,f);

      print!("\n");
    }


  self.terminator.print(coll,f);

  print!("\n");
}


}




