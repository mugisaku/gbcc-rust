

use std::rc::Rc;
use super::opcode;


pub union
MultiWord
{
   ui8: u8,
  ui16: u16,
  ui32: u32,
  ui64: u64,

   si8: i8,
  si16: i16,
  si32: i32,
  si64: i64,

  fpn32: f32,
  fpn64: f64,

}


impl
MultiWord
{


pub fn  get_u8(&self)->   u8{unsafe{self.ui8}}
pub fn  get_u16(&self)-> u16{unsafe{self.ui16}}
pub fn  get_u32(&self)-> u32{unsafe{self.ui32}}
pub fn  get_u64(&self)-> u64{unsafe{self.ui64}}


}




pub enum
Operand
{
  LabelForAbsolute(String),
  LabelForRelative(String),

  Byte1(MultiWord),
  Byte2(MultiWord),
  Byte4(MultiWord),
  Byte8(MultiWord),

}


pub enum
Line
{
  Instruction(Instruction),
  Label(String),
  Comment(String),

}


impl
Line
{


pub fn
print(&self)
{
    match self
    {
  Line::Instruction(instr)=>{  print!("  ");  instr.print();},
  Line::Label(s)=>{print!(":{}",s);},
  Line::Comment(s)=>{print!("#{}",s);},
    }
}


}




pub struct
Instruction
{
  opcode: u8,

  operand: Option<Operand>,

}


impl
Instruction
{


pub fn
new(op: u8, opr: Option<Operand>)-> Instruction
{
  Instruction{ opcode: op, operand: opr}
}


pub fn
get_size(&self)-> usize
{
    match self.opcode
    {
  opcode::pshs8=> {return 2;}
  opcode::pshs16=>{return 3;}
  opcode::pshs32=>{return 5;}
  opcode::pshu8=> {return 2;}
  opcode::pshu16=>{return 3;}
  opcode::pshu32=>{return 5;}
  opcode::pshf32=>{return 5;}
  opcode::pshb64=>{return 9;}
  _=>{return 1;}
    }
}


pub fn
write16(v: u16, i: usize, buf: &mut Vec<u8>)
{
  buf[i  ] = ( v    &0xFF) as u8;
  buf[i+1] = ((v>>8)&0xFF) as u8;
}


pub fn
write32(v: u32, i: usize, buf: &mut Vec<u8>)
{
  buf[i  ] = ( v     &0xFF) as u8;
  buf[i+1] = ((v>> 8)&0xFF) as u8;
  buf[i+2] = ((v>>16)&0xFF) as u8;
  buf[i+3] = ((v>>24)&0xFF) as u8;
}


pub fn
write64(v: u64, i: usize, buf: &mut Vec<u8>)
{
  buf[i  ] = ( v     &0xFF) as u8;
  buf[i+1] = ((v>> 8)&0xFF) as u8;
  buf[i+2] = ((v>>16)&0xFF) as u8;
  buf[i+3] = ((v>>24)&0xFF) as u8;
  buf[i+4] = ((v>>32)&0xFF) as u8;
  buf[i+5] = ((v>>40)&0xFF) as u8;
  buf[i+6] = ((v>>48)&0xFF) as u8;
  buf[i+7] = ((v>>56)&0xFF) as u8;
}


pub fn
write_pos(&self, v: usize, i: usize, buf: &mut Vec<u8>)-> usize
{
    match self.opcode
    {
  opcode::pshs8=> {},
  opcode::pshs16=>{},
  opcode::pshs32=>{},
  opcode::pshu8=> {  if v <=  u8::MAX as usize{buf[i] = v as u8;}               return 2;},
  opcode::pshu16=>{  if v <= u16::MAX as usize{Self::write16(v as u16,i,buf);}  return 3;},
  opcode::pshu32=>{  if v <= u32::MAX as usize{Self::write32(v as u32,i,buf);}  return 5;},
  opcode::pshf32=>{},
  opcode::pshb64=>{  Self::write64(v as u64,i,buf);  return 9;},
  _=>{},
    }


  0
}


pub fn
assemble(&self, addr_ls: &Vec<Address>, index_base: usize, buf: &mut Vec<u8>)-> usize
{
  buf[index_base] = self.opcode;

    if let Some(opr) = &self.operand
    {
        match opr
        {
      Operand::LabelForAbsolute(s)=>
            {
                if let Some(offset) = find(addr_ls,s)
                {
                  return self.write_pos(offset,index_base+1,buf);
                }

              else
                {
                  print!("abs label \"{}\" is not found",s);

                  return 0;
                }
            },
      Operand::LabelForRelative(s)=>
            {
                if let Some(offset) = find(addr_ls,s)
                {
                  return self.write_pos(offset,index_base+1,buf);
                }

              else
                {
                  print!("rel label \"{}\" is not found",s);

                  return 0;
                }
            },
      Operand::Byte1(mw)=>
            {
              buf[index_base+1] = mw.get_u8();

              return 2;
            }
      Operand::Byte2(mw)=>
            {
              Self::write16(mw.get_u16(),index_base+1,buf);

              return 3;
            },
      Operand::Byte4(mw)=>
            {
              Self::write32(mw.get_u32(),index_base+1,buf);

              return 5;
            },
      Operand::Byte8(mw)=>
            {
              Self::write64(mw.get_u64(),index_base+1,buf);

              return 9;
            }
        }
    }


  1
}


pub fn
print(&self)
{
  print!("{}  ",opcode::get_name(self.opcode));

    if let Some(opr) = &self.operand
    {
        unsafe
        {
            match opr
            {
          Operand::LabelForAbsolute(s)=>{print!("$A: {}",s);}
          Operand::LabelForRelative(s)=>{print!("$R: {}",s);}
          Operand::Byte1(mw)=>{print!("{}",mw.ui8);}
          Operand::Byte2(mw)=>{print!("{}",mw.ui16);}
          Operand::Byte4(mw)=>{print!("{}",mw.ui32);}
          Operand::Byte8(mw)=>{print!("{}",mw.ui64);}
            }
        }
    }
}


}




