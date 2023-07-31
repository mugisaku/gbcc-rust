

use super::memory::{
  Word,
  WORD_SIZE,
};

use super::allocation::{
  Allocation,
  AllocationLink,
};

use super::block::{
  BlockLink,
};

use super::function::{
  FunctionLink,
};

use super::collection::{
  Collection,

};




#[derive(Clone)]
pub enum
Operand
{
  AllocationLink(AllocationLink),

  ImmediateValue(Word),

}


impl
Operand
{


pub fn
from_identifier(name: &str)-> Operand
{
  Operand::AllocationLink(AllocationLink::new(name))
}


pub fn
from_u32(u: u32)-> Operand
{
  Operand::ImmediateValue(Word::from(u))
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>)-> Result<(),()>
{
    if let Operand::AllocationLink(ln) = self
    {
      ln.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)
    }

  else
    {
      Ok(())
    }
}


pub fn
print(&self, coll: &Collection)
{
    match self
    {
  Operand::AllocationLink(l)=>{l.print(coll);},
  Operand::ImmediateValue(w)=>{print!("(imm, i:{})",w.get_i64());},
    }
}


}




pub fn
new_operand_list()-> Vec<Operand>
{
  Vec::new()
}




pub struct
CallInfo
{
  pub(crate) target: FunctionLink,

  pub(crate) return_size: usize,

  pub(crate) argument_list: Vec<Operand>,

}


impl
CallInfo
{


pub fn
new(alo_name: &str, ret_sz: usize)-> CallInfo
{
  CallInfo{target: FunctionLink::Unresolved(String::from(alo_name)), return_size: ret_sz, argument_list: Vec::new()}
}


pub fn
push(&mut self, o: Operand)
{
  self.argument_list.push(o);
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
  let  mut new_fln_opt: Option<FunctionLink> = None;

    if let FunctionLink::Unresolved(name) = &self.target
    {
        for i in 0..fname_ls.len()
        {
            if fname_ls[i] == name.as_str()
            {
              new_fln_opt = Some(FunctionLink::Resolved(i));

              break;
            }
        }


        if let None = new_fln_opt
        {
          println!("CallInfo::resolve error: target resolve is failed");

          return Err(());
        }
    }


    if let Some(new_fln) = new_fln_opt
    {
      self.target = new_fln;
    }


    for a in &mut self.argument_list
    {
        if a.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_err()
        {
          println!("CallInfo::resolve error: argument_list resolve is failed");

          return Err(());
        }
    }


  Ok(())
}


pub fn
print(&self, coll: &Collection)
{
  self.target.print(coll);

  print!(" (RET_SZ: {}) ",self.return_size);

    for a in &self.argument_list
    {
      a.print(coll);

      print!(", ");
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
print(&self, coll: &Collection)
{
  self.condition.print(coll);

  print!(" ");

  self.on_true.print(coll);

  print!(" ");

  self.on_false.print(coll);
}


}




#[derive(Clone,Copy)]
pub enum
UnaryOperator
{
  ExS8,
  ExS16,
  ExS32,
  ExF32,

  StoF,
  FtoS,

  Not,

  Neg,
  NegF,

  LogicalNot,

}


impl
UnaryOperator
{


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::ExS8=>{print!("exs8");}
  UnaryOperator::ExS16=>{print!("exs16");}
  UnaryOperator::ExS32=>{print!("exs32");}
  UnaryOperator::ExF32=>{print!("exs32");}
  UnaryOperator::StoF=>{print!("stof");}
  UnaryOperator::FtoS=>{print!("ftos");}
  UnaryOperator::Not=>{print!("not");}
  UnaryOperator::Neg=>{print!("neg");}
  UnaryOperator::NegF=>{print!("negf");}
  UnaryOperator::LogicalNot=>{print!("logical_not");}
    }
}


}




#[derive(Clone,Copy)]
pub enum
BinaryOperator
{
  AddI, SubI, MulI, DivI, RemI,
  AddU, SubU, MulU, DivU, RemU,
  AddF, SubF, MulF, DivF, RemF,

  Shl, Shr, Or, And, Xor,

  Eq, Neq,

  LtI, LteqI, GtI, GteqI,
  LtU, LteqU, GtU, GteqU,
  LtF, LteqF, GtF, GteqF,

  LogicalAnd, LogicalOr,

}


