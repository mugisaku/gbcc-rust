



pub enum
ImmKind
{
  U8, U16, U32, U64,
  I8, I16, I32, I64,
  F32, F64,
  
}


#[derive(Clone)]
pub enum
Opcode
{
  Nop,

  Push0,
  Push1,
  Push2,
  Push3,
  Push4,
  Push5,
  Push6,
  Push7,
  Push8,

  Pop,
  Dup,

  Sx8, Sx16, Sx32,
  Tr8, Tr16, Tr32,

  B32toF,
  FtoB32,

  Xs,

  Lpc, Lfp, Lsp,
  Li,
  Ld8, Ld16, Ld32, Ld64,
  St8, St16, St32, St64,

  Neg, Negf,
  Not, Notl,

  Itof, Ftoi,


  Addi, Subi, Muli, Divi, Remi,
  Addu, Subu, Mulu, Divu, Remu,
  Addf, Subf, Mulf, Divf, Remf,

  Shl, Shr, And, Or, Xor,

  Eq,  Neq,
  Eqf, Neqf,

  Lti, Lteqi, Gti, Gteqi,
  Ltu, Ltequ, Gtu, Gtequ,
  Ltf, Lteqf, Gtf, Gteqf,

  Andl, Orl,

   Jmp,
   Brz,
  Brnz,

  Prcal,
    Cal,

  Ret,
  Hlt,

  Pri,

}


impl
Opcode
{


pub fn
to_str(&self)-> &'static str
{
    match self
    {
  Self::Nop=>{"nop"}

  Self::Push0=>{"push0"}
  Self::Push1=>{"push1"}
  Self::Push2=>{"push2"}
  Self::Push3=>{"push3"}
  Self::Push4=>{"push4"}
  Self::Push5=>{"push5"}
  Self::Push6=>{"push6"}
  Self::Push7=>{"push7"}
  Self::Push8=>{"push8"}

  Self::Pop=>{"pop"}
  Self::Dup=>{"dup"}

  Self::Sx8 =>{"sx8"}
  Self::Sx16=>{"sx16"}
  Self::Sx32=>{"sx32"}
  Self::Tr8 =>{"tr8"}
  Self::Tr16=>{"tr16"}
  Self::Tr32=>{"tr32"}

  Self::B32toF=>{"b32tof"}
  Self::FtoB32=>{"ftob32"}

  Self::Xs=>{"xs"}

  Self::Lpc=>{"lpc"}
  Self::Lfp=>{"lfp"}
  Self::Lsp=>{"lsp"}

  Self::Li  =>{"li"}
  Self::Ld8 =>{"ld8"}
  Self::Ld16=>{"ld16"}
  Self::Ld32=>{"ld32"}
  Self::Ld64=>{"ld64"}
  Self::St8 =>{"st8"}
  Self::St16=>{"st16"}
  Self::St32=>{"st32"}
  Self::St64=>{"st64"}

  Self::Neg =>{"neg"}
  Self::Negf=>{"negf"}
  Self::Not =>{"not"}
  Self::Notl=>{"notl"}

  Self::Itof=>{"itof"}
  Self::Ftoi=>{"ftoi"}


  Self::Addi=>{"addi"}
  Self::Subi=>{"subi"}
  Self::Muli=>{"muli"}
  Self::Divi=>{"divi"}
  Self::Remi=>{"remi"}
  Self::Addu=>{"addu"}
  Self::Subu=>{"subu"}
  Self::Mulu=>{"mulu"}
  Self::Divu=>{"divu"}
  Self::Remu=>{"remu"}
  Self::Addf=>{"addf"}
  Self::Subf=>{"subf"}
  Self::Mulf=>{"mulf"}
  Self::Divf=>{"divf"}
  Self::Remf=>{"remf"}

  Self::Shl=>{"shl"}
  Self::Shr=>{"shr"}
  Self::And=>{"and"}
  Self::Or =>{"or"}
  Self::Xor=>{"xor"}

  Self::Eq  =>{"eq"}
  Self::Neq =>{"neq"}
  Self::Eqf =>{"eqf"}
  Self::Neqf=>{"neqf"}

  Self::Lti  =>{"lti"}
  Self::Lteqi=>{"lteqi"}
  Self::Gti  =>{"gti"}
  Self::Gteqi=>{"gteqi"}
  Self::Ltu  =>{"ltu"}
  Self::Ltequ=>{"ltequ"}
  Self::Gtu  =>{"gtu"}
  Self::Gtequ=>{"gtequ"}
  Self::Ltf  =>{"ltf"}
  Self::Lteqf=>{"lteqf"}
  Self::Gtf  =>{"gtf"}
  Self::Gteqf=>{"gteqf"}

  Self::Andl=>{"andl"}
  Self::Orl=>{"orl"}

  Self::Jmp=>{"jmp"}
  Self::Brz=>{"brz"}
  Self::Brnz=>{"brnz"}

  Self::Prcal=>{"prcal"}
  Self::Cal=>{"cal"}

  Self::Ret=>{"ret"}
  Self::Hlt=>{"hlt"}

  Self::Pri=>{"pri"}
    }
}


pub fn
print(&self)
{
  print!("{}",self.to_str());
}


}


