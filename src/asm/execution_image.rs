

use super::instruction::*;
use crate::token::ParsedNumber;




pub struct
CodeElement
{
  pub(crate) main_instruction: Instruction,
  pub(crate) sub1_instruction_opt: Option<Instruction>,
  pub(crate) sub2_instruction_opt: Option<Instruction>,

  pub(crate) position: usize,
  pub(crate)     size: usize,

}


impl
CodeElement
{


pub fn
print(&self)
{
  print!("{:05} ",self.position);

  self.main_instruction.print();

    if let Some(sub_instr) = &self.sub1_instruction_opt
    {
      print!("\n  ");

      sub_instr.print();
    }


    if let Some(sub_instr) = &self.sub2_instruction_opt
    {
      print!("\n  ");

      sub_instr.print();
    }
}


}




pub struct
ConstContext<'a>
{
   loc_sz: usize,
   glo_sz: usize,
  para_sz: usize,
  ctrl_sz: usize,

  name_list: &'a [String],

}


pub struct
VolatileContext
{
  position: usize,

  label_position_list: Vec<(String,usize)>,

}


pub struct
Routine
{
  label: String,

  parameter_list: Vec<String>,
   variable_list: Vec<String>,

  position: usize,

  base_instruction_list: Vec<Instruction>,

  element_list: Vec<CodeElement>,

}


