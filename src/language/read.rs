

use crate::token::{
  ParsedNumber,

};


use crate::syntax::{
  Directory,
  Object,
  ObjectData,
  Cursor,
};


use crate::language::expression::{
  UnaryOperator,
  BinaryOperator,
  AssignOperator,
  Expression,
  TableElement,

};


use crate::language::statement::{
  IfBranch,
  Block,
  Statement,
  For,

};


use super::type_kind::*;


use crate::language::element::{
  Element,
  Function,
  Symbol,
  Class,
  Field,

};




pub fn
read_type_kind(dir: &Directory)-> TypeKind
{
  let  mut cur = Cursor::new(dir);

  let  mut tk = TypeKind::Undefined;

  let  mut prefix = String::new();

    if let Some(s) = cur.get_others_string()
    {
      prefix = s.clone();

      cur.advance(1);
    }


    if let Some(s) = cur.get_identifier()
    {
      tk = TypeKind::Class(s.clone());

           if prefix == "&"{tk = TypeKind::Reference(Box::new(tk));}
      else if prefix == "*"{tk = TypeKind::Pointer(  Box::new(tk));}
      else{panic!();}
    }


  tk
}


pub fn
read_parameter(dir: &Directory)-> Field
{
  let  mut cur = Cursor::new(dir);

  let  mut name = String::new();
  let  mut type_kind = TypeKind::Undefined;

    if let Some(s) = cur.get_identifier()
    {
      name = s.clone();

      cur.advance(2);
    }


    if let Some(d) = cur.get_directory()
    {
      type_kind = read_type_kind(d);
    }


  Field{name, complete_flag: false, offset: 0, type_kind, type_code: TypeCode::new()}
}


pub fn
read_parameter_list(dir: &Directory)-> Vec<Field>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls = Vec::<Field>::new();

  cur.advance(1);

    while let Some(p_d) = cur.get_directory_with_name("parameter")
    {
      let  p = read_parameter(p_d);

      ls.push(p);

      cur.advance(2);
    }


  ls
}

 
pub fn
read_fn(dir: &Directory)-> Function
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id) = cur.get_identifier()
    {
      let  name = id.clone();

      cur.advance(1);

        if let Some(parals_d) = cur.get_directory_with_name("parameter_list")
        {
          let  parameter_list = read_parameter_list(parals_d);

          let  mut return_type_kind = TypeKind::Undefined;

          cur.advance(1);

            if let Some(_) = cur.get_others_string()
            {
              cur.advance(1);

                if let Some(ty_d) = cur.get_directory()
                {
                  return_type_kind = read_type_kind(ty_d);

                  cur.advance(1);
                }
            }


            if let Some(stmts_d) = cur.seek_directory_with_name("statement_list")
            {
              let  block = read_block(stmts_d);

              return Function::new(name,parameter_list,return_type_kind,block);
            }
        }
    }


  panic!();
}


pub fn
read_variable(dir: &Directory)-> Symbol
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      let  mut type_kind = TypeKind::Undefined;

      cur.advance(1);

        if let Some(_) = cur.get_others_string()
        {
          cur.advance(1);

            if let Some(ty_d) = cur.get_directory()
            {
              type_kind = read_type_kind(ty_d);

              cur.advance(1);
            }
        }


      let  mut expression_opt: Option<Expression> = None;

        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
          expression_opt = Some(read_expression(e_d));
        }


      return Symbol{name, complete_flag: false, flags: 0, offset: 0, type_kind, type_code: TypeCode::new(), expression_opt, initial_value_opt: None};
    }


  panic!();
}


pub fn
read_element(dir: &Directory)-> Element
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "fn"
        {
          return Element::Function(read_fn(d));
        }

      else
        if d_name == "const"
        {
          let  mut sym = read_variable(d);

          sym.set_const_flag();

          return Element::Symbol(sym);
        }

      else
        if d_name == "static"
        {
          let  mut sym = read_variable(d);

          sym.set_static_flag();

          return Element::Symbol(sym);
        }
    }


  panic!();
}




