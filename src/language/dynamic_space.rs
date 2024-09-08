

use std::collections::LinkedList;


use super::expression::{
  Expression,
  UnaryOperator,
  BinaryOperator,
  AssignOperator,

};


use super::dynamic_machine::{
  Operation,
  UnaryOperation,
  BinaryOperation,

};


use super::dynamic_dictionary::{
  get_dictionary

};


use super::dynamic_read::{
  read_declaration,

};




#[derive(Clone)]
pub struct
Element
{
  pub(crate) name: String,
  pub(crate) heap_reference: usize,

}


impl
Element
{


pub fn
new(name: &str, r: usize)-> Self
{
  Self{
    name: name.to_string(),
    heap_reference: r,
  }
}


}




pub enum
Declaration
{
  Fn(String,Function),
  Let(String,Option<Expression>),
  Const(String,Expression),

}




pub struct
Symbol
{
  pub(crate) name: String,
  pub(crate) index: usize,
  pub(crate) value: Value,

}


impl
Symbol
{


pub fn
new(name: &str, index: usize)-> Self
{
  Self{
    name: name.to_string(),
    index,
    value: Value::Null,
  }
}


}




pub struct
Const
{
  pub(crate) name: String,
  pub(crate) expression: Expression,
  pub(crate) value: Value,

}


pub struct
Space
{
  pub(crate) const_list: LinkedList<Const>,
  pub(crate)   let_list: Vec<(String,Option<Expression>,Value)>,
  pub(crate)    fn_list: Vec<(String,Function,Vec<Operation>)>,

}


impl
Space
{


pub fn
new()-> Self
{
  Self{
    const_list: LinkedList::new(),
      let_list: Vec::new(),
       fn_list: Vec::new(),
  }
}


pub fn
read(&mut self, s: &str)
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dynamic_dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"declaration",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(d_dir) = cur.get_directory()
        {
            if let Ok(decl) = super::dynamic_read::read_declaration(d_dir)
            {
                match decl
                {
              Declaration::Fn(name,f)=>
                    {
                      self.fn_list.push((name,f,Vec::new()));
                    }
              Declaration::Let(name,e_opt)=>
                    {
                      self.let_list.push((name,e_opt,Value::Null));
                    }
              Declaration::Const(name,e)=>
                    {
                      self.const_list.push_back(Const{name, expression: e, value: Value::Null});
                    }
               }
            }


          cur.advance(1);
        }
    }

  else
    {
      println!("Space::read error: parse is failed");
    }
}


pub fn
find_fn(&self, name: &str)-> Option<&Function>
{
    for i in 0..self.fn_list.len()
    {
      let  (f_name,f,_) = &self.fn_list[i];

        if f_name == name
        {
          return Some(f);
        }
    }


  None
}


pub fn
find_const_value(&self, name: &str)-> Option<&Value>
{
    for c in &self.const_list
    {
        if &c.name == name
        {
          return Some(&c.value);
        }
    }


  None
}


pub fn
calculate(e: &Expression, const_list: &LinkedList<Const>)-> Result<Value,()>
{
  Err(())
}


pub fn
calculate_const_values(&mut self)
{
  let  mut tmp: LinkedList<Const> = LinkedList::new();
  let  mut  ok: LinkedList<Const> = LinkedList::new();
  let  mut err: LinkedList<Const> = LinkedList::new();

  std::mem::swap(&mut self.const_list,&mut tmp);

    while tmp.len() != 0
    {
        while let Some(c) = tmp.pop_back()
        {
            if let Ok(v) = Self::calculate(&c.expression,&ok)
            {
            }
        }


        if err.is_empty()
        {
          break;
        }


      std::mem::swap(&mut err,&mut tmp);
    }


  std::mem::swap(&mut self.const_list,&mut ok);
}


fn
count_name(tbl: &Vec<Symbol>, name: &str)-> usize
{
  let  mut count: usize = 0;

    for sym in tbl
    {
        if sym.name == name
        {
          count += 1;
        }
    }


  count
}


