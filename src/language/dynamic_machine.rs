

use super::expression::{
  Expression,
  BinaryOperator,
  UnaryOperator,
};


use super::dynamic_space::{
  Space,
  Statement,
  Block,
  Function,

};


use super::dynamic_value::{
  Element,
  Value,

};




#[derive(Clone)]
pub enum
UnaryOperation
{
  Neg,
  Not,
  LogicalNot,

}


#[derive(Clone)]
pub enum
BinaryOperation
{
  Add, Sub, Mul, Div, Rem,
  Shl, Shr, And, Or, Xor,

  LogicalAnd, LogicalOr,

  Eq, Neq, Lt, Lteq, Gt, Gteq,

}


pub enum
Operation
{
  None,

  LoadN,
  LoadU,
  LoadB(bool),
  LoadI(i64),
  LoadF(f64),
  LoadS(String),
  LoadGlo(usize),
  LoadLoc(usize),
  LoadArg(usize),
  StoreGlo(usize),
  StoreLoc(usize),
  StoreArg(usize),
  Dump,

  Cal, Ret, Jmp(usize), Brz(usize), Brnz(usize),

  Unary(  UnaryOperation),
  Binary(BinaryOperation),

}




pub struct
Machine
{
  pub(crate) null_value: Value,

  pub(crate)  heap: Vec<Value>,
  pub(crate) stack: Vec<Value>,

  pub(crate) freed_list: Vec<usize>,

  pub(crate) function_list_ptr: *const Vec<(String,Function)>,
  pub(crate) operation_list_ptr: *const Vec<Operation>,

  pub(crate) pc: usize,
  pub(crate) bp: usize,

}


impl
Machine
{


pub fn
new()-> Self
{
  Self{
    null_value: Value::Null,
     heap: Vec::new(),
    stack: Vec::new(),
    freed_list: Vec::new(),
     function_list_ptr: std::ptr::null(),
    operation_list_ptr: std::ptr::null(),
    pc: 0,
    bp: 0,
  }
}


pub fn
allocate(&mut self)-> usize
{
    if let Some(i) = self.freed_list.pop()
    {
      return i;
    }


  let  i = self.stack.len();

  self.stack.push(Value::Null);

  i
}


pub fn
deallocate(&mut self, i: usize)
{
    if i < self.stack.len()
    {
      self.freed_list.push(i);
    }
}


pub fn
push(&mut self, v: Value)
{
  self.stack.push(v);
}


pub fn
pop(&mut self)-> Value
{
    if let Some(v) = self.stack.pop()
    {
      return v;
    }


  Value::Null
}


pub fn
get_heap_value(&self, i: usize)-> &Value
{
  &self.heap[i]
}


pub fn
get_heap_value_mut(&mut self, i: usize)-> &Value
{
  &mut self.heap[i]
}


pub fn
get_stack_value(&self, i: usize)-> &Value
{
  &self.stack[i]
}


pub fn
get_stack_value_mut(&mut self, i: usize)-> &Value
{
  &mut self.stack[i]
}


pub fn
store(&mut self, i: usize)
{
    if let Some(v) = self.stack.pop()
    {
      self.stack[i] = v;
    }
}


pub fn
dereference<'a>(v: &'a Value, heap: &'a Vec<Value>, stack: &'a Vec<Value>)-> &'a Value
{
    if let Value::HeapReference(i) = v
    {
      &heap[*i]
    }

  else
    if let Value::StackReference(i) = v
    {
      &stack[*i]
    }

  else
    {
      v
    }
}


pub fn
cal(&mut self)
{
  self.push(Value::ArgumentCounter(0));
  self.push(Value::ProgramPointer(self.operation_list_ptr));
  self.push(Value::ProgramCounter(self.pc));
  self.push(Value::BasePointer(self.bp));
}


pub fn
ret(&mut self)
{
  let      bp = self.bp;
  let  mut sp =      bp;

    if let Value::ArgumentCounter(n) = self.stack[bp]
    {
      sp -= n;
    }


    if let Value::ProgramPointer(ptr) = self.stack[bp+1]
    {
      self.operation_list_ptr = ptr;
    }


    if let Value::ProgramCounter(c) = self.stack[bp+2]
    {
      self.pc = c;
    }


    if let Value::BasePointer(p) = self.stack[bp+3]
    {
      self.bp = p;
    }


    while self.stack.len() >= sp
    {
      let  _ = self.stack.pop();
    }
}