impl
Routine
{


pub fn
new()-> Self
{
  Self{
    label: String::new(),

    position: 0,

    parameter_list: Vec::new(),
     variable_list: Vec::new(),

    base_instruction_list: Vec::new(),

    element_list: Vec::new(),
  }
}


pub fn
make_bare_code_element_list(&self)-> Vec<CodeElement>
{
  let  mut ls = Vec::<CodeElement>::new();

    for _ in 0..self.variable_list.len()
    {
      let  e = CodeElement{main_instruction: Instruction::push0(), sub1_instruction_opt: None, sub2_instruction_opt: None, position: 0, size: 1};

      ls.push(e);
    }


    for inst in &self.base_instruction_list
    {
      let  mut instruction = inst.clone();

      let  sz = instruction.get_size();

      let  e = CodeElement{main_instruction: instruction, sub1_instruction_opt: None, sub2_instruction_opt: None, position: 0, size: sz};

      ls.push(e);
    }


  ls
}


pub fn
get_base_size(&self)-> usize
{
  let  mut sz = 0usize;

    for instr in &self.base_instruction_list
    {
      sz += instr.get_size();
    }


  sz
}


pub fn
get_size(&self)-> usize
{
    if let Some(last) = &self.element_list.last()
    {
      return last.position+last.size;
    }


  0
}


fn
find_pos(ls: &[(String,usize)], s: &str)-> Option<usize>
{
    for e in ls
    {
        if e.0 == s
        {
          return Some(e.1);
        }
    }


  None
}


fn
find(ls: &[String], s: &str)-> Option<usize>
{
    for i in 0..ls.len()
    {
        if &ls[i] == s
        {
          return Some(i);
        }
    }


  None
}


pub fn
get_required_size(v: usize)-> usize
{
       if v <=  i8::MAX as usize{1}
  else if v <= i16::MAX as usize{2}
  else if v <= i32::MAX as usize{4}
  else                          {8}
}


fn
to_push_i_instruction(v: isize, sz: usize)-> (Instruction,Instruction)
{
    match sz
    {
  1=>{(Instruction::Opcode(PUSHI8) ,Instruction::ImmI8( v as i8 ))}
  2=>{(Instruction::Opcode(PUSHI16),Instruction::ImmI16(v as i16))}
  4=>{(Instruction::Opcode(PUSHI32),Instruction::ImmI32(v as i32))}
  8=>{(Instruction::Opcode(PUSH64) ,Instruction::ImmI64(v as i64))}
  _=>{panic!();}
    }
}


fn
to_push_u_instruction(v: usize, sz: usize)-> (Instruction,Instruction)
{
    match sz
    {
  1=>{(Instruction::Opcode(PUSHU8) ,Instruction::ImmU8( v as u8 ))}
  2=>{(Instruction::Opcode(PUSHU16),Instruction::ImmU16(v as u16))}
  4=>{(Instruction::Opcode(PUSHU32),Instruction::ImmU32(v as u32))}
  8=>{(Instruction::Opcode(PUSH64) ,Instruction::ImmU64(v as u64))}
  _=>{panic!();}
    }
}


pub fn
get_i_size(i: i64)-> usize
{
       if (i <=  i8::MAX as i64) && (i >=  i8::MIN as i64){1}
  else if (i <= i16::MAX as i64) && (i >= i16::MIN as i64){2}
  else if (i <= i32::MAX as i64) && (i >= i32::MIN as i64){4}
  else{8}
}


pub fn
get_u_size(u: u64)-> usize
{
       if u <=  u8::MAX as u64{1}
  else if u <= u16::MAX as u64{2}
  else if u <= u32::MAX as u64{4}
  else{8}
}


pub fn
get_f_size(f: f64)-> usize
{
       if (f <= f32::MAX as f64) && (f >= f32::MIN as f64){4}
  else{8}
}


pub fn
tuneup_element(&self, cctx: &ConstContext, vctx: &mut VolatileContext, e: &mut CodeElement)
{
  e.position = vctx.position;

  e.sub1_instruction_opt = None;
  e.sub2_instruction_opt = None;

    match e.main_instruction.clone()
    {
  Instruction::PushI(i)=>
    {
      let  sz = Self::get_i_size(i);

        match sz
        {
      1=>{  e.main_instruction = Instruction::Opcode(PUSHI8);   e.sub1_instruction_opt = Some(Instruction::ImmI8( i as  i8));}
      2=>{  e.main_instruction = Instruction::Opcode(PUSHI16);  e.sub1_instruction_opt = Some(Instruction::ImmI16(i as i16));}
      4=>{  e.main_instruction = Instruction::Opcode(PUSHI32);  e.sub1_instruction_opt = Some(Instruction::ImmI32(i as i32));}
      8=>{  e.main_instruction = Instruction::Opcode(PUSH64);   e.sub1_instruction_opt = Some(Instruction::ImmI64(i as i64));}
      _=>{panic!();}
        }


      e.size = 1+sz;
    }
  Instruction::PushU(u)=>
    {
      let  sz = Self::get_u_size(u);

        match sz
        {
      1=>{  e.main_instruction = Instruction::Opcode(PUSHU8);   e.sub1_instruction_opt = Some(Instruction::ImmU8( u as  u8));}
      2=>{  e.main_instruction = Instruction::Opcode(PUSHU16);  e.sub1_instruction_opt = Some(Instruction::ImmU16(u as u16));}
      4=>{  e.main_instruction = Instruction::Opcode(PUSHU32);  e.sub1_instruction_opt = Some(Instruction::ImmU32(u as u32));}
      8=>{  e.main_instruction = Instruction::Opcode(PUSH64);   e.sub1_instruction_opt = Some(Instruction::ImmU64(u as u64));}
      _=>{panic!();}
        }


      e.size = 1+sz;
    }
  Instruction::PushF(f)=>
    {
      let  sz = Self::get_f_size(f);

        match sz
        {
      4=>{  e.main_instruction = Instruction::Opcode(PUSHF32);  e.sub1_instruction_opt = Some(Instruction::ImmF32(f as f32));}
      8=>{  e.main_instruction = Instruction::Opcode(PUSH64);   e.sub1_instruction_opt = Some(Instruction::ImmF64(f as f64));}
      _=>{panic!();}
        }


      e.size = 1+sz;
    }
  Instruction::PushDst(s)=>
    {
      e.size = 1+cctx.ctrl_sz;
    }
  Instruction::PushGlo(s)=>
    {
      e.size = 1+cctx.glo_sz+1;

      let  i = Self::find(cctx.name_list,&s).unwrap();
println!("gloooo {} {}",i,cctx.glo_sz);
      let  (i1,i2) = Self::to_push_u_instruction(i,cctx.glo_sz);

      e.main_instruction = i1;
      e.sub1_instruction_opt = Some(i2);
      e.sub2_instruction_opt = Some(Instruction::glo());
    }
  Instruction::PushArg(s)=>
    {
      e.size = 1+cctx.para_sz+1;

      let  i = Self::find(&self.parameter_list,&s).unwrap();

      let  (i1,i2) = Self::to_push_u_instruction(i,cctx.para_sz);

      e.main_instruction = i1;
      e.sub1_instruction_opt = Some(i2);
      e.sub2_instruction_opt = Some(Instruction::arg());
    }
  Instruction::PushLoc(s)=>
    {
      e.size = 1+cctx.loc_sz+1;

      let  i = Self::find(&self.variable_list,&s).unwrap();

      let  (i1,i2) = Self::to_push_u_instruction(i,cctx.loc_sz);

      e.main_instruction = i1;
      e.sub1_instruction_opt = Some(i2);
      e.sub2_instruction_opt = Some(Instruction::loc());
    }
  Instruction::Label(s)=>
    {
      vctx.label_position_list.push((s.clone(),vctx.position));
    }
  _=>{}
    }


  vctx.position += e.size;
}


pub fn
tuneup(&mut self, name_list: &[String], glo_sz: usize)-> Result<(),()>
{
  let  cctx = ConstContext{glo_sz: glo_sz,
                          ctrl_sz: Self::get_required_size(self.get_base_size()),
                          para_sz: Self::get_required_size(self.parameter_list.len()),
                           loc_sz: Self::get_required_size(self.variable_list.len()),
                          name_list,
  };


  let  mut vctx = VolatileContext{position: 0usize,
                                 label_position_list: Vec::new(),
  };


  let  mut tmp_element_list = self.make_bare_code_element_list();

    for e in &mut tmp_element_list
    {
      self.tuneup_element(&cctx,&mut vctx,e);
    }


    for e in &mut tmp_element_list
    {
        if let Instruction::PushDst(s) = &e.main_instruction
        {
            if let Some(pos) = Self::find_pos(&vctx.label_position_list,s)
            {
              let  v = (pos as isize)-((e.position+e.size) as isize);

              let  (i1,i2) = Self::to_push_i_instruction(v,cctx.ctrl_sz);

              e.main_instruction     = i1;
              e.sub1_instruction_opt = Some(i2);
            }

          else
            {
              return Err(());
            }
        }
    }


  self.element_list = tmp_element_list;

  Ok(())
}


pub fn
write_to(&self, buf: &mut Vec<u8>)
{
    for e in &self.element_list
    {
      e.main_instruction.assemble_to(buf);

        if let Some(sub_instr) = &e.sub1_instruction_opt
        {
          sub_instr.assemble_to(buf);
        }


        if let Some(sub_instr) = &e.sub2_instruction_opt
        {
          sub_instr.assemble_to(buf);
        }
    }
}


pub fn
print(&self)
{
  print!("{} ",&self.label);

    for para in &self.parameter_list
    {
      print!("{} ",para);
    }


  print!("| ");

    for var in &self.variable_list
    {
      print!("{} ",var);
    }


  print!("\npos: {}, sz: {}, \n{{\n",self.position,self.get_size());

    for instr in &self.base_instruction_list
    {
      instr.print();

      print!("\n");
    }


  print!("\n}} -> {{\n");

    for e in &self.element_list
    {
      e.print();

      print!("\n");
    }


  print!("\n}}\n");
}


}