pub fn
read_expression_or_assign(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

    if let Some(e_dir) = cur.get_directory_with_name("expression")
    {
      let  e = read_expression(e_dir);

      cur.advance(1);

        if let Some(o_dir) = cur.get_directory_with_name("assign_operator")
        {
          let  o = read_assign_operator(o_dir);

          cur.advance(1);

            if let Some(r_dir) = cur.get_directory_with_name("expression")
            {
              let  r = read_expression(r_dir);

              return Statement::Expression(e,Some((o,r)));
            }
        }

      else
        {
          return Statement::Expression(e,None);
        }
    }


  panic!();
}


pub fn
read_assign_operator(dir: &Directory)-> AssignOperator
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s ==   "="{return AssignOperator::Nop;}
      else if s ==  "+="{return AssignOperator::Add;}
      else if s ==  "-="{return AssignOperator::Sub;}
      else if s ==  "*="{return AssignOperator::Mul;}
      else if s ==  "/="{return AssignOperator::Div;}
      else if s ==  "%="{return AssignOperator::Rem;}
      else if s == "<<="{return AssignOperator::Shl;}
      else if s == ">>="{return AssignOperator::Shr;}
      else if s ==  "&="{return AssignOperator::And;}
      else if s ==  "|="{return AssignOperator::Or;}
      else if s ==  "^="{return AssignOperator::Xor;}
    }


  panic!();
}


pub fn
read_return(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.seek_directory_with_name("expression")
    {
      let  e = read_expression(d);

      return Statement::Return(Some(e));
    }


  Statement::Return(None)
}


pub fn
read_if(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    while let Some(expr_d) = cur.get_directory_with_name("expression")
    {
      let  expression = read_expression(expr_d);
      let  mut  first_stmt = Statement::Empty;
      let  mut second_stmt = Statement::Empty;

      cur.advance(1);

        if let Some(first_stmt_d) = cur.get_directory_with_name("statement")
        {
          first_stmt = read_statement(first_stmt_d);

          cur.advance(1);

            if cur.test_keyword("else")
            {
              cur.advance(1);

                if let Some(second_stmt_d) = cur.get_directory_with_name("statement")
                {
                  second_stmt = read_statement(second_stmt_d);
                }
            }


          let  br = IfBranch{expression, on_true: Box::new(first_stmt), on_false: Box::new(second_stmt)};

          return Statement::If(br);
        }
    }


  panic!();
}


pub fn
read_while(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
      let  condition = read_expression(expr_d);

      cur.advance(1);

        if let Some(ls_d) = cur.get_directory_with_name("statement_list")
        {
          let  blk = read_block(ls_d);

          return Statement::While(condition,blk);
        }
    }


  panic!();
}


pub fn
read_loop(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(ls_d) = cur.get_directory_with_name("statement_list")
    {
      let  blk = read_block(ls_d);

      return Statement::Loop(blk);
    }


  panic!();
}


pub fn
read_for(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      let  name = s.clone();

      cur.advance(2);

        if let Some(expr_d) = cur.get_directory_with_name("expression")
        {
          let  expr = read_expression(expr_d);

          cur.advance(1);

            if let Some(blk_d) = cur.get_directory_with_name("statement_list")
            {
              let  blk = read_block(blk_d);

              let  f = For::new(name,expr,blk);

              return Statement::For(f);
            }
        }
    }


  panic!();
}


pub fn
read_print_s(dir: &Directory)-> String
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_string()
    {
      return s.clone();
    }


  panic!();
}


pub fn
read_print_v(dir: &Directory)-> String
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      return s.clone();
    }


  panic!();
}


pub fn
read_block(dir: &Directory)-> Block
{
  let  mut cur = Cursor::new(dir);

  let  mut name = String::new();

  let  mut statement_list: Vec<Statement> = Vec::new();

  cur.advance(1);

    while let Some(d) = cur.get_directory()
    {
      let  stmt = read_statement(d);

      statement_list.push(stmt);

      cur.advance(1);
    }


  Block::new(name,statement_list)
}


