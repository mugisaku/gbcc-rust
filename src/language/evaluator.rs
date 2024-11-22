

use super::compile_for_expression::{
  compile,

};


use super::expression::{
  Expression,
  AssignOperator,
  UnaryOperator,
  BinaryOperator,

};


use super::memory::{
  Memory,

};

use super::symbol::{
  SymbolDirectory,
  SymbolKind,

};


use super::type_info::{
  TypeInfo,

};


const WORD_SIZE: usize = 8;




#[derive(Clone)]
pub enum
FieldIndex
{
    Global(usize),
  Argument(usize),
     Local(usize),
 Temporary(usize),

}


impl
FieldIndex
{


pub fn
print(&self)
{
    match self
    {
  Self::Global(i)   =>{print!("glo{}",*i);}
  Self::Argument(i) =>{print!("arg{}",*i);}
  Self::Local(i)    =>{print!("loc{}",*i);}
  Self::Temporary(i)=>{print!("tmp{}",*i);}
    }
}


}




#[derive(Clone)]
pub struct
Destination
{
  pub(crate) field_index: FieldIndex,

}


impl
Destination
{


pub fn
new_temporary(pos: usize)-> Self
{
  Self{
    field_index: FieldIndex::Temporary(pos+(WORD_SIZE-1)/WORD_SIZE),
  }
}


pub fn
add(&self, sz: usize)-> Self
{
  let  n = (sz+7)/WORD_SIZE;

    match &self.field_index
    {
  FieldIndex::Global(i)   =>{Self{field_index: FieldIndex::Global(   (*i)+n)}}
  FieldIndex::Argument(i) =>{Self{field_index: FieldIndex::Argument( (*i)+n)}}
  FieldIndex::Local(i)    =>{Self{field_index: FieldIndex::Local(    (*i)+n)}}
  FieldIndex::Temporary(i)=>{Self{field_index: FieldIndex::Temporary((*i)+n)}}
    }
}


pub fn
to_src(&self)-> Source
{
  Source{field_index: self.field_index.clone()}
}



}




#[derive(Clone)]
pub struct
Source
{
  pub(crate) field_index: FieldIndex

}


impl
Source
{


pub fn
new_temporary(pos: usize)-> Self
{
  Self{
    field_index: FieldIndex::Temporary(pos+(WORD_SIZE-1)/WORD_SIZE),
  }
}


}




#[derive(Clone)]
pub enum
OpcodeA
{
  Nop,

  AddI, SubI, MulI, DivI, RemI,
  AddU, SubU, MulU, DivU, RemU,
  AddF, SubF, MulF, DivF, RemF,

  Shl, Shr, And, Or, Xor,

  Eq, Neq,

  LtI, LteqI, GtI, GteqI,
  LtU, LteqU, GtU, GteqU,
  LtF, LteqF, GtF, GteqF,

  LogicalAnd, LogicalOr,

}


impl
OpcodeA
{


pub fn
print(&self)
{
    match self
    {
  Self::Nop=>{print!("nop");},

  Self::AddI=>{print!("addi");},
  Self::SubI=>{print!("subi");},
  Self::MulI=>{print!("muli");},
  Self::DivI=>{print!("divi");},
  Self::RemI=>{print!("remi");},
  Self::AddU=>{print!("addu");},
  Self::SubU=>{print!("subu");},
  Self::MulU=>{print!("mulu");},
  Self::DivU=>{print!("divu");},
  Self::RemU=>{print!("remu");},
  Self::AddF=>{print!("addf");},
  Self::SubF=>{print!("subf");},
  Self::MulF=>{print!("mulf");},
  Self::DivF=>{print!("divf");},
  Self::RemF=>{print!("remf");},

  Self::Shl=>{print!("shl");},
  Self::Shr=>{print!("shr");},
  Self::And=>{print!("and");},
  Self::Or =>{print!("or");},
  Self::Xor=>{print!("xor");},

  Self::Eq =>{print!("eq");},
  Self::Neq=>{print!("neq");},

  Self::LtI  =>{print!("lti");},
  Self::LteqI=>{print!("lteqi");},
  Self::GtI  =>{print!("gti");},
  Self::GteqI=>{print!("gteqi");},
  Self::LtU  =>{print!("ltu");},
  Self::LteqU=>{print!("ltequ");},
  Self::GtU  =>{print!("gtu");},
  Self::GteqU=>{print!("gtequ");},
  Self::LtF  =>{print!("ltf");},
  Self::LteqF=>{print!("lteqf");},
  Self::GtF  =>{print!("gtf");},
  Self::GteqF=>{print!("gteqf");},

  Self::LogicalAnd=>{print!("logical_and");},
  Self::LogicalOr =>{print!("logical_or");},
    }
}


}



