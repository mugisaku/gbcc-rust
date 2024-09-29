

use super::expression::{
  Expression,
  BinaryOperator,
  UnaryOperator,
  AssignOperator,

};


use super::dynamic_space::{
  Space,
  Statement,
  Block,
  Function,
  Symbol,

};


use super::dynamic_value::{
  Element,
  Value,

};




pub enum
Operation
{
  None,

  PrintS(String),
  PrintGlo(usize),
  PrintLoc(usize),
  PrintArg(usize),

  AllocateLoc(usize),

  LoadN,
  LoadU,
  LoadB(bool),
  LoadI(i64),
  LoadF(f64),
  LoadS(String),
  LoadT(Vec<Element>),
  LoadGloRef(usize),
  LoadLocRef(usize),
  LoadArgRef(usize),
  Dump,

  Cal(usize), Ret, RetN, Jmp(usize), Brz(usize), Brnz(usize),

  Unary(  UnaryOperator),
  Binary(BinaryOperator),
  Assign(AssignOperator),

}


impl
Operation
{


pub fn
is_control(&self)-> bool
{
    match self
    {
  Operation::Cal(_)
 |Operation::Ret
 |Operation::Jmp(_)
 |Operation::Brz(_)
 |Operation::Brnz(_)=>{true}
  _=>{false}
    }
}


pub fn
print(&self)
{
    match self
    {
  Operation::None=>{print!("");},

  Operation::PrintS(s)=>{print!("print \"{}\"",s);},
  Operation::PrintGlo(i)=>{print!("print {}",*i);},
  Operation::PrintLoc(i)=>{print!("print {}",*i);},
  Operation::PrintArg(i)=>{print!("print {}",*i);},

  Operation::AllocateLoc(n)=>{print!("allocate l({})",*n);},

  Operation::LoadN=>{print!("ld null");},
  Operation::LoadU=>{print!("ld undefined");},
  Operation::LoadB(b)=>{print!("ld {}",*b);},
  Operation::LoadI(i)=>{print!("ld {}",*i);},
  Operation::LoadF(f)=>{print!("ld {}",*f);},
  Operation::LoadS(s)=>{print!("ld \"{}\"",s);},
  Operation::LoadT(ls)=>{print!("ld [...]");},
  Operation::LoadGloRef(i)=>{print!("ld g({})",*i);},
  Operation::LoadLocRef(i)=>{print!("ld l({})",*i);},
  Operation::LoadArgRef(i)=>{print!("ld a({})",*i);},
  Operation::Dump=>{print!("dmp");},

  Operation::Cal(n)=>{print!("cal {}",n);},
  Operation::Ret=>{print!("ret");},
  Operation::RetN=>{print!("retn");},
  Operation::Jmp(i)=>{print!("jmp {}",*i);},
  Operation::Brz(i)=>{print!("brz {}",*i);},
  Operation::Brnz(i)=>{print!("brnz {}",*i);},

  Operation::Unary(o)=>{o.print_mnemonic();},
  Operation::Binary(o)=>{o.print_mnemonic();},
  Operation::Assign(o)=>
        {
          print!("Assign(");

            if let Some(bo) = o.get_relational_operator()
            {
              bo.print_mnemonic();
            }


          print!(")");
        },
    }
}


}




pub struct
HeapDevice
{
  pub(crate) core: Vec<Value>,
  pub(crate) freed_list: Vec<usize>,

}


impl
HeapDevice
{


pub fn
new()-> Self
{
  Self{
          core: Vec::new(),
    freed_list: Vec::new(),
  }
}


pub fn
allocate(&mut self)-> usize
{
    if let Some(i) = self.freed_list.pop()
    {
      return i;
    }


  let  i = self.core.len();

  self.core.push(Value::Null);

  i
}


pub fn
deallocate(&mut self, i: usize)
{
    if i < self.core.len()
    {
      self.freed_list.push(i);
    }
}


pub fn
get_ref(&self, i: usize)-> &Value
{
  &self.core[i]
}


}




pub struct
StackDevice
{
  pub(crate) core: Vec<Value>,

}