fn
check_name(tbl: &Vec<Symbol>)-> Result<(),()>
{
    for sym in tbl
    {
        if Self::count_name(tbl,&sym.name) != 1
        {
          return Err(());
        }
    }


  Ok(())
}


fn
create_symbol_table(&self)-> Vec<Symbol>
{
  let  mut symtbl: Vec<Symbol> = vec![Symbol::new("",0)];

    for c in &self.const_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(&c.name,i));
    }


    for (name,_,_) in &self.let_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(name,i));
    }


    for (name,_,_) in &self.fn_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(name,i));
    }


  symtbl
}


pub fn
compile(&mut self)-> Result<Vec<Symbol>,()>
{
  let  mut symtbl = self.create_symbol_table();

    if Self::check_name(&symtbl).is_err()
    {
      return Err(());
    }


    for c in &self.const_list
    {
    }


    for (name,_,_) in &self.let_list
    {
    }


    for (name,_,_) in &self.fn_list
    {
    }


  Ok(symtbl)
}


pub fn
print(&self)
{
    for (name,e_opt,_) in &self.let_list
    {
      print!("let  {}",name);

        if let Some(e) = e_opt
        {
          print!(": ");

          e.print();
        }


      print!(";\n");
    }


    for c in &self.const_list
    {
      print!("const  {}: ",&c.name);

      c.expression.print();

      print!(";\n");
    }


    for (name,f,_) in &self.fn_list
    {
      print!("fn  {}(",name);

        for name in &f.parameter_list
        {
          print!("{},",name);
        }


      print!(")\n");

      f.block.print();
    }
}


}




pub enum
Statement
{
  Empty,
  Let(String,Option<Expression>),
  Const(String,Option<Expression>),
  Expression(Expression,Option<(AssignOperator,Expression)>),
  If(Vec<(Expression,Block)>,Option<Block>),
  While(Expression,Block),
  Loop(Block),
  Block(Block),
  Return(Option<Expression>),
  Break,
  Continue,

}


impl
Statement
{


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");}
  Statement::Let(name,e_opt)=>
        {
          print!("let  {}",name);

            if let Some(e) = e_opt
            {
              print!(": ");

              e.print();
            }
        }
  Statement::Const(name,e_opt)=>
        {
          print!("const  {}",name);

            if let Some(e) = e_opt
            {
              print!(": ");

              e.print();
            }
        }
  Statement::Expression(e,ass_opt)=>
        {
          e.print();

            if let Some((ass_op,r)) = ass_opt
            {
              ass_op.print();

              r.print();
            }
        }
  Statement::If(ls,blk_opt)=>
       {
            if let Some((first_e,first_blk)) = ls.first()
            {
              print!("if ");

              first_e.print();

              first_blk.print();

              print!("\n");

                for (e,blk) in ls
                {
                  print!("else if ");

                  e.print();

                  blk.print();

                  print!("\n");
                }
            }


            if let Some(blk) = blk_opt
            {
              print!("else");

              blk.print();
            }
        }
  Statement::While(e,blk)=>
        {
          print!("while ");

          e.print();

          blk.print();
        }
  Statement::Loop(blk)=>
        {
          print!("loop");

          blk.print();
        }
  Statement::Block(blk)=>
        {
          blk.print();
        }
  Statement::Return(e_opt)=>
        {
          print!("return ");

            if let Some(e) = e_opt
            {
              e.print();
            }
        }
  Statement::Break=>{print!("break");}
  Statement::Continue=>{print!("continue");}
    }
}


}




pub struct
Block
{
  pub(crate) statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
print(&self)
{
  print!("{{\n");

    for stmt in &self.statement_list
    {
      stmt.print();

      print!("\n");
    }


  print!("}}\n");
}


}




pub struct
Function
{
  pub(crate) parameter_list: Vec<String>,
  pub(crate) block: Block,

}




