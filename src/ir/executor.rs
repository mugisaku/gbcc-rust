

use std::cmp;
use super::block::*;
use super::memory::{
  Memory,
  Word,
  WORD_SIZE,
  get_word_size_aligned,
};


use super::function::{
  Function,
  VariableInfo,
  StorageClass,
};




pub struct
Library
{
  function_list: Vec<Function>,

  variable_info_list: Vec<VariableInfo>,

}


impl
Library
{


pub fn
new()-> Library
{
  Library{ function_list: Vec::new(), variable_info_list: Vec::new()}
}


pub fn
get_variable_info_list(&self)-> &Vec<VariableInfo>
{
  &self.variable_info_list
}


pub fn
get_next_offset(&self)-> i64
{
    if let Some(vi) = self.variable_info_list.last()
    {
      get_word_size_aligned(vi.offset+(vi.size as i64))
    }

  else
    {
      0
    }
}


pub fn
add_function(&mut self, f: Function)
{
  let  fi = self.function_list.len();

  self.allocate_word(&f.name,Word::from_u64(fi as u64));

  self.function_list.push(f);
}


pub fn
find_function(&self, name: &str)-> Option<&Function>
{
    for f in &self.function_list
    {
        if f.name == name
        {
          return Some(f);
        }
    }


  None
}


pub fn
allocate_word(&mut self, name: &str, w: Word)
{
  let  off = self.get_next_offset();

  self.variable_info_list.push(VariableInfo{ name: String::from(name), storage_class: StorageClass::Global, offset: off, size: WORD_SIZE as u64, initial_value: Some(Memory::from_word(w))});
}


pub fn
allocate_memory(&mut self, name: &str, m: Memory)
{
  let  off = self.get_next_offset();

  let  sz = m.get_size();

  self.variable_info_list.push(VariableInfo{ name: String::from(name), storage_class: StorageClass::Global, offset: off, size: sz as u64, initial_value: Some(m)});
}


pub fn
allocate_space(&mut self, name: &str, sz: usize)
{
  let  off = self.get_next_offset();

  self.variable_info_list.push(VariableInfo{ name: String::from(name), storage_class: StorageClass::Global, offset: off, size: sz as u64, initial_value: None});
}


pub fn
find_variable_info(&self, name: &str)-> Option<&VariableInfo>
{
    for vi in &self.variable_info_list
    {
        if vi.name == name
        {
          return Some(vi);
        }
    }


  None
}



}




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

  word_count_list: Vec<WordCount>,

  ap: u64,//ArgumentPointer

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
            word_count_list: Vec::new(),
            ap: 0
          }
}


pub fn
change_bi(&mut self, bl: &BlockLink)
{
    if let Some(i) = bl.index
    {
      self.pbi = self.bi           ;
                 self.bi = i as u64;

      self.pc = 0;
    }

  else
    {
      print!("change_bi error: BlockLink {} has no index",&bl.name);
    }
}


pub fn
get_absolute_address(&self, adr_src_opt: &Option<AddressSource>)-> u64
{
    if let Some(adr_src) = adr_src_opt
    {
        match adr_src
        {
      AddressSource::GlobalOffset(off)=>{*off as u64}
      AddressSource::LocalOffset(off)=> {((self.bp as i64)+(Self::SYSTEM_RESERVED_SIZE as i64)+*off) as u64}
      AddressSource::ArgumentOffset(off)=> {((self.bp as i64)+*off) as u64}
        }
    }

  else
    {
      0
    }
}


