

use super::expression::{
  Expression,
  UnaryOperator,
  BinaryOperator,
  AssignOperator,

};


use super::dynamic_machine::{
  Operation,

};


use super::dynamic_dictionary::{
  get_dictionary

};


use super::dynamic_read::{
  read_declaration,

};


use super::dynamic_value::{
  Value,
  Element,

};




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

  pub(crate) ro_flag: bool,

}


impl
Symbol
{


pub fn
new(name: &str, index: usize, ro_flag: bool)-> Self
{
  Self{
    name: name.to_string(),
    index,
    value: Value::Null,
    ro_flag,
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
Var
{
  pub(crate) name: String,
  pub(crate) expression_opt: Option<Expression>,
  pub(crate) value: Value,

}


pub struct
Space
{
  pub(crate) const_list: Vec<Const>,
  pub(crate)   let_list: Vec<Var>,
  pub(crate)    fn_list: Vec<(String,Function,Vec<Operation>)>,

}


impl
Space
{


pub fn
new()-> Self
{
  Self{
    const_list: Vec::new(),
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
                      self.let_list.push(Var{name, expression_opt: e_opt, value: Value::Null});
                    }
              Declaration::Const(name,e)=>
                    {
                      self.const_list.push(Const{name, expression: e, value: Value::Null});
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


fn
find_const(const_list: &Vec<Const>, name: &str)-> Option<Value>
{
    for c in const_list
    {
        if c.name == name
        {
          return Some(c.value.clone());
        }
    }


  None
}


pub fn
calculate_unary(o: &UnaryOperator, v: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  UnaryOperator::Neg=>{Value::neg(v)},
  UnaryOperator::Not=>{Value::not(v)},
  UnaryOperator::LogicalNot=>{Value::logical_not(v)},
  _=>{Value::Undefined},
    }
}


pub fn
calculate_binary(o: &BinaryOperator, lv: &Value, rv: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  BinaryOperator::Add=>{Value::add(lv,rv)},
  BinaryOperator::Sub=>{Value::sub(lv,rv)},
  BinaryOperator::Mul=>{Value::mul(lv,rv)},
  BinaryOperator::Div=>{Value::div(lv,rv)},
  BinaryOperator::Rem=>{Value::rem(lv,rv)},
  BinaryOperator::Shl=>{Value::shl(lv,rv)},
  BinaryOperator::Shr=>{Value::shr(lv,rv)},
  BinaryOperator::And=>{Value::and(lv,rv)},
  BinaryOperator::Or=>{Value::or(lv,rv)},
  BinaryOperator::Xor=>{Value::xor(lv,rv)},
  BinaryOperator::Eq=>{Value::eq(lv,rv)},
  BinaryOperator::Neq=>{Value::neq(lv,rv)},
  BinaryOperator::Lt=>{Value::lt(lv,rv)},
  BinaryOperator::Lteq=>{Value::lteq(lv,rv)},
  BinaryOperator::Gt=>{Value::gt(lv,rv)},
  BinaryOperator::Gteq=>{Value::gteq(lv,rv)},
  BinaryOperator::LogicalAnd=>{Value::logical_and(lv,rv)},
  BinaryOperator::LogicalOr=>{Value::logical_or(lv,rv)},
    }
}


pub fn
calculate(e: &Expression, const_list: &Vec<Const>)-> Result<Value,()>
{
    match e
    {
  Expression::Identifier(s)=>
        {
               if s == "true"{return Ok(Value::Boolean(true));}
          else if s == "false"{return Ok(Value::Boolean(false));}
          else if s == "null"{return Ok(Value::Null);}
          else if s == "undefined"{return Ok(Value::Undefined);}
          else
            if let Some(v) = Self::find_const(const_list,s)
            {
              return Ok(v);
            }
        },
  Expression::Boolean(b)=>{return Ok(Value::Boolean(*b));},
  Expression::Integer(u)=>{return Ok(Value::Integer(*u as i64));},
  Expression::Floating(f)=>{return Ok(Value::Floating(*f));},
  Expression::String(s)=>{return Ok(Value::String(s.clone()));},
  Expression::SubExpression(sube)=>
        {
          return Self::calculate(sube,const_list);
        },
  Expression::Unary(o,e)=>
        {
            if let Ok(v) = Self::calculate(e,const_list)
            {
              return Ok(Self::calculate_unary(o,&v,const_list));
            }
        },
  Expression::Call(f,args)=>
        {
          panic!();
        },
  Expression::Subscript(target,index)=>
        {
          panic!();
        },
  Expression::Access(target,name)=>
        {
          panic!();
        },
  Expression::Binary(o,l,r)=>
        {
            if let Ok(lv) = Self::calculate(l,const_list)
            {
                if let Ok(rv) = Self::calculate(r,const_list)
                {
                  return Ok(Self::calculate_binary(o,&lv,&rv,const_list));
                }
            }
        },
  _=>{}
    }


  Err(())
}


pub fn
calculate_const_values(const_list: &mut Vec<Const>)
{
  let  mut tmp: Vec<Const> = Vec::new();
  let  mut  ok: Vec<Const> = Vec::new();
  let  mut err: Vec<Const> = Vec::new();

  let  mut last_err_n = 0;

  std::mem::swap(const_list,&mut tmp);

    while tmp.len() != 0
    {
        while let Some(mut c) = tmp.pop()
        {
            if let Ok(v) = Self::calculate(&c.expression,&ok)
            {
              c.value = v;

              ok.push(c);
            }

          else
            {
              err.push(c);
            }
        }


        if err.is_empty()
        {
          break;
        }


      let  err_n = err.len();

        if last_err_n == err_n
        {
          panic!();
        }


      last_err_n = err_n;

      std::mem::swap(&mut err,&mut tmp);
    }


  std::mem::swap(const_list,&mut ok);
}


pub fn
calculate_let_values(let_list: &mut Vec<Var>, const_list: &Vec<Const>)
{
    for var in let_list
    {
        if let Some(e) = &var.expression_opt
        {
            if let Ok(v) = Self::calculate(e,const_list)
            {
              var.value = v;
            }

          else
            {
              panic!();
            }
        }
    }
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
check_name(tbl: &Vec<Symbol>)
{
    for sym in tbl
    {
        if Self::count_name(tbl,&sym.name) != 1
        {
          panic!();
        }
    }
}


fn
create_symbol_table(&self)-> Vec<Symbol>
{
  let  mut symtbl: Vec<Symbol> = vec![Symbol::new("",0,true)];

    for c in &self.const_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(&c.name,i,true));
    }


    for v in &self.let_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(&v.name,i,false));
    }


    for (name,_,_) in &self.fn_list
    {
      let  i = symtbl.len();

      symtbl.push(Symbol::new(name,i,true));
    }


  symtbl
}


pub fn
compile(&mut self)-> Vec<Symbol>
{
  let  mut symtbl = self.create_symbol_table();

  Self::check_name(&symtbl);

  Self::calculate_const_values(&mut self.const_list);
  Self::calculate_let_values(&mut self.let_list,&self.const_list);

    for (name,f,op_ls) in &mut self.fn_list
    {
      *op_ls = CompileContext::start(f,&symtbl);

        for sym in &mut symtbl
        {
            if &sym.name == name
            {
              sym.value = Value::ProgramPointer(op_ls as *const Vec<Operation>);

              break;
            }
        }
    }


  symtbl
}


pub fn
print(&self)
{
    for v in &self.let_list
    {
      print!("let  {}",&v.name);

        if let Some(e) = &v.expression_opt
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


pub fn
print_operations(&self)
{
    for (name,f,op_ls) in &self.fn_list
    {
      print!("fn  {}(",name);

        for name in &f.parameter_list
        {
          print!("{},",name);
        }


      print!(")\n");

        for i in 0..op_ls.len()
        {
          print!("[{:>4}]  ",i);

          let  op = &op_ls[i];

            if op.is_control()
            {
              print!("*");
            }

          else
            {
              print!(" ");
            }


          op.print();

          print!("\n");
        }


      print!("\n");
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
  For(String,Expression,Block),
  Loop(Block),
  Block(Block),
  Return(Option<Expression>),
  Break,
  Continue,
  PrintS(String),
  PrintV(String),

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

                for i in 1..ls.len()
                {
                  let  (e,blk) = &ls[i];

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
  Statement::For(name,e,blk)=>
        {
          print!("for {} in ",name);

          e.print();

          blk.print();
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
          print!("//plain block");

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
  Statement::PrintS(s)=>{print!("print \"{}\"",s);}
  Statement::PrintV(s)=>{print!("print {}",s);}
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


impl
Function
{


}




pub struct
BlockFrame<'a>
{
  pub(crate) variable_list: Vec<(bool,String,usize)>,
  pub(crate) next_index: usize,
  pub(crate) parent_ref_opt: Option<&'a Self>,

}


impl<'a>
BlockFrame<'a>
{


pub fn
new(blk: &Block, parent_ref_opt: Option<&'a Self>)-> Self
{
  let  mut next_index: usize = if let Some(parent_ref) = parent_ref_opt{
    parent_ref.next_index
  } else{0};

  let  mut variable_list: Vec<(bool,String,usize)> = Vec::new();

    for stmt in &blk.statement_list
    {
        if let Statement::Let(name,_) = stmt
        {
          variable_list.push((false,name.clone(),next_index));

          next_index += 1;
        }

      else
        if let Statement::Const(name,_) = stmt
        {
          variable_list.push((true,name.clone(),next_index));

          next_index += 1;
        }
    }


  Self{
    variable_list,
    next_index,
    parent_ref_opt,
  }
}


pub fn
show(&mut self, name: &str)-> Option<usize>
{
    for (v_visibility,v_name,v_index) in &mut self.variable_list
    {
        if v_name == name
        {
          *v_visibility = true;

          return Some(*v_index);
        }
    }


  None
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


    if let Some(parent_ref) = self.parent_ref_opt
    {
      return parent_ref.find(name);
    }


  None
}


pub fn
print(&self)
{
    for (v_visibility,v_name,v_index) in &self.variable_list
    {
      let  s = if *v_visibility{"+"} else{"-"};

      println!("{}{}({})",&s,v_name,v_index);
    }


    if let Some(parent_ref) = self.parent_ref_opt
    {
      parent_ref.print();
    }
}


}




pub struct
ControlBlockFrame<'a>
{
  pub(crate) name: String,
  pub(crate) id: usize,
  pub(crate) parent_ref_opt: Option<&'a Self>,

}


impl<'a>
ControlBlockFrame<'a>
{


pub fn
new(base_name: &str, parent_ref_opt: Option<&'a Self>)-> Self
{
  let  id = if let Some(parent_ref) = parent_ref_opt{
    parent_ref.id+1
  } else{0};


  let  name = format!("{}_{}",base_name,id);

  Self{
    name,
    id,
    parent_ref_opt,
  }
}


pub fn
get_label(&self, suffix: &str)-> String
{
  format!("{}{}",&self.name,suffix)
}


}




pub struct
Position
{
  pub(crate) name: String,
  pub(crate) value: usize,

	}




pub struct
CompileContext<'a,'b>
{
  pub(crate)   symbol_table_ref: &'a Vec<Symbol>,
  pub(crate) parameter_list_ref: &'b Vec<String>,

  pub(crate) operation_list: Vec<Operation>,

  pub(crate) if_id: usize,
  pub(crate) index_max: usize,

  pub(crate) position_request_list: Vec<Position>,
  pub(crate)         position_list: Vec<Position>,

}


impl<'a,'b>
CompileContext<'a,'b>
{


pub fn
start(f_ref: &'b Function, symbol_table_ref: &'a Vec<Symbol>)-> Vec<Operation>
{
  let  mut ctx = Self{
    symbol_table_ref,
    parameter_list_ref: &f_ref.parameter_list,
    operation_list: Vec::new(),
    if_id: 0,
    index_max: 0,
    position_request_list: Vec::new(),
    position_list: Vec::new(),
  };


  ctx.operation_list.push(Operation::AllocateLoc(0));

  ctx.process_block(&f_ref.block,None,None);

    if let Some(first) = ctx.operation_list.first_mut()
    {
         if let Operation::AllocateLoc(n) = first
         {
           *n = ctx.index_max;
         }
    }


  Self::resolve_position_requests_all(&ctx.position_request_list,&ctx.position_list,&mut ctx.operation_list);

  ctx.operation_list
}


pub fn
resolve_position_request(name: &str, pos_ls: &Vec<Position>, op: &mut Operation)
{
    for pos in pos_ls
    {
        if &pos.name == name
        {
            match op
            {
          Operation::Jmp(i) =>{*i = pos.value;}
          Operation::Brz(i) =>{*i = pos.value;}
          Operation::Brnz(i)=>{*i = pos.value;}
          _=>{panic!();}
            }


          return;
        }
    }


  panic!();
}


pub fn
resolve_position_requests_all(posreq_ls: &Vec<Position>, pos_ls: &Vec<Position>, op_ls: &mut Vec<Operation>)
{
    for posreq in posreq_ls
    {
      let  op = &mut op_ls[posreq.value];

      Self::resolve_position_request(&posreq.name,pos_ls,op);
    }
}


pub fn
push_jmp(&mut self, name: String)
{
  let  i = self.operation_list.len();

  self.operation_list.push(Operation::Jmp(0));

  self.position_request_list.push(Position{name, value: i});
}


pub fn
push_brz(&mut self, name: String)
{
  let  i = self.operation_list.len();

  self.operation_list.push(Operation::Brz(0));

  self.position_request_list.push(Position{name, value: i});
}


pub fn
push_brnz(&mut self, name: String)
{
  let  i = self.operation_list.len();

  self.operation_list.push(Operation::Brnz(0));

  self.position_request_list.push(Position{name, value: i});
}


pub fn
push_position(&mut self, name: String)
{
  let  i = self.operation_list.len();

  self.position_list.push(Position{name, value: i});
}


pub fn
find_parameter(&self, name: &str)-> Option<usize>
{
  let  l = self.parameter_list_ref.len();

    for i in 0..l
    {
      let  ii = l-1-i;

        if &self.parameter_list_ref[ii] == name
        {
          return Some(ii);
        }
    }


  None
}


pub fn
find_global(&self, name: &str)-> Option<usize>
{
    for sym in self.symbol_table_ref
    {
        if &sym.name == name
        {
          return Some(sym.index);
        }
    }


  None
}


pub fn
process_expression(&mut self, expr: &Expression, bf_ref: &BlockFrame)
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
            if let Some(i) = bf_ref.find(&s)
            {
              self.operation_list.push(Operation::LoadLocRef(i));
            }

          else
            if let Some(i) = self.find_parameter(&s)
            {
              self.operation_list.push(Operation::LoadArgRef(i));
            }

          else
            if let Some(i) = self.find_global(&s)
            {
              self.operation_list.push(Operation::LoadGloRef(i));
            }

          else
            {
              println!("process_expression error: {} not found",s);

              bf_ref.print();

              panic!();
            }
        },
  Expression::Boolean(b) =>{self.operation_list.push(Operation::LoadB(*b));},
  Expression::Integer(u) =>{self.operation_list.push(Operation::LoadI(*u as i64));},
  Expression::Floating(f)=>{self.operation_list.push(Operation::LoadF(*f));},
  Expression::String(s)  =>{self.operation_list.push(Operation::LoadS(s.clone()));},
  Expression::SubExpression(e)=>
        {
          self.process_expression(e,bf_ref);
        },
  Expression::Unary(o,e)=>
        {
          self.process_expression(e,bf_ref);

          self.operation_list.push(Operation::Unary(o.clone()));
        },
  Expression::Call(f,args)=>
        {
          self.process_expression(f,bf_ref);

            for a in args.iter().rev()
            {
              self.process_expression(a,bf_ref);
            }


          self.operation_list.push(Operation::Cal(args.len()));
        },
  Expression::Subscript(target,index)=>
        {
        },
  Expression::Access(target,name)=>
        {
          self.process_expression(target,bf_ref);
        },
  Expression::Binary(o,l,r)=>
        {
          self.process_expression(l,bf_ref);
          self.process_expression(r,bf_ref);

          self.operation_list.push(Operation::Binary(o.clone()));
        },
    }
}




pub fn
process_block(&mut self, blk: &Block, parent_bf_ref_opt: Option<&BlockFrame>, cbf_ref_opt: Option<&ControlBlockFrame>)
{
  let  mut bf = BlockFrame::new(blk,parent_bf_ref_opt);

  self.index_max = std::cmp::max(self.index_max,bf.next_index);

    for stmt in &blk.statement_list
    {
      self.process_statement(stmt,&mut bf,cbf_ref_opt);
    }
}


pub fn
process_statement(&mut self, stmt: &Statement, bf_ref: &mut BlockFrame, cbf_ref_opt: Option<&ControlBlockFrame>)
{
    match stmt
    {
  Statement::Empty=>{}
  Statement::Let(name,e_opt)=>
        {
            if let Some(i) = bf_ref.show(name)
            {
                if let Some(e) = e_opt
                {
                  self.operation_list.push(Operation::LoadLocRef(i));

                  self.process_expression(e,bf_ref);

                  self.operation_list.push(Operation::Assign(AssignOperator::Nop));
                }
            }
        }
  Statement::Const(name,e_opt)=>
        {
            if let Some(i) = bf_ref.show(name)
            {
                if let Some(e) = e_opt
                {
                  self.operation_list.push(Operation::LoadLocRef(i));

                  self.process_expression(e,bf_ref);

                  self.operation_list.push(Operation::Assign(AssignOperator::Nop));
                }
            }
        }
  Statement::Expression(e,ass_opt)=>
        {
          self.process_expression(e,bf_ref);

            if let Some((o,re)) = ass_opt
            {
              self.process_expression(re,bf_ref);

              self.operation_list.push(Operation::Assign(o.clone()));
            }


          self.operation_list.push(Operation::Dump);
        }
  Statement::If(ls,blk_opt)=>
        {
          let  base_name = format!("If{}",self.if_id);

          self.if_id += 1;

            for i in 0..ls.len()
            {
              let  (e,blk) = &ls[i];

              self.process_expression(e,bf_ref);

              self.push_brnz(format!("{}_{}",&base_name,i));
            }


            if let Some(blk) = blk_opt
            {
              self.process_block(blk,Some(bf_ref),cbf_ref_opt);
            }


          self.push_jmp(format!("{}_End",&base_name));

            for i in 0..ls.len()
            {
              let  (e,blk) = &ls[i];

              self.push_position(format!("{}_{}",&base_name,i));

              self.process_block(blk,Some(bf_ref),cbf_ref_opt);

              self.push_jmp(format!("{}_End",&base_name));
            }


          self.push_position(format!("{}_End",&base_name));
        }
  Statement::For(name,e,blk)=>
        {
          let  cbf = ControlBlockFrame::new("For",cbf_ref_opt);

          self.push_position(cbf.get_label("_Start"));

          self.process_expression(e,bf_ref);

          self.push_brnz(cbf.get_label("_End"));

          self.process_block(blk,Some(bf_ref),Some(&cbf));

          self.push_jmp(cbf.get_label("_Start"));

          self.push_position(cbf.get_label("_End"));
        }
  Statement::While(e,blk)=>
        {
          let  cbf = ControlBlockFrame::new("While",cbf_ref_opt);

          self.push_position(cbf.get_label("_Start"));

          self.process_expression(e,bf_ref);

          self.push_brz(cbf.get_label("_End"));

          self.process_block(blk,Some(bf_ref),Some(&cbf));

          self.push_jmp(cbf.get_label("_Start"));

          self.push_position(cbf.get_label("_End"));
        }
  Statement::Loop(blk)=>
        {
          let  cbf = ControlBlockFrame::new("Loop",cbf_ref_opt);

          self.push_position(cbf.get_label("_Start"));

          self.process_block(blk,Some(bf_ref),Some(&cbf));

          self.push_jmp(cbf.get_label("_Start"));

          self.push_position(cbf.get_label("_End"));
        }
  Statement::Block(blk)=>
        {
          self.process_block(blk,Some(bf_ref),cbf_ref_opt);
        }
  Statement::Return(e_opt)=>
        {
            if let Some(e) = e_opt
            {
              self.process_expression(e,bf_ref);

              self.operation_list.push(Operation::Ret);
            }

          else
            {
              self.operation_list.push(Operation::RetN);
            }
        }
  Statement::Break=>
        {
            if let Some(cbf_ref) = cbf_ref_opt
            {
              self.push_jmp(cbf_ref.get_label("_End"));
            }

          else
            {
              panic!();
            }
        }
  Statement::Continue=>
        {
            if let Some(cbf_ref) = cbf_ref_opt
            {
              self.push_jmp(cbf_ref.get_label("_Start"));
            }

          else
            {
              panic!();
            }
        }
  Statement::PrintS(s)=>
        {
          self.operation_list.push(Operation::PrintS(s.clone()));
        }
  Statement::PrintV(s)=>
        {
            if let Some(i) = bf_ref.find(s)
            {
              self.operation_list.push(Operation::PrintLoc(i));
            }

          else
            if let Some(i) = self.find_parameter(s)
            {
              self.operation_list.push(Operation::PrintArg(i));
            }

          else
            if let Some(i) = self.find_global(s)
            {
              self.operation_list.push(Operation::PrintGlo(i));
            }

          else
            {
              println!("process_statement error: {} not found",s);

              bf_ref.print();

              panic!();
            }
        }
    }
}


pub fn
print_position_lists(&self)
{
  println!("posreq{{");

    for posreq in &self.position_request_list
    {
      println!("{}: {}",&posreq.name,posreq.value);
    }

  println!("}} pos{{");

    for pos in &self.position_list
    {
      println!("{}: {}",&pos.name,pos.value);
    }


  println!("}}");

}


}