pub struct
Address
{
   name: String,
  index: usize,

}


pub fn
find(ls: &Vec<Address>, name: &str)-> Option<usize>
{
    for addr in ls
    {
        if addr.name == name
        {
          return Some(addr.index);
        }
    }


  None
}


pub struct
Note
{
  lines: Vec<Line>,

}


impl
Note
{


pub fn
new()-> Note
{
  let mut  nt = Note{ lines: Vec::new()};

  nt
}


pub fn
put_label(&mut self, s: &str)
{
  self.lines.push(Line::Label(String::from(s)));
}


pub fn
put_comment(&mut self, s: &str)
{
  self.lines.push(Line::Comment(String::from(s)));
}


pub fn
put_instruction(&mut self, instr: Instruction)
{
  self.lines.push(Line::Instruction(instr));
}


pub fn   put_nop(&mut self){self.put_instruction(Instruction::new(opcode::nop,None));}
pub fn  put_ldu8(&mut self){self.put_instruction(Instruction::new(opcode::ldu8,None));}
pub fn put_ldu16(&mut self){self.put_instruction(Instruction::new(opcode::ldu16,None));}
pub fn put_ldu32(&mut self){self.put_instruction(Instruction::new(opcode::ldu32,None));}
pub fn  put_lds8(&mut self){self.put_instruction(Instruction::new(opcode::lds8,None));}
pub fn put_lds16(&mut self){self.put_instruction(Instruction::new(opcode::lds16,None));}
pub fn put_lds32(&mut self){self.put_instruction(Instruction::new(opcode::lds32,None));}
pub fn put_ldf32(&mut self){self.put_instruction(Instruction::new(opcode::ldf32,None));}
pub fn  put_ld64(&mut self){self.put_instruction(Instruction::new(opcode::ld64,None));}
pub fn  put_stu8(&mut self){self.put_instruction(Instruction::new(opcode::stu8,None));}
pub fn put_stu16(&mut self){self.put_instruction(Instruction::new(opcode::stu16,None));}
pub fn put_stu32(&mut self){self.put_instruction(Instruction::new(opcode::stu32,None));}
pub fn  put_sts8(&mut self){self.put_instruction(Instruction::new(opcode::sts8,None));}
pub fn put_sts16(&mut self){self.put_instruction(Instruction::new(opcode::sts16,None));}
pub fn put_sts32(&mut self){self.put_instruction(Instruction::new(opcode::sts32,None));}
pub fn put_stf32(&mut self){self.put_instruction(Instruction::new(opcode::stf32,None));}
pub fn  put_st64(&mut self){self.put_instruction(Instruction::new(opcode::st64,None));}

pub fn put_adds(&mut self){self.put_instruction(Instruction::new(opcode::adds,None));}
pub fn put_subs(&mut self){self.put_instruction(Instruction::new(opcode::subs,None));}
pub fn put_muls(&mut self){self.put_instruction(Instruction::new(opcode::muls,None));}
pub fn put_divs(&mut self){self.put_instruction(Instruction::new(opcode::divs,None));}
pub fn put_rems(&mut self){self.put_instruction(Instruction::new(opcode::rems,None));}
pub fn put_addu(&mut self){self.put_instruction(Instruction::new(opcode::addu,None));}
pub fn put_subu(&mut self){self.put_instruction(Instruction::new(opcode::subu,None));}
pub fn put_mulu(&mut self){self.put_instruction(Instruction::new(opcode::mulu,None));}
pub fn put_divu(&mut self){self.put_instruction(Instruction::new(opcode::divu,None));}
pub fn put_remu(&mut self){self.put_instruction(Instruction::new(opcode::remu,None));}
pub fn put_addf(&mut self){self.put_instruction(Instruction::new(opcode::addf,None));}
pub fn put_subf(&mut self){self.put_instruction(Instruction::new(opcode::subf,None));}
pub fn put_mulf(&mut self){self.put_instruction(Instruction::new(opcode::mulf,None));}
pub fn put_divf(&mut self){self.put_instruction(Instruction::new(opcode::divf,None));}
pub fn put_remf(&mut self){self.put_instruction(Instruction::new(opcode::remf,None));}

pub fn put_land(&mut self){self.put_instruction(Instruction::new(opcode::land,None));}
pub fn  put_lor(&mut self){self.put_instruction(Instruction::new(opcode::lor,None));}
pub fn put_lnot(&mut self){self.put_instruction(Instruction::new(opcode::lnot,None));}
pub fn  put_neg(&mut self){self.put_instruction(Instruction::new(opcode::neg,None));}
pub fn put_negf(&mut self){self.put_instruction(Instruction::new(opcode::negf,None));}

pub fn put_shl(&mut self){self.put_instruction(Instruction::new(opcode::shl,None));}
pub fn put_shr(&mut self){self.put_instruction(Instruction::new(opcode::shr,None));}
pub fn put_and(&mut self){self.put_instruction(Instruction::new(opcode::and,None));}
pub fn  put_or(&mut self){self.put_instruction(Instruction::new(opcode::or,None));}
pub fn put_xor(&mut self){self.put_instruction(Instruction::new(opcode::xor,None));}
pub fn  put_eq(&mut self){self.put_instruction(Instruction::new(opcode::eq,None));}
pub fn put_neq(&mut self){self.put_instruction(Instruction::new(opcode::neq,None));}
pub fn put_not(&mut self){self.put_instruction(Instruction::new(opcode::not,None));}

pub fn   put_lts(&mut self){self.put_instruction(Instruction::new(opcode::lts,None));}
pub fn put_lteqs(&mut self){self.put_instruction(Instruction::new(opcode::lteqs,None));}
pub fn   put_gts(&mut self){self.put_instruction(Instruction::new(opcode::gts,None));}
pub fn put_gteqs(&mut self){self.put_instruction(Instruction::new(opcode::gteqs,None));}
pub fn   put_ltu(&mut self){self.put_instruction(Instruction::new(opcode::ltu,None));}
pub fn put_ltequ(&mut self){self.put_instruction(Instruction::new(opcode::ltequ,None));}
pub fn   put_gtu(&mut self){self.put_instruction(Instruction::new(opcode::gtu,None));}
pub fn put_gtequ(&mut self){self.put_instruction(Instruction::new(opcode::gtequ,None));}
pub fn   put_ltf(&mut self){self.put_instruction(Instruction::new(opcode::ltf,None));}
pub fn put_lteqf(&mut self){self.put_instruction(Instruction::new(opcode::lteqf,None));}
pub fn   put_gtf(&mut self){self.put_instruction(Instruction::new(opcode::gtf,None));}
pub fn put_gteqf(&mut self){self.put_instruction(Instruction::new(opcode::gteqf,None));}

pub fn  put_psh0(&mut self){self.put_instruction(Instruction::new(opcode::psh0,None));}
pub fn  put_psh1(&mut self){self.put_instruction(Instruction::new(opcode::psh1,None));}
pub fn  put_psh2(&mut self){self.put_instruction(Instruction::new(opcode::psh2,None));}
pub fn  put_psh4(&mut self){self.put_instruction(Instruction::new(opcode::psh4,None));}
pub fn  put_psh8(&mut self){self.put_instruction(Instruction::new(opcode::psh8,None));}
pub fn  put_pshfw(&mut self){self.put_instruction(Instruction::new(opcode::pshfw,None));}

pub fn  put_pshu8(&mut self, v:  u8){self.put_instruction(Instruction::new(opcode::pshu8, Some(Operand::Byte1(MultiWord{ ui8: v}))));}
pub fn put_pshu16(&mut self, v: u16){self.put_instruction(Instruction::new(opcode::pshu16,Some(Operand::Byte2(MultiWord{ ui16: v}))));}
pub fn put_pshu32(&mut self, v: u32){self.put_instruction(Instruction::new(opcode::pshu32,Some(Operand::Byte4(MultiWord{ ui32: v}))));}
pub fn put_pshu64(&mut self, v: u64){self.put_instruction(Instruction::new(opcode::pshb64,Some(Operand::Byte8(MultiWord{ ui64: v}))));}
pub fn  put_pshs8(&mut self, v:  i8){self.put_instruction(Instruction::new(opcode::pshs8, Some(Operand::Byte1(MultiWord{ si8: v}))));}
pub fn put_pshs16(&mut self, v: i16){self.put_instruction(Instruction::new(opcode::pshs16,Some(Operand::Byte2(MultiWord{ si16: v}))));}
pub fn put_pshs32(&mut self, v: i32){self.put_instruction(Instruction::new(opcode::pshs32,Some(Operand::Byte4(MultiWord{ si32: v}))));}
pub fn put_pshs64(&mut self, v: i64){self.put_instruction(Instruction::new(opcode::pshb64,Some(Operand::Byte8(MultiWord{ si64: v}))));}
pub fn put_pshf32(&mut self, v: f32){self.put_instruction(Instruction::new(opcode::pshf32,Some(Operand::Byte4(MultiWord{ fpn32: v}))));}
pub fn put_pshf64(&mut self, v: f64){self.put_instruction(Instruction::new(opcode::pshb64,Some(Operand::Byte8(MultiWord{ fpn64: v}))));}


pub fn
put_pshu(&mut self, v: u64)-> usize
{
       if v == 0{  self.put_psh0();  return 1;}
  else if v == 1{  self.put_psh1();  return 1;}
  else if v == 2{  self.put_psh2();  return 1;}
  else if v == 4{  self.put_psh4();  return 1;}
  else if v == 8{  self.put_psh8();  return 1;}
  else if v <=  u8::MAX as u64{  self.put_pshu8( v as  u8);  return 1;}
  else if v <= u16::MAX as u64{  self.put_pshu16(v as u16);  return 2;}
  else if v <= u32::MAX as u64{  self.put_pshu32(v as u32);  return 4;}
  else                        {  self.put_pshu64(v       );  return 8;}
}


pub fn
put_pshf(&mut self, v: f64)-> usize
{
    if (v <=  f32::MAX as f64)
    && (v >=  f32::MIN as f64)
    {
      self.put_pshf32(v as f32);

      return 4;
    }

  else
    {
      self.put_pshf64(v);

      return 8;
    }
}


pub fn   put_relpos8(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu8,Some(Operand::LabelForRelative(String::from(s)))));}
pub fn  put_relpos16(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu16,Some(Operand::LabelForRelative(String::from(s)))));}
pub fn  put_relpos32(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu32,Some(Operand::LabelForRelative(String::from(s)))));}
pub fn  put_relpos64(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshb64,Some(Operand::LabelForRelative(String::from(s)))));}