pub struct
UndefinedSpace
{
  label: String,

  position: usize,
      size: usize,

}


pub struct
GlobalData
{
  label: String,

  position: usize,

  content: Vec<u8>,

}


impl
GlobalData
{


pub fn
new()-> Self
{
  Self{
    label: String::new(),
    position: 0,
    content: Vec::new(),
  }
}


pub fn
set_label(&mut self, s: String)-> &mut Self
{
  self.label = s;

  self
}


pub fn
read<'a>(&mut self, mut iter: crate::token::Iterator<'a>)-> Result<crate::token::Iterator<'a>,()>
{
  Ok(iter)
}


}




pub enum
Rest
{
  None,

  Colon,

  Identifier(String),
   Int(i64),
   Uint(u64),
  Float(f64),

}


pub struct
Source
{
      global_data_list: Vec<GlobalData>,
  undefined_space_list: Vec<UndefinedSpace>,
          routine_list: Vec<Routine>,

}


impl
Source
{


pub fn
new()-> Self
{
  Self{
        global_data_list: Vec::new(),
    undefined_space_list: Vec::new(),
            routine_list: Vec::new(),
  }
}


pub fn
find_undefined_space(&self, name: &str)-> Option<&UndefinedSpace>
{
    for usp in &self.undefined_space_list
    {
        if &usp.label == name
        {
          return Some(usp);
        }
    }


  None
}


pub fn
find_global_data(&self, name: &str)-> Option<&GlobalData>
{
    for gd in &self.global_data_list
    {
        if &gd.label == name
        {
          return Some(gd);
        }
    }


  None
}


pub fn
find_routine(&self, name: &str)-> Option<&Routine>
{
    for rou in &self.routine_list
    {
        if &rou.label == name
        {
          return Some(rou);
        }
    }


  None
}


pub fn
read_global_data(&mut self, name: &str, iter: &mut crate::token::Iterator)-> Result<(),()>
{
  let  label = name.to_string();

    if let Some(first_pn) = iter.get_number()
    {
      let  mut content = Vec::<u8>::new();

      content.push(first_pn.i_part as u8);

      iter.advance();

        while let Some(pn) = iter.get_number()
        {
          content.push(pn.i_part as u8);

          iter.advance();
        }


        if iter.is_others(';')
        {
          iter.advance();

          self.global_data_list.push(GlobalData{label, position: 0, content});

          return Ok(());
        }
    }

  else
    if let Some(s) = iter.get_string()
    {
      let  content = s.as_bytes().to_vec();

      iter.advance();

        if iter.is_others(';')
        {
          iter.advance();

          self.global_data_list.push(GlobalData{label, position: 0, content});

          return Ok(());
        }
    }


  Err(())
}


pub fn
read_undefined_space(&mut self, name: &str, iter: &mut crate::token::Iterator)-> Result<(),()>
{
    if let Some(pn) = iter.get_number()
    {
      let  size = pn.i_part as usize;

      iter.advance();

        if iter.is_others(';')
        {
          iter.advance();

          self.undefined_space_list.push(UndefinedSpace{label: name.to_string(), position: 0, size});

          return Ok(());
        }
    }


  Err(())
}


pub fn
to_rest_from_number(pn: &ParsedNumber)-> Rest
{
    if let Some(f) = pn.get_float(){Rest::Float(f)}
  else                             {Rest::Uint(pn.i_part)}
}


pub fn
to_negative(res: Rest)-> Rest
{
    match res
    {
  Rest::Int(i)=>  {Rest::Int(-i)}
  Rest::Uint(u)=> {Rest::Int(-(u as i64))}
  Rest::Float(f)=>{Rest::Float(-f)}
  _=>{res}
    }
}


pub fn
read_until_terminator(iter: &mut crate::token::Iterator)-> Result<(String,Rest),()>
{
    if let Some(first) = iter.get_identifier()
    {
      let  first_s = first.clone();

      iter.advance();

      let  mut neg = false;

        if let Some(a) = iter.get_others()
        {
            match a
            {
          '}'=>{return Ok((first_s,Rest::None));}
          ':'=>{  iter.advance();  return Ok((first_s,Rest::Colon));}
          ';'=>{  iter.advance();  return Ok((first_s,Rest::None ));}
          '-'=>{  iter.advance();  neg = true;}
          _=>{}
            }
        }


      let  mut res = Rest::None;

        if let Some(second) = iter.get_identifier()
        {
          res = Rest::Identifier(second.clone());

          iter.advance();
        }

      else
        if let Some(pn) = iter.get_number()
        {
          res = Self::to_rest_from_number(pn);

            if neg
            {
              res = Self::to_negative(res);
            }


          iter.advance();
        }

      else
        {
          println!("read_until_terminator failed. unknown token appeared");

          return Err(());
        }


        if iter.is_others(';')
        {
          iter.advance();
        }


      return Ok((first_s,res));
    }


  println!("read_until_terminator failed.");

  iter.current().unwrap().print();

  Err(())
}


fn
make_instruction(first: &str, res: &Rest)-> Result<Instruction,()>
{
    if let Rest::Colon = res
    {
      return Ok(Instruction::Label(first.to_string()));
    }


       if first == "nop"{Ok(Instruction::nop())}
  else if first =="addi"{Ok(Instruction::addi())}
  else if first =="subi"{Ok(Instruction::subi())}
  else if first =="muli"{Ok(Instruction::muli())}
  else if first =="divi"{Ok(Instruction::divi())}
  else if first =="remi"{Ok(Instruction::remi())}
  else if first =="addu"{Ok(Instruction::addu())}
  else if first =="subu"{Ok(Instruction::subu())}
  else if first =="mulu"{Ok(Instruction::mulu())}
  else if first =="divu"{Ok(Instruction::divu())}
  else if first =="remu"{Ok(Instruction::remu())}
  else if first =="addf"{Ok(Instruction::addf())}
  else if first =="subf"{Ok(Instruction::subf())}
  else if first =="mulf"{Ok(Instruction::mulf())}
  else if first =="divf"{Ok(Instruction::divf())}
  else if first =="remf"{Ok(Instruction::remf())}
  else if first =="shl"{Ok(Instruction::shl())}
  else if first =="shr"{Ok(Instruction::shr())}
  else if first =="and"{Ok(Instruction::and())}
  else if first =="or"{Ok(Instruction::eq())}
  else if first =="xor"{Ok(Instruction::xor())}
  else if first =="eq"{Ok(Instruction::eq())}
  else if first =="neq"{Ok(Instruction::neq())}
  else if first =="eqf"{Ok(Instruction::eqf())}
  else if first =="neqf"{Ok(Instruction::neqf())}
  else if first =="lti"{Ok(Instruction::lti())}
  else if first =="lteqi"{Ok(Instruction::lteqi())}
  else if first =="gti"{Ok(Instruction::gti())}
  else if first =="gteqi"{Ok(Instruction::gteqi())}
  else if first =="ltu"{Ok(Instruction::ltu())}
  else if first =="ltequ"{Ok(Instruction::ltequ())}
  else if first =="gtu"{Ok(Instruction::gtu())}
  else if first =="gtequ"{Ok(Instruction::gtequ())}
  else if first =="ltf"{Ok(Instruction::ltf())}
  else if first =="lteqf"{Ok(Instruction::lteqf())}
  else if first =="gtf"{Ok(Instruction::gtf())}
  else if first =="gteqf"{Ok(Instruction::gteqf())}
  else if first =="land"{Ok(Instruction::land())}
  else if first =="lor"{Ok(Instruction::lor())}
  else if first =="neg"{Ok(Instruction::neg())}
  else if first =="negf"{Ok(Instruction::negf())}
  else if first =="not"{Ok(Instruction::not())}
  else if first =="lnot"{Ok(Instruction::lnot())}
  else if first =="itou"{Ok(Instruction::itou())}
  else if first =="utoi"{Ok(Instruction::utoi())}
  else if first =="itof"{Ok(Instruction::itof())}
  else if first =="ftoi"{Ok(Instruction::ftoi())}
  else if first =="push0"{Ok(Instruction::push0())}
  else if first =="push1"{Ok(Instruction::push1())}
  else if first =="push2"{Ok(Instruction::push2())}
  else if first =="push3"{Ok(Instruction::push3())}
  else if first =="push4"{Ok(Instruction::push4())}
  else if first =="push5"{Ok(Instruction::push5())}
  else if first =="push6"{Ok(Instruction::push6())}
  else if first =="push7"{Ok(Instruction::push7())}
  else if first =="push8"{Ok(Instruction::push8())}
  else if first =="pop"{Ok(Instruction::pop())}
  else if first =="dup"{Ok(Instruction::dup())}
  else if first =="ldi8"{Ok(Instruction::ldi8())}
  else if first =="ldi16"{Ok(Instruction::ldi16())}
  else if first =="ldi32"{Ok(Instruction::ldi32())}
  else if first =="ldu8"{Ok(Instruction::ldu8())}
  else if first =="ldu16"{Ok(Instruction::ldu16())}
  else if first =="ldu32"{Ok(Instruction::ldu32())}
  else if first =="ldf32"{Ok(Instruction::ldf32())}
  else if first =="ld64"{Ok(Instruction::ld64())}
  else if first =="sti8"{Ok(Instruction::sti8())}
  else if first =="sti16"{Ok(Instruction::sti16())}
  else if first =="sti32"{Ok(Instruction::sti32())}
  else if first =="stu8"{Ok(Instruction::stu8())}
  else if first =="stu16"{Ok(Instruction::stu16())}
  else if first =="stu32"{Ok(Instruction::stu32())}
  else if first =="stf32"{Ok(Instruction::stf32())}
  else if first =="st64"{Ok(Instruction::st64())}
  else if first =="glo"{Ok(Instruction::glo())}
  else if first =="arg"{Ok(Instruction::arg())}
  else if first =="loc"{Ok(Instruction::loc())}
  else if first =="spx"{Ok(Instruction::spx())}
  else if first =="prcal"{Ok(Instruction::prcal())}
  else if first =="cal"{Ok(Instruction::cal())}
  else if first =="jmp"{Ok(Instruction::jmp())}
  else if first =="brz"{Ok(Instruction::brz())}
  else if first =="brnz"{Ok(Instruction::brnz())}
  else if first =="ret"{Ok(Instruction::ret())}
  else if first =="pri"{Ok(Instruction::pri())}
  else if first =="pru"{Ok(Instruction::pru())}
  else if first =="prf"{Ok(Instruction::prf())}
  else if first =="repo"{Ok(Instruction::repo())}
  else if first =="hlt"{Ok(Instruction::hlt())}
  else if first =="push"
    {
        match res
        {
      Rest::Int(i)  =>{Ok(Instruction::PushI(*i))}
      Rest::Uint(u) =>{Ok(Instruction::PushU(*u))}
      Rest::Float(f)=>{Ok(Instruction::PushF(*f))}
      _=>{Err(())}
        }
    }
  else if first =="pushglo"
    {
        if let Rest::Identifier(s) = res{Ok(Instruction::PushGlo(s.clone()))} else{Err(())}
    }
  else if first =="pusharg"
    {
        if let Rest::Identifier(s) = res{Ok(Instruction::PushArg(s.clone()))} else{Err(())}
    }
  else if first =="pushloc"
    {
        if let Rest::Identifier(s) = res{Ok(Instruction::PushLoc(s.clone()))} else{Err(())}
    }
  else if first =="pushdst"
    {
        if let Rest::Identifier(s) = res{Ok(Instruction::PushDst(s.clone()))} else{Err(())}
    }

  else
    {
      println!("make_instruction failed. unkown mnemonic {}",first);

      Err(())
    }
}


pub fn
read_routine(&mut self, name: &str, iter: &mut crate::token::Iterator)-> Result<(),()>
{
  let  mut rou = Routine::new();

  rou.label = name.to_string();

    loop
    {
        if let Some(tok) = iter.current()
        {
            if let Some(para) = tok.get_identifier()
            {
              rou.parameter_list.push(para.clone());

              iter.advance();

              continue;
            }

          else
            if tok.is_others('|')
            {
              iter.advance();

              break;
            }
        }


      return Err(());
    }


    loop
    {
        if let Some(tok) = iter.current()
        {
            if let Some(var) = tok.get_identifier()
            {
              rou.variable_list.push(var.clone());

              iter.advance();

              continue;
            }

          else
            if tok.is_others('{')
            {
              iter.advance();

              break;
            }
        }


      return Err(());
    }


    loop
    {
        if let Some(tok) = iter.current()
        {
            if tok.is_others('}')
            {
              iter.advance();

              break;
            }

          else
            if let Ok((first_s,res)) = Self::read_until_terminator(iter)
            {
                if let Ok(instr) = Self::make_instruction(&first_s,&res)
                {
                  rou.base_instruction_list.push(instr);

                  continue;
                }
            }


          println!("read_routine failed.");

          return Err(());
        }
    }


  self.routine_list.push(rou);

  Ok(())
}


pub fn
preprocess(&mut self)-> Result<(),()>
{
  use super::memory::WORD_SIZE;
  use super::memory::get_word_aligned;

  let  mut pos = WORD_SIZE*(self.routine_list.len()
                           +self.global_data_list.len()
                           +self.undefined_space_list.len());

  let  mut name_list = Vec::<String>::new();

    for rou in &mut self.routine_list
    {
      let  sz = rou.get_base_size();

      rou.position = pos      ;
                     pos += sz;

      pos = get_word_aligned(pos);

      name_list.push(rou.label.clone());
    }


    for gd in &mut self.global_data_list
    {
      gd.position = pos                    ;
                    pos += gd.content.len();

      pos = get_word_aligned(pos);

      name_list.push(gd.label.clone());
    }


    for usp in  &mut self.undefined_space_list
    {
      usp.position = pos            ;
                     pos += usp.size;

      pos = get_word_aligned(pos);

      name_list.push(usp.label.clone());
    }


  let  glo_sz = Routine::get_required_size(name_list.len());

    for rou in &mut self.routine_list
    {
        if rou.tuneup(&name_list,glo_sz).is_err()
        {
          return Err(());
        }
    }


  Ok(())
}


pub fn
assemble(&self, start_routine_name: &str)-> ExecutionImage
{
  let  mut buf = Vec::<u8>::new();
  let  mut symbol_list = Vec::<(String,usize)>::new();

    for rou in &self.routine_list
    {
      symbol_list.push((rou.label.clone(),rou.position));

        for b in rou.position.to_ne_bytes()
        {
          buf.push(b);
        }
    }


    for gd in &self.global_data_list
    {
      symbol_list.push((gd.label.clone(),gd.position));

        for b in gd.position.to_ne_bytes()
        {
          buf.push(b);
        }
    }


    for usp in &self.undefined_space_list
    {
      symbol_list.push((usp.label.clone(),usp.position));

        for b in usp.position.to_ne_bytes()
        {
          buf.push(b);
        }
    }


    for rou in &self.routine_list
    {
        while buf.len() < rou.position
        {
          buf.push(NOP);
        }


      rou.write_to(&mut buf);
    }


    for gd in &self.global_data_list
    {
        while buf.len() < gd.position
        {
          buf.push(NOP);
        }


        for b in &gd.content
        {
          buf.push(*b);
        }
    }


    if let Some(rou) = &self.find_routine(start_routine_name)
    {
      ExecutionImage{binary: buf, entry_point: rou.position, symbol_list}
    }

  else
    {
      panic!();
    }
}


pub fn
print(&self)
{
    for rou in &self.routine_list
    {
      print!("routine ");

      rou.print();

      print!("\n\n");
    }


    for gdat in &self.global_data_list
    {
      print!("data {} ",&gdat.label);

        for b in &gdat.content
        {
          print!("{} ",b);
        }


      print!(";\n\n");
    }


    for usp in &self.undefined_space_list
    {
      print!("space {} {};\n\n",&usp.label,usp.size);
    }
}


}