#[derive(Clone)]
pub enum
Value
{
  Null,
  Undefined,
  Boolean(bool),
  Integer(i64),
  Floating(f64),
  String(String),

  HeapReference(usize),
  StackReference(usize),
  ProgramReference(usize),

  Mutable(Box<Value>),

  Table(Vec<Element>),

  BasePointer(usize),
  ProgramPointer(*const Vec<Operation>),
  ProgramCounter(usize),
  ArgumentCounter(usize),

}


impl
Value
{


pub fn
to_bool(&self)-> bool
{
    match self
    {
  Value::Boolean(b)=>{return *b;}
  Value::Integer(i)=>{return *i != 0;}
  Value::Floating(f)=>{return *f != 0.0;}
  Value::Mutable(v)=>{return v.to_bool();}
  _=>{}
    }


  false
}


pub fn
to_int(&self)-> i64
{
    match self
    {
  Value::Boolean(b)=>{return if *b{1} else{0};}
  Value::Integer(i)=>{return *i;}
  Value::Floating(f)=>{return *f as i64;}
  Value::Mutable(v)=>{return v.to_int();}
  _=>{}
    }


  0
}


pub fn
to_float(&self)-> f64
{
    match self
    {
  Value::Integer(i)=>{return *i as f64;}
  Value::Floating(f)=>{return *f;}
  Value::Mutable(v)=>{return v.to_float();}
  _=>{}
    }


  0.0
}


pub fn
to_string(&self)-> String
{
    match self
    {
  Value::Boolean(b)=>{return if *b{"true".to_string()} else{"false".to_string()};}
  Value::Integer(i)=>{return format!("{}",*i);}
  Value::Floating(f)=>{return format!("{}",*f);}
  Value::String(s)=>{return s.clone();}
  Value::Mutable(v)=>{return v.to_string();}
  _=>{}
    }


  "".to_string()
}


pub fn
to_program_reference(&self)-> usize
{
    if let Value::ProgramReference(i) = self
    {
      return *i;
    }


  0
}


}




pub struct
BlockFrame
{
  pub(crate) parent_ptr: *mut BlockFrame,
  pub(crate) variable_list: Vec<(bool,String,usize)>,

}


impl
BlockFrame
{


pub fn
new(parent_ptr: *mut Self, blk: &Block, counter: &mut usize)-> Self
{
  let  mut variable_list: Vec<(bool,String,usize)> = Vec::new();

    for stmt in &blk.statement_list
    {
        if let Statement::Let(name,_) = stmt
        {
          variable_list.push((false,name.clone(),*counter));

          *counter += 1;
        }
    }


  Self{
    parent_ptr,
    variable_list,
  }
}


pub fn
finish(&self, counter: &mut usize)-> *mut Self
{
  *counter -= self.variable_list.len();

  self.parent_ptr
}


pub fn
show(&mut self, name: &str)
{
    for (v_visibility,v_name,_) in &mut self.variable_list
    {
        if v_name == name
        {
          *v_visibility = true;

          break;
        }
    }
}


pub fn
find(&self, name: &str)-> Option<usize>
{
    for (v_visibility,v_name,v_index) in &self.variable_list
    {
        if *v_visibility && (v_name == name)
        {
          return Some(*v_index);
        }
    }


    if self.parent_ptr != std::ptr::null_mut()
    {
      return unsafe{&*self.parent_ptr}.find(name);
    }


  None
}


}




pub struct
LabelFrame
{
  pub(crate) parent_ptr: *const Self,
  pub(crate) name: String,

}


impl
LabelFrame
{


pub fn
new(parent_ptr: *const Self, name: &str)-> Self
{
  Self{
    parent_ptr,
    name: name.to_string(),
  }
}


pub fn
finish(&self)-> *const Self
{
  self.parent_ptr
}


}




