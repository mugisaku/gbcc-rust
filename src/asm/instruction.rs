



#[derive(Clone)]
pub enum
Instruction
{
  Opcode(u8),

  Data(u8),

  ImmI8(i8),
  ImmI16(i16),
  ImmI32(i32),
  ImmI64(i64),
  ImmU8(u8),
  ImmU16(u16),
  ImmU32(u32),
  ImmU64(u64),
  ImmF32(f32),
  ImmF64(f64),

  PushI(i64),
  PushU(u64),
  PushF(f64),

  PushDst(String),
  PushGlo(String),
  PushArg(String),
  PushLoc(String),

  Label(String),

}


impl
Instruction
{


pub fn
get_size(&self)-> usize
{
    match self
    {
  Self::Opcode(_)=> {1}
  Self::Data(_)=> {1}
  Self::ImmI8(_)=> {1}
  Self::ImmI16(_)=>{2}
  Self::ImmI32(_)=>{4}
  Self::ImmI64(_)=>{8}
  Self::ImmU8(_)=> {1}
  Self::ImmU16(_)=>{2}
  Self::ImmU32(_)=>{4}
  Self::ImmU64(_)=>{8}
  Self::ImmF32(_)=>{4}
  Self::ImmF64(_)=>{8}
  Self::PushI(_)=>{1+8}
  Self::PushU(_)=>{1+8}
  Self::PushF(_)=>{1+8}
  Self::PushDst(_)=>{1+8}
  Self::PushGlo(_)=>{2+8}
  Self::PushArg(_)=>{2+8}
  Self::PushLoc(_)=>{2+8}
  Self::Label(_)=>{0}
    }
}


pub fn
assemble_to(&self, buf: &mut Vec<u8>)
{
    match self
    {
  Self::Opcode(sym)=>{buf.push(*sym);}
  Self::Data(b)=>{buf.push(*b);}
  Self::ImmI8(i)=> {buf.push(*i as u8);}
  Self::ImmI16(i)=>{for b in i.to_be_bytes(){buf.push(b);}}
  Self::ImmI32(i)=>{for b in i.to_be_bytes(){buf.push(b);}}
  Self::ImmI64(i)=>{for b in i.to_be_bytes(){buf.push(b);}}
  Self::ImmU8(u)=> {buf.push(*u as u8);}
  Self::ImmU16(u)=>{for b in u.to_be_bytes(){buf.push(b);}}
  Self::ImmU32(u)=>{for b in u.to_be_bytes(){buf.push(b);}}
  Self::ImmU64(u)=>{for b in u.to_be_bytes(){buf.push(b);}}
  Self::ImmF32(f)=>{for b in f.to_be_bytes(){buf.push(b);}}
  Self::ImmF64(f)=>{for b in f.to_be_bytes(){buf.push(b);}}
  Self::PushI(i)=>{  buf.push(PUSH64);  for b in i.to_be_bytes(){buf.push(b);}}
  Self::PushU(u)=>{  buf.push(PUSH64);  for b in u.to_be_bytes(){buf.push(b);}}
  Self::PushF(f)=>{  buf.push(PUSH64);  for b in f.to_be_bytes(){buf.push(b);}}
  Self::Label(_)=>{}
  _=>{panic!();}
    }
}


pub fn   nop()-> Self{Self::Opcode(NOP)}
pub fn  addi()-> Self{Self::Opcode(ADDI)}
pub fn  addu()-> Self{Self::Opcode(ADDU)}
pub fn  addf()-> Self{Self::Opcode(ADDF)}
pub fn  subi()-> Self{Self::Opcode(SUBI)}
pub fn  subu()-> Self{Self::Opcode(SUBU)}
pub fn  subf()-> Self{Self::Opcode(SUBF)}
pub fn  muli()-> Self{Self::Opcode(MULI)}
pub fn  mulu()-> Self{Self::Opcode(MULU)}
pub fn  mulf()-> Self{Self::Opcode(MULF)}
pub fn  divi()-> Self{Self::Opcode(DIVI)}
pub fn  divu()-> Self{Self::Opcode(DIVU)}
pub fn  divf()-> Self{Self::Opcode(DIVF)}
pub fn  remi()-> Self{Self::Opcode(REMI)}
pub fn  remu()-> Self{Self::Opcode(REMU)}
pub fn  remf()-> Self{Self::Opcode(REMF)}

pub fn  shl()-> Self{Self::Opcode(SHL)}
pub fn  shr()-> Self{Self::Opcode(SHR)}
pub fn  and()-> Self{Self::Opcode(AND)}
pub fn  or()->  Self{Self::Opcode(OR)}
pub fn  xor()-> Self{Self::Opcode(XOR)}

pub fn  eq()->   Self{Self::Opcode(EQ)}
pub fn  neq()->  Self{Self::Opcode(NEQ)}
pub fn  eqf()->  Self{Self::Opcode(EQF)}
pub fn  neqf()-> Self{Self::Opcode(NEQF)}

pub fn  lti()->   Self{Self::Opcode(LTI)}
pub fn  lteqi()-> Self{Self::Opcode(LTEQI)}
pub fn  gti()->   Self{Self::Opcode(GTI)}
pub fn  gteqi()-> Self{Self::Opcode(GTEQI)}
pub fn  ltu()->   Self{Self::Opcode(LTU)}
pub fn  ltequ()-> Self{Self::Opcode(LTEQU)}
pub fn  gtu()->   Self{Self::Opcode(GTU)}
pub fn  gtequ()-> Self{Self::Opcode(GTEQU)}
pub fn  ltf()->   Self{Self::Opcode(LTF)}
pub fn  lteqf()-> Self{Self::Opcode(LTEQF)}
pub fn  gtf()->   Self{Self::Opcode(GTF)}
pub fn  gteqf()-> Self{Self::Opcode(GTEQF)}

pub fn  land()-> Self{Self::Opcode(LAND)}
pub fn  lor()->  Self{Self::Opcode(LOR)}

pub fn  neg()->  Self{Self::Opcode(NEG)}
pub fn  negf()-> Self{Self::Opcode(NEGF)}
pub fn  not()->  Self{Self::Opcode(NOT)}
pub fn  lnot()-> Self{Self::Opcode(LNOT)}

pub fn  itou()-> Self{Self::Opcode(ITOU)}
pub fn  utoi()-> Self{Self::Opcode(UTOI)}
pub fn  itof()-> Self{Self::Opcode(ITOF)}
pub fn  ftoi()-> Self{Self::Opcode(FTOI)}

pub fn  push0()-> Self{Self::Opcode(PUSH0)}
pub fn  push1()-> Self{Self::Opcode(PUSH1)}
pub fn  push2()-> Self{Self::Opcode(PUSH2)}
pub fn  push3()-> Self{Self::Opcode(PUSH3)}
pub fn  push4()-> Self{Self::Opcode(PUSH4)}
pub fn  push5()-> Self{Self::Opcode(PUSH5)}
pub fn  push6()-> Self{Self::Opcode(PUSH6)}
pub fn  push7()-> Self{Self::Opcode(PUSH7)}
pub fn  push8()-> Self{Self::Opcode(PUSH8)}
pub fn  pop()->   Self{Self::Opcode(POP)}
pub fn  dup()->   Self{Self::Opcode(DUP)}

pub fn  ldi8()->  Self{Self::Opcode(LDI8)}
pub fn  ldi16()-> Self{Self::Opcode(LDI16)}
pub fn  ldi32()-> Self{Self::Opcode(LDI32)}
pub fn  ldu8()->  Self{Self::Opcode(LDU8)}
pub fn  ldu16()-> Self{Self::Opcode(LDU16)}
pub fn  ldu32()-> Self{Self::Opcode(LDU32)}
pub fn  ldf32()-> Self{Self::Opcode(LDF32)}

pub fn  sti8()->  Self{Self::Opcode(STI8)}
pub fn  sti16()-> Self{Self::Opcode(STI16)}
pub fn  sti32()-> Self{Self::Opcode(STI32)}
pub fn  stu8()->  Self{Self::Opcode(STU8)}
pub fn  stu16()-> Self{Self::Opcode(STU16)}
pub fn  stu32()-> Self{Self::Opcode(STU32)}
pub fn  stf32()-> Self{Self::Opcode(STF32)}

pub fn  ld64()-> Self{Self::Opcode(LD64)}
pub fn  st64()-> Self{Self::Opcode(ST64)}

pub fn  glo()->Self{Self::Opcode(GLO)}
pub fn  arg()->Self{Self::Opcode(ARG)}
pub fn  loc()->Self{Self::Opcode(LOC)}
pub fn  spx()->Self{Self::Opcode(SPX)}

pub fn  prcal()->  Self{Self::Opcode(PRCAL)}
pub fn  cal()->  Self{Self::Opcode(CAL)}
pub fn  jmp()->  Self{Self::Opcode(JMP)}
pub fn  brz()->   Self{Self::Opcode(BRZ)}
pub fn  brnz()->   Self{Self::Opcode(BRNZ)}
pub fn  ret()->  Self{Self::Opcode(RET)}
pub fn pri()->  Self{Self::Opcode(PRI)}
pub fn pru()->  Self{Self::Opcode(PRU)}
pub fn prf()->  Self{Self::Opcode(PRF)}
pub fn repo()->  Self{Self::Opcode(REPO)}
pub fn  hlt()->  Self{Self::Opcode(HLT)}


pub fn
print_symbol(sym: u8)
{
    match sym
    {
  NOP=>{print!("nop");}

  ADDI=>{print!("addi");},
  SUBI=>{print!("subi");},
  MULI=>{print!("muli");},
  DIVI=>{print!("divi");},
  REMI=>{print!("remi");},
  ADDU=>{print!("addu");},
  SUBU=>{print!("subu");},
  MULU=>{print!("mulu");},
  DIVU=>{print!("divu");},
  REMU=>{print!("remu");},
  ADDF=>{print!("addf");},
  SUBF=>{print!("subf");},
  MULF=>{print!("mulf");},
  DIVF=>{print!("divf");},
  REMF=>{print!("remf");},

  SHL=>{print!("shl");},
  SHR=>{print!("shr");},
  AND=>{print!("and");},
  OR =>{print!("or");},
  XOR=>{print!("xor");},

  EQ =>{print!("eq");},
  NEQ=>{print!("neq");},

  EQF =>{print!("eqf");},
  NEQF=>{print!("neqf");},

  LTI  =>{print!("lti");},
  LTEQI=>{print!("lteqi");},
  GTI  =>{print!("gti");},
  GTEQI=>{print!("gteqi");},
  LTU  =>{print!("ltu");},
  LTEQU=>{print!("ltequ");},
  GTU  =>{print!("gtu");},
  GTEQU=>{print!("gtequ");},
  LTF  =>{print!("ltf");},
  LTEQF=>{print!("lteqf");},
  GTF  =>{print!("gtf");},
  GTEQF=>{print!("gteqf");},

  LAND=>{print!("land");},
  LOR =>{print!("lor");},

  NEG =>{print!("neg");},
  NEGF=>{print!("negf");},
  NOT =>{print!("not");},
  LNOT=>{print!("lnot");},
  ITOU=>{print!("itou");},
  UTOI=>{print!("utoi");},
  ITOF=>{print!("itof");},
  FTOI=>{print!("ftoi");},

  PUSH0  =>{print!("push0");}
  PUSH1  =>{print!("push1");}
  PUSH2  =>{print!("push2");}
  PUSH3  =>{print!("push3");}
  PUSH4  =>{print!("push4");}
  PUSH5  =>{print!("push5");}
  PUSH6  =>{print!("push6");}
  PUSH7  =>{print!("push7");}
  PUSH8  =>{print!("push8");}
  PUSHI8 =>{print!("pushi8");}
  PUSHI16=>{print!("pushi16");}
  PUSHI32=>{print!("pushi32");}
  PUSHU8 =>{print!("pushu8");}
  PUSHU16=>{print!("pushu16");}
  PUSHU32=>{print!("pushu32");}
  PUSHF32=>{print!("pushf32");}
  PUSH64 =>{print!("push64");}
  POP    =>{print!("pop");}
  DUP    =>{print!("dup");}

  LDI8 =>{print!("ldi8");},
  LDI16=>{print!("ldi16");},
  LDI32=>{print!("ldi32");},
  LDU8 =>{print!("ldu8");},
  LDU16=>{print!("ldu16");},
  LDU32=>{print!("ldu32");},
  LDF32=>{print!("ldf32");},
  LD64 =>{print!("ld64");},

  STI8 =>{print!("sti8");},
  STI16=>{print!("sti16");},
  STI32=>{print!("sti32");},
  STU8 =>{print!("stu8");},
  STU16=>{print!("stu16");},
  STU32=>{print!("stu32");},
  STF32=>{print!("stf32");},
  ST64 =>{print!("st64");},

  GLO=>{print!("glo");},
  ARG=>{print!("arg");},
  LOC=>{print!("loc");},
  SPX=>{print!("spx");},

  PRCAL=>{print!("prcal");}
  CAL=>{print!("cal");}
  JMP=>{print!("jmp");}
  BRZ=>{print!("brz");}
  BRNZ=>{print!("brnz");}
  RET=>{print!("ret");}
  PRI=>{print!("pri");}
  PRU=>{print!("pru");}
  PRF=>{print!("prf");}
  REPO=>{print!("repo");}
  HLT=>{print!("hlt");}
  _=>{}
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Opcode(sym)=>{Self::print_symbol(*sym);}
  Self::Data(b)=>    {print!("Data({})",*b);}
  Self::ImmI8(i)=> {print!("immi8 {}",*i);}
  Self::ImmI16(i)=>{print!("immi16 {}",*i);}
  Self::ImmI32(i)=>{print!("immi32 {}",*i);}
  Self::ImmI64(i)=>{print!("immi64 {}",*i);}
  Self::ImmU8(u)=> {print!("immu8 {}",*u);}
  Self::ImmU16(u)=>{print!("immu16 {}",*u);}
  Self::ImmU32(u)=>{print!("immu32 {}",*u);}
  Self::ImmU64(u)=>{print!("immu64 {}",*u);}
  Self::ImmF32(f)=>{print!("immf32 {}",*f);}
  Self::ImmF64(f)=>{print!("immf32 {}",*f);}
  Self::PushI(i)=>{print!("pushi {}",*i);}
  Self::PushU(u)=>{print!("pushu {}",*u);}
  Self::PushF(f)=>{print!("pushf {}",*f);}
  Self::PushDst(s)=>{print!("pushdst {}",s);}
  Self::PushGlo(s)=>{print!("pushglo {}",s);}
  Self::PushArg(s)=>{print!("pusharg {}",s);}
  Self::PushLoc(s)=>{print!("pushloc {}",s);}
  Self::Label(s)=>{print!("[{}]",s);}
    }
}


}