impl
BinaryOperator
{


pub fn
print(&self)
{
    match self
    {
  BinaryOperator::AddI=>{print!("addi");},
  BinaryOperator::SubI=>{print!("subi");},
  BinaryOperator::MulI=>{print!("muli");},
  BinaryOperator::DivI=>{print!("divi");},
  BinaryOperator::RemI=>{print!("remi");},

  BinaryOperator::AddU=>{print!("addu");},
  BinaryOperator::SubU=>{print!("subu");},
  BinaryOperator::MulU=>{print!("mulu");},
  BinaryOperator::DivU=>{print!("divu");},
  BinaryOperator::RemU=>{print!("remu");},

  BinaryOperator::AddF=>{print!("addf");},
  BinaryOperator::SubF=>{print!("subf");},
  BinaryOperator::MulF=>{print!("mulf");},
  BinaryOperator::DivF=>{print!("divf");},
  BinaryOperator::RemF=>{print!("remf");},

  BinaryOperator::Shl=>{print!("shl");},
  BinaryOperator::Shr=>{print!("shr");},
  BinaryOperator::Or=>{print!("or");},
  BinaryOperator::And=>{print!("and");},
  BinaryOperator::Xor=>{print!("xor");},

  BinaryOperator::Eq=>{print!("eq");},
  BinaryOperator::Neq=>{print!("neq");},

  BinaryOperator::LtI=>{print!("lti");},
  BinaryOperator::LteqI=>{print!("lteqi");},
  BinaryOperator::GtI=>{print!("gti");},
  BinaryOperator::GteqI=>{print!("gteqi");},

  BinaryOperator::LtU=>{print!("ltu");},
  BinaryOperator::LteqU=>{print!("ltequ");},
  BinaryOperator::GtU=>{print!("gtu");},
  BinaryOperator::GteqU=>{print!("gtequ");},

  BinaryOperator::LtF=>{print!("ltf");},
  BinaryOperator::LteqF=>{print!("lteqf");},
  BinaryOperator::GtF=>{print!("gtf");},
  BinaryOperator::GteqF=>{print!("gteqf");},

  BinaryOperator::LogicalAnd=>{print!("logical_and");},
  BinaryOperator::LogicalOr=>{print!("logical_or");},
    }
}


}




pub struct
PhiOperand
{
  pub(crate)  from: BlockLink,
  pub(crate) value: Operand,

}


impl
PhiOperand
{


pub fn
make(blk_name: &str, o: Operand)-> PhiOperand
{
  PhiOperand{from: BlockLink::Unresolved(String::from(blk_name)), value: o}
}


}


pub fn
new_phi_operand_list()-> Vec<PhiOperand>
{
  Vec::new()
}




pub enum
AllocatingOperation
{
  Unary(Operand,UnaryOperator),
  Binary(Operand,Operand,BinaryOperator),

  Allocate(usize),

  Address(AllocationLink),

  Phi(Vec<PhiOperand>),
  Call(CallInfo),

}


impl
AllocatingOperation
{


pub fn
get_size(&self)-> usize
{
    match self
    {
  AllocatingOperation::Unary(_,_)=>   {WORD_SIZE},
  AllocatingOperation::Binary(_,_,_)=>{WORD_SIZE},
  AllocatingOperation::Allocate(sz)=> {*sz},
  AllocatingOperation::Address(_)=>{WORD_SIZE},
  AllocatingOperation::Phi(_)=>  {WORD_SIZE},
  AllocatingOperation::Call(ci)=>{ci.return_size},
    }
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
    match self
    {
  AllocatingOperation::Unary(o,_)=>{o.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls)},
  AllocatingOperation::Binary(l,r,_)=>
        {
            if l.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            && r.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              println!("AllocatingOperation::resolve error: Binary resolve is failed");

              Err(())
            }
        },
  AllocatingOperation::Phi(phio_ls)=>
        {
            for phio in phio_ls
            {
                if phio.value.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls).is_err()
                {
                  println!("AllocatingOperation::resolve error: Phi resolve is failed");

                  return Err(());
                }
            }


          return Ok(());
        },
  AllocatingOperation::Call(ci)=>{ci.resolve(fi,p_alo_ls,l_alo_ls,g_alo_ls,fname_ls)},
  _=>{Ok(())},
    }
}


pub fn
print(&self, coll: &Collection)
{
    match self
    {
  AllocatingOperation::Unary(o,u)=>
        {
          o.print(coll);

          print!(" ");

          u.print();
        },
  AllocatingOperation::Binary(l,r,b)=>
        {
          l.print(coll);

          print!(" ");

          r.print(coll);

          print!(" ");

          b.print();
        },
  AllocatingOperation::Allocate(sz)=>
        {
          print!("allocate {}",sz);
        },
  AllocatingOperation::Address(l)=>
        {
          print!("address ");

          l.print(coll);
        },
  AllocatingOperation::Phi(ops)=>
        {
          print!("phi ");

            for o in ops
            {
              o.from.print(coll);

              o.value.print(coll);

              print!(",");
            }
        },
  AllocatingOperation::Call(ci)=>
        {
          print!("cal ");

          ci.print(coll);
        },
    }
}


}




