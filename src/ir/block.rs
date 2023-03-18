

use std::convert::From;
use super::memory::Word;




#[derive(Clone)]
pub enum
AddressSource
{
    GlobalOffset(i64),
     LocalOffset(i64),
  ArgumentOffset(i64),

}


impl
AddressSource
{


pub fn
print(&self)
{
    match self
    {
  AddressSource::GlobalOffset(i)=>{print!("g:{}",i);},
  AddressSource::LocalOffset(i)=> {print!("l:{}",i);},
  AddressSource::ArgumentOffset(i)=> {print!("a:{}",i);},
    }
}


}




#[derive(Clone)]
pub enum
OperandLiteral
{
  Identifier(String),

  ImmediateValue(Word),

}


impl
OperandLiteral
{


pub fn
from(name: &str)-> OperandLiteral
{
  OperandLiteral::Identifier(String::from(name))
}


pub fn
print(&self)
{
    match self
    {
  OperandLiteral::Identifier(s)=>{print!("{}",s);},
  OperandLiteral::ImmediateValue(w)=>{print!("(imm, i:{})",w.get_i64());},
    }
}


}




#[derive(Clone)]
pub struct
Operand
{
  pub(crate) literal: OperandLiteral,

  pub(crate) address_source: Option<AddressSource>,

}


impl
Operand
{


pub fn
print(&self)
{
  self.literal.print();

  print!("(");

    if let Some(adr_src) = &self.address_source
    {
      adr_src.print();
    }

  else
    {
      print!("?");
    }


  print!(")");
}


}


impl
From<&str> for Operand
{


fn
from(id: &str)-> Operand
{
  Operand{ literal: OperandLiteral::from(id), address_source: None}
}


}


impl
From<i64> for Operand
{


fn
from(i: i64)-> Operand
{
  Operand{ literal: OperandLiteral::ImmediateValue(Word::from(i)), address_source: None}
}


}


impl
From<u64> for Operand
{


fn
from(u: u64)-> Operand
{
  Operand{ literal: OperandLiteral::ImmediateValue(Word::from(u)), address_source: None}
}


}


impl
From<f64> for Operand
{


fn
from(f: f64)-> Operand
{
  Operand{ literal: OperandLiteral::ImmediateValue(Word::from(f)), address_source: None}
}


}


impl
From<i32> for Operand
{


fn
from(i: i32)-> Operand
{
  Operand{ literal: OperandLiteral::ImmediateValue(Word::from(i)), address_source: None}
}


}


impl
From<f32> for Operand
{


fn
from(f: f32)-> Operand
{
  Operand{ literal: OperandLiteral::ImmediateValue(Word::from(f)), address_source: None}
}


}




pub fn
new_operand_list()-> Vec<Operand>
{
  Vec::new()
}




#[derive(Clone)]
pub struct
VariableLink
{
  pub(crate) name: String,

  pub(crate) address_source: Option<AddressSource>,

}


impl
VariableLink
{


pub fn
new(name: &str)-> VariableLink
{
  VariableLink{ name: String::from(name), address_source: None}
}


pub fn
print(&self)
{
  print!("{}(",&self.name);

    if let Some(adr_src) = &self.address_source
    {
      adr_src.print();
    }

  else
    {
      print!("?");
    }


  print!(")");
}


}




#[derive(Clone)]
pub struct
BlockLink
{
  pub(crate) name: String,

  pub(crate) index: Option<u64>,

}


impl
BlockLink
{


pub fn
new(name: &str)-> BlockLink
{
  BlockLink{ name: String::from(name), index: None}
}


pub fn
print(&self)
{
  print!("{}",&self.name);

    if let Some(i) = self.index
    {
      print!("({})",i);
    }
}


}




#[derive(Clone,Copy)]
pub struct
WordCount
{
  number: u64,
}


impl
WordCount
{


pub fn  zero()-> WordCount{WordCount{ number: 0}}
pub fn   one()-> WordCount{WordCount{ number: 1}}
pub fn  from(n: u64)-> WordCount{WordCount{ number: n}}

pub fn  get_size(&self)-> u64{8*self.number}

pub fn  print(&self){print!("({} bytes)",self.get_size());}


}




#[derive(Clone)]
pub struct
CallInfo
{
  pub(crate) target: VariableLink,

  pub(crate) return_word_count: WordCount,

  pub(crate) argument_list: Vec<Operand>,

}


impl
CallInfo
{


pub fn
new(name: &str, ret_wc: WordCount)-> CallInfo
{
  CallInfo{ target: VariableLink::new(name), return_word_count: ret_wc, argument_list: Vec::new()}
}


pub fn
push(&mut self, o: Operand)
{
  self.argument_list.push(o);
}


pub fn
print(&self)
{
  self.target.print();

  print!(" ");

  self.return_word_count.print();

  print!(" ");

    for a in &self.argument_list
    {
      a.print();

      print!(", ");
    }
}


}


pub struct
BranchInfo
{
  pub(crate) condition: VariableLink,

  pub(crate) on_true:  BlockLink,
  pub(crate) on_false: BlockLink,

}


