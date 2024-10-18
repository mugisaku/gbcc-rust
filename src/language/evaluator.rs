

use super::expression::{
  Expression,

};


use super::memory::{
  Memory,

};


use super::type_info::{
  TypeInfo,
  NumberKind,

};


const WORD_SIZE: usize = 8;


#[derive(Clone)]
pub enum
FieldIndex
{
  Global(usize),
   Local(usize),

}


#[derive(Clone)]
pub struct
Destination
{
  field_index: FieldIndex,

}


#[derive(Clone)]
pub struct
Source
{
  field_index: FieldIndex

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


#[derive(Clone)]
pub enum
OpcodeB
{
  Nop,

  NegI, NgeF,
  Not,
  ItoU, UtoI, ItoF, FtoI,

}


#[derive(Clone)]
pub enum
Instruction
{
  OperationA(OpcodeA,Destination,Source,Source),
  OperationB(OpcodeB,Destination,Source       ),

  LdI(Destination,i64),
  LdU(Destination,u64),
  LdF(Destination,f64),

}


pub struct
ExpressionEvaluator
{
  pub(crate) memory: Memory,
  pub(crate) instruction_list: Vec<Instruction>,
  pub(crate) pc: usize,
  pub(crate) bp: usize,
  pub(crate) sp: usize,

}


impl
ExpressionEvaluator
{


/*
pub fn
calculate_unary(m: &mut Memory, o: &UnaryOperator, v: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  UnaryOperator::Neg=>{Value::neg(v)},
  UnaryOperator::Not=>{Value::not(v)},
  UnaryOperator::LogicalNot=>{Value::logical_not(v)},
  _=>{Value::Undefined},
    }
}


pub fn
calculate_binary(m: &mut Memory, o: &BinaryOperator, lv: &Value, rv: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  BinaryOperator::Add=>{Value::add(lv,rv)},
  BinaryOperator::Sub=>{Value::sub(lv,rv)},
  BinaryOperator::Mul=>{Value::mul(lv,rv)},
  BinaryOperator::Div=>{Value::div(lv,rv)},
  BinaryOperator::Rem=>{Value::rem(lv,rv)},
  BinaryOperator::Shl=>{Value::shl(lv,rv)},
  BinaryOperator::Shr=>{Value::shr(lv,rv)},
  BinaryOperator::And=>{Value::and(lv,rv)},
  BinaryOperator::Or=>{Value::or(lv,rv)},
  BinaryOperator::Xor=>{Value::xor(lv,rv)},
  BinaryOperator::Eq=>{Value::eq(lv,rv)},
  BinaryOperator::Neq=>{Value::neq(lv,rv)},
  BinaryOperator::Lt=>{Value::lt(lv,rv)},
  BinaryOperator::Lteq=>{Value::lteq(lv,rv)},
  BinaryOperator::Gt=>{Value::gt(lv,rv)},
  BinaryOperator::Gteq=>{Value::gteq(lv,rv)},
  BinaryOperator::LogicalAnd=>{Value::logical_and(lv,rv)},
  BinaryOperator::LogicalOr=>{Value::logical_or(lv,rv)},
    }
}

*/
pub fn
calculate(e: &Expression, dst: Destination)-> Result<(Instruction,TypeInfo),()>
{
    match e
    {
  Expression::Identifier(s)=>
        {
               if s ==  "true"{return Ok((Instruction::LdU(dst,1),TypeInfo::Bool));}
          else if s == "false"{return Ok((Instruction::LdU(dst,0),TypeInfo::Bool));}
/*
          else
            if let Some(v) = Self::find_const(const_list,s)
            {
              return Ok(v);
            }
*/
        },
  Expression::Boolean(b) =>{return Ok((Instruction::LdU(dst,if *b{1} else{0}),TypeInfo::Bool));},
  Expression::Integer(u) =>{return Ok((Instruction::LdU(dst,*u),TypeInfo::Number(NumberKind::IntLiteral)));},
  Expression::Floating(f)=>{return Ok((Instruction::LdF(dst,*f),TypeInfo::Number(NumberKind::FloatLiteral)));},
  Expression::SubExpression(sube)=>
        {
          return Self::calculate(e,dst);
        },
  Expression::Unary(o,e)=>
        {
            if let Ok((instr,ti)) = Self::calculate(e,dst)
            {
//              return Ok(Self::calculate_unary(o,&v,const_list));
            }
        },
  Expression::Call(f,args)=>
        {
          panic!();
        },
  Expression::Subscript(target,index)=>
        {
          panic!();
        },
  Expression::Access(target,name)=>
        {
          panic!();
        },
  Expression::Binary(o,l,r)=>
        {
            if let Ok((linstr,lti)) = Self::calculate(l,dst.clone())
            {
//                if let Ok((rinstr,rti)) = Self::calculate(r)
                {
//                  return Ok(Self::calculate_binary(o,&lv,&rv,const_list));
                }
            }
        },
  _=>{}
    }


  Err(())
}


pub fn
reset(&mut self, e: &Expression)
{
  self.instruction_list.clear();
  self.pc = 0;

  
}


fn
get_address(&self, fi: &FieldIndex)-> usize
{
  0
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

  OpcodeB::NegI=>{self.memory.negi(dst_addr,src_addr);}
  OpcodeB::NgeF=>{self.memory.negf(dst_addr,src_addr);}
  OpcodeB::Not =>{self.memory.not( dst_addr,src_addr);}
  OpcodeB::ItoU=>{self.memory.itou(dst_addr,src_addr);}
  OpcodeB::UtoI=>{self.memory.utoi(dst_addr,src_addr);}
  OpcodeB::ItoF=>{self.memory.itof(dst_addr,src_addr);}
  OpcodeB::FtoI=>{self.memory.ftoi(dst_addr,src_addr);}
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
step(&mut self)-> Option<()>
{
    if self.pc < self.instruction_list.len()
    {
      let  instr = self.instruction_list[self.pc].clone();

      self.pc += 1;

        match &instr
        {
      Instruction::OperationA(op,dst,src1,src2)=>{self.operate_a(op,dst,src1,src2);}
      Instruction::OperationB(op,dst,src      )=>{self.operate_b(op,dst,src      );}
      Instruction::LdI(dst,i)=>{self.ldi(dst,*i);}
      Instruction::LdU(dst,u)=>{self.ldu(dst,*u);}
      Instruction::LdF(dst,f)=>{self.ldf(dst,*f);}
        }


      return Some(());
    }


  None
}


}




