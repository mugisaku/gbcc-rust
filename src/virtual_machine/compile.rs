

use std::rc::Rc;
use crate::language::typesystem::TypeInfo;
use crate::language::typesystem::*;
use crate::language::element;
use crate::language::expression::Expression;
use crate::language::space::Space;
use crate::language::space::Function;
use crate::language::statement::Statement;
use crate::language::statement::Block;
use crate::virtual_machine::assembly::Note;
use crate::virtual_machine::assembly::Line;
use crate::virtual_machine::assembly;
use crate::virtual_machine::opcode;


pub struct
Context
{
}




pub enum
Object
{
  GlobalSymbol(Symbol),
   LocalSymbol(Symbol),
  TypeInfo(TypeInfo),

}


pub struct
LocalObjectTable<'a>
{
  parent: Option<&'a LocalObjectTable<'a>>,

  objects: Vec<Object>,

  size: usize,

}


impl<'a>
LocalObjectTable<'a>
{


pub fn
new()-> LocalObjectTable<'a>
{
  LocalObjectTable{ parent: None, objects: Vec::new(), size: 0}
}


pub fn
new_with_parent(parent: &'a LocalObjectTable)-> LocalObjectTable<'a>
{
  LocalObjectTable{ parent: Some(parent), objects: Vec::new(), size: 0}
}


}




pub struct
FunctionEntry<'a>
{
  function: &'a Function,

  offset: usize,

  binary: Vec<u8>,

}


impl<'a,'b>
FunctionEntry<'a>
{


pub fn
compile_expression(e: &Expression, got: &GlobalObjectTable, lot: &LocalObjectTable, note: &mut Note)-> Result<&'a TypeInfo,()>
{
    match e
    {
  Expression::Operand(o)=>
        {
            match &**o
            {
          element::Operand::Identifier(s)=>
                {
                       if **s ==  "true"{return Ok(&bool_ti);}
                  else if **s == "false"{return Ok(&bool_ti);}
                },
          element::Operand::Integer(i)=>
                {
                  return match note.put_pshu(*i)
                    {
                  _=>{Ok(&u64_ti)},
                    };
                },
          element::Operand::Floating(f)=>
                {
                  note.put_pshf(*f);

                  return Ok(&f64_ti);
                },
          element::Operand::Character(c)=>
                {
                  note.put_pshu8((*c) as u8);

                  return Ok(&char_ti);
                },
          element::Operand::String(s)=>
                {
                },
          element::Operand::Expression(ee)=>
                {
                  return Self::compile_expression(&ee,got,lot,note);
                },
            }
        },
  Expression::Unary(u)=>
        {
        },
  Expression::Binary(b)=>
        {
        },
  Expression::Primary(p)=>
        {
        },
  Expression::Assign(a)=>
        {
        },
  Expression::Empty=>
        {
        },
    }


  Err(())
}


pub fn
compile_block(blk: &Block, got: &GlobalObjectTable, parent: &LocalObjectTable, cond_block_name: Option<&str>, note: &mut Note)
{
  let mut  lot = LocalObjectTable::new_with_parent(parent);

    for stmt in blk.get_statements()
    {
        match stmt
        {
      Statement::Block(coblk)=>
            {
              Self::compile_block(coblk,got,parent,cond_block_name,note);
            },
      Statement::If(coblks)=>
            {
            },
      Statement::While(coblk)=>
            {
                if let Some(e) = coblk.get_condition()
                {
                  Self::compile_expression(e,got,parent,note);
                }


              Self::compile_block(coblk,got,parent,cond_block_name,note);
            },
      Statement::Break=>
            {
                if let Some(s) = cond_block_name
                {
                  let  end_s = format!("{}_END",s);

                  note.put_relpos16(s);
                }
            },
      Statement::Continue=>
            {
                if let Some(s) = cond_block_name
                {
                  note.put_relpos16(s);
                }
            },
      Statement::Return(opt)=>
            {
                if let Some(e) = opt
                {
                  Self::compile_expression(e,got,parent,note);
                }
            },
      Statement::Expression(e)=>
            {
              let  res = Self::compile_expression(e,got,parent,note);

                if let Ok(ti) = res
                {
                }

              else
                {
crate::report!();
                }
            },
      Statement::VariableDeclaration(vdecl)=>{},
      _=>{},
        }
    }
}


pub fn
compile_function(&self, got: &GlobalObjectTable, note: &mut Note)
{
  let mut  lot = LocalObjectTable::new();

  Self::compile_block(self.function.get_block(),got,&lot,None,note);
}


}



pub struct
GlobalObjectTable<'a>
{
  function_entries: Vec<FunctionEntry<'a>>,

}


impl<'a>
GlobalObjectTable<'a>
{


pub fn
new()-> GlobalObjectTable<'a>
{
  GlobalObjectTable{ function_entries: Vec::new()}
}


pub fn
add_function_entry(&mut self, f: &'a Function)
{
  let  fe = FunctionEntry{ function: f, offset:  0, binary: Vec::new()};

  self.function_entries.push(fe);
}


pub fn
find_function_entry(&mut self, name: &str)-> Option<usize>
{
  let mut  i: usize = 0;

    for fe in &self.function_entries
    {
        if **fe.function.get_name() == name
        {
          return Some(i);
        }


      i += 1;
    }


  None
}


}




pub fn
compile(sp: &Space)-> Result<Vec<u8>,()>
{
  let mut  got = GlobalObjectTable::new();

  let mut  note = Note::new();

    for f in sp.get_function_table()
    {
      got.add_function_entry(f);
    }


  let mut  offset: usize = 0;

    for fe in &got.function_entries
    {
      let  bin = fe.compile_function(&got,&mut note);
    }


  note.assemble()
}




pub struct
Symbol
{
  name: Rc<String>,

  offset: u64,
    size: u64,

}




