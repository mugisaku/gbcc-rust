

use super::allocation::{
  Allocation,
  AllocationID,
  Operand,
  Source,
  Destination,
};

use super::line::{
  Line,

};

use super::allocating_operation::{
  AllocatingOperation,
  UnaryOperator,
  BinaryOperator,
  PhiPair,
  JumpDestination,
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

  debug_flag: bool,

  main_return_value_address: usize,
  main_return_value_size: usize,

}


#[allow(dead_code)]
impl
Executor
{


pub const SYSTEM_RESERVED_STACK_SIZE: usize = WORD_SIZE*16;


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
            debug_flag: false,
            main_return_value_address: 0,
            main_return_value_size: 0,
          }
}


pub fn   enable_debug(&mut self){self.debug_flag =  true;}
pub fn  disable_debug(&mut self){self.debug_flag = false;}

pub fn    halt(&mut self){self.halt_flag =  true;}
pub fn  unhalt(&mut self){self.halt_flag = false;}
pub fn  is_halted(&mut self)-> bool{self.halt_flag}


pub fn
change_bi(&mut self, dst_i: usize)
{
  self.pbi = self.bi               ;
             self.bi = dst_i as u64;

  self.pc = self.bi+1;
}


pub fn
get_absolute_address(&self, o: &impl Operand)-> usize
{
  let  off = o.get_offset();

    match o.get_id()
    {
  AllocationID::Global(_)=>    {                   off}
  AllocationID::Local(_)=>     {(self.bp as usize)+off}
  AllocationID::Parameter(_)=> {(self.bp as usize)-off}
    }
}


pub fn
get_word(&self, o: &Source)-> Word
{
  let  addr = self.get_absolute_address(o);

    if addr != 0
    {
      let  w = self.memory.get_word(addr);

        if self.debug_flag
        {
          println!("\nget: VALUE {} from ADDRESS {}",w.get_i64(),addr);
        }


      return w;
    }


  println!("unkown operand");

  Word::from_u64(0)
}


pub fn  get_i64(&self, o: &Source)-> i64{self.get_word(o).get_i64()}
pub fn  get_u64(&self, o: &Source)-> u64{self.get_word(o).get_u64()}
pub fn  get_f64(&self, o: &Source)-> f64{self.get_word(o).get_f64()}

pub fn  get_bool(&self, o: &Source)-> bool{self.get_word(o).get_u64() != 0}


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
    if self.debug_flag
    {
      println!("\n put: VALUE {} to ADDRESS {}",w.get_i64(),addr);
    }


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
ready(&mut self, coll: &Collection, start_f_name: &str, arg_ls: Vec<Source>)-> Result<(),()>
{
    if let Some((f,fi)) = coll.find_function(start_f_name)
    {
      self.main_return_value_address = self.bp as usize;
      self.main_return_value_size    = f.return_size;

      self.bp = get_aligned((self.bp as usize)+f.return_size) as u64;
      self.sp = self.bp;

      return self.new_frame(&coll.function_list,self.main_return_value_address as usize,fi,&arg_ls);
    }


  Err(())
}




fn
operate_unary(&mut self, dst_addr: usize, o: &Source, u: UnaryOperator)
{
  let  src = self.get_word(o);

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
operate_binary(&mut self, dst_addr: usize, lo: &Source, ro: &Source, b: BinaryOperator)
{
  let  l = self.get_word(lo);
  let  r = self.get_word(ro);

    match b
    {
  BinaryOperator::AddI=>{self.put_i64(dst_addr,l.get_i64()+r.get_i64());},
  BinaryOperator::SubI=>{self.put_i64(dst_addr,l.get_i64()-r.get_i64());},
  BinaryOperator::MulI=>{self.put_i64(dst_addr,l.get_i64()*r.get_i64());},
  BinaryOperator::DivI=>{self.put_i64(dst_addr,l.get_i64()/r.get_i64());},
  BinaryOperator::RemI=>{self.put_i64(dst_addr,l.get_i64()%r.get_i64());},
  BinaryOperator::AddU=>{self.put_u64(dst_addr,l.get_u64()+r.get_u64());},
  BinaryOperator::SubU=>{self.put_u64(dst_addr,l.get_u64()-r.get_u64());},
  BinaryOperator::MulU=>{self.put_u64(dst_addr,l.get_u64()*r.get_u64());},
  BinaryOperator::DivU=>{self.put_u64(dst_addr,l.get_u64()/r.get_u64());},
  BinaryOperator::RemU=>{self.put_u64(dst_addr,l.get_u64()%r.get_u64());},
  BinaryOperator::AddF=>{self.put_f64(dst_addr,l.get_f64()+r.get_f64());},
  BinaryOperator::SubF=>{self.put_f64(dst_addr,l.get_f64()-r.get_f64());},
  BinaryOperator::MulF=>{self.put_f64(dst_addr,l.get_f64()*r.get_f64());},
  BinaryOperator::DivF=>{self.put_f64(dst_addr,l.get_f64()/r.get_f64());},
  BinaryOperator::RemF=>{self.put_f64(dst_addr,l.get_f64()%r.get_f64());},
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
operate_address(&mut self, dst_addr: usize, target: &Source)
{
  let  addr = self.get_absolute_address(target);

    if addr != 0
    {
      self.put_u64(dst_addr,addr as u64);
    }
}


fn
operate_phi(&mut self, dst_addr: usize, sz: usize, opls: &Vec<PhiPair>, defau: &Source)
{
    for o in opls
    {
//println!("{} ? {} ",o.from.index,self.pbi);
        if o.from.index == (self.pbi as usize)
        {
          let  src_addr = self.get_absolute_address(&o.value);

          Self::transfer(&mut self.memory,dst_addr,src_addr,sz,self.debug_flag);

          return;
        }
    }


  let  src_addr = self.get_absolute_address(defau);

  Self::transfer(&mut self.memory,dst_addr,src_addr,sz,self.debug_flag);
}


fn
transfer(m: &mut Memory, dst_addr: usize, src_addr: usize, sz: usize, debug: bool)
{
    if debug
    {
      println!("\n transfer: to ADDRESS {} from ADDRESS {}, {} bytes",dst_addr,src_addr,sz);
    }


    for off in 0..sz
    {
      let  v = m.get_u8(src_addr+off);

      m.put_u8(dst_addr+off,v);
    }
}


fn
stack_argument_list(&mut self, para_ls: &Vec<Allocation>, arg_ls: &Vec<Source>)-> Result<usize,()>
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

          let  src_addr = self.get_absolute_address(arg);

          Self::transfer(&mut self.memory,(self.sp as usize)+off,src_addr,alo.size,self.debug_flag);

          off = get_aligned(off+alo.size);
        }


      return Ok(off);
    }


  Err(())
}