#[derive(Clone)]
pub enum
OpcodeB
{
  Nop,

  NegI, NegF,
  Not,
  LogicalNot,
  ItoU, UtoI, ItoF, FtoI,

}


impl
OpcodeB
{


pub fn
print(&self)
{
    match self
    {
  Self::Nop=>{print!("nop");},

  Self::NegI      =>{print!("negi");},
  Self::NegF      =>{print!("negf");},
  Self::Not       =>{print!("not");},
  Self::LogicalNot=>{print!("logical_not");},
  Self::ItoU      =>{print!("itou");},
  Self::UtoI      =>{print!("utoi");},
  Self::ItoF      =>{print!("itof");},
  Self::FtoI      =>{print!("ftoi");},
    }
}


}


#[derive(Clone)]
pub enum
Instruction
{
  Nop,

  OperationA(OpcodeA,Destination,Source,Source),
  OperationB(OpcodeB,Destination,Source       ),

  LdI(Destination,i64),
  LdU(Destination,u64),
  LdF(Destination,f64),

  Cal(Source,usize,usize),

  Assign(AssignOperator),

  Mvsp(isize),
  Jmp(isize),
  Brz(isize),
  Brnz(isize),
  Trv(usize),
  Ret,

  Break,
  Continue,
  Exit,

}


impl
Instruction
{


pub fn
print(&self)
{
    match self
    {
  Self::Nop=>{print!("nop");}

  Self::OperationA(op,dst,src1,src2)=>
        {
          dst.field_index.print();
          print!(" = ");
          op.print();
          print!(" ");
          src1.field_index.print();
          print!(" ");
          src2.field_index.print();
        }
  Self::OperationB(op,dst,src)=>
        {
          dst.field_index.print();
          print!(" = ");
          op.print();
          print!(" ");
          src.field_index.print();
        }
  Self::LdI(dst,i)=>{  dst.field_index.print();  print!(" = ldi {}",*i);}
  Self::LdU(dst,u)=>{  dst.field_index.print();  print!(" = ldu {}",*u);}
  Self::LdF(dst,f)=>{  dst.field_index.print();  print!(" = ldf {}",*f);}
  Self::Cal(src,retval_sz,args_sz)=>
        {
          print!("cal ");

          src.field_index.print();

          print!(" {} {}",*retval_sz,*args_sz);
        }
  Self::Assign(o)=>{  print!("assign");  o.print();}
  Self::Mvsp(n)=>{print!("mvsp {}",*n);}
  Self::Jmp(n)=>{print!("jmp {}",*n);}
  Self::Brz(n)=>{print!("brz {}",*n);}
  Self::Brnz(n)=>{print!("brnz {}",*n);}
  Self::Trv(sz)=>{print!("trv {}",*sz);}
  Self::Ret=>{print!("ret");}
  Self::Break=>{print!("break");}
  Self::Continue=>{print!("continue");}
  Self::Exit=>{print!("exit");}
    }
}


}




pub struct
ExpressionEvaluator
{
  pub(crate) memory: Memory,
  pub(crate) instruction_list: Vec<Instruction>,
  pub(crate) pc: usize,
  pub(crate) bp: usize,
  pub(crate) sp: usize,

  pub(crate) final_value_dst: Destination,
  pub(crate) final_value_type_info: TypeInfo,

}


