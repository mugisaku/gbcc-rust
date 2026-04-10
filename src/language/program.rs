

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::evaluate::*;




const BYTES_SIZE: usize = 10;


pub struct
Node
{
  offset: usize,

  bytes: [u8; BYTES_SIZE],

  size: usize,

  label_position_opt: Option<usize>,

}


impl
Node
{


pub fn
push_byte(&mut self, b: u8)
{
    if self.size < BYTES_SIZE
    {
      self.bytes[self.size] = b;

      self.size += 1;
    }

  else{panic!();}
}


pub fn
push_u16(&mut self, u: u16)
{
  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      self.push_byte(b);
    }
}


pub fn
push_u32(&mut self, u: u32)
{
  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      self.push_byte(b);
    }
}


pub fn
push_u64(&mut self, u: u64)
{
  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      self.push_byte(b);
    }
}


pub fn
write_bool(&mut self, b: bool)
{
  self.push_byte(if b{Opcode::Push1} else{Opcode::Push0} as u8);
}


pub fn
write_int(&mut self, i: i64)
{
    if i >= 0
    {
        match i
        {
      0=>{self.push_byte(Opcode::Push0 as u8);}
      1=>{self.push_byte(Opcode::Push1 as u8);}
      2=>{self.push_byte(Opcode::Push2 as u8);}
      3=>{self.push_byte(Opcode::Push3 as u8);}
      4=>{self.push_byte(Opcode::Push4 as u8);}
      5=>{self.push_byte(Opcode::Push5 as u8);}
      6=>{self.push_byte(Opcode::Push6 as u8);}
      7=>{self.push_byte(Opcode::Push7 as u8);}
      8=>{self.push_byte(Opcode::Push8 as u8);}
      _=>
        {
          self.push_byte(Opcode::Li as u8);

               if i <= ( u8::MAX as i64){  self.push_byte(ImmKind::U8  as u8);  self.push_byte(i as  u8);}
          else if i <= (u16::MAX as i64){  self.push_byte(ImmKind::U16 as u8);  self.push_u16( i as u16);}
          else if i <= (u32::MAX as i64){  self.push_byte(ImmKind::U32 as u8);  self.push_u32( i as u32);}
          else                          {  self.push_byte(ImmKind::U64 as u8);  self.push_u64( i as u64);}
        }
        }
    }

  else
    {
      let  u = i.abs();

      self.push_byte(Opcode::Li as u8);

           if u <= ( u8::MAX as i64){  self.push_byte(ImmKind::I8  as u8);  self.push_byte(i as  u8);}
      else if u <= (u16::MAX as i64){  self.push_byte(ImmKind::I16 as u8);  self.push_u16( i as u16);}
      else if u <= (u32::MAX as i64){  self.push_byte(ImmKind::I32 as u8);  self.push_u32( i as u32);}
      else                          {  self.push_byte(ImmKind::I64 as u8);  self.push_u64( i as u64);}
    }
}


pub fn
write_float(&mut self, f: f64)
{
  self.push_byte(Opcode::Li as u8);

    if f.abs() < (f32::MAX as f64)
    {
      let  bits = (f as f32).to_bits();

      self.push_byte(ImmKind::F32 as u8);

      self.push_u32(bits);
    }

  else
    {
      let  bits = f.to_bits();

      self.push_byte(ImmKind::F64 as u8);

      self.push_u64(bits);
    }
}


pub fn
reset(&mut self, ln: &AsmLine)
{
  let  op = ln.get_opcode().clone();

    match op
    {
  Opcode::Li=>
    {
        match ln.get_postfix()
        {
      Postfix::Bool(b) =>{self.write_bool(*b);}
      Postfix::Int(i)  =>{self.write_int(*i);}
      Postfix::Float(f)=>{self.write_float(*f);}
      _=>{panic!();}
        }
    }
  Opcode::Jmp
 |Opcode::Brz
 |Opcode::Brnz=>
    {
      self.push_byte(op as u8);
      self.push_byte(ImmKind::I64 as u8);
      self.push_u64(0);
    }
  _=>
    {
      self.push_byte(op as u8);
    }
    }
}


pub fn
rewrite_imm(&mut self, target_offset: usize)
{
  let  a = target_offset as isize;
  let  b = (self.offset+self.size) as isize;

  let  dist = a-b;

  self.size = 2;

  self.push_u64(dist as usize as u64);
}


pub fn
output(&self, dst_bytes: &mut Vec<u8>)
{
    for i in 0..self.size
    {
      dst_bytes.push(self.bytes[i]);
    }
}


}




pub struct
Program
{
  name: String,

  stack_size: usize,

  table: AsmTable,

  bytes: Vec<u8>,

  offset: usize,

}


