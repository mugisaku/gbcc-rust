

use super::allocation::{
  Allocation,
  AllocationKind,
  AllocationLink,
};

use super::line::{
  Line,
  BlockLink,
};

use super::allocating_operation::{
  AllocatingOperation,
  UnaryOperator,
  BinaryOperator,
  Operand,
  PhiOperand,
  CallInfo,
};

use super::memory::{
  Memory,
  Word,
  WORD_SIZE,
  get_aligned,
};


use super::function::{
  Function,
  FunctionLink,
};


use super::collection::{
  Collection,
};




pub enum
RunResult
{
  Stop,
  StopByError,
  Finish(Memory),

}


pub struct
Executor
{
  memory: Memory,

   pc: u64,//ProgramCounter(OperationIndex)
   bp: u64,//BasePointer
   sp: u64,//StackPointer
   fi: u64,//FunctionIndex
   bi: u64,//BlockIndex
  pbi: u64,//PreviousBlockIndex

  calling_depth: usize,

  halt_flag: bool,

  main_return_value_address: usize,
  main_return_value_size: usize,

}


#[allow(dead_code)]
impl
Executor
{


const SYSTEM_RESERVED_SIZE: usize = WORD_SIZE*16;


pub fn
new(memsz: usize)-> Executor
{
  Executor{ memory: Memory::new(65536),
            pc: 0,
            bp: 0,
            sp: 0,
            fi: 0,
            bi: 0,
            pbi: 0,
            calling_depth: 0,
            halt_flag: true,
            main_return_value_address: 0,
            main_return_value_size: 0,
          }
}


pub fn    halt(&mut self){self.halt_flag =  true;}
pub fn  unhalt(&mut self){self.halt_flag = false;}
pub fn  is_halted(&mut self)-> bool{self.halt_flag}


pub fn
change_bi(&mut self, bl: &BlockLink)
{
    if let BlockLink::Resolved(i) = bl
    {
      self.pbi = self.bi            ;
                 self.bi = *i as u64;

      self.pc = self.bi+1;

      return;
    }


  print!("change_bi error");

    if let BlockLink::Unresolved(name) = bl
    {
      print!(": block <{}> is not found",name);
    }
}


pub fn
get_absolute_address(&self, alo: &Allocation)-> usize
{
  let  off = alo.offset;

    match alo.kind
    {
  AllocationKind::Global=>    {off}
  AllocationKind::Local=>     {(self.bp as usize)+Self::SYSTEM_RESERVED_SIZE+off}
  AllocationKind::Parameter=> {(self.bp as usize)-off}
    }
}


pub fn
get_absolute_address_by_link(&self, coll: &Collection, ln: &AllocationLink)-> usize
{
    if let Some(alo) = coll.get_allocation_by_link(ln)
    {
      return self.get_absolute_address(alo);
    }


  0
}


pub fn
get_word(&self, coll: &Collection, o: &Operand)-> Word
{
    if let Operand::ImmediateValue(v) = o
    {
      return *v;
    }

  else
    if let Operand::AllocationLink(ln) = o
    {
      let  addr = self.get_absolute_address_by_link(coll,ln);

        if addr != 0
        {
    //self.print_context();
    //println!("\naddr: {}",addr);
          return self.memory.get_word(addr);
        }
    }


  println!("unkown operand");

  Word::from_u64(0)
}


pub fn  get_i64(&self, coll: &Collection, o: &Operand)-> i64{self.get_word(coll,o).get_i64()}
pub fn  get_u64(&self, coll: &Collection, o: &Operand)-> u64{self.get_word(coll,o).get_u64()}
pub fn  get_f64(&self, coll: &Collection, o: &Operand)-> f64{self.get_word(coll,o).get_f64()}

pub fn  get_bool(&self, coll: &Collection, o: &Operand)-> bool{self.get_word(coll,o).get_u64() != 0}


pub fn  put_i64(&mut self, addr: usize, v: i64){self.memory.put_i64(addr,v);}
pub fn  put_u64(&mut self, addr: usize, v: u64){self.memory.put_u64(addr,v);}
pub fn  put_f64(&mut self, addr: usize, v: f64){self.memory.put_f64(addr,v);}

pub fn
put_bool(&mut self, addr: usize, b: bool)
{
  let  v: u64 = if b{1}else{0};

  self.memory.put_u64(addr,v);
}

pub fn
put_word(&mut self, addr: usize, w: Word)
{
  self.memory.put_word(addr,w);
}




pub fn
reset(&mut self, coll: &Collection)-> Result<(),()>
{
  self.pc  = 0;
  self.fi  = 0;
  self.bi  = 0;
  self.pbi = 0;
  self.calling_depth = 0;
  self.main_return_value_address = 0;
  self.main_return_value_size = 0;

  self.memory.zerofill();

    for alo in &coll.allocation_list
    {
        if let Some(m) = &alo.memory_opt
        {
            if self.memory.read(alo.offset,m,0,None).is_err()
            {
              return Err(());
            }
        }
    }


            self.bp = get_aligned(coll.allocation_area_end) as u64;
  self.sp = self.bp                                               ;

  Ok(())
}




pub fn
ready(&mut self, coll: &Collection, start_f_name: &str, arg_ls: Vec<Operand>)-> Result<(),()>
{
    if let Some((f,fi)) = coll.find_function(start_f_name)
    {
      self.main_return_value_address = self.bp as usize;
      self.main_return_value_size    = f.return_size;

      self.bp = get_aligned((self.bp as usize)+f.return_size) as u64;
      self.sp = self.bp;

      return self.new_frame(coll,self.main_return_value_address as usize,fi,&arg_ls);
    }


  Err(())
}




fn
operate_unary(&mut self, coll: &Collection, dst_addr: usize, o: &Operand, u: UnaryOperator)
{
  let  src = self.get_word(coll,o);

    match u
    {
  UnaryOperator::ExS8=> {self.put_i64(dst_addr,src.get_i8() as i64);},
  UnaryOperator::ExS16=>{self.put_i64(dst_addr,src.get_i16() as i64);},
  UnaryOperator::ExS32=>{self.put_i64(dst_addr,src.get_i32() as i64);},
  UnaryOperator::ExF32=>{self.put_f64(dst_addr,src.get_f32() as f64);},

  UnaryOperator::StoF=>{self.put_f64(dst_addr,src.get_i64() as f64);},
  UnaryOperator::FtoS=>{self.put_i64(dst_addr,src.get_f64() as i64);},

  UnaryOperator::Not=>{self.put_u64(dst_addr,!src.get_u64());},

  UnaryOperator::Neg=> {self.put_i64(dst_addr,-src.get_i64());},
  UnaryOperator::NegF=>{self.put_f64(dst_addr,-src.get_f64());},

  UnaryOperator::LogicalNot=>{self.put_bool(dst_addr,!src.get_bool());},
    }
}


fn
operate_binary(&mut self, coll: &Collection, dst_addr: usize, lo: &Operand, ro: &Operand, b: BinaryOperator)
{
  let  l = self.get_word(coll,lo);
  let  r = self.get_word(coll,ro);

    match b
    {
  BinaryOperator::AddI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::SubI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::MulI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::DivI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::RemI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::AddU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::SubU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::MulU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::DivU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::RemU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::AddF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::SubF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::MulF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::DivF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::RemF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::Shl=>{self.put_u64(dst_addr,l.get_u64()<<r.get_u64());},
  BinaryOperator::Shr=>{self.put_u64(dst_addr,l.get_u64()>>r.get_u64());},
  BinaryOperator::Or=> {self.put_u64(dst_addr,l.get_u64()|r.get_u64());},
  BinaryOperator::And=>{self.put_u64(dst_addr,l.get_u64()&r.get_u64());},
  BinaryOperator::Xor=>{self.put_u64(dst_addr,l.get_u64()^r.get_u64());},
  BinaryOperator::Eq=> {self.put_bool(dst_addr,l.get_u64() ==  r.get_u64());},
  BinaryOperator::Neq=>{self.put_bool(dst_addr,l.get_u64() !=  r.get_u64());},
  BinaryOperator::LtI=>  {self.put_bool(dst_addr,l.get_i64() <  r.get_i64());},
  BinaryOperator::LteqI=>{self.put_bool(dst_addr,l.get_i64() <= r.get_i64());},
  BinaryOperator::GtI=>  {self.put_bool(dst_addr,l.get_i64() >  r.get_i64());},
  BinaryOperator::GteqI=>{self.put_bool(dst_addr,l.get_i64() >= r.get_i64());},
  BinaryOperator::LtU=>  {self.put_bool(dst_addr,l.get_u64() <  r.get_u64());},
  BinaryOperator::LteqU=>{self.put_bool(dst_addr,l.get_u64() <= r.get_u64());},
  BinaryOperator::GtU=>  {self.put_bool(dst_addr,l.get_u64() >  r.get_u64());},
  BinaryOperator::GteqU=>{self.put_bool(dst_addr,l.get_u64() >= r.get_u64());},
  BinaryOperator::LtF=>  {self.put_bool(dst_addr,l.get_f64() <  r.get_f64());},
  BinaryOperator::LteqF=>{self.put_bool(dst_addr,l.get_f64() <= r.get_f64());},
  BinaryOperator::GtF=>  {self.put_bool(dst_addr,l.get_f64() >  r.get_f64());},
  BinaryOperator::GteqF=>{self.put_bool(dst_addr,l.get_f64() >= r.get_f64());},
  BinaryOperator::LogicalAnd=>{self.put_bool(dst_addr,l.get_bool() && r.get_bool());},
  BinaryOperator::LogicalOr=> {self.put_bool(dst_addr,l.get_bool() || r.get_bool());},
    }
}


fn
operate_address(&mut self, coll: &Collection, dst_addr: usize, target: &AllocationLink)
{
  let  addr = self.get_absolute_address_by_link(coll,target);

    if addr != 0
    {
      self.put_u64(dst_addr,addr as u64);
    }
}


fn
operate_phi(&mut self, coll: &Collection, dst_addr: usize, opls: &Vec<PhiOperand>)
{
    for o in opls
    {
        if let BlockLink::Resolved(bi) = o.from
        {
            if bi == (self.pbi as usize)
            {
              self.put_word(dst_addr,self.get_word(coll,&o.value));

              return;
            }
        }
    }


  println!("operate phi error: no one operand is matched");

  self.halt();
}


fn
transfer_operand(&mut self, coll: &Collection, dst_addr: usize, sz: usize, o: &Operand)
{
    if let Operand::ImmediateValue(v) = o
    {
      self.put_word(dst_addr,*v);
    }

  else
    if let Operand::AllocationLink(ln) = o
    {
      let  addr = self.get_absolute_address_by_link(coll,ln);

        if addr != 0
        {
          let  _ = self.memory.copy(dst_addr,addr,sz);
        }
    }
}


fn
stack_argument_list(&mut self, coll: &Collection, para_ls: &Vec<Allocation>, arg_ls: &Vec<Operand>)-> Result<usize,()>
{
  let  len = para_ls.len();

  let  mut off: usize = 0;

    if len == arg_ls.len()
    {
        for n in 0..len
        {
          let  i = len-1-n;

          let  alo = &para_ls[i];
          let  arg =  &arg_ls[i];

          self.transfer_operand(coll,(self.sp as usize)+off,alo.size,arg);

          off = get_aligned(off+alo.size);
        }


      return Ok(off);
    }


  Err(())
}


fn
new_frame(&mut self, coll: &Collection, retval_addr: usize, fi: usize, arg_ls: &Vec<Operand>)-> Result<(),()>
{
    if fi < coll.function_list.len()
    {
      let  f = &coll.function_list[fi];

        if let Ok(off) = self.stack_argument_list(coll,&f.parameter_list,&arg_ls)
        {
          let  new_bp = self.sp+(off as u64);

          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*0),retval_addr as u64);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*1),self.pc);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*2),self.bp);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*3),self.sp);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*4),self.fi);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*5),self.bi);
          self.memory.put_u64((new_bp as usize)+(WORD_SIZE*6),self.pbi);

          self.pc  = 0;
          self.bp  = new_bp;
          self.sp  = new_bp+(Self::SYSTEM_RESERVED_SIZE as u64)+(f.get_allocation_size() as u64);
          self.fi  = fi as u64;
          self.bi  = 0;
          self.pbi = 0;

          self.calling_depth += 1;

          return Ok(());
        }
    }


  Err(())
}


