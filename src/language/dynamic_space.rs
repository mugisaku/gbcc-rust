

use super::expression::{
  TableElement,
  Expression,
  UnaryOperator,
  BinaryOperator,
  AssignOperator,

};


use super::type_info::{
  StorageInfo,

};


use super::statement::{
  Statement,
  Block,
  For,
  VariableInfo,

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
  pub(crate) symbol_list: Vec<Symbol>,
  pub(crate)  const_list: Vec<Const>,
  pub(crate)    let_list: Vec<Var>,
  pub(crate)     fn_list: Vec<(String,Function,Vec<Operation>)>,

}


impl
Space
{


pub fn
new()-> Self
{
  Self{
    symbol_list: Vec::new(),
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
to_element_list(src: &Vec<TableElement>, const_list: &Vec<Const>)-> Vec<Element>
{
  let  mut dst: Vec<Element> = Vec::new();

    for te in src
    {
        if let Ok(v) = Space::calculate(&te.expression,const_list)
        {
          dst.push(Element::new(&te.name,v));
        }

      else
        {
          panic!();
        }
    }


  dst
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
  Expression::Table(ls)=>
        {
          return Ok(Value::Table(Self::to_element_list(ls,const_list)));
        },
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
compile(&mut self)
{
  let  mut symtbl = self.create_symbol_table();

  Self::check_name(&symtbl);

  Self::calculate_const_values(&mut self.const_list);
  Self::calculate_let_values(&mut self.let_list,&self.const_list);

    for (name,f,op_ls) in &mut self.fn_list
    {
      f.block.scan();

      *op_ls = CompileContext::start(f,&symtbl,&self.const_list);

        for sym in &mut symtbl
        {
            if &sym.name == name
            {
              sym.value = Value::ProgramPointer(op_ls as *const Vec<Operation>);

              break;
            }
        }
    }


  self.symbol_list = symtbl;
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
ControlBlockFrame
{
  pub(crate) base_name: String,

}


impl
ControlBlockFrame
{


pub fn
new(id_ref: &mut usize)-> Self
{
  let  id = *id_ref     ;
            *id_ref += 1;

  Self{
    base_name: format!("{}",id),
  }
}


pub fn
get_start_label(&self)-> String
{
  format!("{}_Start",&self.base_name)
}


pub fn
get_restart_label(&self)-> String
{
  format!("{}_Restart",&self.base_name)
}


pub fn
get_end_label(&self)-> String
{
  format!("{}_End",&self.base_name)
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
  pub(crate)   symbol_list_ref: &'a Vec<Symbol>,
  pub(crate)    const_list_ref: &'a Vec<Const>,
  pub(crate) parameter_list_ref: &'b Vec<String>,

  pub(crate) operation_list: Vec<Operation>,

  pub(crate) ctrl_id: usize,
  pub(crate)   if_id: usize,

  pub(crate) position_request_list: Vec<Position>,
  pub(crate)         position_list: Vec<Position>,

}


impl<'a,'b>
CompileContext<'a,'b>
{


pub fn
start(f_ref: &'b Function, symbol_list_ref: &'a Vec<Symbol>, const_list_ref: &'a Vec<Const>)-> Vec<Operation>
{
  let  mut ctx = Self{
    symbol_list_ref,
     const_list_ref,
    parameter_list_ref: &f_ref.parameter_list,
    operation_list: Vec::new(),
    ctrl_id: 0,
    if_id: 0,
    position_request_list: Vec::new(),
    position_list: Vec::new(),
  };


  ctx.operation_list.push(Operation::AllocateLoc(f_ref.block.stack_allocation_count));

  let  vi = VariableInfo::new(String::new(),None);

  ctx.process_block(&f_ref.block,&vi,None);


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
    for sym in self.symbol_list_ref
    {
        if &sym.name == name
        {
          return Some(sym.index);
        }
    }


  None
}


pub fn
process_expression(&mut self, expr: &Expression, last_vi: &'b VariableInfo, offset: usize)-> StorageInfo
{
  let  mut si = StorageInfo::new();

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
            if let Some(vi) = last_vi.find(&s)
            {
              self.operation_list.push(Operation::LoadLocRef(vi.storage_info.index));
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

              last_vi.print();

              panic!();
            }
        },
  Expression::Boolean(b) =>{self.operation_list.push(Operation::LoadB(*b));},
  Expression::Integer(u) =>{self.operation_list.push(Operation::LoadI(*u as i64));},
  Expression::Floating(f)=>{self.operation_list.push(Operation::LoadF(*f));},
  Expression::String(s)  =>{self.operation_list.push(Operation::LoadS(s.clone()));},
  Expression::Table(ls)=>
        {
          let  e_ls = Space::to_element_list(ls,self.const_list_ref);

          self.operation_list.push(Operation::LoadT(e_ls));
        },
  Expression::SubExpression(e)=>
        {
          self.process_expression(e,last_vi,offset);
        },
  Expression::Unary(o,e)=>
        {
          self.process_expression(e,last_vi,offset);

          self.operation_list.push(Operation::Unary(o.clone()));
        },
  Expression::Call(f,args)=>
        {
          self.process_expression(f,last_vi,offset);

            for a in args.iter().rev()
            {
              self.process_expression(a,last_vi,offset);
            }


          self.operation_list.push(Operation::Cal(args.len()));
        },
  Expression::Subscript(target,index)=>
        {
          self.process_expression(target,last_vi,offset);
          self.process_expression( index,last_vi,offset);

          self.operation_list.push(Operation::Subscript);
        },
  Expression::Access(target,name)=>
        {
          self.process_expression(target,last_vi,offset);

          self.operation_list.push(Operation::Access(name.clone()));
        },
  Expression::Binary(o,l,r)=>
        {
          self.process_expression(l,last_vi,offset);
          self.process_expression(r,last_vi,offset);

          self.operation_list.push(Operation::Binary(o.clone()));
        },
    }


  si
}


pub fn
process_block(&mut self, blk: &'b Block, last_vi: &'b VariableInfo, cbf_ref_opt: Option<&ControlBlockFrame>)
{
    for stmt in &blk.statement_list
    {
      self.process_statement(stmt,last_vi,cbf_ref_opt);
    }
}


pub fn
process_if(&mut self, ls: &'b Vec<(Expression,Block)>, blk_opt: &'b Option<Block>, last_vi: &'b VariableInfo, cbf_ref_opt: Option<&ControlBlockFrame>)
{
  let  base_name = format!("If{}",self.if_id);

  self.if_id += 1;

    for i in 0..ls.len()
    {
      let  (e,blk) = &ls[i];

      self.process_expression(e,last_vi,0);

      self.push_brnz(format!("{}_{}",&base_name,i));
    }


    if let Some(blk) = blk_opt
    {
      self.process_block(blk,last_vi,cbf_ref_opt);
    }


  self.push_jmp(format!("{}_End",&base_name));

    for i in 0..ls.len()
    {
      let  (e,blk) = &ls[i];

      self.push_position(format!("{}_{}",&base_name,i));

      self.process_block(blk,last_vi,cbf_ref_opt);

      self.push_jmp(format!("{}_End",&base_name));
    }


  self.push_position(format!("{}_End",&base_name));
}


pub fn
process_for(&mut self, fo: &'b For, mut last_vi: &'b VariableInfo, cbf_ref_opt: Option<&ControlBlockFrame>)
{
  let  cbf = ControlBlockFrame::new(&mut self.ctrl_id);

  let  cur_i = fo.current_vi.storage_info.index;
  let  end_i =     fo.end_vi.storage_info.index;

  self.operation_list.push(Operation::LoadLocRef(cur_i));

  self.operation_list.push(Operation::LoadI(0));

  self.operation_list.push(Operation::Assign(AssignOperator::Nop));

  self.operation_list.push(Operation::Dump);


  self.operation_list.push(Operation::LoadLocRef(end_i));

  self.process_expression(fo.get_end_expression(),last_vi,0);

  self.operation_list.push(Operation::Assign(AssignOperator::Nop));


  self.operation_list.push(Operation::LoadI(0));
  self.operation_list.push(Operation::Binary(BinaryOperator::Gt));

  self.push_brz(cbf.get_end_label());

  self.push_position(cbf.get_start_label());


  last_vi = &fo.current_vi;


  self.process_block(&fo.block,last_vi,Some(&cbf));

  self.push_position(cbf.get_restart_label());

  self.operation_list.push(Operation::LoadLocRef(cur_i));
  self.operation_list.push(Operation::LoadI(1));
  self.operation_list.push(Operation::Assign(AssignOperator::Add));

  self.operation_list.push(Operation::LoadLocRef(end_i));
  self.operation_list.push(Operation::Binary(BinaryOperator::Lt));

  self.push_brnz(cbf.get_start_label());

  self.push_position(cbf.get_end_label());
}


pub fn
process_print_v(&mut self, s: &str, last_vi: &'b VariableInfo, cbf_ref_opt: Option<&ControlBlockFrame>)
{
    if let Some(vi) = last_vi.find(s)
    {
      self.operation_list.push(Operation::PrintLoc(vi.storage_info.index));
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

      last_vi.print();

      panic!();
    }
}


pub fn
process_statement(&mut self, stmt: &'b Statement, mut last_vi: &'b VariableInfo, cbf_ref_opt: Option<&ControlBlockFrame>)
{
    match stmt
    {
  Statement::Empty=>{}
  Statement::Let(vi)=>
        {
            if let Some(e) = &vi.expression_opt
            {
              self.operation_list.push(Operation::LoadLocRef(vi.storage_info.index));

              self.process_expression(e,last_vi,0);

              self.operation_list.push(Operation::Assign(AssignOperator::Nop));
            }


          last_vi = vi;
        }
  Statement::Const(vi)=>
        {
        }
  Statement::Expression(e,ass_opt)=>
        {
          self.process_expression(e,last_vi,0);

            if let Some((o,re)) = ass_opt
            {
              self.process_expression(re,last_vi,0);

              self.operation_list.push(Operation::Assign(o.clone()));
            }


          self.operation_list.push(Operation::Dump);
        }
  Statement::If(ls,blk_opt)=>
        {
          self.process_if(ls,blk_opt,last_vi,cbf_ref_opt);
        }
  Statement::For(fo)=>
        {
          self.process_for(fo,last_vi,cbf_ref_opt);
        }
  Statement::While(e,blk)=>
        {
          let  cbf = ControlBlockFrame::new(&mut self.ctrl_id);

          self.push_jmp(cbf.get_restart_label());

          self.push_position(cbf.get_start_label());

          self.process_block(blk,last_vi,Some(&cbf));

          self.push_position(cbf.get_restart_label());

          self.process_expression(e,last_vi,0);

          self.push_brnz(cbf.get_start_label());

          self.push_position(cbf.get_end_label());
        }
  Statement::Loop(blk)=>
        {
          let  cbf = ControlBlockFrame::new(&mut self.ctrl_id);

          self.push_position(cbf.get_start_label());

          self.process_block(blk,last_vi,Some(&cbf));

          self.push_position(cbf.get_restart_label());

          self.push_jmp(cbf.get_start_label());

          self.push_position(cbf.get_end_label());
        }
  Statement::Block(blk)=>
        {
          self.process_block(blk,last_vi,cbf_ref_opt);
        }
  Statement::Return(e_opt)=>
        {
            if let Some(e) = e_opt
            {
              self.process_expression(e,last_vi,0);

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
              self.push_jmp(cbf_ref.get_end_label());
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
              self.push_jmp(cbf_ref.get_start_label());
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
          self.process_print_v(s,last_vi,cbf_ref_opt);
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




