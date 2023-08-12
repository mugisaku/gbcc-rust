

use super::allocation::{
  Allocation,
  AllocationLink,
};

use super::collection::{
  Collection,
};


use super::allocating_operation::{
  Operand,
  UnaryOperator,
  BinaryOperator,
  AllocatingOperation,
  PhiOperand,
  CallInfo,
};


use super::function::{
  Function,
  FunctionLink,
};

use super::memory::{
  WORD_SIZE,
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
resolve(&mut self, ls: &Vec<(String,usize)>)-> Result<(),()>
{
    if let BlockLink::Unresolved(name) = self
    {
        for e in ls
        {
            if e.0 == name.as_str()
            {
              *self = BlockLink::Resolved(e.1);

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
            if let Some(name) = f.get_block_name(*i)
            {
              print!("{}",name);
            }
        },
    }
}


}




pub enum
Line
{
  AllocatingOperation(AllocationLink,usize,AllocatingOperation),

  CopyWord(AllocationLink,AllocationLink),
  CopyString(AllocationLink,AllocationLink,usize),
  Message(String),
  Print(AllocationLink,char),
  BlockOpen(String),
  Jump(BlockLink),
  Branch(AllocationLink,BlockLink,BlockLink),
  Return(Option<Operand>),

}


#[allow(dead_code)]
impl
Line
{


pub fn
get_allocation_data(&self)-> Option<(String,usize)>
{
    if let Line::AllocatingOperation(ln,sz,op) = self
    {
        if let AllocationLink::Unresolved(name) = ln
        {
          return Some((name.clone(),*sz));
        }
    }


  None
}


pub fn
resolve(&mut self, fi: usize, blkop_ls: &Vec<(String,usize)>, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
    match self
    {
  Line::AllocatingOperation(ln,_,ao)=>
        {
            if ln.resolve(fi,         p_alo_ls,l_alo_ls,g_alo_ls         ).is_ok()
            && ao.resolve(fi,blkop_ls,p_alo_ls,l_alo_ls,g_alo_ls,fname_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              println!("Line::resolve error: AllocatingOperation resolve is failed");

              Err(())
            }
        }
  Line::CopyWord(dst,src)=>
        {
            if dst.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            && src.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Line::CopyString(dst,src,_)=>
        {
            if dst.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            && src.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Line::Message(_)=>{Ok(())},
  Line::Print(target,_)=>
        {
          target.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
        },
  Line::BlockOpen(_)=>
        {
          Ok(())
        },
  Line::Jump(dst)=>
        {
          dst.resolve(blkop_ls)
        },
  Line::Branch(cond,on_true,on_false)=>
        {
            if     cond.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            &&  on_true.resolve(blkop_ls).is_ok()
            && on_false.resolve(blkop_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Line::Return(o_opt)=>
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
  Line::AllocatingOperation(ln,_,ao)=>
        {
          ln.print(coll,1);

          print!(" = ");

          ao.print(coll,f);
        }
  Line::CopyWord(src,dst)=>
        {
          print!("copy_word ");

          src.print(coll,0);

          print!(" ");

          dst.print(coll,0);
        },
  Line::CopyString(dst,src,sz)=>
        {
          print!("copy_string ");

          src.print(coll,0);

          print!(" ");

          dst.print(coll,0);

          print!(" {}",*sz);
        },
  Line::Message(s)=>
        {
          print!("message \"{}\"",s);
        },
  Line::Print(target,c)=>
        {
          print!("print ");

          target.print(coll,0);

          print!(" {}",c);
        },
  Line::BlockOpen(name)=>
        {
          print!("BLOCK {}:",name);
        },
  Line::Jump(dst)=>
        {
          print!("jmp ");

          dst.print(coll,f);
        },
  Line::Branch(cond,on_true,on_false)=>
        {
          print!("br ");

          cond.print(coll,0);

          print!(" ");

          on_true.print(coll,f);

          print!(" ");

          on_false.print(coll,f);
        },
  Line::Return(o_opt)=>
        {
          print!("ret ");

            if let Some(o) = o_opt
            {
              o.print(coll);
            }
        },
    }
}




fn
un(dst: &str, sz: usize, o: Operand, u: UnaryOperator)-> Line
{
  let  ao = AllocatingOperation::Unary(o,u);

  Line::AllocatingOperation(AllocationLink::new(dst),sz,ao)
}


pub fn        exs8(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::ExS8)}
pub fn       exs16(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::ExS16)}
pub fn       exs32(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::ExS32)}
pub fn       exf32(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::ExF32)}
pub fn        stof(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::StoF)}
pub fn        ftos(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::FtoS)}
pub fn         not(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::Not)}
pub fn logical_not(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::LogicalNot)}
pub fn         neg(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::Neg)}
pub fn        negf(dst: &str, o: Operand)-> Line{Self::un(dst,WORD_SIZE,o,UnaryOperator::NegF)}