impl
StackDevice
{


pub fn
new()-> Self
{
  Self{
    core: Vec::new(),
  }
}


pub fn
install(&mut self, symtbl: &Vec<Symbol>)-> usize
{
  let  sz = symtbl.len();

  self.core.resize(sz,Value::Null);

    for sym in symtbl
    {
      let  v = &mut self.core[sym.index];

      *v = sym.value.clone();
    }


  println!("{} symbols is allocated",sz);

  sz
}


pub fn
extend(&mut self, n: usize)
{
  let  new_len = self.core.len()+n;

  println!("stack size: {} -> {}",self.core.len(),new_len);

  self.core.resize(new_len,Value::Null);
}


pub fn
get_size(&self)-> usize
{
  self.core.len()
}


pub fn
get_ref(&self, i: usize)-> &Value
{
  &self.core[i]
}


pub fn
get_ptr(&self, i: usize)-> *const Value
{
  unsafe{self.core.as_ptr().add(i)}
}


pub fn
push(&mut self, v: Value)
{
  self.core.push(v);
}


pub fn
pop(&mut self)-> Value
{
    if let Some(v) = self.core.pop()
    {
      return v;
    }


  panic!();
}


pub fn
top(&self)-> &Value
{
  self.core.last().unwrap()
}


pub fn
top_mut(&mut self)-> &mut Value
{
  self.core.last_mut().unwrap()
}


}




const SYSTEM_RESERVED_STACK_SIZE: usize = 4;


pub enum
StepResult
{
  Ok,
  Err,
  Hlt,
  Fin(Value),

}


pub struct
Machine
{
  pub(crate)  heap:  HeapDevice,
  pub(crate) stack: StackDevice,

  pub(crate) operation_list_ptr: *const Vec<Operation>,

  pub(crate) pc: usize,
  pub(crate) bp: usize,

  pub(crate) call_counter: usize,

  pub(crate) debug_flag: bool,

}