pub fn
read_statement(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
        if s == ";"
        {
          return Statement::Empty;
        }
    }

  else
    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "block"
        {
          let  blk = read_block(d);

          return Statement::Block(blk);
        }

      else
        if d_name == "if"
        {
          return read_if(d);
        }

      else
        if d_name == "for"
        {
          return read_for(d);
        }

      else
        if d_name == "while"
        {
          return read_while(d);
        }

      else
        if d_name == "loop"
        {
          return read_loop(d);
        }

      else
        if d_name == "break"
        {
          return Statement::Break;
        }

      else
        if d_name == "continue"
        {
          return Statement::Continue;
        }

      else
        if d_name == "return"
        {
          return read_return(d);
        }

      else
        if d_name == "let"
        {
          let  sym = read_variable(d);

          return Statement::Let(sym);
        }

      else
        if d_name == "const"
        {
          let  sym = read_variable(d);

          return Statement::Const(sym);
        }

      else
        if d_name == "static"
        {
          let  sym = read_variable(d);

          return Statement::Static(sym);
        }

      else
        if d_name == "print_s"
        {
          let  s = read_print_s(d);

          return Statement::PrintS(s);
        }

      else
        if d_name == "print_v"
        {
          let  s = read_print_v(d);

          return Statement::PrintV(s);
        }

      else
        if d_name == "expression_or_assign"
        {
          let  st = read_expression_or_assign(d);

          return st;
        }
    }


  panic!();
}




pub fn
read_expression(dir: &Directory)-> Expression
{
  let  mut cur = Cursor::new(dir);

    if let Some(o_dir) = cur.get_directory_with_name("operand")
    {
      let  mut e = read_operand(o_dir);

      cur.advance(1);

        while let Some(b_dir) = cur.get_directory_with_name("binary_operator")
        {
          let  b = read_binary_operator(b_dir);

          cur.advance(1);

            if let Some(next_o_dir) = cur.get_directory_with_name("operand")
            {
              let  next_e = read_operand(next_o_dir);

              cur.advance(1);

              let  l = Box::new(     e);
              let  r = Box::new(next_e);

              e = Expression::Binary(b,l,r);
            }

          else
            {
              panic!();
            }
        }


      return e;
    }


  panic!();
}




pub fn
read_unary_operator(dir: &Directory)-> UnaryOperator
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s == "~"{return UnaryOperator::Not;}
      else if s == "!"{return UnaryOperator::LogicalNot;}
      else if s == "-"{return UnaryOperator::Neg;}
      else if s == "*"{return UnaryOperator::Deref;}
    }


  panic!();
}


pub fn
read_binary_operator(dir: &Directory)-> BinaryOperator
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
           if s ==  "+"{return BinaryOperator::Add;}
      else if s ==  "-"{return BinaryOperator::Sub;}
      else if s ==  "*"{return BinaryOperator::Mul;}
      else if s ==  "/"{return BinaryOperator::Div;}
      else if s ==  "%"{return BinaryOperator::Rem;}
      else if s == "<<"{return BinaryOperator::Shl;}
      else if s == ">>"{return BinaryOperator::Shr;}
      else if s ==  "&"{return BinaryOperator::And;}
      else if s ==  "|"{return BinaryOperator::Or;}
      else if s ==  "^"{return BinaryOperator::Xor;}
      else if s == "=="{return BinaryOperator::Eq;}
      else if s == "!="{return BinaryOperator::Neq;}
      else if s ==  "<"{return BinaryOperator::Lt;}
      else if s == "<="{return BinaryOperator::Lteq;}
      else if s ==  ">"{return BinaryOperator::Gt;}
      else if s == ">="{return BinaryOperator::Gteq;}
      else if s == "&&"{return BinaryOperator::LogicalAnd;}
      else if s == "||"{return BinaryOperator::LogicalOr;}
    }


  panic!();
}


