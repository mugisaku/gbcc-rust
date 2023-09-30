

use super::memory::{
  Word,
  WORD_SIZE,
};

use super::allocation::{
  Allocation,
  AllocationID,
  Operand,
  Source,
  Destination,
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




pub trait
JumpOperand
{


fn  new(name: &str)-> Self;

fn  get_name(&self)-> &String;
fn  set_index(&mut self, i: usize);

fn
link(&mut self, ls: &Vec<(String,usize)>)-> Result<(),()>
{
    for e in ls
    {
        if e.0 == self.get_name().as_str()
        {
          self.set_index(e.1);

          return Ok(());
        }
    }


  Err(())
}


}




pub struct
JumpDestination
{
  pub(crate) name: String,
  pub(crate) index: usize,

}


impl
JumpOperand for JumpDestination
{


fn
new(name: &str)-> JumpDestination
{
  JumpDestination{
    name: String::from(name),
    index: 0,
  }
}


fn  get_name(&self)-> &String{&self.name}
fn  set_index(&mut self, i: usize){self.index = i;}


}




pub struct
JumpSource
{
  pub(crate) name: String,
  pub(crate) index: usize,

}


impl
JumpOperand for JumpSource
{


fn
new(name: &str)-> JumpSource
{
  JumpSource{
    name: String::from(name),
    index: 0,
  }
}


fn  get_name(&self)-> &String{&self.name}
fn  set_index(&mut self, i: usize){self.index = i;}


}




pub struct
CallInfo
{
  pub(crate) function_name: String,
  pub(crate) function_index: usize,

  pub(crate) argument_list: Vec<Source>,

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
#[allow(dead_code)]
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
PhiPair
{
  pub(crate)  from: JumpSource,
  pub(crate) value: Source,

}


pub struct
PhiPairListWrapper
{
  pub(crate) list: Vec<PhiPair>,

}


impl
PhiPairListWrapper
{


pub fn
add(mut self, from: &str, value: &str)-> PhiPairListWrapper
{
  self.list.push(PhiPair{from: JumpSource::new(from), value: Source::new(value)});

  self
}


pub fn
release(mut self)-> Vec<PhiPair>
{
  self.list
}


}


pub fn
build_phi_pairs()-> PhiPairListWrapper
{
  PhiPairListWrapper{
    list: Vec::new(),
  }
}




pub enum
AllocatingOperation
{
   Unary(Source,UnaryOperator),
  Binary(Source,Source,BinaryOperator),

  Allocate,

  MoveU64(u64),
  MoveF64(f64),

  Address(Source),

  Phi(Vec<PhiPair>,Source),
  Call(CallInfo),

}


impl
AllocatingOperation
{


pub fn
link_to_allocation(&mut self, g_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, para_ls: &Vec<Allocation>)-> Result<(),()>
{
    match self
    {
  AllocatingOperation::Unary(o,_)=>
        {
          o.link(g_alo_ls,l_alo_ls,para_ls)
        },
  AllocatingOperation::Binary(l,r,_)=>
        {
            if  l.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
             && r.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  AllocatingOperation::Address(l)=>
        {
          l.link(g_alo_ls,l_alo_ls,para_ls)
        },
  AllocatingOperation::Phi(ops,defau)=>
        {
            for o in ops
            {
                if o.value.link(g_alo_ls,l_alo_ls,para_ls).is_err()
                {
                  return Err(());
                }
            }


          defau.link(g_alo_ls,l_alo_ls,para_ls)
        },
  AllocatingOperation::Call(ci)=>
        {
            for a in &mut ci.argument_list
            {
                if a.link(g_alo_ls,l_alo_ls,para_ls).is_err()
                {
                  return Err(());
                }
            }


          Ok(())
        },
  _=>
        {
          Ok(())
        },
    }
}


pub fn
print(&self)
{
    match self
    {
  AllocatingOperation::Unary(o,u)=>
        {
          print!("{} <",&o.name);

          u.print();

          print!(">");
        },
  AllocatingOperation::Binary(l,r,b)=>
        {
          print!("{} {} <",&l.name,&r.name);

          b.print();

          print!(">");
        },
  AllocatingOperation::Allocate=>
        {
          print!("<allocate>");
        },
  AllocatingOperation::MoveU64(u)=>
        {
          print!("<movu64> {}",*u);
        },
  AllocatingOperation::MoveF64(f)=>
        {
          print!("<movf64> {}",*f);
        },
  AllocatingOperation::Address(l)=>
        {
          print!("<address> {}",&l.name);
        },
  AllocatingOperation::Phi(ops,defau)=>
        {
          print!("<phi> ");

            for o in ops
            {
              print!("[{} {}]",&o.from.name,&o.value.name);

              print!(",");
            }


          print!("  defau {}",&defau.name);
        },
  AllocatingOperation::Call(ci)=>
        {
          print!("<cal> {}(",&ci.function_name);

            for a in &ci.argument_list
            {
              print!("{}, ",&a.name);
            }


          print!(")");
        },
    }
}


}