impl
Machine
{


pub fn
new()-> Self
{
  Self{
     heap:  HeapDevice::new(),
    stack: StackDevice::new(),
    operation_list_ptr: std::ptr::null(),
    pc: 0,
    bp: 0,
    call_counter: 0,
    debug_flag: true,
  }
}


pub fn
ready_main(&mut self, symtbl: &Vec<Symbol>)
{
    for sym in symtbl
    {
        if &sym.name == "main"
        {
            if let Value::ProgramPointer(ptr) = &sym.value
            {
              self.operation_list_ptr = *ptr;

              self.stack.extend(SYSTEM_RESERVED_STACK_SIZE);

              return;
            }
        }
    }


  panic!();
}


pub fn
setup(&mut self, symtbl: &Vec<Symbol>)
{
  self.pc           = 0;
  self.call_counter = 0;

  self.bp = self.stack.install(symtbl);

  self.ready_main(symtbl);
}




pub fn
get_program_pointer(&self, n: usize)-> Option<*const Vec<Operation>>
{
  let  l = self.stack.get_size();

  let  v = &self.stack.get_ref(l-1-n);

    if let Value::ProgramPointer(pp) = self.dereference_value(v)
    {
      return Some(*pp);
    }


  None
}


fn
dereference_value<'a>(&'a self, v: &'a Value)-> &'a Value
{
    if let Value::HeapReference(i) = v
    {
      self.dereference_value(self.heap.get_ref(*i))
    }

  else
    if let Value::StackReference(i) = v
    {
      self.dereference_value(self.stack.get_ref(*i))
    }

  else
    {
      v
    }
}


pub fn
dereference_top(&self)-> &Value
{
  self.stack.top()
}


pub fn
dereference_two_of_top(&self)-> (&Value,&Value)
{
  let  i = self.stack.get_size();

    if i >= 2
    {
      return (self.dereference_value(unsafe{&*self.stack.get_ptr(i-2)}),
              self.dereference_value(unsafe{&*self.stack.get_ptr(i-1)}));
    }


  panic!();
}


fn
dereference_value_mut_ptr(heap_ptr: *mut Value, stack_ptr: *mut Value, ptr: *mut Value)-> *mut Value
{
    if let Value::HeapReference(i) = unsafe{&*ptr}
    {
      Self::dereference_value_mut_ptr(heap_ptr,stack_ptr,unsafe{heap_ptr.add(*i)})
    }

  else
    if let Value::StackReference(i) = unsafe{&*ptr}
    {
      Self::dereference_value_mut_ptr(heap_ptr,stack_ptr,unsafe{stack_ptr.add(*i)})
    }

  else
    {
      ptr
    }
}


pub fn
dereference_top_mut(&mut self)-> &mut Value
{
  let   heap_ptr =  self.heap.core.as_mut_ptr();
  let  stack_ptr = self.stack.core.as_mut_ptr();

  let  ptr = Self::dereference_value_mut_ptr(heap_ptr,stack_ptr,unsafe{stack_ptr.add(self.stack.get_size()-1)});

  unsafe{&mut *ptr}
}


pub fn
print_value(&self, v: &Value)
{
    if let Value::HeapReference(i) = v
    {
      self.print_value(self.heap.get_ref(*i))
    }

  else
    if let Value::StackReference(i) = v
    {
      self.print_value(self.stack.get_ref(*i))
    }

  else
    {
      v.print();
    }
}


pub fn
cal(&mut self, ac: usize)
{
  let  old_bp = self.bp                        ;
                self.bp = self.stack.get_size();

    if self.debug_flag
    {
      println!("bp: {} -> {}",old_bp,self.bp);
    }


  let  pp = self.get_program_pointer(ac).unwrap();

  self.stack.push(Value::ProgramPointer(self.operation_list_ptr));

  self.operation_list_ptr = pp;

  self.stack.push(Value::ArgumentCounter(ac));
  self.stack.push(Value::ProgramCounter(self.pc));
  self.stack.push(Value::BasePointer(old_bp));

  self.pc = 0;

  self.call_counter += 1;

    if self.debug_flag
    {
      println!("called");
    }
}


pub fn
ret(&mut self, v: Value)-> StepResult
{
    if self.call_counter == 0
    {
      self.operation_list_ptr = std::ptr::null();

      return StepResult::Hlt;
    }


  let      bp = self.bp;
  let  mut sp =      bp;

    if let Value::ProgramPointer(ptr) = self.stack.get_ref(bp)
    {
      self.operation_list_ptr = *ptr;
    }

  else
    {
      panic!();
    }


    if let Value::ArgumentCounter(n) = self.stack.get_ref(bp+1)
    {
      sp -= n;
    }

  else
    {
      panic!();
    }


    if let Value::ProgramCounter(c) = self.stack.get_ref(bp+2)
    {
      self.pc = *c;
    }

  else
    {
      panic!();
    }


    if let Value::BasePointer(p) = self.stack.get_ref(bp+3)
    {
        if self.debug_flag
        {
          println!("bp: {} -> {}",self.bp,*p);
        }


      self.bp = *p;
    }

  else
    {
      panic!();
    }


  self.stack.core.truncate(sp);

  *self.stack.top_mut() = v;

  self.call_counter -= 1;

    if self.debug_flag
    {
      println!("returned");
    }


  StepResult::Ok
}


pub fn
print_pc_change(&self, new_pc: usize)
{
  println!("pc: {} -> {}",self.pc-1,new_pc);
}


pub fn
brz(&mut self, i: usize)
{
  let  v = self.stack.pop();

    if v.to_int() == 0
    {
        if self.debug_flag
        {
          self.print_pc_change(i);
        }


      self.pc = i;
    }
}


pub fn
brnz(&mut self, i: usize)
{
  let  v = self.stack.pop();

    if v.to_int() != 0
    {
        if self.debug_flag
        {
          self.print_pc_change(i);
        }


      self.pc = i;
    }
}


pub fn
operate_unary(&mut self, uo: &UnaryOperator)
{
  let  v = self.stack.pop();

  let  new_v = match uo
    {
  UnaryOperator::Neg=>       {Value::neg(&v)}
  UnaryOperator::Not=>       {Value::not(&v)}
  UnaryOperator::LogicalNot=>{Value::logical_not(&v)}
  _=>{Value::Undefined}
    };


  self.stack.push(new_v);
}


fn
operate_binary_internal(&mut self, bo: &BinaryOperator)-> Value
{
  let  (lv,rv) = self.dereference_two_of_top();

    match bo
    {
  BinaryOperator::Add=>       {Value::add(lv,rv)}
  BinaryOperator::Sub=>       {Value::sub(lv,rv)}
  BinaryOperator::Mul=>       {Value::mul(lv,rv)}
  BinaryOperator::Div=>       {Value::div(lv,rv)}
  BinaryOperator::Rem=>       {Value::rem(lv,rv)}
  BinaryOperator::Shl=>       {Value::shl(lv,rv)}
  BinaryOperator::Shr=>       {Value::shr(lv,rv)}
  BinaryOperator::And=>       {Value::and(lv,rv)}
  BinaryOperator::Or=>        {Value::or(lv,rv)}
  BinaryOperator::Xor=>       {Value::xor(lv,rv)}
  BinaryOperator::LogicalAnd=>{Value::logical_and(lv,rv)}
  BinaryOperator::LogicalOr=> {Value::logical_or(lv,rv)}
  BinaryOperator::Eq=>        {Value::eq(lv,rv)}
  BinaryOperator::Neq=>       {Value::neq(lv,rv)}
  BinaryOperator::Lt=>        {Value::lt(lv,rv)}
  BinaryOperator::Lteq=>      {Value::lteq(lv,rv)}
  BinaryOperator::Gt=>        {Value::gt(lv,rv)}
  BinaryOperator::Gteq=>      {Value::gteq(lv,rv)}
  _=>{Value::Undefined}
    }
}


pub fn
operate_binary(&mut self, bo: &BinaryOperator)
{
  let  v = self.operate_binary_internal(bo);

  let  _ = self.stack.pop();

  *self.stack.top_mut() = v;
}


pub fn
operate_assign(&mut self, ao: &AssignOperator)
{
  let  v =  if let Some(bo) = ao.get_relational_operator(){
      let  v = self.operate_binary_internal(&bo);

      let  _ = self.stack.pop();

      v
    }

  else
    {
      self.stack.pop()
    };


  *self.dereference_top_mut() = v;
}


pub fn
step(&mut self)-> StepResult
{
    if self.operation_list_ptr == std::ptr::null()
    {
      return StepResult::Hlt;
    }


  let  pc = self.pc;

  let  operation_list = unsafe{&*self.operation_list_ptr};

    if pc >= operation_list.len()
    {
      return self.ret(Value::Null);
    }


  self.pc += 1;

    match unsafe{operation_list.get_unchecked(pc)}
    {
  Operation::None=>{},
  Operation::PrintS(s)=>{println!("[machine print] {}",s);},
  Operation::PrintGlo(i)=>
        {
          let  v = Value::StackReference(*i);

          print!("[machine print] ");

          self.print_value(&v);

          println!("");
        },
  Operation::PrintLoc(i)=>
        {
          let  v = Value::StackReference(self.bp+SYSTEM_RESERVED_STACK_SIZE+*i);

          print!("[machine print] ");

          self.print_value(&v);

          println!("");
        },
  Operation::PrintArg(i)=>
        {
          let  v = Value::StackReference(self.bp-1-*i);

          print!("[machine print] ");

          self.print_value(&v);

          println!("");
        },
  Operation::AllocateLoc(n)=>{self.stack.extend(*n);},
  Operation::LoadN=>{self.stack.push(Value::Null);},
  Operation::LoadU=>{self.stack.push(Value::Undefined);},
  Operation::LoadB(b)=>{self.stack.push(Value::Boolean(*b));},
  Operation::LoadI(i)=>{self.stack.push(Value::Integer(*i));},
  Operation::LoadF(f)=>{self.stack.push(Value::Floating(*f));},
  Operation::LoadS(s)=>{self.stack.push(Value::String(s.clone()));},
  Operation::LoadT(ls)=>{self.stack.push(Value::Table(ls.clone()));},
  Operation::LoadGloRef(i)=>{self.stack.push(Value::StackReference(                                   *i));},
  Operation::LoadLocRef(i)=>{self.stack.push(Value::StackReference(self.bp+SYSTEM_RESERVED_STACK_SIZE+*i));},
  Operation::LoadArgRef(i)=>{self.stack.push(Value::StackReference(self.bp                         -1-*i));},
  Operation::Dump=>{let  _ = self.stack.pop();},
  Operation::Cal(n)=>{self.cal(*n);},
  Operation::Ret=>{  let  v = self.stack.pop();  return self.ret(v);},
  Operation::RetN=>{return self.ret(Value::Null);},
  Operation::Jmp(i)=>
        {
            if self.debug_flag
            {
              self.print_pc_change(*i);
            }


          self.pc = *i;
        },
  Operation::Brz(i)=>{self.brz(*i);},
  Operation::Brnz(i)=>{self.brnz(*i);},
  Operation::Unary(uo)=> {self.operate_unary(uo);}
  Operation::Binary(bo)=>{self.operate_binary(bo);}
  Operation::Assign(ao)=>{self.operate_assign(ao);}
    }


  StepResult::Ok
}


pub fn
run(&mut self, limit_opt: Option<usize>)
{
    if let Some(mut limit) = limit_opt
    {
        if limit != 0
        {
            while let StepResult::Ok = self.step()
            {
              limit -= 1;

                if limit == 0
                {
                  break;
                }
            }
        }
    }

  else
    {
        while let StepResult::Ok = self.step()
        {
        }
    }
}


}




