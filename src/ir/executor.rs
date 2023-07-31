

use super::allocation::{
  Allocation,
  AllocationLink,
};

use super::line::{
  NonAllocatingOperation,
  Line,
};

use super::allocating_operation::{
  AllocatingOperation,
  UnaryOperator,
  BinaryOperator,
  Operand,
  PhiOperand,
  CallInfo,
};

use super::block::{
  Terminator,
  Block,
  BlockLink,
  BranchInfo,
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
StepResult
{
    Continue,
  NoContinue,

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

  next_fi: Option<u64>,

  address_of_return_value: u64,

  ap: u64,//ArgumentPointer

  halt_flag: bool,

}


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
            next_fi: None,
            address_of_return_value: 0,
            ap: 0,
            halt_flag: true,
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

      self.pc = 0;
    }
}


pub fn
get_absolute_address(&self, ln: &AllocationLink)-> u64
{
/*
    match ln
    {
  AddressSource::GlobalOffset(off)=>{*off as u64}
  AddressSource::LocalOffset(off)=> {((self.bp as i64)+(Self::SYSTEM_RESERVED_SIZE as i64)+*off) as u64}
  AddressSource::ArgumentOffset(off)=> {((self.bp as i64)+*off) as u64}
    }
*/
0
}


pub fn
get_word(&self, o: &Operand)-> Word
{
    if let Operand::ImmediateValue(v) = o
    {
      return *v;
    }

  else
    if let Operand::AllocationLink(ln) = o
    {
      let  addr = self.get_absolute_address(ln);

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


pub fn  get_i64(&self, o: &Operand)-> i64{self.get_word(o).get_i64()}
pub fn  get_u64(&self, o: &Operand)-> u64{self.get_word(o).get_u64()}
pub fn  get_f64(&self, o: &Operand)-> f64{self.get_word(o).get_f64()}

pub fn  get_bool(&self, o: &Operand)-> bool{self.get_word(o).get_u64() != 0}


pub fn  put_i64(&mut self, addr: u64, v: i64){self.memory.put_i64(addr,v);}
pub fn  put_u64(&mut self, addr: u64, v: u64){self.memory.put_u64(addr,v);}
pub fn  put_f64(&mut self, addr: u64, v: f64){self.memory.put_f64(addr,v);}

pub fn
put_bool(&mut self, addr: u64, b: bool)
{
  let  v: u64 = if b{1}else{0};

  self.memory.put_u64(addr,v);
}

pub fn
put_word(&mut self, addr: u64, w: Word)
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
  self.next_fi = None;
  self.address_of_return_value = 0;

  self.memory.zerofill();

    for alo in &coll.allocation_list
    {
/*
        if let Some(m) = &alo.initial_value
        {
            if self.memory.read(vi.offset as u64,m,0,None).is_err()
            {
              return Err(());
            }
        }
*/
    }


//            self.bp = coll.get_next_offset() as u64;
//  self.sp = self.bp                                ;

  Ok(())
}




pub fn
push_argument(&mut self, coll: &Collection, arg: Operand)-> Result<(),()>
{
/*
    if let Some(wc) = self.word_count_list.pop()
    {
//println!("argument is pushed to address {}",self.ap);
//arg.print();
      let  w = self.get_word(&arg);

      self.memory.put_word(self.ap,w);

      self.ap += WORD_SIZE as u64;

      return Ok(());
    }
*/


  println!("needed no more operand");

  Err(())
}


pub fn
prepare_first_call(&mut self, coll: &Collection, fn_name: &str)-> Result<(),()>
{
    if let Some(alo) = coll.find_allocation(fn_name)
    {
      let  dst_addr = self.sp+(WORD_SIZE as u64);

/*
        if let Some(init_v) = &vi.initial_value
        {
          let  fi = init_v.get_u64(0);

          let  f = &coll.function_list[fi as usize];

          let  sz = f.return_size as u64;

          self.memory.put_u64(self.bp,sz);

          self.sp += (WORD_SIZE as u64)+sz;

          return self.prepare_call(coll,dst_addr,fi);
        }
*/
    }


  Err(())
}


pub fn
prepare_call(&mut self, coll: &Collection, dst_addr: u64, f_index: u64)-> Result<(),()>
{
    if (f_index as usize) < coll.function_list.len()
    {
      self.next_fi = Some(f_index);

      let  f = &coll.function_list[f_index as usize];

      self.address_of_return_value = dst_addr;

//      self.word_count_list.clear();

        for p in &f.parameter_list
        {
//          self.word_count_list.push(p.word_count.clone());
        }


      self.ap = self.sp;

      return Ok(());
    }


  println!("[prepare_call error] {} is invalid function index",f_index);

  Err(())
}


pub fn
raise_call(&mut self, coll: &Collection)-> Result<(),()>
{
    if let Some(fi) = self.next_fi
    {
      let  f = &coll.function_list[fi as usize];

/*
        if 0
        {
          self.new_frame(coll);

          return Ok(());
        }

      else
        {
          println!("number of arguments is not enough");
        }
*/
    }

  else
    {
      println!("no function index is set");
    }


  Err(())
}


pub fn
new_frame(&mut self, coll: &Collection)
{
    if let Some(fi) = self.next_fi
    {
      let  f = &coll.function_list[fi as usize];

      let  new_bp = self.ap;

      self.memory.put_u64(new_bp+(WORD_SIZE as u64*0),self.address_of_return_value);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*1),self.pc);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*2),self.bp);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*3),self.sp);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*4),self.fi);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*5),self.bi);
      self.memory.put_u64(new_bp+(WORD_SIZE as u64*6),self.pbi);

      self.pc  = 0;
      self.bp  = new_bp;
      self.sp  = new_bp+(Self::SYSTEM_RESERVED_SIZE as u64)+(f.get_allocation_size() as u64);
      self.fi  = fi;
      self.bi  = 0;
      self.pbi = 0;

      self.calling_depth += 1;
    }
}