fn
new_frame(&mut self, f_ls: &Vec<Function>, retval_addr: usize, fi: usize, arg_ls: &Vec<Source>)-> Result<(),()>
{
    if fi < f_ls.len()
    {
      let  f = &f_ls[fi];

        if let Ok(off) = self.stack_argument_list(&f.parameter_list,&arg_ls)
        {
            if self.debug_flag
            {
              println!("\n new frame: RETURN_VALUE_ADDRESS {}",retval_addr);
            }


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
          self.sp  = new_bp+(f.local_stack_size as u64);
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
        if self.debug_flag
        {
          println!("\n remove frame: ");
        }


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
operate_call(&mut self, f_ls: &Vec<Function>, dst_addr: usize, ci: &CallInfo)
{
  self.new_frame(f_ls,dst_addr,ci.function_index,&ci.argument_list);
}


fn
operate(&mut self, f_ls: &Vec<Function>, ln: &Line)
{
    match ln
    {
  Line::AllocatingOperation(dst,sz,ao)=>
        {
          let  dst_addr = self.get_absolute_address(dst);

            match ao
            {
          AllocatingOperation::Unary(o,u)=>{self.operate_unary(dst_addr,o,*u);},
          AllocatingOperation::Binary(l,r,b)=>{self.operate_binary(dst_addr,l,r,*b);},
          AllocatingOperation::Allocate=>{},
          AllocatingOperation::MoveU64(u)=>{self.put_word(dst_addr,Word::from_u64(*u));},
          AllocatingOperation::MoveF64(f)=>{self.put_word(dst_addr,Word::from_f64(*f));},
          AllocatingOperation::Address(target)=>{self.operate_address(dst_addr,target);},
          AllocatingOperation::Phi(opls,defau)=>{self.operate_phi(dst_addr,*sz,opls,defau);},
          AllocatingOperation::Call(ci)=>{self.operate_call(f_ls,dst_addr,ci);},
            }
        }
  Line::CopyWord(dst,src)=>
        {
          let  dst_addr = self.get_absolute_address(dst);
          let  src_addr = self.get_absolute_address(src);

          let  v = self.memory.get_word(src_addr);

          self.memory.put_word(dst_addr,v);
        }
  Line::CopyString(dst,src,sz)=>
        {
          let  dst_addr = self.get_absolute_address(dst);
          let  src_addr = self.get_absolute_address(src);

          Self::transfer(&mut self.memory,dst_addr,src_addr,*sz,self.debug_flag);
        }
  Line::Message(s)=>
        {
          println!("[message] {}",s);
        }
  Line::Print(target,c)=>
        {
          let  src_addr = self.get_absolute_address(target);

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
  Line::Label(name)=>
        {
//          println!("executor::operate warning: passed Label {}",name);
        },
  Line::Jump(bl)=>
        {
          self.change_bi(bl.index);
        },
  Line::Branch(cond,on_true,on_false)=>
        {
            if self.get_bool(cond)
            {
              self.change_bi(on_true.index);
            }

          else
            {
              self.change_bi(on_false.index);
            }
        },
  Line::Return(opt)=>
        {
            if let Some((src,sz)) = opt
            {
              let  dst_addr = self.memory.get_u64(self.bp as usize) as usize;
              let  src_addr = self.get_absolute_address(src);

              Self::transfer(&mut self.memory,dst_addr,src_addr,*sz,self.debug_flag);
            }


          self.remove_frame();
        },
    }
}


pub fn
step(&mut self, f_ls: &Vec<Function>)-> Result<(),()>
{
    if self.calling_depth != 0
    {
self.print_context();
      let  f = &f_ls[self.fi as usize];

      let  pc = self.pc as usize;

        if pc < f.line_list.len()
        {
          let  ln = &f_ls[self.fi as usize].line_list[pc];

          self.pc += 1;

          self.operate(f_ls,ln);
        }


      return Ok(());
    }


  Err(())
}


pub fn
run(&mut self, f_ls: &Vec<Function>, count_opt: Option<usize>)-> RunResult
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


        if self.step(f_ls).is_err()
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
print_local_variables(&self, f_ls: &Vec<Function>)
{
  let  f = &f_ls[self.fi as usize];

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




