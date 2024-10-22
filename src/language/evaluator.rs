

use super::expression::{
  Expression,
  UnaryOperator,
  BinaryOperator,

};


use super::memory::{
  Memory,

};


use super::type_info::{
  SymbolNode,
  SymbolKind,
  TypeInfo,
  NumberKind,
  IntKind,
  FloatKind,

};


const WORD_SIZE: usize = 8;


pub struct
CompileResult
{
  pub(crate) dst: Destination,

  pub(crate) type_info: TypeInfo,

}


impl
CompileResult
{


pub fn
new(dst: Destination, type_info: TypeInfo)-> Self
{
  Self{
    dst,
    type_info,
  }
}


pub fn
next_dst(&self)-> Destination
{
  self.dst.add(self.type_info.get_size())
}


}




#[derive(Clone)]
pub enum
FieldIndex
{
  Global(usize),
   Local(usize),

}


impl
FieldIndex
{


pub fn
print(&self)
{
    match self
    {
  Self::Global(i)=>{print!("g{}",*i);}
  Self::Local(i) =>{print!("l{}",*i);}
    }
}


}




#[derive(Clone)]
pub struct
Destination
{
  field_index: FieldIndex,

}


impl
Destination
{


pub fn
add(&self, sz: usize)-> Self
{
  let  n = (sz+7)/WORD_SIZE;

    match &self.field_index
    {
  FieldIndex::Global(i)=>{Self{field_index: FieldIndex::Global((*i)+n)}}
  FieldIndex::Local(i) =>{Self{field_index: FieldIndex::Local( (*i)+n)}}
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


pub enum
TypeCheckResultA{I, U, F, Char, Bool, Bitwise, Err}


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
compile_unary(o: &UnaryOperator, res: &CompileResult, buf: &mut Vec<Instruction>)-> Result<CompileResult,()>
{
  let  (op,ti): (OpcodeB,TypeInfo) = match o
    {
  UnaryOperator::Neg=>
        {
            if let TypeInfo::Number(nk) = &res.type_info
            {
                match nk
                {
              NumberKind::SignedInt(_)=>{(OpcodeB::NegI,res.type_info.clone())}
              NumberKind::Float(_)    =>{(OpcodeB::NegF,res.type_info.clone())}
              _=>{(OpcodeB::Nop,TypeInfo::Unknown)}
                }
            }

          else
            {
              (OpcodeB::Nop,TypeInfo::Unknown)
            }
        },
  UnaryOperator::Not=>
        {
            if let TypeInfo::Number(nk) = &res.type_info
            {
                match nk
                {
              NumberKind::SignedInt(_)  =>{(OpcodeB::Not,res.type_info.clone())}
              NumberKind::UnsignedInt(_)=>{(OpcodeB::Not,res.type_info.clone())}
              _=>{(OpcodeB::Nop,TypeInfo::Unknown)}
                }
            }

          else
            {
              (OpcodeB::Nop,TypeInfo::Unknown)
            }
        },
  UnaryOperator::LogicalNot=>{if let TypeInfo::Bool = &res.type_info{(OpcodeB::LogicalNot,TypeInfo::Bool)} else{(OpcodeB::Nop,TypeInfo::Unknown)}},
  _=>{(OpcodeB::Nop,TypeInfo::Unknown)},
    };


    if let OpcodeB::Nop = op
    {
      Err(())
    }

  else
    {
      buf.push(Instruction::OperationB(op,res.next_dst(),res.dst.to_src()));

      Ok(CompileResult::new(res.next_dst(),ti))
    }
}


fn
typecheck_n(l: &NumberKind, r: &NumberKind)-> (TypeCheckResultA,TypeInfo)
{
    if let NumberKind::SignedInt(l_ik) = l
    {
        if let NumberKind::SignedInt(r_ik) = r
        {
            if let Some(ik) = IntKind::check(l_ik,r_ik)
            {
              let  nk = NumberKind::SignedInt(ik);

              return (TypeCheckResultA::I,TypeInfo::Number(nk));
            }
        }
    }

  else
    if let NumberKind::UnsignedInt(l_ik) = l
    {
        if let NumberKind::UnsignedInt(r_ik) = r
        {
            if let Some(ik) = IntKind::check(l_ik,r_ik)
            {
              let  nk = NumberKind::UnsignedInt(ik);

              return (TypeCheckResultA::U,TypeInfo::Number(nk));
            }
        }
    }

  else
    if let NumberKind::Float(l_fk) = l
    {
        if let NumberKind::Float(r_fk) = r
        {
            if let Some(fk) = FloatKind::check(l_fk,r_fk)
            {
              let  nk = NumberKind::Float(fk);

              return (TypeCheckResultA::F,TypeInfo::Number(nk));
            }
        }
    }


  (TypeCheckResultA::Err,TypeInfo::Unknown)
}


fn
typecheck_a(l: &TypeInfo, r: &TypeInfo)-> (TypeCheckResultA,TypeInfo)
{
    if let TypeInfo::Number(l_nk) = l
    {
        if let TypeInfo::Number(r_nk) = r
        {
          return Self::typecheck_n(l_nk,r_nk);
        }
    }

  else
    if let TypeInfo::Char = l
    {
        if let TypeInfo::Char = r
        {
          return (TypeCheckResultA::Char,TypeInfo::Char);
        }
    }

  else
    if let TypeInfo::Bool = l
    {
        if let TypeInfo::Bool = r
        {
          return (TypeCheckResultA::Bool,TypeInfo::Bool);
        }
    }


  (TypeCheckResultA::Err,TypeInfo::Unknown)
}


fn
get_opcode_a(res: TypeCheckResultA, i: OpcodeA, u: OpcodeA, f: OpcodeA)-> OpcodeA
{
    match res
    {
  TypeCheckResultA::I=>{i}
  TypeCheckResultA::U=>{u}
  TypeCheckResultA::F=>{f}
  _=>{OpcodeA::Nop}
    }
}


pub fn
compile_binary(o: &BinaryOperator, lres: &CompileResult, rres: &CompileResult, buf: &mut Vec<Instruction>)-> Result<CompileResult,()>
{
  let  (res,ti) = Self::typecheck_a(&lres.type_info,&rres.type_info);

  let  op = match o
    {
  BinaryOperator::Add       =>{Self::get_opcode_a(res,OpcodeA::AddI,OpcodeA::AddU,OpcodeA::AddF)},
  BinaryOperator::Sub       =>{Self::get_opcode_a(res,OpcodeA::SubI,OpcodeA::SubU,OpcodeA::SubF)},
  BinaryOperator::Mul       =>{Self::get_opcode_a(res,OpcodeA::MulI,OpcodeA::MulU,OpcodeA::MulF)},
  BinaryOperator::Div       =>{Self::get_opcode_a(res,OpcodeA::DivI,OpcodeA::DivU,OpcodeA::DivF)},
  BinaryOperator::Rem       =>{Self::get_opcode_a(res,OpcodeA::RemI,OpcodeA::RemU,OpcodeA::RemF)},
  BinaryOperator::Shl       =>{if let TypeCheckResultA::Bitwise = res{OpcodeA::Shl} else{OpcodeA::Nop}},
  BinaryOperator::Shr       =>{if let TypeCheckResultA::Bitwise = res{OpcodeA::Shr} else{OpcodeA::Nop}},
  BinaryOperator::And       =>{if let TypeCheckResultA::Bitwise = res{OpcodeA::And} else{OpcodeA::Nop}},
  BinaryOperator::Or        =>{if let TypeCheckResultA::Bitwise = res{OpcodeA::Or } else{OpcodeA::Nop}},
  BinaryOperator::Xor       =>{if let TypeCheckResultA::Bitwise = res{OpcodeA::Xor} else{OpcodeA::Nop}},
  BinaryOperator::Eq        =>{Self::get_opcode_a(res,OpcodeA::Eq,   OpcodeA::Eq,   OpcodeA::Eq   )},
  BinaryOperator::Neq       =>{Self::get_opcode_a(res,OpcodeA::Neq,  OpcodeA::Neq,  OpcodeA::Neq  )},
  BinaryOperator::Lt        =>{Self::get_opcode_a(res,OpcodeA::LtI,  OpcodeA::LtU,  OpcodeA::LtF  )},
  BinaryOperator::Lteq      =>{Self::get_opcode_a(res,OpcodeA::LteqI,OpcodeA::LteqU,OpcodeA::LteqF)},
  BinaryOperator::Gt        =>{Self::get_opcode_a(res,OpcodeA::GtI,  OpcodeA::GtU,  OpcodeA::GtF  )},
  BinaryOperator::Gteq      =>{Self::get_opcode_a(res,OpcodeA::GteqI,OpcodeA::GteqU,OpcodeA::GteqF)},
  BinaryOperator::LogicalAnd=>{if let TypeCheckResultA::Bool = res{OpcodeA::LogicalAnd} else{OpcodeA::Nop}},
  BinaryOperator::LogicalOr =>{if let TypeCheckResultA::Bool = res{OpcodeA::LogicalOr } else{OpcodeA::Nop}},
    };


    if let OpcodeA::Nop = op
    {
      Err(())
    }

  else
    {
      buf.push(Instruction::OperationA(op,rres.next_dst(),lres.dst.to_src(),rres.dst.to_src()));

      Ok(CompileResult::new(rres.next_dst(),ti))
    }
}


pub fn
compile(e: &Expression, root_nd: &SymbolNode, dst: Destination, buf: &mut Vec<Instruction>)-> Result<CompileResult,()>
{
    match e
    {
  Expression::Identifier(s)=>
        {
               if s ==  "true"{  buf.push(Instruction::LdU(dst.clone(),1));  return Ok(CompileResult::new(dst,TypeInfo::Bool));}
          else if s == "false"{  buf.push(Instruction::LdU(dst.clone(),0));  return Ok(CompileResult::new(dst,TypeInfo::Bool));}
          else
            if let Some(k) = root_nd.find_any(s)
            {
                match k
                {
              SymbolKind::Type(ti)=>{return Ok(CompileResult::new(dst,TypeInfo::External(ti as *const TypeInfo)));}
              SymbolKind::Variable(ti,offset)=>{}
              _=>{}
                }
            }
        },
  Expression::Boolean(b) =>{  buf.push(Instruction::LdU(dst.clone(),if *b{1} else{0}));  return Ok(CompileResult::new(dst,TypeInfo::Bool));},
  Expression::Integer(u) =>{  buf.push(Instruction::LdU(dst.clone(),*u));                return Ok(CompileResult::new(dst,TypeInfo::new_uliteral()));},
  Expression::Floating(f)=>{  buf.push(Instruction::LdF(dst.clone(),*f));                return Ok(CompileResult::new(dst,TypeInfo::new_fliteral()));},
  Expression::SubExpression(sube)=>
        {
          return Self::compile(e,root_nd,dst,buf);
        },
  Expression::Unary(o,e)=>
        {
            if let Ok(res) = Self::compile(e,root_nd,dst,buf)
            {
              return Self::compile_unary(o,&res,buf);
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
            if let Ok(lres) = Self::compile(l,root_nd,dst,buf)
            {
                if let Ok(rres) = Self::compile(r,root_nd,lres.next_dst(),buf)
                {
                  return Self::compile_binary(o,&lres,&rres,buf);
                }
            }
        },
  _=>{}
    }


  Err(())
}


pub fn
reset(&mut self, e: &Expression, root_nd: &SymbolNode)
{
  self.instruction_list.clear();
  self.pc = 0;
  self.bp = 0;
  self.sp = 0;

  self.final_value_type_info = TypeInfo::Unknown;

  let  dst = Destination{field_index: FieldIndex::Local(0)};

    if let Ok(res) = Self::compile(e,root_nd,dst,&mut self.instruction_list)
    {
      self.final_value_dst = res.dst.clone();
      self.final_value_type_info = res.type_info;
    }

  else
    {
      panic!();
    }
}


fn
get_address(&self, fi: &FieldIndex)-> usize
{
    match fi
    {
  FieldIndex::Global(v)=>{        (8*(*v))}
  FieldIndex::Local(v) =>{self.bp+(8*(*v))}
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




