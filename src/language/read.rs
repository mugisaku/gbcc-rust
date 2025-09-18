

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
  OpId,
  Expression,
  TableElement,

};


use crate::language::declaration::{
  Parameter,
  Declaration,
  ObjectDecl,
  FunctionDecl,

};


use crate::language::statement::{
  Branch,
  Block,
  Statement,
  For,

};


use super::ty::Type;




pub fn
read_type(dir: &Directory)-> Type
{
  let  mut cur = Cursor::new(dir);

  let  mut prefix = String::new();

    if let Some(s) = cur.get_others_string()
    {
      prefix = s.clone();

      cur.advance(1);
    }


    if let Some(s) = cur.get_identifier()
    {
      let  ty = Type::Alias(s.clone());

           if prefix == "&"{return Type::Reference(Box::new(ty));}
      else if prefix == "*"{return Type::Pointer(  Box::new(ty));}
      else{panic!();}
    }


  panic!();
}


pub fn
read_parameter(dir: &Directory)-> Parameter
{
  let  mut cur = Cursor::new(dir);

  let  mut name = String::new();
  let  mut ty = Type::Void;

    if let Some(s) = cur.get_identifier()
    {
      name = s.clone();

      cur.advance(2);
    }


    if let Some(d) = cur.get_directory()
    {
      return Parameter{name, ty: read_type(d)};
    }


  panic!();
}


pub fn
read_parameter_list(dir: &Directory)-> Vec<Parameter>
{
  let  mut cur = Cursor::new(dir);

  let  mut ls = Vec::<Parameter>::new();

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
read_function_decl(dir: &Directory)-> FunctionDecl
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

          let  mut return_ty = Type::Void;

          cur.advance(1);

            if let Some(_) = cur.get_others_string()
            {
              cur.advance(1);

                if let Some(ty_d) = cur.get_directory()
                {
                  return_ty = read_type(ty_d);

                  cur.advance(1);
                }
            }


            if let Some(stmts_d) = cur.seek_directory_with_name("block")
            {
              let  block = read_block(stmts_d);

              return FunctionDecl{name,parameter_list,return_ty,block};
            }
        }
    }


  panic!();
}


pub fn
read_object_decl(dir: &Directory)-> ObjectDecl
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(id_s) = cur.get_identifier()
    {
      let  name = id_s.clone();

      let  mut ty = Type::Void;

      cur.advance(1);

        if let Some(_) = cur.get_others_string()
        {
          cur.advance(1);

            if let Some(ty_d) = cur.get_directory()
            {
              ty = read_type(ty_d);

              cur.advance(1);
            }
        }


      let  mut expression = Expression::Void;

        if let Some(e_d) = cur.seek_directory_with_name("expression")
        {
          expression = read_expression(e_d);
        }


      return ObjectDecl{name, ty, expression};
    }


  panic!();
}




pub fn
read_declaration(dir: &Directory)-> Declaration
{
  let  mut cur = Cursor::new(dir);

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d_name == "function"
        {
          return Declaration::Function(read_function_decl(d));
        }

      else
        if d_name == "let"
        {
          let  od = read_object_decl(d);

          return Declaration::Let(od);
        }

      else
        if d_name == "const"
        {
          let  od = read_object_decl(d);

          return Declaration::Const(od);
        }

      else
        if d_name == "static"
        {
          let  od = read_object_decl(d);

          return Declaration::Static(od);
        }
    }


  panic!();
}




pub fn
read_assign(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

    if let Some(l_dir) = cur.get_directory_with_name("expression")
    {
      let  l = read_expression(l_dir);

      cur.advance(1);

        if let Some(o_dir) = cur.get_directory_with_name("assign_operator")
        {
          let  o = read_assign_operator(o_dir);

          cur.advance(1);

            if let Some(r_dir) = cur.get_directory_with_name("expression")
            {
              let  r = read_expression(r_dir);

              return Statement::Assign(o,l,r);
            }
        }
    }


  panic!();
}