pub fn
get_word(&self, o: &Operand)-> Word
{
    if let OperandLiteral::ImmediateValue(v) = &o.literal
    {
      return *v;
    }


  let  addr = self.get_absolute_address(&o.address_source);

    if addr != 0
    {
//self.print_context();
//println!("\naddr: {}",addr);
      self.memory.get_word(addr)
    }

  else
    {
      println!("unkown operand");

      Word::from_u64(0)
    }
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
reset(&mut self, lib: &Library)-> Result<(),()>
{
  self.pc  = 0;
  self.fi  = 0;
  self.bi  = 0;
  self.pbi = 0;
  self.calling_depth = 0;
  self.next_fi = None;
  self.address_of_return_value = 0;
  self.word_count_list.clear();

  self.memory.zerofill();

    for vi in &lib.variable_info_list
    {
        if let Some(m) = &vi.initial_value
        {
            if self.memory.read(vi.offset as u64,m,0,None).is_err()
            {
              return Err(());
            }
        }
    }


            self.bp = lib.get_next_offset() as u64;
  self.sp = self.bp                               ;

  Ok(())
}




pub fn
push_argument(&mut self, lib: &Library, arg: Operand)
{
    if let Some(wc) = self.word_count_list.pop()
    {
//println!("argument is pushed to address {}",self.ap);
//arg.print();
      let  w = self.get_word(&arg);

      self.memory.put_word(self.ap,w);

      self.ap += WORD_SIZE as u64;
    }

  else
    {
      println!("needed no more operand");
    }
}


pub fn
prepare_first_call(&mut self, lib: &Library, fn_name: &str)-> Result<(),()>
{
    if let Some(vi) = lib.find_variable_info(fn_name)
    {
      let  dst_addr = self.sp+(WORD_SIZE as u64);

        if let Some(init_v) = &vi.initial_value
        {
          let  fi = init_v.get_u64(0);

          let  f = &lib.function_list[fi as usize];

          let  sz = f.return_word_count.get_size();

          self.memory.put_u64(self.bp,sz);

          self.sp += (WORD_SIZE as u64)+sz;

          return self.prepare_call(lib,dst_addr,fi);
        }
    }


  Err(())
}


pub fn
prepare_call(&mut self, lib: &Library, dst_addr: u64, f_index: u64)-> Result<(),()>
{
    if (f_index as usize) < lib.function_list.len()
    {
      self.next_fi = Some(f_index);

      let  f = &lib.function_list[f_index as usize];

      self.address_of_return_value = dst_addr;

      self.word_count_list.clear();

        for p in &f.parameter_list
        {
          self.word_count_list.push(p.word_count.clone());
        }


      self.ap = self.sp;

      return Ok(());
    }


  Err(())
}


pub fn
raise_call(&mut self, lib: &Library)-> Result<(),()>
{
    if let Some(fi) = self.next_fi
    {
      let  f = &lib.function_list[fi as usize];

        if self.word_count_list.len() == 0
        {
          self.new_frame(lib);

          return Ok(());
        }

      else
        {
          println!("number of arguments is not enough");
        }
    }

  else
    {
      println!("no function index is set");
    }


  Err(())
}


pub fn
new_frame(&mut self, lib: &Library)
{
    if let Some(fi) = self.next_fi
    {
      let  f = &lib.function_list[fi as usize];

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
      self.sp  = new_bp+(Self::SYSTEM_RESERVED_SIZE as u64)+f.allocation_size;
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
operate_load(&mut self, dst_addr: u64, src_addr: u64, sz: usize)
{
       if sz == 1{self.memory.put_u8( dst_addr,self.memory.get_u8(src_addr));}
  else if sz == 2{self.memory.put_u16(dst_addr,self.memory.get_u16(src_addr));}
  else if sz == 4{self.memory.put_u32(dst_addr,self.memory.get_u32(src_addr));}
  else if sz == 8{self.memory.put_u64(dst_addr,self.memory.get_u64(src_addr));}
  else
    {
        for off in 0..sz
        {
          let  v = self.memory.get_u8(src_addr+(off as u64));

          self.memory.put_u8(dst_addr+(off as u64),v);
        }
    }
}


pub fn
operate_phi(&mut self, dst_addr: u64, opls: &Vec<PhiOperand>)
{
    for o in opls
    {
        if let Some(bi) = o.from.index
        {
            if bi == self.pbi
            {
              self.put_word(dst_addr,self.get_word(&o.value));
            }
        }
    }
}


pub fn
operate_call(&mut self, lib: &Library, dst_addr: u64, ci: &CallInfo)
{
  let  fnptr_addr = self.get_absolute_address(&ci.target.address_source);

  let  fnptr = self.memory.get_u64(fnptr_addr);

  self.prepare_call(lib,dst_addr,fnptr);
}


pub fn
operate(&mut self, lib: &Library, ln: &Line)-> StepResult
{
    match ln
    {
  Line::AllocatingOperation(vl,ao)=>
        {
          let  dst_addr = self.get_absolute_address(&vl.address_source);

            match ao
            {
          AllocatingOperation::Unary(o,u)=>{self.operate_unary(dst_addr,o,*u);},
          AllocatingOperation::Binary(l,r,b)=>{self.operate_binary(dst_addr,l,r,*b);},
          AllocatingOperation::Allocate(wc)=>{},
          AllocatingOperation::Copy(o)=>{self.put_word(dst_addr,self.get_word(o));},
          AllocatingOperation::Load(src,sz)=>{self.operate_load(dst_addr,self.get_absolute_address(&src.address_source),*sz);},
          AllocatingOperation::Address(target)=>{self.put_u64(dst_addr,self.get_absolute_address(&target.address_source));},
          AllocatingOperation::Phi(opls)=>{self.operate_phi(dst_addr,opls);},
          AllocatingOperation::Call(ci)=>{self.operate_call(lib,dst_addr,ci);},
            }
        }
  Line::NonAllocatingOperation(nao)=>
        {
            match nao
            {
          NonAllocatingOperation::Store(dst,src,sz)=>
                {
                  let  dst_addr = self.get_absolute_address(&dst.address_source);

                  self.operate_load(dst_addr,self.get_absolute_address(&src.address_source),*sz);
                }
            }
        }
    }


  StepResult::Continue
}


pub fn
terminate(&mut self, lib: &Library, tm: &Terminator)-> StepResult
{
    match tm
    {
  Terminator::Jump(bl)=>
        {
          self.change_bi(bl);
        },
  Terminator::Branch(bi)=>
        {
          let  addr = self.get_absolute_address(&bi.condition.address_source);

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


  StepResult::NoContinue
}


pub fn
step(&mut self, lib: &Library)-> StepResult
{
    if self.calling_depth != 0
    {
      let  f = &lib.function_list[self.fi as usize];

      let  blk = &f.block_list[self.bi as usize];

        if (self.pc as usize) < blk.line_list.len()
        {
          let  ln = &blk.line_list[self.pc as usize];

          self.pc += 1;

          return self.operate(lib,ln);
        }

      else
        if let Some(t) = &blk.terminator
        {
          return self.terminate(lib,t);
        }
    }


  StepResult::NoContinue
}


pub fn
run(&mut self, lib: &Library, count_opt: Option<usize>)
{
    loop
    {
        if let Some(mut count) = count_opt
        {
            if count == 0
            {
              break;
            }


          count -= 1;
        }


        if let StepResult::NoContinue = self.step(lib)
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
print_local_variables(&self, lib: &Library)
{
  let  f = &lib.function_list[self.fi as usize];

    for vi in &f.variable_info_list
    {
        if vi.offset < 0
        {
          let  addr = ((self.bp as i64)+vi.offset) as u64;

          let  i = self.memory.get_u64(addr);

          println!("addr:{}, value:{}",addr,i);
        }
    }
}


pub fn
print_context(&self)
{
  println!("[pc:{}, bp:{}, sp:{}]",self.pc,self.bp,self.sp);
}


}




