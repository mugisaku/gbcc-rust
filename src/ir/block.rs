

use super::memory::{
  Word,
};




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
Operand
{
  Identifier(String),

  ImmediateValue(Word),

}


impl
Operand
{


pub fn
print(&self)
{
    match self
    {
  Operand::Identifier(s)=>{print!("{}",s);},
  Operand::ImmediateValue(w)=>{print!("(imm, i:{})",w.get_i64());},
    }
}


}




pub fn
new_operand_list()-> Vec<Operand>
{
  Vec::new()
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
  pub(crate) target: String,

  pub(crate) return_word_count: WordCount,

  pub(crate) argument_list: Vec<Operand>,

}


impl
CallInfo
{


pub fn
new(name: &str, ret_wc: WordCount)-> CallInfo
{
  CallInfo{ target: String::from(name), return_word_count: ret_wc, argument_list: Vec::new()}
}


pub fn
push(&mut self, o: Operand)
{
  self.argument_list.push(o);
}


pub fn
print(&self)
{
  print!("{} ",&self.target);

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
  pub(crate) condition: String,

  pub(crate) on_true:  String,
  pub(crate) on_false: String,

}


impl
BranchInfo
{


pub fn
new(cond: &str, on_true: &str, on_false: &str)-> BranchInfo
{
  BranchInfo{ condition: String::from(cond), on_true: String::from(on_true), on_false: String::from(on_false)}
}


pub fn
print(&self)
{
  print!("{} {} {}",&self.condition,&self.on_true,&self.on_false);
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

  Address(String),

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
          print!("address {}",o);
        },
  AllocatingOperation::Phi(ops)=>
        {
          print!("phi ");

            for o in ops
            {
              print!("{} ",&o.from);

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
  CopyWord(String,String),
  CopyString(String,String,usize),
  Message(String),
  Print(String,char),

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
          print!("copy_word (src){} (dst){}",src,dst);
        },
  NonAllocatingOperation::CopyString(dst,src,sz)=>
        {
          print!("copy_string (src){} (dst){} {}",src,dst,*sz);
        },
  NonAllocatingOperation::Message(s)=>
        {
          print!("message \"{}\"",s);
        },
  NonAllocatingOperation::Print(target,c)=>
        {
          print!("print {} {}",target,c);
        },
    }
}


}




pub enum
Line
{
     AllocatingOperation(String,AllocatingOperation),
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
  Line::AllocatingOperation(dst,ao)=>
        {
          print!("{} = ",dst);

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
  Jump(String),
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
  Terminator::Jump(dst)=>
        {
          print!("jmp {}",dst);
        },
  Terminator::Branch(bi)=>
        {
          print!("br ");
          bi.print();
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
  pub(crate)  from: String,
  pub(crate) value: Operand,

}


impl
PhiOperand
{


pub fn
make(blk_name: &str, o: Operand)-> PhiOperand
{
  PhiOperand{ from: String::from(blk_name), value: o}
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
  self.terminator = Some(Terminator::Jump(String::from(label)));
}


pub fn
set_br(&mut self, var_name: &str, on_true: &str, on_false: &str)
{
  let  bi = BranchInfo{ condition: String::from(var_name), on_true: String::from(on_true), on_false: String::from(on_false)};

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