pub fn
brz(&mut self, i: usize)
{
  let  v = self.pop();

    if v.to_int() == 0
    {
      self.pc = i;
    }
}


pub fn
brnz(&mut self, i: usize)
{
  let  v = self.pop();

    if v.to_int() != 0
    {
      self.pc = i;
    }
}


pub fn
operate_unary(&mut self, uo: UnaryOperation)
{
  let  v = self.pop();

  let  new_v = match uo
    {
  UnaryOperation::Neg=>       {Value::neg(&v)}
  UnaryOperation::Not=>       {Value::not(&v)}
  UnaryOperation::LogicalNot=>{Value::logical_not(&v)}
    };


  self.stack.push(new_v);
}


pub fn
operate_binary(&mut self, bo: BinaryOperation)
{
  let  rv_tmp = self.pop();
  let  lv_tmp = self.pop();

  let  rv = Self::dereference(&rv_tmp,&self.heap,&self.stack);
  let  lv = Self::dereference(&lv_tmp,&self.heap,&self.stack);

  let  new_v = match bo
    {
  BinaryOperation::Add=>       {Value::add(lv,rv)}
  BinaryOperation::Sub=>       {Value::sub(lv,rv)}
  BinaryOperation::Mul=>       {Value::mul(lv,rv)}
  BinaryOperation::Div=>       {Value::div(lv,rv)}
  BinaryOperation::Rem=>       {Value::rem(lv,rv)}
  BinaryOperation::Shl=>       {Value::shl(lv,rv)}
  BinaryOperation::Shr=>       {Value::shr(lv,rv)}
  BinaryOperation::And=>       {Value::and(lv,rv)}
  BinaryOperation::Or=>        {Value::or(lv,rv)}
  BinaryOperation::Xor=>       {Value::xor(lv,rv)}
  BinaryOperation::LogicalAnd=>{Value::logical_and(lv,rv)}
  BinaryOperation::LogicalOr=> {Value::logical_or(lv,rv)}
  BinaryOperation::Eq=>        {Value::eq(lv,rv)}
  BinaryOperation::Neq=>       {Value::neq(lv,rv)}
  BinaryOperation::Lt=>        {Value::lt(lv,rv)}
  BinaryOperation::Lteq=>      {Value::lteq(lv,rv)}
  BinaryOperation::Gt=>        {Value::gt(lv,rv)}
  BinaryOperation::Gteq=>      {Value::gteq(lv,rv)}
    };


  self.stack.push(new_v);
}


pub fn
step(&mut self)
{
  let  pc = self.pc;

  self.pc += 1;

  let  operation_list = unsafe{&*self.operation_list_ptr};

    match &operation_list[pc]
    {
  Operation::None=>{},
  Operation::LoadN=>{self.push(Value::Null);},
  Operation::LoadU=>{self.push(Value::Undefined);},
  Operation::LoadB(b)=>{self.push(Value::Boolean(*b));},
  Operation::LoadI(i)=>{self.push(Value::Integer(*i));},
  Operation::LoadF(f)=>{self.push(Value::Floating(*f));},
  Operation::LoadS(s)=>{self.push(Value::String(s.clone()));},
  Operation::LoadGlo(i)=>{self.push(Value::StackReference(          *i));},
  Operation::LoadLoc(i)=>{self.push(Value::StackReference(self.bp+4+*i));},
  Operation::LoadArg(i)=>{self.push(Value::StackReference(self.bp  -*i));},
  Operation::StoreGlo(i)=>{self.store(          *i);},
  Operation::StoreLoc(i)=>{self.store(self.bp+4+*i);},
  Operation::StoreArg(i)=>{self.store(self.bp  -*i);},
  Operation::Dump=>{let  _ = self.stack.pop();},
  Operation::Cal=>{self.cal();},
  Operation::Ret=>{self.ret();},
  Operation::Jmp(i)=>{self.pc = *i;},
  Operation::Brz(i)=>{self.brz(*i);},
  Operation::Brnz(i)=>{self.brnz(*i);},
  Operation::Unary(uo)=> {self.operate_unary(uo.clone());}
  Operation::Binary(bo)=>{self.operate_binary(bo.clone());}
    }
}


}