pub fn
read_postfix_operator(dir: &Directory, e: Box<Expression>)-> Expression
{
  let  cur = Cursor::new(dir);

    if let Some(subdir) = cur.get_directory()
    {
      let  name = subdir.get_name();

           if name == "access"   {return read_access(subdir,e);}
      else if name == "subscript"{return read_subscript(subdir,e);}
      else if name == "call"     {return read_call(subdir,e);}
    }


  panic!();
}


pub fn
read_access(dir: &Directory, e: Box<Expression>)-> Expression
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(o) = cur.get_object()
    {
        if let ObjectData::Identifier(s) = o.get_data()
        {
          return Expression::Access(e,s.clone());
        }
    }


  panic!();
}


pub fn
read_subscript(dir: &Directory, target_e: Box<Expression>)-> Expression
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(e_dir) = cur.get_directory_with_name("expression")
    {
      let  e = read_expression(e_dir);

      return Expression::Subscript(target_e,Box::new(e));
    }


  panic!();
}


pub fn
read_call(dir: &Directory, fe: Box<Expression>)-> Expression
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

  let  mut args: Vec<Expression> = Vec::new();

    if let Some(first_e_dir) = cur.get_directory_with_name("expression")
    {
      let  e = read_expression(first_e_dir);

      args.push(e);

      cur.advance(2);

        while let Some(e_dir) = cur.get_directory_with_name("expression")
        {
          let  e = read_expression(e_dir);

          args.push(e);

          cur.advance(2);
        }
    }


  Expression::Call(fe,args)
}


pub fn
read_table_element(dir: &Directory)-> TableElement
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      let  s = id.clone();

      cur.advance(2);

        if let Some(e_dir) = cur.get_directory_with_name("expression")
        {
          let  e = read_expression(e_dir);

          return TableElement::new(s,e);
        }
    }


  panic!();
}


pub fn
read_table(dir: &Directory)-> Vec<TableElement>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls: Vec<TableElement> = Vec::new();

  cur.advance(1);

    while let Some(te_dir) = cur.get_directory_with_name("table_element")
    {
      let  te = read_table_element(te_dir);

      cur.advance(2);

      ls.push(te);
    }


  ls
}


pub fn
read_operand_core(dir: &Directory)-> Expression
{
  let  mut cur = Cursor::new(dir);

    if let Some(id) = cur.get_identifier()
    {
      return Expression::Identifier(id.clone());
    }

  else
    if let Some(t_dir) = cur.get_directory_with_name("table")
    {
      let  ls = read_table(t_dir);

      return Expression::Table(ls);
    }

  else
    if let Some(o) = cur.get_object()
    {
        match o.get_data()
        {
      ObjectData::Number(pn)=>
        {
            if let Some(f) = pn.get_float()
            {
              return Expression::Float(f);
            }

          else
            {
              return Expression::Int(pn.i_part);
            }
        }
      ObjectData::String(s)=>{return Expression::String(s.clone());},
      ObjectData::OthersString(s)=>
          {
              if s == "("
              {
                cur.advance(1);

                  if let Some(e_dir) = cur.get_directory_with_name("expression")
                  {
                    let  e = read_expression(e_dir);

                    return Expression::SubExpression(Box::new(e));
                  }
              }
          },
      _=>{},
        }
    }


  panic!();
}


pub fn
read_operand(dir: &Directory)-> Expression
{
  let  mut cur = Cursor::new(dir);

  let  mut un_ls: Vec<UnaryOperator> = Vec::new();

    while let Some(un_dir) = cur.get_directory_with_name("unary_operator")
    {
      let  pre = read_unary_operator(un_dir);

      cur.advance(1);

      un_ls.push(pre);
    }


    if let Some(core_dir) = cur.get_directory_with_name("operand_core")
    {
      let  mut e = read_operand_core(core_dir);

      cur.advance(1);

        while let Some(post_dir) = cur.get_directory_with_name("postfix_operator")
        {
          let  new_e = read_postfix_operator(post_dir,Box::new(e));

          cur.advance(1);

          e = new_e;
        }


        while let Some(un) = un_ls.pop()
        {
          e = Expression::Unary(un,Box::new(e));
        }


      return e;
    }


  panic!();
}