pub fn
bin(dst: &str, sz: usize, l: Operand, r: Operand, b: BinaryOperator)-> Line
{
  let  ao = AllocatingOperation::Binary(l,r,b);

  Line::AllocatingOperation(AllocationLink::new(dst),sz,ao)
}


pub fn  addi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::AddI)}
pub fn  addu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::AddU)}
pub fn  addf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::AddF)}
pub fn  subi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::SubI)}
pub fn  subu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::SubU)}
pub fn  subf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::SubF)}
pub fn  muli(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::MulI)}
pub fn  mulu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::MulU)}
pub fn  mulf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::MulF)}
pub fn  divi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::DivI)}
pub fn  divu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::DivU)}
pub fn  divf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::DivF)}
pub fn  remi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::RemI)}
pub fn  remu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::RemU)}
pub fn  remf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::RemF)}
pub fn  shl(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Shl)}
pub fn  shr(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Shr)}
pub fn  and(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::And)}
pub fn   or(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Or)}
pub fn  xor(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Xor)}


pub fn     eq(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Eq)}
pub fn    neq(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::Neq)}
pub fn    lti(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LtI)}
pub fn    ltu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LtU)}
pub fn    ltf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LtF)}
pub fn  lteqi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LteqI)}
pub fn  ltequ(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LteqU)}
pub fn  lteqf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LteqF)}
pub fn    gti(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GtI)}
pub fn    gtu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GtU)}
pub fn    gtf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GtF)}
pub fn  gteqi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GteqI)}
pub fn  gtequ(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GteqU)}
pub fn  gteqf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::GteqF)}

pub fn  logical_and(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LogicalAnd)}
pub fn   logical_or(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,WORD_SIZE,l,r,BinaryOperator::LogicalOr)}


pub fn
alo(dst: &str, sz: usize)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),sz,AllocatingOperation::Allocate)
}


pub fn
addr(dst: &str, src: &str)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),WORD_SIZE,AllocatingOperation::Address(AllocationLink::new(src)))
}


pub fn
phi(dst: &str, sz: usize, ops: Vec<PhiOperand>)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),sz,AllocatingOperation::Phi(ops))
}


pub fn
cal(dst: &str, ret_sz: usize, target: &str, args: Vec<Operand>)-> Line
{
  let  ci = CallInfo{target: FunctionLink::new(target), argument_list: args};

  Line::AllocatingOperation(AllocationLink::new(dst),ret_sz,AllocatingOperation::Call(ci))
}


pub fn
cpyw(dst: &str, src: &str)-> Line
{
  let  dst_al = AllocationLink::new(dst);
  let  src_al = AllocationLink::new(src);

  Line::CopyWord(dst_al,src_al)
}


pub fn
cpys(dst: &str, src: &str, sz: usize)-> Line
{
  let  dst_al = AllocationLink::new(dst);
  let  src_al = AllocationLink::new(src);

  Line::CopyString(dst_al,src_al,sz)
}


pub fn
msg(s: &str)-> Line
{
  Line::Message(String::from(s))
}


pub fn
pr(s: &str, c: char)-> Line
{
  Line::Print(AllocationLink::new(s),c)
}


pub fn
blkop(name: &str)-> Line
{
  Line::BlockOpen(String::from(name))
}


pub fn
jmp(name: &str)-> Line
{
  Line::Jump(BlockLink::new(name))
}


pub fn
br(cond: &str, on_true: &str, on_false: &str)-> Line
{
  Line::Branch(AllocationLink::new(cond),BlockLink::new(on_true),BlockLink::new(on_false))
}


pub fn
ret(op_opt: Option<Operand>)-> Line
{
  Line::Return(op_opt)
}




}




pub fn
new_line_list()-> Vec<Line>
{
  Vec::new()
}