impl
std::convert::From<u8> for Opcode
{


fn
from(b: u8)-> Self
{
    match b
    {
  (op) if op == Self::Nop as u8=>{Self::Nop}
  (op) if op == Self::Push0 as u8=>{Self::Push0}
  (op) if op == Self::Push1 as u8=>{Self::Push1}
  (op) if op == Self::Push2 as u8=>{Self::Push2}
  (op) if op == Self::Push3 as u8=>{Self::Push3}
  (op) if op == Self::Push4 as u8=>{Self::Push4}
  (op) if op == Self::Push5 as u8=>{Self::Push5}
  (op) if op == Self::Push6 as u8=>{Self::Push6}
  (op) if op == Self::Push7 as u8=>{Self::Push7}
  (op) if op == Self::Push8 as u8=>{Self::Push8}
  (op) if op == Self::Pop as u8=>{Self::Pop}
  (op) if op == Self::Dup as u8=>{Self::Dup}
  (op) if op == Self::Xs as u8=>{Self::Xs}
  (op) if op == Self::Lpc as u8=>{Self::Lpc}
  (op) if op == Self::Lfp as u8=>{Self::Lfp}
  (op) if op == Self::Lsp as u8=>{Self::Lsp}
  (op) if op == Self::Li as u8=>{Self::Li}
  (op) if op == Self::Ld8 as u8=>{Self::Ld8}
  (op) if op == Self::Ld16 as u8=>{Self::Ld16}
  (op) if op == Self::Ld32 as u8=>{Self::Ld32}
  (op) if op == Self::Ld64 as u8=>{Self::Ld64}
  (op) if op == Self::St8 as u8=>{Self::St8}
  (op) if op == Self::St16 as u8=>{Self::St16}
  (op) if op == Self::St32 as u8=>{Self::St32}
  (op) if op == Self::St64 as u8=>{Self::St64}
  (op) if op == Self::Sx8 as u8=>{Self::Sx8}
  (op) if op == Self::Sx16 as u8=>{Self::Sx16}
  (op) if op == Self::Sx32 as u8=>{Self::Sx32}
  (op) if op == Self::Tr8 as u8=>{Self::Tr8}
  (op) if op == Self::Tr16 as u8=>{Self::Tr16}
  (op) if op == Self::Tr32 as u8=>{Self::Tr32}
  (op) if op == Self::B32toF as u8=>{Self::B32toF}
  (op) if op == Self::FtoB32 as u8=>{Self::FtoB32}
  (op) if op == Self::Neg  as u8=>{Self::Neg}
  (op) if op == Self::Negf as u8=>{Self::Negf}
  (op) if op == Self::Not  as u8=>{Self::Not}
  (op) if op == Self::Notl as u8=>{Self::Notl}
  (op) if op == Self::Itof as u8=>{Self::Itof}
  (op) if op == Self::Ftoi as u8=>{Self::Ftoi}
  (op) if op == Self::Addi as u8=>{Self::Addi}
  (op) if op == Self::Subi as u8=>{Self::Subi}
  (op) if op == Self::Muli as u8=>{Self::Muli}
  (op) if op == Self::Divi as u8=>{Self::Divi}
  (op) if op == Self::Remi as u8=>{Self::Remi}
  (op) if op == Self::Addu as u8=>{Self::Addu}
  (op) if op == Self::Subu as u8=>{Self::Subu}
  (op) if op == Self::Mulu as u8=>{Self::Mulu}
  (op) if op == Self::Divu as u8=>{Self::Divu}
  (op) if op == Self::Remu as u8=>{Self::Remu}
  (op) if op == Self::Addf as u8=>{Self::Addf}
  (op) if op == Self::Subf as u8=>{Self::Subf}
  (op) if op == Self::Mulf as u8=>{Self::Mulf}
  (op) if op == Self::Divf as u8=>{Self::Divf}
  (op) if op == Self::Remf as u8=>{Self::Remf}
  (op) if op == Self::Shl as u8=>{Self::Shl}
  (op) if op == Self::Shr as u8=>{Self::Shr}
  (op) if op == Self::And as u8=>{Self::And}
  (op) if op == Self::Or  as u8=>{Self::Or}
  (op) if op == Self::Xor as u8=>{Self::Xor}
  (op) if op == Self::Eq  as u8=>{Self::Eq}
  (op) if op == Self::Neq as u8=>{Self::Neq}
  (op) if op == Self::Eqf  as u8=>{Self::Eqf}
  (op) if op == Self::Neqf as u8=>{Self::Neqf}
  (op) if op == Self::Lti   as u8=>{Self::Lti}
  (op) if op == Self::Lteqi as u8=>{Self::Lteqi}
  (op) if op == Self::Gti   as u8=>{Self::Gti}
  (op) if op == Self::Gteqi as u8=>{Self::Gteqi}
  (op) if op == Self::Ltu   as u8=>{Self::Ltu}
  (op) if op == Self::Ltequ as u8=>{Self::Ltequ}
  (op) if op == Self::Gtu   as u8=>{Self::Gtu}
  (op) if op == Self::Gtequ as u8=>{Self::Gtequ}
  (op) if op == Self::Ltf   as u8=>{Self::Ltf}
  (op) if op == Self::Lteqf as u8=>{Self::Lteqf}
  (op) if op == Self::Gtf   as u8=>{Self::Gtf}
  (op) if op == Self::Gteqf as u8=>{Self::Gteqf}
  (op) if op == Self::Andl as u8=>{Self::Andl}
  (op) if op == Self::Orl  as u8=>{Self::Orl}
  (op) if op == Self::Jmp as u8=>{Self::Jmp}
  (op) if op == Self::Brz as u8=>{Self::Brz}
  (op) if op == Self::Brnz as u8=>{Self::Brnz}
  (op) if op == Self::Prcal as u8=>{Self::Prcal}
  (op) if op == Self::Cal as u8=>{Self::Cal}
  (op) if op == Self::Ret as u8=>{Self::Ret}
  (op) if op == Self::Hlt as u8=>{Self::Hlt}
  (op) if op == Self::Pri as u8=>{Self::Pri}
  _=>{panic!();}
    }
}


}