pub fn   put_abspos8(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu8,Some(Operand::LabelForAbsolute(String::from(s)))));}
pub fn  put_abspos16(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu16,Some(Operand::LabelForAbsolute(String::from(s)))));}
pub fn  put_abspos32(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshu32,Some(Operand::LabelForAbsolute(String::from(s)))));}
pub fn  put_abspos64(&mut self, s: &str){self.put_instruction(Instruction::new(opcode::pshb64,Some(Operand::LabelForAbsolute(String::from(s)))));}

pub fn put_xsp(&mut self){self.put_instruction(Instruction::new(opcode::xsp,None));}
pub fn put_ssp(&mut self){self.put_instruction(Instruction::new(opcode::ssp,None));}
pub fn put_maa(&mut self){self.put_instruction(Instruction::new(opcode::maa,None));}

pub fn put_jmp(&mut self){self.put_instruction(Instruction::new(opcode::jmp,None));}
pub fn  put_br(&mut self){self.put_instruction(Instruction::new(opcode::br,None));}
pub fn put_cal(&mut self){self.put_instruction(Instruction::new(opcode::cal,None));}
pub fn put_ret(&mut self){self.put_instruction(Instruction::new(opcode::ret,None));}
pub fn put_retd(&mut self){self.put_instruction(Instruction::new(opcode::retd,None));}
pub fn put_reti(&mut self){self.put_instruction(Instruction::new(opcode::reti,None));}
pub fn put_hlt(&mut self){self.put_instruction(Instruction::new(opcode::hlt,None));}
pub fn put_prnu64(&mut self){self.put_instruction(Instruction::new(opcode::prnu64,None));}
pub fn put_putlog0(&mut self){self.put_instruction(Instruction::new(opcode::putlog0,None));}
pub fn put_putlog1(&mut self){self.put_instruction(Instruction::new(opcode::putlog1,None));}


