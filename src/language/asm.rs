

use super::*;
use crate::source_file::{
  SourceInfo,
  Error,

};


#[derive(Clone)]
pub enum
Opcode
{
  Nop,

  Pushid,
  Pushpc,
  Pushfp,
  Pushsp,

  Push8,
  Push16,
  Push32,
  Push64,

  Xs8,
  Xs16,
  Xs32,

  Jmp8,
  Jmp16,
  Jmp32,

  Brz8,
  Brz16,
  Brz32,

  Brnz8,
  Brnz16,
  Brnz32,

  Pop,
  Dup,

  Ld_i8, Ld_i16, Ld_i32, Ld_i64, 
  Ld_u8, Ld_u16, Ld_u32, 
  St_i8, St_i16, St_i32, St_i64,

  Neg, Not,

  Add, Sub, Mul, Div, Rem,
  Shl, Shr, And, Or, Xor,
  Eq, Neq, Lt, Lteq, Gt, Gteq,

  Lnot, Land, Lor,

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

  Self::Pushid=>{"pushid"}
  Self::Pushpc=>{"pushpc"}
  Self::Pushfp=>{"pushfp"}
  Self::Pushsp=>{"pushsp"}

  Self::Push8 =>{"push8"}
  Self::Push16=>{"push16"}
  Self::Push32=>{"push32"}
  Self::Push64=>{"push64"}

  Self::Xs8 =>{"xs8"}
  Self::Xs16=>{"xs16"}
  Self::Xs32=>{"xs32"}

  Self::Jmp8 =>{"jmp8"}
  Self::Jmp16=>{"jmp16"}
  Self::Jmp32=>{"jmp32"}

  Self::Brz8 =>{"brz8"}
  Self::Brz16=>{"brz16"}
  Self::Brz32=>{"brz32"}

  Self::Brnz8 =>{"brnz8"}
  Self::Brnz16=>{"brnz16"}
  Self::Brnz32=>{"brnz32"}

  Self::Pop=>{"pop"}
  Self::Dup=>{"dup"}

  Self::Ld_i8=>{"ld_i8"}
  Self::Ld_i16=>{"ld_i16"}
  Self::Ld_i32=>{"ld_i32"}
  Self::Ld_i64=>{"ld_i64"}
  Self::Ld_u8=>{"ld_u8"}
  Self::Ld_u16=>{"ld_u16"}
  Self::Ld_u32=>{"ld_u32"}
  Self::St_i8=>{"st_i8"}
  Self::St_i16=>{"st_i16"}
  Self::St_i32=>{"st_i32"}
  Self::St_i64=>{"st_i64"}

  Self::Neg =>{"neg"}
  Self::Not =>{"not"}

  Self::Add=>{"add"}
  Self::Sub=>{"sub"}
  Self::Mul=>{"mul"}
  Self::Div=>{"div"}
  Self::Rem=>{"rem"}

  Self::Shl=>{"shl"}
  Self::Shr=>{"shr"}
  Self::And=>{"and"}
  Self::Or =>{"or"}
  Self::Xor=>{"xor"}

  Self::Lnot=>{"lnot"}
  Self::Land=>{"land"}
  Self::Lor =>{"lor"}

  Self::Eq  =>{"eq"}
  Self::Neq =>{"neq"}

  Self::Lt  =>{"lt"}
  Self::Lteq=>{"lteq"}
  Self::Gt  =>{"gt"}
  Self::Gteq=>{"gteq"}

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
std::convert::TryFrom<u8> for Opcode
{


type  Error = ();


fn
try_from(b: u8)-> Result<Self,Self::Error>
{
    match b
    {
  (op) if op == Self::Nop as u8=>{Ok(Self::Nop)}
  (op) if op == Self::Pushid as u8=>{Ok(Self::Pushid)}
  (op) if op == Self::Pushpc as u8=>{Ok(Self::Pushpc)}
  (op) if op == Self::Pushfp as u8=>{Ok(Self::Pushfp)}
  (op) if op == Self::Pushsp as u8=>{Ok(Self::Pushsp)}
  (op) if op == Self::Push8 as u8=>{Ok(Self::Push8)}
  (op) if op == Self::Push16 as u8=>{Ok(Self::Push16)}
  (op) if op == Self::Push32 as u8=>{Ok(Self::Push32)}
  (op) if op == Self::Push64 as u8=>{Ok(Self::Push64)}
  (op) if op == Self::Xs8 as u8=>{Ok(Self::Xs8)}
  (op) if op == Self::Xs16 as u8=>{Ok(Self::Xs16)}
  (op) if op == Self::Xs32 as u8=>{Ok(Self::Xs32)}
  (op) if op == Self::Jmp8 as u8=>{Ok(Self::Jmp8)}
  (op) if op == Self::Jmp16 as u8=>{Ok(Self::Jmp16)}
  (op) if op == Self::Jmp32 as u8=>{Ok(Self::Jmp32)}
  (op) if op == Self::Brz8 as u8=>{Ok(Self::Brz8)}
  (op) if op == Self::Brz16 as u8=>{Ok(Self::Brz16)}
  (op) if op == Self::Brz32 as u8=>{Ok(Self::Brz32)}
  (op) if op == Self::Brnz8 as u8=>{Ok(Self::Brnz8)}
  (op) if op == Self::Brnz16 as u8=>{Ok(Self::Brnz16)}
  (op) if op == Self::Brnz32 as u8=>{Ok(Self::Brnz32)}
  (op) if op == Self::Pop as u8=>{Ok(Self::Pop)}
  (op) if op == Self::Dup as u8=>{Ok(Self::Dup)}
  (op) if op == Self::Ld_i8 as u8=>{Ok(Self::Ld_i8)}
  (op) if op == Self::Ld_i16 as u8=>{Ok(Self::Ld_i16)}
  (op) if op == Self::Ld_i32 as u8=>{Ok(Self::Ld_i32)}
  (op) if op == Self::Ld_i64 as u8=>{Ok(Self::Ld_i64)}
  (op) if op == Self::Ld_u8 as u8=>{Ok(Self::Ld_u8)}
  (op) if op == Self::Ld_u16 as u8=>{Ok(Self::Ld_u16)}
  (op) if op == Self::Ld_u32 as u8=>{Ok(Self::Ld_u32)}
  (op) if op == Self::St_i8 as u8=>{Ok(Self::St_i8)}
  (op) if op == Self::St_i16 as u8=>{Ok(Self::St_i16)}
  (op) if op == Self::St_i32 as u8=>{Ok(Self::St_i32)}
  (op) if op == Self::St_i64 as u8=>{Ok(Self::St_i64)}
  (op) if op == Self::Neg  as u8=>{Ok(Self::Neg)}
  (op) if op == Self::Not  as u8=>{Ok(Self::Not)}
  (op) if op == Self::Add as u8=>{Ok(Self::Add)}
  (op) if op == Self::Sub as u8=>{Ok(Self::Sub)}
  (op) if op == Self::Mul as u8=>{Ok(Self::Mul)}
  (op) if op == Self::Div as u8=>{Ok(Self::Div)}
  (op) if op == Self::Rem as u8=>{Ok(Self::Rem)}
  (op) if op == Self::Shl as u8=>{Ok(Self::Shl)}
  (op) if op == Self::Shr as u8=>{Ok(Self::Shr)}
  (op) if op == Self::And as u8=>{Ok(Self::And)}
  (op) if op == Self::Or  as u8=>{Ok(Self::Or)}
  (op) if op == Self::Xor as u8=>{Ok(Self::Xor)}
  (op) if op == Self::Lnot as u8=>{Ok(Self::Lnot)}
  (op) if op == Self::Land as u8=>{Ok(Self::Land)}
  (op) if op == Self::Lor as u8=>{Ok(Self::Lor)}
  (op) if op == Self::Eq  as u8=>{Ok(Self::Eq)}
  (op) if op == Self::Neq as u8=>{Ok(Self::Neq)}
  (op) if op == Self::Lt   as u8=>{Ok(Self::Lt)}
  (op) if op == Self::Lteq as u8=>{Ok(Self::Lteq)}
  (op) if op == Self::Gt   as u8=>{Ok(Self::Gt)}
  (op) if op == Self::Gteq as u8=>{Ok(Self::Gteq)}
  (op) if op == Self::Cal as u8=>{Ok(Self::Cal)}
  (op) if op == Self::Ret as u8=>{Ok(Self::Ret)}
  (op) if op == Self::Hlt as u8=>{Ok(Self::Hlt)}
  (op) if op == Self::Pri as u8=>{Ok(Self::Pri)}
  _=>{Err(())}
    }
}


}




#[derive(Clone,Default)]
struct
LineInfo
{
  offset: usize,
    size: usize,

}


#[derive(Clone)]
struct
Destination
{
  label: String,
  index: usize,

}




#[derive(Clone)]
pub enum
AsmLine
{
  Label(String),

  Opcode(Opcode),

  Push8(i8),
  Push16(i16),
  Push32(i32),
  Push64(i64),

    Xs8(u8),
   Xs16(u16),
   Xs32(u32),

   Jmp(i32,Destination),
   Brz(i32,Destination),
  Brnz(i32,Destination),

}


impl
AsmLine
{


pub fn
make_push(i: i64)-> Self
{
  let  iabs = i.abs();

       if iabs <= ( i8::MAX as i64){Self::Push8( i as i8)}
  else if iabs <= (i16::MAX as i64){Self::Push16(i as i16)}
  else if iabs <= (i32::MAX as i64){Self::Push32(i as i32)}
  else                             {Self::Push64(i as i64)}
}




pub fn
make_xs(n: usize)-> Self
{
       if n <= ( u8::MAX as usize){Self::Xs8( n as  u8)}
  else if n <= (u16::MAX as usize){Self::Xs16(n as u16)}
  else if n <= (u32::MAX as usize){Self::Xs32(n as u32)}
  else{panic!();}
}


pub fn
make_jmp(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Jmp(0,dst)
}


pub fn
make_brz(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Brz(0,dst)
}


pub fn
make_brnz(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Brnz(0,dst)
}


pub fn
get_size_of_i(i: i32)-> usize
{
    match i.abs()
    {
  (v) if v <= ( i8::MAX as i32)=>{1}
  (v) if v <= (i16::MAX as i32)=>{2}
  _=>{4}
    }
}


pub fn
get_size(&self)-> usize
{
    match self
    {
  Self::Label(_)=>{0}

  Self::Opcode(_)=>{1}

  Self::Push8(_)
 |Self::Xs8(_)=>{2}

  Self::Push16(_)
 |Self::Xs16(_)=>{3}

  Self::Push32(_)
 |Self::Xs32(_)=>{5}

  Self::Push64(_)=>{9}

  Self::Jmp(i,_) =>{1+Self::get_size_of_i(*i)}
  Self::Brz(i,_) =>{1+Self::get_size_of_i(*i)}
  Self::Brnz(i,_)=>{1+Self::get_size_of_i(*i)}
    }
}


pub fn
write_u8_to(op: Opcode, u: u8, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  buf.push(u);
}


pub fn
write_u16_to(op: Opcode, u: u16, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_u32_to(op: Opcode, u: u32, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_u64_to(op: Opcode, u: u64, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_to(&self, buf: &mut Vec<u8>)
{
    match self
    {
  Self::Label(_)=>{}

  Self::Opcode(op)=>{buf.push(op.clone() as u8);}

  Self::Push8(i) =>{Self::write_u8_to(Opcode::Push8 ,*i as u8,buf);}
  Self::Push16(i)=>{Self::write_u16_to(Opcode::Push16,*i as u16,buf);}
  Self::Push32(i)=>{Self::write_u32_to(Opcode::Push32,*i as u32,buf);}
  Self::Push64(i)=>{Self::write_u64_to(Opcode::Push64,*i as u64,buf);}
  Self::Xs8(u) =>{Self::write_u8_to(Opcode::Xs8 ,*u,buf);}
  Self::Xs16(u)=>{Self::write_u16_to(Opcode::Xs16,*u,buf);}
  Self::Xs32(u)=>{Self::write_u32_to(Opcode::Xs32,*u,buf);}
  Self::Jmp(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Jmp8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Jmp16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Jmp32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
  Self::Brz(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Brz8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Brz16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Brz32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
  Self::Brnz(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Brnz8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Brnz16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Brnz32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
    }
}


pub fn
print_to(&self, buf: &mut String)
{
    match self
    {
  Self::Label(s)=>{buf.push_str(&format!("[{}]",s));}

  Self::Opcode(op)=>{buf.push_str(op.to_str());}

  Self::Push8(i) =>{buf.push_str(&format!("push8 {}",*i));}
  Self::Push16(i)=>{buf.push_str(&format!("push16 {}",*i));}
  Self::Push32(i)=>{buf.push_str(&format!("push32 {}",*i));}
  Self::Push64(i)=>{buf.push_str(&format!("push64 {}",*i));}

  Self::Xs8(u) =>{buf.push_str(&format!("xs8 {}",*u));}
  Self::Xs16(u)=>{buf.push_str(&format!("xs16 {}",*u));}
  Self::Xs32(u)=>{buf.push_str(&format!("xs32 {}",*u));}

  Self::Jmp(i,dst)=>{buf.push_str(&format!("jmp {}({})",&dst.label,*i));}
  Self::Brz(i,dst)=>{buf.push_str(&format!("brz {}({})",&dst.label,*i));}
  Self::Brnz(i,dst)=>{buf.push_str(&format!("brnz {}({})",&dst.label,*i));}
    }
}


pub fn
print(&self)
{
  let  mut buf = String::new();

  self.print_to(&mut buf);

  print!("{}",&buf);
}


}




#[derive(Clone)]
pub enum
AsmEvalKind
{
  Void,
  Value,

  DerefI8,
  DerefI16,
  DerefI32,
  DerefI64,
  DerefU8,
  DerefU16,
  DerefU32,

}


#[derive(Clone)]
pub struct
AsmEvalText
{
  lines: Vec<AsmLine>,

  kind: AsmEvalKind,

}


impl
AsmEvalText
{


pub fn
new()-> Self
{
  Self{lines: Vec::new(), kind: AsmEvalKind::Void}
}


pub fn
is_deref(&self)-> bool
{
    match &self.kind
    {
  AsmEvalKind::DerefI8
 |AsmEvalKind::DerefI16
 |AsmEvalKind::DerefI32
 |AsmEvalKind::DerefI64
 |AsmEvalKind::DerefU8
 |AsmEvalKind::DerefU16
 |AsmEvalKind::DerefU32=>{true}
  _=>{false}
    }
}


pub fn
get_kind(&self)-> &AsmEvalKind
{
  &self.kind
}


pub fn
change_kind(&mut self, k: AsmEvalKind)
{
  self.push_load();

  self.kind = k;
}


pub fn
push_to_ptr(&mut self)
{
    match &self.kind
    {
  AsmEvalKind::Void=>{panic!();}
  _=>{}
    }


  self.kind = AsmEvalKind::Value;
}


pub fn
push_opcode(&mut self, opcode: Opcode)
{
  self.lines.push(AsmLine::Opcode(opcode));
}


pub fn
push_bool(&mut self, b: bool)
{
  self.lines.push(AsmLine::Push8(if b{1} else{0}));

  self.kind = AsmEvalKind::Value;
}


pub fn
push_i64(&mut self, i: i64)
{
  self.lines.push(AsmLine::make_push(i));

  self.kind = AsmEvalKind::Value;
}


pub fn
push_global_var(&mut self, off: usize)
{
  self.push_i64(off as i64);

  self.kind = AsmEvalKind::DerefI64;
}


pub fn
push_fn(&mut self, off: usize)
{
  self.push_i64(off as i64);
  self.push_opcode(Opcode::Ld_i64);

  self.kind = AsmEvalKind::Value;
}


pub fn
push_local_var(&mut self, off: isize)
{
  self.push_opcode(Opcode::Pushfp);
  self.push_i64(off as i64);
  self.push_opcode(Opcode::Add);

  self.kind = AsmEvalKind::DerefI64;
}


pub fn
push_call(&mut self, args: Vec<Self>)
{
  self.push_load();

  let  arg_n = args.len();

    for a in args
    {
      self.lines.extend(a.lines);
    }


  self.push_i64(arg_n as i64);

  self.push_opcode(Opcode::Cal);

  self.kind = AsmEvalKind::Value;
}


pub fn
push_load(&mut self)
{
    match &self.kind
    {
  AsmEvalKind::Void    =>{panic!();}
  AsmEvalKind::Value   =>{}
  AsmEvalKind::DerefI8 =>{self.push_opcode(Opcode::Ld_i8 );}
  AsmEvalKind::DerefI16=>{self.push_opcode(Opcode::Ld_i16);}
  AsmEvalKind::DerefI32=>{self.push_opcode(Opcode::Ld_i32);}
  AsmEvalKind::DerefI64=>{self.push_opcode(Opcode::Ld_i64);}
  AsmEvalKind::DerefU8 =>{self.push_opcode(Opcode::Ld_u8 );}
  AsmEvalKind::DerefU16=>{self.push_opcode(Opcode::Ld_u16);}
  AsmEvalKind::DerefU32=>{self.push_opcode(Opcode::Ld_u32);}
    }


  self.kind = AsmEvalKind::Value;
}


pub fn
push_unary(&mut self, op: &str)
{
  self.push_load();

    match op
    {
  (s) if s == "-"=>{self.push_opcode(Opcode::Neg);}
  (s) if s == "^"=>{self.push_opcode(Opcode::Not);}
  (s) if s == "!"=>{self.push_opcode(Opcode::Lnot);}
  _=>{panic!();}
    }
}


pub fn
push_binary(&mut self, mut other: Self, op_s: &str)
{
   self.push_load();
  other.push_load();

  self.lines.extend(other.lines);

  let  op = match op_s
    {
  (s) if s ==  "+"=>{Opcode::Add}
  (s) if s ==  "-"=>{Opcode::Sub}
  (s) if s ==  "*"=>{Opcode::Mul}
  (s) if s ==  "/"=>{Opcode::Div}
  (s) if s ==  "%"=>{Opcode::Rem}
  (s) if s == "<<"=>{Opcode::Shl}
  (s) if s == ">>"=>{Opcode::Shr}
  (s) if s ==  "&"=>{Opcode::And}
  (s) if s ==  "|"=>{Opcode::Or}
  (s) if s ==  "^"=>{Opcode::Xor}
  (s) if s == "&&"=>{Opcode::Land}
  (s) if s == "||"=>{Opcode::Lor}
  (s) if s == "=="=>{Opcode::Eq}
  (s) if s == "!="=>{Opcode::Neq}
  (s) if s ==  "<"=>{Opcode::Lt}
  (s) if s == "<="=>{Opcode::Lteq}
  (s) if s ==  ">"=>{Opcode::Gt}
  (s) if s == ">="=>{Opcode::Gteq}
  _=>{panic!();}
    };


  self.push_opcode(op);
}


pub fn
push_text(&mut self, mut other: Self)
{
  self.lines.append(&mut other.lines);
}


}




#[derive(Clone)]
pub struct
AsmText
{
  lines: Vec<(AsmLine,LineInfo)>,

}


impl
AsmText
{


pub fn
new()-> Self
{
  let  li = LineInfo::default();

  Self{
    lines: vec![(AsmLine::Opcode(Opcode::Nop),li)],
  }
}


pub fn
set_xs(&mut self, sz: usize)
{
    if let Some((ln,_)) = self.lines.first_mut()
    {
      *ln = AsmLine::make_xs(sz/WORD_SIZE);
    }
}


pub fn
terminate(&mut self)
{
    if let Some((ln,_)) = self.lines.last()
    {
        if let AsmLine::Opcode(op) = ln
        {
            if (op.clone() as u8) == (Opcode::Ret as u8)
            {
              return;
            }
        }
    }


  self.push_opcode(Opcode::Ret);
}


pub fn
push_label(&mut self, s: &str)
{
  let  li = LineInfo::default();

  self.lines.push((AsmLine::Label(s.to_string()),li));
}


pub fn
push_opcode(&mut self, opcode: Opcode)
{
  let  ln = AsmLine::Opcode(opcode);

  let  li = LineInfo::default();

  self.lines.push((ln,li));
}


pub fn
push_line(&mut self, ln: AsmLine)
{
  let  li = LineInfo::default();

  self.lines.push((ln,li));
}


pub fn
push_eval_text(&mut self, et: AsmEvalText)
{
    for ln in et.lines
    {
      self.push_line(ln);
    }
}


pub fn
push_i64(&mut self, i: i64)
{
  self.push_line(AsmLine::make_push(i));
}


pub fn
push_jmp(&mut self, s: &str)
{
  self.push_line(AsmLine::make_jmp(s));
}


pub fn
push_brz(&mut self, s: &str)
{
  self.push_line(AsmLine::make_brz(s));
}


pub fn
push_brnz(&mut self, s: &str)
{
  self.push_line(AsmLine::make_brnz(s));
}


pub fn
try_push_assign(&mut self, srcinf: &SourceInfo, mut l: AsmEvalText, mut r: AsmEvalText, op: &str)-> Result<(),Error>
{
  let  k = l.kind.clone();

    if op == "="
    {
      r.push_load();

      l.push_text(r);
    }

  else
    {
      l.push_opcode(Opcode::Dup);
      l.push_load();

           if op ==  "+="{l.push_binary(r,"+");}
      else if op ==  "-="{l.push_binary(r,"-");}
      else if op ==  "*="{l.push_binary(r,"*");}
      else if op ==  "/="{l.push_binary(r,"/");}
      else if op ==  "%="{l.push_binary(r,"%");}
      else if op == "<<="{l.push_binary(r,"<<");}
      else if op == ">>="{l.push_binary(r,">>");}
      else if op ==  "&="{l.push_binary(r,"&");}
      else if op ==  "|="{l.push_binary(r,"|");}
      else if op ==  "^="{l.push_binary(r,"^");}
      else{panic!()}
    }


  let  op = match k
    {
  AsmEvalKind::DerefI8 =>{Opcode::St_i8 }
  AsmEvalKind::DerefI16=>{Opcode::St_i16}
  AsmEvalKind::DerefI32=>{Opcode::St_i32}
  AsmEvalKind::DerefI64=>{Opcode::St_i64}
  AsmEvalKind::DerefU8 =>{Opcode::St_i8 }
  AsmEvalKind::DerefU16=>{Opcode::St_i16}
  AsmEvalKind::DerefU32=>{Opcode::St_i32}
  _=>
    {
      return Err(srcinf.to_error(format!("push_assign error: assign to non deref")));
    }
    };


  self.push_eval_text(l);

  self.push_opcode(op);

  Ok(())
}


pub fn
to_bytes(&self)-> Vec<u8>
{
  let  mut bytes = Vec::<u8>::new();

    for (ln,_) in &self.lines
    {
      ln.write_to(&mut bytes);
    }


  bytes
}


fn
get_label_index(&self, s: &str)-> usize
{
    for i in 0..self.lines.len()
    {
        if let AsmLine::Label(ln_s) = &self.lines[i].0
        {
            if ln_s == s
            {
              return i;
            }
        }
    }


  panic!("{} not found",s);
}


fn
prepare_for_finalize(&mut self)-> Vec<usize>
{
  let  mut ls = Vec::<usize>::new();

  let  mut off = 0usize;

    for i in 0..self.lines.len()
    {
      let  mut dst_i = 0usize;

        match &self.lines[i].0
        {
      AsmLine::Jmp(_,dst) =>{dst_i = self.get_label_index(&dst.label);}
      AsmLine::Brnz(_,dst)=>{dst_i = self.get_label_index(&dst.label);}
      AsmLine::Brz(_,dst) =>{dst_i = self.get_label_index(&dst.label);}
      _=>{}
        }


        match &mut self.lines[i].0
        {
      AsmLine::Jmp(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = dst_i;
          ls.push(i);
        }
      AsmLine::Brnz(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = dst_i;
          ls.push(i);
        }
      AsmLine::Brz(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = dst_i;
          ls.push(i);
        }
      _=>{}
        }


      let  sz = self.lines[i].0.get_size();

      self.lines[i].1.offset = off;
      self.lines[i].1.size   =  sz;

      off += sz;
    }


  ls
}


fn
update_info(&mut self)-> bool
{
  let  mut  off = 0usize;
  let  mut flag = 0usize;

    for (ln,li) in &mut self.lines
    {
      let  sz = ln.get_size();

        if li.offset != off
        {
          li.offset = off;

          flag |= 1;
        }


      li.size  = sz;
          off += sz;
    }


  flag != 0
}


fn
calculate_offset(&self, base_i: usize, dst_i: usize)-> i32
{
  let   dst = &self.lines[ dst_i].1;
  let  base = &self.lines[base_i].1;

  let  dist = (dst.offset as isize)-((base.offset+base.size) as isize);

  dist as i32
}


fn
update_jump_offset(&mut self, ls: &Vec<usize>)
{
    for i in ls
    {
      let  mut off = 0i32;

        match &self.lines[*i].0
        {
      AsmLine::Jmp(_,dst)=> {off = self.calculate_offset(*i,dst.index);}
      AsmLine::Brnz(_,dst)=>{off = self.calculate_offset(*i,dst.index);}
      AsmLine::Brz(_,dst)=> {off = self.calculate_offset(*i,dst.index);}
      _=>{panic!();}
        }


        match &mut self.lines[*i].0
        {
      AsmLine::Jmp(jmp_off,_)=> {*jmp_off = off;}
      AsmLine::Brnz(jmp_off,_)=>{*jmp_off = off;}
      AsmLine::Brz(jmp_off,_)=> {*jmp_off = off;}
      _=>{panic!();}
        }
    }
}


pub fn
finalize(&mut self)
{
  let  ls = self.prepare_for_finalize();

  self.update_jump_offset(&ls);

    while self.update_info()
    {
      self.update_jump_offset(&ls);
    }
}


pub fn
print_to(&self, buf: &mut String, offset: usize)
{
  let  mut n = 0usize;

    for (ln,li) in &self.lines
    {
        if let AsmLine::Label(s) = ln
        {
          buf.push('[');
          buf.push_str(s);
          buf.push(']');
        }

      else
        {
          let  s = format!("  {:05}({:05}) ",li.offset,offset+li.offset);

          buf.push_str(&s);

          ln.print_to(buf);
        }


      buf.push('\n');
    }
}


pub fn
print(&self, offset: usize)
{
  let  mut buf = String::new();

  self.print_to(&mut buf,offset);

  print!("{}",&buf);
}


}