pub fn
read_assign_operator(dir: &Directory)-> OpId
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
        if (s ==   "=")
        || (s ==  "+=")
        || (s ==  "-=")
        || (s ==  "*=")
        || (s ==  "/=")
        || (s ==  "%=")
        || (s == "<<=")
        || (s == ">>=")
        || (s ==  "&=")
        || (s ==  "|=")
        || (s ==  "^=")
        {return OpId::from(s.as_str());}
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
read_else_if(dir: &Directory)-> Branch
{
  let  mut cur = Cursor::new(dir);

  cur.advance(2);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
      let  expr = read_expression(expr_d);

      cur.advance(1);

        if let Some(blk_d) = cur.get_directory_with_name("block")
        {
          let  mut br = Branch{expression_opt: Some(expr), block: read_block(blk_d), sub_branch_opt: None};

          cur.advance(1);

            if let Some(elif_d) = cur.get_directory_with_name("else_if")
            {
              br.sub_branch_opt = Some(Box::new(read_else_if(elif_d)));
            }

          else
            if let Some(el_d) = cur.get_directory_with_name("else")
            {
              br.sub_branch_opt = Some(Box::new(read_else(el_d)));
            }


          return br;
        }
    }


  panic!();
}


pub fn
read_else(dir: &Directory)-> Branch
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(blk_d) = cur.get_directory_with_name("block")
    {
      return Branch{expression_opt: None, block: read_block(blk_d), sub_branch_opt: None};
    }


  panic!();
}


pub fn
read_if(dir: &Directory)-> Statement
{
  let  mut cur = Cursor::new(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
      let  expr = read_expression(expr_d);

      cur.advance(1);

        if let Some(blk_d) = cur.get_directory_with_name("block")
        {
          let  mut br = Branch{expression_opt: Some(expr), block: read_block(blk_d), sub_branch_opt: None};

          cur.advance(1);

            if let Some(elif_d) = cur.get_directory_with_name("else_if")
            {
              br.sub_branch_opt = Some(Box::new(read_else_if(elif_d)));
            }

          else
            if let Some(el_d) = cur.get_directory_with_name("else")
            {
              br.sub_branch_opt = Some(Box::new(read_else(el_d)));
            }


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

        if let Some(ls_d) = cur.get_directory_with_name("block")
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

    if let Some(ls_d) = cur.get_directory_with_name("block")
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

            if let Some(blk_d) = cur.get_directory_with_name("block")
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
        if d_name == "declaration"
        {
          let  decl = read_declaration(d);

          return Statement::Declaration(decl);
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
        if d_name == "expression"
        {
          let  st = Statement::Expression(read_expression(d));

          return st;
        }

      else
        if d_name == "assign"
        {
          let  st = read_assign(d);

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

              e = Expression::BinaryOp(b,l,r);
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
read_unary_operator(dir: &Directory)-> OpId
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
        if (s == "!") || (s == "-") || (s == "*")
        {return OpId::from(s.as_str());}
    }


  panic!();
}


pub fn
read_binary_operator(dir: &Directory)-> OpId
{
  let  cur = Cursor::new(dir);

    if let Some(s) = cur.get_others_string()
    {
        if (s ==  "+") || (s ==  "-") || (s ==  "*") || (s ==  "/") || (s ==  "%")
        || (s == "<<") || (s == ">>") || (s ==  "&") || (s ==  "|") || (s ==  "^")
        || (s == "==") || (s == "!=") || (s ==  "<") || (s == "<=") ||  (s ==  ">") || (s == ">=")
        || (s == "&&") || (s == "||")
        {return OpId::from(s.as_str());}
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
          return Expression::AccessOp(e,s.clone());
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

      return Expression::SubscriptOp(target_e,Box::new(e));
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


  Expression::CallOp(fe,args)
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
              return Expression::Uint(pn.i_part);
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

  let  mut un_ls: Vec<OpId> = Vec::new();

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
          e = Expression::UnaryOp(un,Box::new(e));
        }


      return e;
    }


  panic!();
}