pub fn
assemble(&self)-> Result<Vec<u8>,()>
{
  let mut  buf: Vec<u8> = Vec::new();
  let mut  addr_ls: Vec<Address> = Vec::new();

  let mut  line_index: usize = 0;

    for ln in &self.lines
    {
        match ln
        {
      Line::Label(s)=>
            {
              addr_ls.push(Address{ name: String::from(s), index: line_index});
            },
      Line::Instruction(instr)=>
            {
              line_index += instr.get_size();
            },
      Line::Comment(_)=>
            {
            },
        }
    }


  buf.resize(line_index,opcode::nop);

  line_index = 0;

    for ln in &self.lines
    {
        if let Line::Instruction(instr) = ln
        {
          let  sz = instr.assemble(&addr_ls,line_index,&mut buf);

            if sz == 0
            {
              return Err(());
            }


          line_index += sz;
        }
    }


  Ok(buf)
}


pub fn
print(&self)
{
    for ln in &self.lines
    {
      ln.print();

      print!("\n");
    }
}


}




pub fn
read16(buf: &Vec<u8>, i: usize)-> u16
{
  let  v: u16 = ((buf[i  ] as u16)   )
              | ((buf[i+1] as u16)<<8)
              ;

  v
}


pub fn
read32(buf: &Vec<u8>, i: usize)-> u32
{
  let  v: u32 = ((buf[i  ] as u32)    )
              | ((buf[i+1] as u32)<< 8)
              | ((buf[i+2] as u32)<<16)
              | ((buf[i+3] as u32)<<24)
              ;

  v
}