fn
remove_frame(&mut self)
{
    if self.calling_depth > 0
    {
      let  old_bp = self.bp as usize;

      self.pc  = self.memory.get_u64(old_bp+(WORD_SIZE*1));
      self.bp  = self.memory.get_u64(old_bp+(WORD_SIZE*2));
      self.sp  = self.memory.get_u64(old_bp+(WORD_SIZE*3));
      self.fi  = self.memory.get_u64(old_bp+(WORD_SIZE*4));
      self.bi  = self.memory.get_u64(old_bp+(WORD_SIZE*5));
      self.pbi = self.memory.get_u64(old_bp+(WORD_SIZE*6));

      self.calling_depth -= 1;
    }
}


fn
operate_call(&mut self, coll: &Collection, dst_addr: usize, ci: &CallInfo)
{
    if let FunctionLink::Resolved(fi) = &ci.target
    {
      self.new_frame(coll,dst_addr,*fi,&ci.argument_list);
    }
}


fn
operate(&mut self, coll: &Collection, ln: &Line)
{
    match ln
    {
  Line::AllocatingOperation(alo_ln,_,ao)=>
        {
          let  dst_addr = self.get_absolute_address_by_link(coll,alo_ln);

            match ao
            {
          AllocatingOperation::Unary(o,u)=>{self.operate_unary(coll,dst_addr,o,*u);},
          AllocatingOperation::Binary(l,r,b)=>{self.operate_binary(coll,dst_addr,l,r,*b);},
          AllocatingOperation::Allocate=>{},
          AllocatingOperation::Address(target)=>{self.operate_address(coll,dst_addr,target);},
          AllocatingOperation::Phi(opls)=>{self.operate_phi(coll,dst_addr,opls);},
          AllocatingOperation::Call(ci)=>{self.operate_call(coll,dst_addr,ci);},
            }
        }
  Line::CopyWord(dst,src)=>
        {
          let  dst_addr = self.get_absolute_address_by_link(coll,dst);
          let  src_addr = self.get_absolute_address_by_link(coll,src);

          let  v = self.memory.get_word(src_addr);

          self.memory.put_word(dst_addr,v);
        }
  Line::CopyString(dst,src,sz)=>
        {
          let  dst_addr = self.get_absolute_address_by_link(coll,dst);
          let  src_addr = self.get_absolute_address_by_link(coll,src);

            for off in 0..*sz
            {
              let  v = self.memory.get_u8(src_addr+off);

              self.memory.put_u8(dst_addr+off,v);
            }
        }
  Line::Message(s)=>
        {
          println!("[message] {}",s);
        }
  Line::Print(target,c)=>
        {
          let  src_addr = self.get_absolute_address_by_link(coll,target);

            if src_addr != 0
            {
              let  w = self.memory.get_word(src_addr);

                match c
                {
              'i'=>{println!("[print] {}",w.get_i64());}
              'u'=>{println!("[print] {}",w.get_u64());}
              'f'=>{println!("[print] {}",w.get_f64());}
              _=>{}
                }
            }
        }
  Line::BlockOpen(name)=>
        {
          println!("executor::operate warning: passed BlockOpen {}",name);
        },
  Line::Jump(bl)=>
        {
          self.change_bi(bl);
        },
  Line::Branch(cond,on_true,on_false)=>
        {
            if let Some(alo) = coll.get_allocation_by_link(cond)
            {
              let  addr = self.get_absolute_address(alo);

                if self.memory.get_word(addr).get_u64() != 0
                {
                  self.change_bi(on_true);
                }

              else
                {
                  self.change_bi(on_false);
                }
            }
        },
  Line::Return(o_opt)=>
        {
            if let Some(o) = o_opt
            {
              let  retval_addr = self.memory.get_u64(self.bp as usize) as usize;

              self.memory.put_word(retval_addr,self.get_word(coll,o));
            }


          self.remove_frame();
        },
    }
}