pub const NOP: u8 = 0;

pub const ADDI: u8 =  1;
pub const SUBI: u8 =  2;
pub const MULI: u8 =  3;
pub const DIVI: u8 =  4;
pub const REMI: u8 =  5;
pub const ADDU: u8 =  6;
pub const SUBU: u8 =  7;
pub const MULU: u8 =  8;
pub const DIVU: u8 =  9;
pub const REMU: u8 = 10;
pub const ADDF: u8 = 11;
pub const SUBF: u8 = 12;
pub const MULF: u8 = 13;
pub const DIVF: u8 = 14;
pub const REMF: u8 = 15;

pub const SHL: u8 = 20;
pub const SHR: u8 = 21;
pub const AND: u8 = 22;
pub const  OR: u8 = 23;
pub const XOR: u8 = 24;

pub const   EQ: u8 = 30;
pub const  NEQ: u8 = 31;
pub const  EQF: u8 = 32;
pub const NEQF: u8 = 33;

pub const   LTI: u8 = 34;
pub const LTEQI: u8 = 35;
pub const   GTI: u8 = 36;
pub const GTEQI: u8 = 37;
pub const   LTU: u8 = 38;
pub const LTEQU: u8 = 39;
pub const   GTU: u8 = 40;
pub const GTEQU: u8 = 41;
pub const   LTF: u8 = 42;
pub const LTEQF: u8 = 43;
pub const   GTF: u8 = 44;
pub const GTEQF: u8 = 45;