#[derive(Clone)]
pub enum
Postfix
{
  None,
  Identifier(String),
  Bool(bool),
  Int(i64),
  Float(f64),

}


impl
Postfix
{


pub fn
print(&self)
{
    match self
    {
  Self::None=>{}
  Self::Identifier(s)=>{print!("{}",s);}
  Self::Bool(b)=>{print!("{}",*b);}
  Self::Int(i)=>{print!("{}",*i);}
  Self::Float(f)=>{print!("{}",*f);}
    }
}


}




#[derive(Clone)]
pub struct
AsmLine
{
   opcode: Opcode,
  postfix: Postfix,

}


impl
AsmLine
{


pub fn
new(opcode: Opcode, postfix: Postfix)->Self
{
  Self{opcode,postfix}
}


pub fn
get_opcode(&self)-> &Opcode
{
  &self.opcode
}


pub fn
get_postfix(&self)-> &Postfix
{
  &self.postfix
}


}




#[derive(Clone)]
pub struct
AsmBlock
{
  label: String,

  lines: Vec<AsmLine>,

  position: usize,

}


impl
AsmBlock
{


pub fn
new(label: &str)-> Self
{
  Self{
    label: label.to_string(),
    lines: Vec::new(),
    position: 0,
  }
}


pub fn
get_label(&self)-> &String
{
  &self.label
}


pub fn
get_lines(&self)-> &Vec<AsmLine>
{
  &self.lines
}


pub fn
get_lines_mut(&mut self)-> &mut Vec<AsmLine>
{
  &mut self.lines
}


pub fn
get_position(&self)-> usize
{
  self.position
}


}