pub fn
remove_frame(&mut self)
{
    if self.calling_depth > 0
    {
      let  old_bp = self.bp;

      self.pc  = self.memory.get_u64(old_bp+(WORD_SIZE as u64*1));
      self.bp  = self.memory.get_u64(old_bp+(WORD_SIZE as u64*2));
      self.sp  = self.memory.get_u64(old_bp+(WORD_SIZE as u64*3));
      self.fi  = self.memory.get_u64(old_bp+(WORD_SIZE as u64*4));
      self.bi  = self.memory.get_u64(old_bp+(WORD_SIZE as u64*5));
      self.pbi = self.memory.get_u64(old_bp+(WORD_SIZE as u64*6));

      self.calling_depth -= 1;
    }
}




pub fn
operate_unary(&mut self, dst_addr: u64, o: &Operand, u: UnaryOperator)
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


pub fn
operate_binary(&mut self, dst_addr: u64, lo: &Operand, ro: &Operand, b: BinaryOperator)
{
  let  l = self.get_word(lo);
  let  r = self.get_word(ro);

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


pub fn
operate_address(&mut self, dst_addr: u64, target: &AllocationLink)
{
  let  addr = self.get_absolute_address(&target);

    if addr != 0
    {
      self.put_u64(dst_addr,addr);
    }
}


pub fn
operate_phi(&mut self, dst_addr: u64, opls: &Vec<PhiOperand>)
{
    for o in opls
    {
        if let BlockLink::Resolved(bi) = o.from
        {
            if bi == (self.pbi as usize)
            {
              self.put_word(dst_addr,self.get_word(&o.value));

              return;
            }
        }
    }


  println!("operate phi error: no one operand is matched");

  self.halt();
}


pub fn
operate_call(&mut self, coll: &Collection, dst_addr: u64, ci: &CallInfo)
{
    if let FunctionLink::Resolved(i) = &ci.target
    {
        if self.prepare_call(coll,dst_addr,*i as u64).is_err()
        {
          self.halt();

          return;
        }


      let  l = ci.argument_list.len();

        for i in 0..l
        {
          let  o = ci.argument_list[l-(1+i)].clone();

            if self.push_argument(coll,o).is_err()
            {
              self.halt();

              return;
            }
        }


        if self.raise_call(coll).is_err()
        {
          self.halt();
        }
    }
}


pub fn
operate(&mut self, coll: &Collection, ln: &Line)-> StepResult
{
    match ln
    {
  Line::AllocatingOperation(ln,ao)=>
        {
          let  dst_addr = self.get_absolute_address(&ln);

            match ao
            {
          AllocatingOperation::Unary(o,u)=>{self.operate_unary(dst_addr,o,*u);},
          AllocatingOperation::Binary(l,r,b)=>{self.operate_binary(dst_addr,l,r,*b);},
          AllocatingOperation::Allocate(wc)=>{},
          AllocatingOperation::Address(target)=>{self.operate_address(dst_addr,target);},
          AllocatingOperation::Phi(opls)=>{self.operate_phi(dst_addr,opls);},
          AllocatingOperation::Call(ci)=>{self.operate_call(coll,dst_addr,ci);},
            }
        }
  Line::NonAllocatingOperation(nao)=>
        {
            match nao
            {
          NonAllocatingOperation::CopyWord(dst,src)=>
                {
                  let  dst_addr = self.get_absolute_address(dst);
                  let  src_addr = self.get_absolute_address(src);

                  let  v = self.memory.get_word(src_addr);

                  self.memory.put_word(dst_addr,v);
                }
          NonAllocatingOperation::CopyString(dst,src,sz)=>
                {
                  let  dst_addr = self.get_absolute_address(dst);
                  let  src_addr = self.get_absolute_address(src);

                    for off in 0..*sz
                    {
                      let  v = self.memory.get_u8(src_addr+(off as u64));

                      self.memory.put_u8(dst_addr+(off as u64),v);
                    }
                }
          NonAllocatingOperation::Message(s)=>
                {
                  println!("[message] {}",s);
                }
          NonAllocatingOperation::Print(target,c)=>
                {
/*
                  let  src_addr = self.get_absolute_address(&target.address_source);

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
*/
                }
            }
        }
    }


  StepResult::Continue
}


pub fn
terminate(&mut self, coll: &Collection, tm: &Terminator)-> StepResult
{
    match tm
    {
  Terminator::Jump(bl)=>
        {
          self.change_bi(bl);

          return StepResult::Continue;
        },
  Terminator::Branch(bi)=>
        {
          let  addr = self.get_absolute_address(&bi.condition);

            if addr != 0
            {
                if self.memory.get_word(addr).get_u64() != 0
                {
                  self.change_bi(&bi.on_true);
                }

              else
                {
                  self.change_bi(&bi.on_false);
                }
            }


          return StepResult::Continue;
        },
  Terminator::Return(o_opt)=>
        {
            if let Some(o) = o_opt
            {
              let  retval_addr = self.memory.get_u64(self.bp);

              self.memory.put_word(retval_addr,self.get_word(o));
            }


          self.remove_frame();

          return StepResult::Continue;
        },
    }
}


pub fn
step(&mut self, coll: &Collection)-> StepResult
{
    if self.calling_depth != 0
    {
//self.print_context();
      let  f = &coll.function_list[self.fi as usize];

      let  blk = &f.block_list[self.bi as usize];

        if (self.pc as usize) < blk.line_list.len()
        {
          let  ln = &blk.line_list[self.pc as usize];

          self.pc += 1;

          return self.operate(coll,ln);
        }

      else
        {
          return self.terminate(coll,&blk.terminator);
        }
    }


  StepResult::NoContinue
}


pub fn
run(&mut self, coll: &Collection, count_opt: Option<usize>)
{
  self.halt_flag = false;

    while !self.halt_flag
    {
        if let Some(mut count) = count_opt
        {
            if count == 0
            {
              break;
            }


          count -= 1;
        }


        if let StepResult::NoContinue = self.step(coll)
        {
          break;
        }
    }
}


pub fn
get_return_value(&self)-> Option<Memory>
{
    if self.calling_depth == 0
    {
      let  sz = self.memory.get_u64(self.bp);

        if sz != 0
        {
          let  mut m = Memory::new(sz as usize);

          m.read(0,&self.memory,self.bp+(WORD_SIZE as u64),Some(sz));

          return Some(m);
        }
    }

  else
    {
      println!("have a no value because now running");
    }


  None
}




pub fn
print_local_variables(&self, coll: &Collection)
{
  let  f = &coll.function_list[self.fi as usize];

    for alo in &f.allocation_list
    {
      let  addr = self.bp+(alo.offset as u64);

      let  i = self.memory.get_u64(addr);

      println!("addr:{}, value:{}",addr,i);
    }
}


pub fn
print_context(&self)
{
  println!("[pc:{}, bp:{}, sp:{}, bi:{}, pbi:{}]",
    self.pc,
    self.bp,
    self.sp,
    self.bi,
    self.pbi,
  );
}


}