pub const LAND: u8 = 46;
pub const  LOR: u8 = 47;


pub const  NEG: u8 = 60;
pub const NEGF: u8 = 61;
pub const  NOT: u8 = 62;
pub const LNOT: u8 = 63;
pub const ITOU: u8 = 64;
pub const UTOI: u8 = 65;
pub const ITOF: u8 = 66;
pub const FTOI: u8 = 67;

pub const   PUSH0: u8 = 80;
pub const   PUSH1: u8 = 81;
pub const   PUSH2: u8 = 82;
pub const   PUSH3: u8 = 83;
pub const   PUSH4: u8 = 84;
pub const   PUSH5: u8 = 85;
pub const   PUSH6: u8 = 86;
pub const   PUSH7: u8 = 87;
pub const   PUSH8: u8 = 88;
pub const  PUSHI8: u8 = 89;
pub const PUSHI16: u8 = 90;
pub const PUSHI32: u8 = 91;
pub const  PUSHU8: u8 = 92;
pub const PUSHU16: u8 = 93;
pub const PUSHU32: u8 = 94;
pub const PUSHF32: u8 = 95;
pub const  PUSH64: u8 = 96;
pub const     POP: u8 = 97;
pub const     DUP: u8 = 98;

pub const  LDI8: u8 = 100;
pub const LDI16: u8 = 101;
pub const LDI32: u8 = 102;
pub const  LDU8: u8 = 103;
pub const LDU16: u8 = 104;
pub const LDU32: u8 = 105;
pub const LDF32: u8 = 106;
pub const  LD64: u8 = 107;

pub const  STI8: u8 = 120;
pub const STI16: u8 = 121;
pub const STI32: u8 = 122;
pub const  STU8: u8 = 123;
pub const STU16: u8 = 124;
pub const STU32: u8 = 125;
pub const STF32: u8 = 126;
pub const  ST64: u8 = 127;

pub const  GLO: u8 = 140;
pub const  ARG: u8 = 141;
pub const  LOC: u8 = 142;
pub const  SPX: u8 = 143;

pub const  PRCAL: u8 = 160;
pub const    CAL: u8 = 161;
pub const    JMP: u8 = 162;
pub const    BRZ: u8 = 163;
pub const   BRNZ: u8 = 164;
pub const    RET: u8 = 165;
pub const   PRI: u8 = 250;
pub const   PRU: u8 = 251;
pub const   PRF: u8 = 252;
pub const   REPO: u8 = 254;
pub const    HLT: u8 = 255;