impl
BranchInfo
{


pub fn
new(cond: &str, on_true: &str, on_false: &str)-> BranchInfo
{
  BranchInfo{ condition: VariableLink::new(cond), on_true: BlockLink::new(on_true), on_false: BlockLink::new(on_false)}
}


pub fn
print(&self)
{
  self.condition.print();

  print!(" ");

  self.on_true.print();

  print!(" ");

  self.on_false.print();
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




pub enum
AllocatingOperation
{
  Unary(Operand,UnaryOperator),
  Binary(Operand,Operand,BinaryOperator),

  Allocate(WordCount),

  Address(VariableLink),

  Phi(Vec<PhiOperand>),
  Call(CallInfo),

}


impl
AllocatingOperation
{


pub fn
get_word_count(&self)-> WordCount
{
    match self
    {
  AllocatingOperation::Unary(_,_)=>   {WordCount::one()},
  AllocatingOperation::Binary(_,_,_)=>{WordCount::one()},
  AllocatingOperation::Allocate(wc)=> {*wc},
  AllocatingOperation::Address(_)=>{WordCount::one()},
  AllocatingOperation::Phi(_)=>  {WordCount::one()},
  AllocatingOperation::Call(ci)=>{ci.return_word_count},
    }
}


pub fn
print(&self)
{
    match self
    {
  AllocatingOperation::Unary(o,u)=>
        {
          o.print();

          print!(" ");

          u.print();
        },
  AllocatingOperation::Binary(l,r,b)=>
        {
          l.print();

          print!(" ");

          r.print();

          print!(" ");

          b.print();
        },
  AllocatingOperation::Allocate(wc)=>
        {
          print!("allocate {}",wc.number);
        },
  AllocatingOperation::Address(o)=>
        {
          print!("address ");

          o.print();
        },
  AllocatingOperation::Phi(ops)=>
        {
          print!("phi ");

            for o in ops
            {
              o.from.print();

              print!(" ");

              o.value.print();

              print!(",");
            }
        },
  AllocatingOperation::Call(ci)=>
        {
          print!("cal ");

          ci.print();
        },
    }
}


}
pub enum
NonAllocatingOperation
{
  CopyWord(VariableLink,VariableLink),
  CopyString(VariableLink,VariableLink,usize),
  Message(String),
  Print(VariableLink,char),

}


impl
NonAllocatingOperation
{


pub fn
print(&self)
{
    match self
    {
  NonAllocatingOperation::CopyWord(src,dst)=>
        {
          print!("copy_word (src)");

          src.print();

          print!(" (dst)");

          dst.print();
        },
  NonAllocatingOperation::CopyString(dst,src,sz)=>
        {
          print!("copy_string (src)");

          src.print();

          print!(" (dst)");

          dst.print();

          print!("{}",*sz);
        },
  NonAllocatingOperation::Message(s)=>
        {
          print!("message \"{}\"",s);
        },
  NonAllocatingOperation::Print(target,c)=>
        {
          print!("print ");

          target.print();

          print!(" {}",c);
        },
    }
}


}




pub enum
Line
{
     AllocatingOperation(VariableLink,AllocatingOperation),
  NonAllocatingOperation(NonAllocatingOperation),

}


impl
Line
{


pub fn
print(&self)
{
    match self
    {
  Line::AllocatingOperation(vl,ao)=>
        {
          vl.print();

          print!(" = ");

          ao.print();
        }
  Line::NonAllocatingOperation(nao)=>
        {
          nao.print();
        }
    }
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
print(&self)
{
    match self
    {
  Terminator::Jump(ln)=>
        {
          print!("jmp ");

          ln.print();
        },
  Terminator::Branch(bi)=>
        {
          print!("br ");
          bi.condition.print();
          print!(" ");
          bi.on_true.print();
          print!(" ");
          bi.on_false.print();
        },
  Terminator::Return(o_opt)=>
        {
          print!("ret ");

            if let Some(o) = o_opt
            {
              o.print();
            }
        },
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
  PhiOperand{ from: BlockLink::new(blk_name), value: o}
}


}


pub fn
new_phi_operand_list()-> Vec<PhiOperand>
{
  Vec::new()
}




pub struct
Block
{
  pub(crate) name: String,

  pub(crate) line_list: Vec<Line>,

  pub(crate) terminator: Option<Terminator>,

}


impl
Block
{


pub fn
new(name: &str)-> Block
{
  Block{ name: String::from(name), line_list: Vec::new(), terminator: None}
}




pub fn
set_jmp(&mut self, label: &str)
{
  self.terminator = Some(Terminator::Jump(BlockLink::new(label)));
}


pub fn
set_br(&mut self, var_name: &str, on_true: &str, on_false: &str)
{
  let  bi = BranchInfo{ condition: VariableLink::new(var_name), on_true: BlockLink::new(on_true), on_false: BlockLink::new(on_false)};

  self.terminator = Some(Terminator::Branch(bi));
}


pub fn
set_ret(&mut self, o_opt: Option<Operand>)
{
  self.terminator = Some(Terminator::Return(o_opt));
}




pub fn
print(&self)
{
  print!(":{}\n",&self.name);

    for l in &self.line_list
    {
      print!("  ");

      l.print();

      print!("\n");
    }


    if let Some(t) = &self.terminator
    {
      print!("  ");

      t.print();
    }


  print!("\n");
}


}