pub fn
step(&mut self, coll: &Collection)-> Result<(),()>
{
    if self.calling_depth != 0
    {
self.print_context();
      let  f = &coll.function_list[self.fi as usize];

        if (self.pc as usize) < f.line_list.len()
        {
          let  ln = &f.line_list[self.pc as usize];

          self.pc += 1;

          self.operate(coll,ln);
        }


      return Ok(());
    }


  Err(())
}


pub fn
run(&mut self, coll: &Collection, count_opt: Option<usize>)-> RunResult
{
  self.halt_flag = false;

    while !self.halt_flag
    {
        if let Some(mut count) = count_opt
        {
            if count == 0
            {
              return RunResult::Stop;
            }


          count -= 1;
        }


        if self.step(coll).is_err()
        {
          return RunResult::Finish(self.get_final_return_value());
        }
    }


  RunResult::Stop
}


pub fn
get_final_return_value(&self)-> Memory
{
  Memory::from_memory(&self.memory,self.main_return_value_address,self.main_return_value_size)
}




pub fn
print_local_variables(&self, coll: &Collection)
{
  let  f = &coll.function_list[self.fi as usize];

    for alo in &f.allocation_list
    {
      let  addr = (self.bp as usize)+alo.offset;

      let  i = self.memory.get_u64(addr);

      println!("addr:{}, value:{}",addr,i);
    }
}


pub fn
print_context(&self)
{
  println!("[pc:{:>5}, bp:{:>5}, sp:{:>5}, bi:{:>5}, pbi:{:>5}]",
    self.pc,
    self.bp,
    self.sp,
    self.bi,
    self.pbi,
  );
}


}




