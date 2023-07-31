

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


use super::block::{
  BranchInfo,
  Terminator,
  Block,
  BlockLink,
};


use super::function::{
  FunctionLink,
};




pub enum
NonAllocatingOperation
{
  CopyWord(AllocationLink,AllocationLink),
  CopyString(AllocationLink,AllocationLink,usize),
  Message(String),
  Print(AllocationLink,char),

}


impl
NonAllocatingOperation
{


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>)-> Result<(),()>
{
    match self
    {
  NonAllocatingOperation::CopyWord(dst,src)=>
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
  NonAllocatingOperation::CopyString(dst,src,_)=>
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
  NonAllocatingOperation::Message(_)=>{Ok(())},
  NonAllocatingOperation::Print(target,_)=>
        {
          target.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
        },
    }
}


pub fn
print(&self, coll: &Collection)
{
    match self
    {
  NonAllocatingOperation::CopyWord(src,dst)=>
        {
          print!("copy_word ");

          src.print(coll);

          print!(" ");

          dst.print(coll);
        },
  NonAllocatingOperation::CopyString(dst,src,sz)=>
        {
          print!("copy_string ");

          src.print(coll);

          print!(" ");

          dst.print(coll);

          print!(" {}",*sz);
        },
  NonAllocatingOperation::Message(s)=>
        {
          print!("message \"{}\"",s);
        },
  NonAllocatingOperation::Print(target,c)=>
        {
          print!("print ");

          target.print(coll);

          print!(" {}",c);
        },
    }
}


}




pub enum
Line
{
     AllocatingOperation(AllocationLink,AllocatingOperation),
  NonAllocatingOperation(NonAllocatingOperation),

}


impl
Line
{


pub fn
get_allocation_data(&self)-> Option<(String,usize)>
{
    if let Line::AllocatingOperation(ln,op) = self
    {
        if let AllocationLink::Unresolved(name) = ln
        {
          return Some((name.clone(),op.get_size()));
        }
    }


  None
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
    match self
    {
  Line::AllocatingOperation(ln,ao)=>
        {
            if ln.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            && ao.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls,fname_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              println!("Line::resolve error: AllocatingOperation resolve is failed");

              Err(())
            }
        }
  Line::NonAllocatingOperation(nao)=>
        {
          nao.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
        }
    }
}


pub fn
print(&self, coll: &Collection)
{
    match self
    {
  Line::AllocatingOperation(ln,ao)=>
        {
          ln.print(coll);

          ao.print(coll);
        }
  Line::NonAllocatingOperation(nao)=>
        {
          nao.print(coll);
        }
    }
}




fn
un(dst: &str, o: Operand, u: UnaryOperator)-> Line
{
  let  ao = AllocatingOperation::Unary(o,u);

  Line::AllocatingOperation(AllocationLink::new(dst),ao)
}


pub fn        exs8(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::ExS8)}
pub fn       exs16(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::ExS16)}
pub fn       exs32(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::ExS32)}
pub fn       exf32(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::ExF32)}
pub fn        stof(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::StoF)}
pub fn        ftos(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::FtoS)}
pub fn         not(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::Not)}
pub fn logical_not(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::LogicalNot)}
pub fn         neg(dst: &str, o: Operand)-> Line{ Self::un(dst,o,UnaryOperator::Neg)}
pub fn        negf(dst: &str, o: Operand)-> Line{Self::un(dst,o,UnaryOperator::NegF)}




pub fn
bin(dst: &str, l: Operand, r: Operand, b: BinaryOperator)-> Line
{
  let  ao = AllocatingOperation::Binary(l,r,b);

  Line::AllocatingOperation(AllocationLink::new(dst),ao)
}


pub fn  addi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::AddI)}
pub fn  addu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::AddU)}
pub fn  addf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::AddF)}
pub fn  subi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::SubI)}
pub fn  subu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::SubU)}
pub fn  subf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::SubF)}
pub fn  muli(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::MulI)}
pub fn  mulu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::MulU)}
pub fn  mulf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::MulF)}
pub fn  divi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::DivI)}
pub fn  divu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::DivU)}
pub fn  divf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::DivF)}
pub fn  remi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::RemI)}
pub fn  remu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::RemU)}
pub fn  remf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::RemF)}
pub fn  shl(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Shl)}
pub fn  shr(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Shr)}
pub fn  and(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::And)}
pub fn   or(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Or)}
pub fn  xor(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Xor)}


pub fn     eq(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Eq)}
pub fn    neq(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::Neq)}
pub fn    lti(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LtI)}
pub fn    ltu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LtU)}
pub fn    ltf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LtF)}
pub fn  lteqi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LteqI)}
pub fn  ltequ(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LteqU)}
pub fn  lteqf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LteqF)}
pub fn    gti(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GtI)}
pub fn    gtu(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GtU)}
pub fn    gtf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GtF)}
pub fn  gteqi(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GteqI)}
pub fn  gtequ(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GteqU)}
pub fn  gteqf(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::GteqF)}

pub fn  logical_and(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LogicalAnd)}
pub fn   logical_or(dst: &str, l: Operand, r: Operand)-> Line{Self::bin(dst,l,r,BinaryOperator::LogicalOr)}


pub fn
alo(dst: &str, sz: usize)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),AllocatingOperation::Allocate(sz))
}


pub fn
addr(dst: &str, src: &str)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),AllocatingOperation::Address(AllocationLink::new(src)))
}


pub fn
phi(dst: &str, ops: Vec<PhiOperand>)-> Line
{
  Line::AllocatingOperation(AllocationLink::new(dst),AllocatingOperation::Phi(ops))
}


pub fn
cal(dst: &str, target: &str, ret_sz: usize, args: Vec<Operand>)-> Line
{
  let  ci = CallInfo{target: FunctionLink::new(target), return_size: ret_sz, argument_list: args};

  Line::AllocatingOperation(AllocationLink::new(dst),AllocatingOperation::Call(ci))
}


pub fn
cpyw(dst: &str, src: &str)-> Line
{
  let  dst_al = AllocationLink::new(dst);
  let  src_al = AllocationLink::new(src);

  Line::NonAllocatingOperation(NonAllocatingOperation::CopyWord(dst_al,src_al))
}


pub fn
cpys(dst: &str, src: &str, sz: usize)-> Line
{
  let  dst_al = AllocationLink::new(dst);
  let  src_al = AllocationLink::new(src);

  Line::NonAllocatingOperation(NonAllocatingOperation::CopyString(dst_al,src_al,sz))
}


pub fn
msg(s: &str)-> Line
{
  Line::NonAllocatingOperation(NonAllocatingOperation::Message(String::from(s)))
}


pub fn
pr(s: &str, c: char)-> Line
{
  Line::NonAllocatingOperation(NonAllocatingOperation::Print(AllocationLink::new(s),c))
}


}




pub fn
new_line_list()-> Vec<Line>
{
  Vec::new()
}