#[derive(Clone)]
pub struct
AsmTable
{
  core: Vec<AsmBlock>,

}


impl
AsmTable
{


pub fn
new()-> Self
{
  Self{core: vec![AsmBlock::new("")]}
}


pub fn
get_core(&self)-> &Vec<AsmBlock>
{
  &self.core
}


pub fn
get_core_mut(&mut self)-> &mut Vec<AsmBlock>
{
  &mut self.core
}


pub fn
find_block(&self, label: &str)-> Option<&AsmBlock>
{
    for blk in &self.core
    {
        if &blk.label == label
        {
          return Some(blk);
        }
    }


  None
}


pub fn
reset_block_position(&mut self)
{
  let  mut pos = 0usize;

    for blk in &mut self.core
    {
      blk.position = pos                   ;
                     pos += blk.lines.len();
    }
}


pub fn
push_table(&mut self, other: &mut Self)
{
  self.core.append(&mut other.core);
}


pub fn
push_label(&mut self, s: &str)
{
  let  blk = AsmBlock::new(s);

  self.core.push(blk);
}


pub fn
push_opcode(&mut self, opcode: Opcode)
{
  let  ln = AsmLine::new(opcode,Postfix::None);

  self.core.last_mut().unwrap().get_lines_mut().push(ln);
}


pub fn
push_line(&mut self, ln: AsmLine)
{
  self.core.last_mut().unwrap().lines.push(ln);
}


pub fn
push_li_bool(&mut self, b: bool)
{
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Bool(b)));
}


pub fn
push_li_int(&mut self, i: i64)
{
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Int(i)));
}


pub fn
push_li_float(&mut self, f: f64)
{
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Float(f)));
}


pub fn
push_li_global_addr(&mut self, off: usize)
{
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Int(off as i64)));
}


pub fn
push_li_fn_addr(&mut self, off: usize)
{
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Int(off as i64)));
  self.push_line(AsmLine::new(Opcode::Ld64,Postfix::None));
}


pub fn
push_li_local_addr(&mut self, off: usize)
{
  self.push_line(AsmLine::new(Opcode::Lfp,Postfix::None));
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Int(off as i64)));
  self.push_line(AsmLine::new(Opcode::Addi,Postfix::None));
}


pub fn
push_li_parameter_addr(&mut self, off: usize)
{
  self.push_line(AsmLine::new(Opcode::Lfp,Postfix::None));
  self.push_line(AsmLine::new(Opcode::Li,Postfix::Int(off as i64)));
  self.push_line(AsmLine::new(Opcode::Subi,Postfix::None));
}


pub fn
push_jmp(&mut self, s: &str)
{
  self.push_line(AsmLine::new(Opcode::Jmp,Postfix::Identifier(s.to_string())));
}


pub fn
push_brz(&mut self, s: &str)
{
  self.push_line(AsmLine::new(Opcode::Brz,Postfix::Identifier(s.to_string())));
}


pub fn
push_brnz(&mut self, s: &str)
{
  self.push_line(AsmLine::new(Opcode::Brnz,Postfix::Identifier(s.to_string())));
}


pub fn
print(&self)
{
  let  mut n = 0usize;

    for blk in &self.core
    {
      println!("{}",&blk.label);

        for ln in &blk.lines
        {
          print!("[{:0>5}] ",n);

          n += 1;

          ln.opcode.print();

          print!(" ");

          ln.postfix.print();

          println!("");
        }


      println!("");
    }
}


}