fn
get_u16(bin: &[u8])-> u16
{
  let  u = ((bin[0] as u16)<<8)
          |((bin[1] as u16)   );

  u16::from_be(u)
}


fn
get_u32(bin: &[u8])-> u32
{
  let  u = ((bin[0] as u32)<<24)
          |((bin[1] as u32)<<16)
          |((bin[2] as u32)<< 8)
          |((bin[3] as u32)    );

  u32::from_be(u)
}


fn
get_u64(bin: &[u8])-> u64
{
  let  u = ((bin[0] as u64)<<54)
          |((bin[1] as u64)<<48)
          |((bin[2] as u64)<<40)
          |((bin[3] as u64)<<32)
          |((bin[4] as u64)<<24)
          |((bin[5] as u64)<<16)
          |((bin[6] as u64)<< 8)
          |((bin[7] as u64)    );

  u64::from_be(u)
}


pub fn
print_binary(bin: &[u8])
{
  let  mut i = 0usize;

    while i < bin.len()
    {
      print!("{} ",i);

      let  b = bin[i];

      i += 1;

      Instruction::print_symbol(b);

      print!("({}) ",b);

        match b
        {
      PUSHI8 =>{  print!("{}",bin[i] as i8);               i += 1;}
      PUSHI16=>{  print!("{}",get_u16(&bin[i..]) as i16);  i += 2;}
      PUSHI32=>{  print!("{}",get_u32(&bin[i..]) as i32);  i += 4;}
      PUSHU8 =>{  print!("{}",bin[i]);                     i += 1;}
      PUSHU16=>{  print!("{}",get_u16(&bin[i..]));         i += 2;}
      PUSHU32=>{  print!("{}",get_u32(&bin[i..]));         i += 4;}
      PUSHF32=>{  print!("{}",f32::from_bits(get_u32(&bin[i..])));  i += 4;}
      PUSH64=> {  print!("{}",get_u64(&bin[i..]));         i += 8;}
      _=>{}
        }


      print!("\n");
    }
}