pub fn
read64(buf: &Vec<u8>, i: usize)-> u64
{
  let  v: u64 = ((buf[i  ] as u64)    )
              | ((buf[i+1] as u64)<< 8)
              | ((buf[i+2] as u64)<<16)
              | ((buf[i+3] as u64)<<24)
              | ((buf[i+4] as u64)<<32)
              | ((buf[i+5] as u64)<<40)
              | ((buf[i+6] as u64)<<48)
              | ((buf[i+7] as u64)<<56)
              ;

  v
}


pub fn
print_as_machine_code(img: &Vec<u8>)
{
  let mut  i: usize = 0;

    while i < img.len()
    {
      print!("[{:4}] ",i);

      let  opcode = img[i];

      i += 1;

      let  ops = super::opcode::get_name(opcode);

        if ops == "NOP"
        {
          print!("unknown opcode");

          break;
        }


      print!("{}  ",ops);

        match opcode
        {
      opcode::pshs8=> {  print!("(i8){}",img[i] as i8);  i += 1;},
      opcode::pshs16=>{  print!("(i16){}",read16(img,i) as i16);  i += 2;},
      opcode::pshs32=>{  print!("(i32){}",read32(img,i) as i32);  i += 4;},
      opcode::pshu8=> {  print!("(u8){}",img[i]);  i += 1;},
      opcode::pshu16=>{  print!("(u16){}",read16(img,i));  i += 2;},
      opcode::pshu32=>{  print!("(u32){}",read32(img,i));  i += 4;},
      opcode::pshf32=>{unsafe{print!("(f32){}",MultiWord{ ui32: read32(img,i)}.fpn32);}  i += 4;},
      opcode::pshb64=>{  print!("(u64){}",read64(img,i));  i += 8;},
      _=>{},
        }


      print!("\n");
    }
}