impl
ExpressionEvaluator
{


pub fn
new()-> Self
{
  Self{
    memory: Memory::new_with_size(256),
    instruction_list: Vec::new(),
    pc: 0,
    bp: 0,
    sp: 0,
    final_value_dst: Destination{field_index: FieldIndex::Local(0)},
    final_value_type_info: TypeInfo::Unknown,
  }
}




pub fn
reset(&mut self, e: &Expression, dir: &SymbolDirectory)
{
  self.pc = 0;
  self.bp = 0;
  self.sp = 0;

  self.final_value_type_info = TypeInfo::Unknown;

/*
  let  (ls,ti) = compile(e,dir);

  self.instruction_list = ls;

  self.final_value_dst = Destination{field_index: FieldIndex::Global(0)};
  self.final_value_type_info = ti;
*/
}


fn
get_address(&self, fi: &FieldIndex)-> usize
{
    match fi
    {
  FieldIndex::Global(v)   =>{        (WORD_SIZE*(*v))}
  FieldIndex::Argument(v) =>{self.bp-(WORD_SIZE*(*v))}
  FieldIndex::Local(v)    =>{self.bp+(WORD_SIZE*(*v))}
  FieldIndex::Temporary(v)=>{self.sp+(WORD_SIZE*(*v))}
    }
}


pub fn
operate_a(&mut self, op: &OpcodeA, dst: &Destination, src1: &Source, src2: &Source)
{
  let   dst_addr = self.get_address(&dst.field_index);
  let  src1_addr = self.get_address(&src1.field_index);
  let  src2_addr = self.get_address(&src2.field_index);

    match op
    {
  OpcodeA::Nop=>{}

  OpcodeA::AddI=>{self.memory.addi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::SubI=>{self.memory.subi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::MulI=>{self.memory.muli(dst_addr,src1_addr,src2_addr);}
  OpcodeA::DivI=>{self.memory.divi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::RemI=>{self.memory.remi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::AddU=>{self.memory.addu(dst_addr,src1_addr,src2_addr);}
  OpcodeA::SubU=>{self.memory.subu(dst_addr,src1_addr,src2_addr);}
  OpcodeA::MulU=>{self.memory.mulu(dst_addr,src1_addr,src2_addr);}
  OpcodeA::DivU=>{self.memory.divu(dst_addr,src1_addr,src2_addr);}
  OpcodeA::RemU=>{self.memory.remu(dst_addr,src1_addr,src2_addr);}
  OpcodeA::AddF=>{self.memory.addf(dst_addr,src1_addr,src2_addr);}
  OpcodeA::SubF=>{self.memory.subf(dst_addr,src1_addr,src2_addr);}
  OpcodeA::MulF=>{self.memory.mulf(dst_addr,src1_addr,src2_addr);}
  OpcodeA::DivF=>{self.memory.divf(dst_addr,src1_addr,src2_addr);}
  OpcodeA::RemF=>{self.memory.remf(dst_addr,src1_addr,src2_addr);}

  OpcodeA::Shl=>{self.memory.shl(dst_addr,src1_addr,src2_addr);}
  OpcodeA::Shr=>{self.memory.shr(dst_addr,src1_addr,src2_addr);}
  OpcodeA::And=>{self.memory.and(dst_addr,src1_addr,src2_addr);}
  OpcodeA::Or =>{self.memory.or( dst_addr,src1_addr,src2_addr);}
  OpcodeA::Xor=>{self.memory.xor(dst_addr,src1_addr,src2_addr);}

  OpcodeA::Eq =>{self.memory.eq( dst_addr,src1_addr,src2_addr);}
  OpcodeA::Neq=>{self.memory.neq(dst_addr,src1_addr,src2_addr);}

  OpcodeA::LtI  =>{self.memory.lti(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::LteqI=>{self.memory.lteqi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::GtI  =>{self.memory.gti(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::GteqI=>{self.memory.gteqi(dst_addr,src1_addr,src2_addr);}
  OpcodeA::LtU  =>{self.memory.ltu(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::LteqU=>{self.memory.ltequ(dst_addr,src1_addr,src2_addr);}
  OpcodeA::GtU  =>{self.memory.gtu(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::GteqU=>{self.memory.gtequ(dst_addr,src1_addr,src2_addr);}
  OpcodeA::LtF  =>{self.memory.ltf(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::LteqF=>{self.memory.lteqf(dst_addr,src1_addr,src2_addr);}
  OpcodeA::GtF  =>{self.memory.gtf(  dst_addr,src1_addr,src2_addr);}
  OpcodeA::GteqF=>{self.memory.gteqf(dst_addr,src1_addr,src2_addr);}

  OpcodeA::LogicalAnd=>{self.memory.logical_and(dst_addr,src1_addr,src2_addr);}
  OpcodeA::LogicalOr =>{self.memory.logical_or( dst_addr,src1_addr,src2_addr);}
    }
}


pub fn
operate_b(&mut self, op: &OpcodeB, dst: &Destination, src: &Source)
{
  let  dst_addr = self.get_address(&dst.field_index);
  let  src_addr = self.get_address(&src.field_index);

    match op
    {
  OpcodeB::Nop=>{}

  OpcodeB::NegI      =>{self.memory.negi(dst_addr,src_addr);}
  OpcodeB::NegF      =>{self.memory.negf(dst_addr,src_addr);}
  OpcodeB::Not       =>{self.memory.not( dst_addr,src_addr);}
  OpcodeB::LogicalNot=>{self.memory.logical_not( dst_addr,src_addr);}
  OpcodeB::ItoU      =>{self.memory.itou(dst_addr,src_addr);}
  OpcodeB::UtoI      =>{self.memory.utoi(dst_addr,src_addr);}
  OpcodeB::ItoF      =>{self.memory.itof(dst_addr,src_addr);}
  OpcodeB::FtoI      =>{self.memory.ftoi(dst_addr,src_addr);}
    }
}


pub fn
ldi(&mut self, dst: &Destination, i: i64)
{
  let  dst_addr = self.get_address(&dst.field_index);

  self.memory.put_i64(dst_addr,i);
}


pub fn
ldu(&mut self, dst: &Destination, u: u64)
{
  let  dst_addr = self.get_address(&dst.field_index);

  self.memory.put_u64(dst_addr,u);
}


pub fn
ldf(&mut self, dst: &Destination, f: f64)
{
  let  dst_addr = self.get_address(&dst.field_index);

  self.memory.put_f64(dst_addr,f);
}


pub fn
assign(&mut self, o: &AssignOperator)
{
    match o
    {
  AssignOperator::Nop=>{},
  AssignOperator::Add=>{},
  AssignOperator::Sub=>{},
  AssignOperator::Mul=>{},
  AssignOperator::Div=>{},
  AssignOperator::Rem=>{},
  AssignOperator::Shl=>{},
  AssignOperator::Shr=>{},
  AssignOperator::And=>{},
  AssignOperator::Or =>{},
  AssignOperator::Xor=>{},
    }
}


pub fn
mvsp(&mut self, v: isize)
{
    if v >= 0
    {
      self.sp += v as usize;
    }

  else
    {
      self.sp -= (-v) as usize;
    }
}


pub fn
jmp(&mut self, v: isize)
{
    if v >= 0
    {
      self.pc += v as usize;
    }

  else
    {
      self.pc -= (-v) as usize;
    }
}


pub fn
brz(&mut self, v: isize)
{
    if self.memory.get_u64(self.sp) == 0
    {
      self.jmp(v);
    }
}


pub fn
brnz(&mut self, v: isize)
{
    if self.memory.get_u64(self.sp) != 0
    {
      self.jmp(v);
    }
}




pub fn
_size_of_return_value(&self)->usize
{
  0
}


pub fn
cal(&mut self, src: &Source, retval_sz: usize, args_sz: usize)
{
  let  src_addr = self.get_address(&src.field_index);

  let  old_pc = self.pc;
  let  old_bp = self.bp;

  self.pc = self.memory.get_u64(src_addr) as usize;
}


pub fn
get_address_of_return_value(&self)->usize
{
  0
}


pub fn
trv(&mut self, sz: usize)
{
  let  dst_addr = self.get_address_of_return_value();

  self.memory.copy(dst_addr,self.sp,sz);
}


pub fn
ret(&mut self)
{
}


pub fn
step(&mut self)-> Option<()>
{
    if self.pc < self.instruction_list.len()
    {
      let  instr = self.instruction_list[self.pc].clone();

      self.pc += 1;

        match &instr
        {
      Instruction::Nop=>{}
      Instruction::OperationA(op,dst,src1,src2)=>{self.operate_a(op,dst,src1,src2);}
      Instruction::OperationB(op,dst,src      )=>{self.operate_b(op,dst,src      );}
      Instruction::LdI(dst,i)=>{self.ldi(dst,*i);}
      Instruction::LdU(dst,u)=>{self.ldu(dst,*u);}
      Instruction::LdF(dst,f)=>{self.ldf(dst,*f);}
      Instruction::Cal(src,retval_sz,args_sz)=>{self.cal(src,*retval_sz,*args_sz);}
      Instruction::Assign(o)=>{self.assign(o);}
      Instruction::Mvsp(v)=>{self.mvsp((*v)*(WORD_SIZE as isize));}
      Instruction::Jmp(v)=>{self.jmp(*v);}
      Instruction::Brz(v)=>{self.brz(*v);}
      Instruction::Brnz(v)=>{self.brnz(*v);}
      Instruction::Trv(sz)=>{self.trv(*sz);}
      Instruction::Ret=>{self.ret();}
      _=>{panic!();}
        }


      return Some(());
    }


  None
}


pub fn
run(&mut self)
{
    loop
    {
        if let None = self.step()
        {
          break;
        }
    }
}


pub fn
get_final_value_as_usize(&self)-> usize
{
  let  addr = self.get_address(&self.final_value_dst.field_index);

  self.memory.get_u64(addr) as usize
}


pub fn
get_final_value_and_type_info(&self)-> (Vec<u8>,TypeInfo)
{
  let  sz = self.final_value_type_info.get_size();

  let  addr = self.get_address(&self.final_value_dst.field_index);

  let  b = self.memory.get_str(addr,sz);

  (b,self.final_value_type_info.clone())
}


pub fn
print_final_value(&self)
{
    for instr in &self.instruction_list
    {
      instr.print();

      println!("");
    }


  print!("return value index is ");

  self.final_value_dst.field_index.print();

  println!("");

  print!("return value type is ");

  self.final_value_type_info.print();

  println!("");

  let  addr = self.get_address(&self.final_value_dst.field_index);

  print!(" = {}({})",self.memory.get_u64(addr),addr);
}


}