pub struct
ExecutionImage
{
  pub(crate) binary: Vec<u8>,

  pub(crate) entry_point: usize,

  pub(crate) symbol_list: Vec<(String,usize)>,

}


use crate::token::Token;


impl
std::convert::TryFrom<&Vec<Token>> for Source
{


type Error = ();


fn
try_from(toks: &Vec<Token>)-> Result<Self,Self::Error>
{
  let  mut iter = crate::token::Iterator::from(toks);

  let  mut src = Source::new();

    loop
    {
        if let Some(first_tok) = iter.next()
        {
            if let Some(compo_name) = first_tok.get_identifier()
            {
                if let Some(second_tok) = iter.next()
                {
                    if let Some(name) = second_tok.get_identifier()
                    {
                           if compo_name == "routine"{if src.read_routine(name,&mut iter).is_err()        {return Err(());}}
                      else if compo_name ==    "data"{if src.read_global_data(name,&mut iter).is_err()    {return Err(());}}
                      else if compo_name ==   "space"{if src.read_undefined_space(name,&mut iter).is_err(){return Err(());}}
                      else{println!("{} wow",compo_name);return Err(());}
                    }
                }
            }
        }

      else
        {
          break;
        }
    }


  Ok(src)
}


}


impl
std::convert::TryFrom<&str> for ExecutionImage
{


type Error = ();


fn
try_from(s: &str)-> Result<Self,Self::Error>
{
  use crate::token;

    if let Ok(mut toks) = token::tokenize::tokenize_from_string(s)
    {
      toks = token::strip_spaces(toks);

        if let Ok(mut src) = Source::try_from(&toks)
        {
            if src.preprocess().is_ok()
            {
              src.print();

              return Ok(src.assemble("start"));
            }
        }
    }


  Err(())
}


}