pub struct
CompileContext<'a,'b>
{
  pub(crate)  variable_list_ref: &'a Vec<String>,
  pub(crate) parameter_list_ref: &'b Vec<String>,

  pub(crate) block_frame_ptr: *mut BlockFrame,

  pub(crate) local_counter: usize,
  pub(crate) conditional_counter: usize,

  pub(crate) operation_list: Vec<Operation>,

  pub(crate) point_list: Vec<(String,usize)>,

  pub(crate) control_label_frame_ptr: *const LabelFrame,

}


impl<'a,'b>
CompileContext<'a,'b>
{


pub fn
start(f_ref: &'b Function, variable_list_ref: &'a Vec<String>)-> Vec<Operation>
{
  let  mut ctx = Self{
    variable_list_ref,
    parameter_list_ref: &f_ref.parameter_list,
    block_frame_ptr: std::ptr::null_mut(),
    operation_list: Vec::new(),
    point_list: Vec::new(),
    local_counter: 0,
    conditional_counter: 0,
    control_label_frame_ptr: std::ptr::null(),
  };


  ctx.process_block(&f_ref.block);

  ctx.operation_list
}


pub fn
push_point(&mut self, name: &str)
{
  let  i = self.operation_list.len();

  self.point_list.push((name.to_string(),i));
}


pub fn
push_unary_operation(&mut self, uo: UnaryOperation)
{
  self.operation_list.push(Operation::Unary(uo));
}


pub fn
push_binary_operation(&mut self, bo: BinaryOperation)
{
  self.operation_list.push(Operation::Binary(bo));
}


pub fn
find_local(&self, name: &str)-> Option<usize>
{
  unsafe{&*self.block_frame_ptr}.find(name)
}


pub fn
find_parameter(&self, name: &str)-> Option<usize>
{
    for i in 0..self.parameter_list_ref.len()
    {
        if &self.parameter_list_ref[i] == name
        {
          return Some(i);
        }
    }


  None
}


pub fn
find_global(&self, name: &str)-> Option<usize>
{
    for i in 0..self.parameter_list_ref.len()
    {
        if &self.parameter_list_ref[i] == name
        {
          return Some(i);
        }
    }


  None
}


pub fn
process_expression(&mut self, expr: &Expression)
{
    match expr
    {
  Expression::Identifier(p)=>
        {
          let  s = p.to_string();

               if s ==      "true"{self.operation_list.push(Operation::LoadB(true));}
          else if s ==     "false"{self.operation_list.push(Operation::LoadB(false));}
          else if s ==      "null"{self.operation_list.push(Operation::LoadN);}
          else if s == "undefined"{self.operation_list.push(Operation::LoadU);}
          else
            if let Some(i) = self.find_local(&s)
            {
              self.operation_list.push(Operation::LoadLoc(i));
            }

          else
            if let Some(i) = self.find_parameter(&s)
            {
              self.operation_list.push(Operation::LoadArg(i));
            }

          else
            if let Some(i) = self.find_global(&s)
            {
              self.operation_list.push(Operation::LoadGlo(i));
            }


          panic!();
        },
  Expression::Boolean(b) =>{self.operation_list.push(Operation::LoadB(*b));},
  Expression::Integer(u) =>{self.operation_list.push(Operation::LoadI(*u as i64));},
  Expression::Floating(f)=>{self.operation_list.push(Operation::LoadF(*f));},
  Expression::String(s)  =>{self.operation_list.push(Operation::LoadS(s.clone()));},
  Expression::SubExpression(e)=>
        {
          self.process_expression(e);
        },
  Expression::Unary(o,e)=>
        {
          self.process_expression(e);

            match o
            {
          UnaryOperator::Neg       =>{self.push_unary_operation(UnaryOperation::Neg);},
          UnaryOperator::Not       =>{self.push_unary_operation(UnaryOperation::Not);},
          UnaryOperator::LogicalNot=>{self.push_unary_operation(UnaryOperation::LogicalNot);},
          _=>{},
            }
        },
  Expression::Call(f,args)=>
        {
        },
  Expression::Subscript(target,index)=>
        {
        },
  Expression::Access(target,name)=>
        {
          self.process_expression(target);
        },
  Expression::Binary(o,l,r)=>
        {
          self.process_expression(l);
          self.process_expression(r);

            match o
            {
          BinaryOperator::Add       =>{self.push_binary_operation(BinaryOperation::Add);},
          BinaryOperator::Sub       =>{self.push_binary_operation(BinaryOperation::Sub);},
          BinaryOperator::Mul       =>{self.push_binary_operation(BinaryOperation::Mul);},
          BinaryOperator::Div       =>{self.push_binary_operation(BinaryOperation::Div);},
          BinaryOperator::Rem       =>{self.push_binary_operation(BinaryOperation::Rem);},
          BinaryOperator::Shl       =>{self.push_binary_operation(BinaryOperation::Shl);},
          BinaryOperator::Shr       =>{self.push_binary_operation(BinaryOperation::Shr);},
          BinaryOperator::And       =>{self.push_binary_operation(BinaryOperation::And);},
          BinaryOperator::Or        =>{self.push_binary_operation(BinaryOperation::Or);},
          BinaryOperator::Xor       =>{self.push_binary_operation(BinaryOperation::Xor);},
          BinaryOperator::Eq        =>{self.push_binary_operation(BinaryOperation::Eq);},
          BinaryOperator::Neq       =>{self.push_binary_operation(BinaryOperation::Neq);},
          BinaryOperator::Lt        =>{self.push_binary_operation(BinaryOperation::Lt);},
          BinaryOperator::Lteq      =>{self.push_binary_operation(BinaryOperation::Lteq);},
          BinaryOperator::Gt        =>{self.push_binary_operation(BinaryOperation::Gt);},
          BinaryOperator::Gteq      =>{self.push_binary_operation(BinaryOperation::Gteq);},
          BinaryOperator::LogicalAnd=>{self.push_binary_operation(BinaryOperation::LogicalAnd);},
          BinaryOperator::LogicalOr =>{self.push_binary_operation(BinaryOperation::LogicalOr);},
            }
        },
    }
}




pub fn
process_block(&mut self, blk: &Block)
{
  let  bf = BlockFrame::new(self.block_frame_ptr,blk,&mut self.local_counter);

    for stmt in &blk.statement_list
    {
      self.process_statement(stmt);
    }


  self.block_frame_ptr = bf.finish(&mut self.local_counter);
}


pub fn
process_statement(&mut self, stmt: &Statement)
{
    match stmt
    {
  Statement::Empty=>{}
  Statement::Let(name,e_opt)=>
        {
          unsafe{&mut *self.block_frame_ptr}.show(name);

            if let Some(e) = e_opt
            {
              self.process_expression(e);
            }
        }
  Statement::Const(name,e_opt)=>
        {
          unsafe{&mut *self.block_frame_ptr}.show(name);

            if let Some(e) = e_opt
            {
              self.process_expression(e);
            }
        }
  Statement::Expression(e,ass_opt)=>
        {
          self.process_expression(e);

          self.operation_list.push(Operation::Dump);
        }
  Statement::If(ls,blk_opt)=>
        {
            for (e,blk) in ls
            {
              self.process_expression(e);

              self.process_block(blk);
            }


            if let Some(blk) = blk_opt
            {
              self.process_block(blk);
            }
        }
  Statement::While(e,blk)=>
        {
          self.process_expression(e);

          self.process_block(blk);
        }
  Statement::Loop(blk)=>
        {
          self.process_block(blk);
        }
  Statement::Block(blk)=>
        {
          self.process_block(blk);
        }
  Statement::Return(e_opt)=>
        {
            if let Some(e) = e_opt
            {
              self.process_expression(e);
            }
        }
  Statement::Break=>{}
  Statement::Continue=>{}
    }
}


}