impl
Program
{


pub fn
new(name: &str, stack_size: usize, mut body: AsmTable)-> Self
{
  let  mut table = AsmTable::new();

    if stack_size != 0
    {
      table.push_li_int((stack_size/WORD_SIZE) as i64);
      table.push_opcode(Opcode::Xs);
    }


  table.get_core_mut().append(body.get_core_mut());

  table.reset_block_position();

  Self{
    name: name.to_string(),
    stack_size,
    table,
    bytes: Vec::new(),
    offset: 0,
  }
}


pub fn
build_nodes(&self)-> Vec<Node>
{
  let  mut nodes = Vec::<Node>::new();

  let  mut offset = 0usize;

    for blk in self.table.get_core()
    {
        for ln in blk.get_lines()
        {
          let  mut label_position_opt = 
            if let Postfix::Identifier(s) = ln.get_postfix()
            {
              Some(self.table.find_block(s).unwrap().get_position())
            }

          else{None};


          let  mut nd = Node{offset, bytes: [0; BYTES_SIZE], size: 0, label_position_opt};

          nd.reset(ln);

          offset += nd.size;

          nodes.push(nd);
        }
    }


  nodes
}


pub fn
update_imm(nodes: &mut Vec<Node>)
{
    for i in 0..nodes.len()
    {
        if let Some(label_position) = &nodes[i].label_position_opt
        {
          let  target_offset = nodes[*label_position].offset;

          nodes[i].rewrite_imm(target_offset);
        }
    }
}


pub fn
update_offset(nodes: &mut Vec<Node>)
{
  let  mut off = 0usize;

    for nd in nodes
    {
      nd.offset = off           ;
                  off += nd.size;
    }
}


pub fn
build(&mut self, offset: usize)-> Result<(),()>
{
  let  mut nodes = self.build_nodes();

  Self::update_imm(&mut nodes);

  self.bytes.clear();

    for nd in &nodes
    {
      nd.output(&mut self.bytes);
    }


    if let Some(last) = self.bytes.last()
    {
      let  ret_code = Opcode::Ret as u8;

        if *last != ret_code
        {
          self.bytes.push(ret_code);
        }
    }


  self.offset = offset;

  Ok(())
}


pub fn
get_bytes(&self)-> &Vec<u8>
{
  &self.bytes
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
set_offset(&mut self, off: usize)
{
  self.offset = off;
}


pub fn
print_lines(&self)
{
  println!("Lines{{");

  self.table.print();

  println!("}}");

  println!("stack size: {}, off: {}, size: {}",self.stack_size,self.offset,self.bytes.len());
}


pub fn
print_bytes(&self)
{
  println!("Bytes{{");

  let  mut off = 0usize;

    while off < self.bytes.len()
    {
      print!("[{:0>5}] ",off);

      let  op = Opcode::from(self.bytes[off]);

      off += 1;

      op.print();

      print!(" ");

        match op
        {
      Opcode::Li
     |Opcode::Jmp
     |Opcode::Brz
     |Opcode::Brnz=>
        {
          let  kb = self.bytes[off];

          off += 1;

            match kb
            {
          (k) if k == (ImmKind::U8 as u8)=>
            {
              print!("{}",self.bytes[off]);

              off += 1;
            }
          (k) if k == (ImmKind::U16 as u8)=>
            {
              let  buf: [u8; 2] = [self.bytes[off  ],
                                   self.bytes[off+1]];

              print!("{}",u16::from_be_bytes(buf));

              off += 2;
            }
          (k) if k == (ImmKind::U32 as u8)=>
            {
              let  buf: [u8; 4] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3]];

              print!("{}",u32::from_be_bytes(buf));

              off += 4;
            }
          (k) if k == (ImmKind::U64 as u8)=>
            {
              let  buf: [u8; 8] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3],
                                   self.bytes[off+4],
                                   self.bytes[off+5],
                                   self.bytes[off+6],
                                   self.bytes[off+7]];

              let  u = u64::from_be_bytes(buf);

              print!("{}",u);

              off += 8;
            }
          (k) if k == (ImmKind::I8 as u8)=>
            {
              print!("{}",self.bytes[off] as i8);

              off += 1;
            }
          (k) if k == (ImmKind::I16 as u8)=>
            {
              let  buf: [u8; 2] = [self.bytes[off  ],
                                   self.bytes[off+1]];

              print!("{}",i16::from_be_bytes(buf));

              off += 2;
            }
          (k) if k == (ImmKind::I32 as u8)=>
            {
              let  buf: [u8; 4] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3]];

              print!("{}",i32::from_be_bytes(buf));

              off += 4;
            }
          (k) if k == (ImmKind::I64 as u8)=>
            {
              let  buf: [u8; 8] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3],
                                   self.bytes[off+4],
                                   self.bytes[off+5],
                                   self.bytes[off+6],
                                   self.bytes[off+7]];

              let  i = i64::from_be_bytes(buf);

              print!("{}",i);

              off += 8;
            }
          (k) if k == (ImmKind::F32 as u8)=>
            {
              let  buf: [u8; 4] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3]];

              print!("{}",f32::from_be_bytes(buf));

              off += 4;
            }
          (k) if k == (ImmKind::F64 as u8)=>
            {
              let  buf: [u8; 8] = [self.bytes[off  ],
                                   self.bytes[off+1],
                                   self.bytes[off+2],
                                   self.bytes[off+3],
                                   self.bytes[off+4],
                                   self.bytes[off+5],
                                   self.bytes[off+6],
                                   self.bytes[off+7]];

              let  f = f64::from_be_bytes(buf);

              print!("{}",f);

              off += 8;
            }
          (k)=>{print!("{}",k);}
            }
        }
      _=>{}
        }


      println!("");
    }


  println!("}}");

  println!("stack size: {}, off: {}, size: {}",self.stack_size,self.offset,self.bytes.len());
}


}




