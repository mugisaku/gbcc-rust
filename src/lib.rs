mod token;
mod source_file;
mod syntax;
mod language;
mod node;
mod object;
mod debug;


use wasm_bindgen::prelude::*;
use crate::language::machine::*;
use crate::language::decl::*;
use crate::language::symbol_table::*;


#[wasm_bindgen]
extern "C"{
pub fn  check(s: &str);
}


static mut SYMTBL: SymbolTable = SymbolTable::new();
static mut EXEC: Exec = Exec::new();
static mut ERR_MSG: String = String::new();
static mut A_MACHINE: Machine = Machine::new();
static mut B_MACHINE: Machine = Machine::new();

#[wasm_bindgen]
pub fn
get_byte(off: usize)-> u8
{unsafe{EXEC.get_u8(off)}}

#[wasm_bindgen]
pub fn
put_byte(off: usize, v: u8)
{unsafe{EXEC.put_u8(off,v);}}

#[wasm_bindgen]
pub fn
get_word(off: usize)-> u32
{unsafe{EXEC.get_u32(off)}}

#[wasm_bindgen]
pub fn
put_word(off: usize, v: u32)
{unsafe{EXEC.put_u32(off,v);}}




#[wasm_bindgen]
pub fn
get_io(s: &str)-> u32
{
  unsafe{
    EXEC.find_io(s).unwrap() as u32
  }
}


#[wasm_bindgen]
pub fn
get_const(s: &str)-> u32
{
  unsafe{
    EXEC.find_const(s).unwrap() as i32 as u32
  }
}


#[wasm_bindgen]
pub fn
process()
{
  unsafe{
    A_MACHINE.run();
//    B_MACHINE.run();
  }
}


#[wasm_bindgen]
pub fn
get_error_message()-> String
{
  unsafe{ERR_MSG.clone()}
}


#[wasm_bindgen]
pub fn
compile(s: &str)-> bool
{
    unsafe
    {
        match Decl::read_as_root(s)
        {
      Ok(root)=>
        {
            match SymbolTable::build(root)
            {
          Ok(symtbl)=>
            {
              SYMTBL = symtbl;

              true
            }
          Err(e)=>
            {
              false
            }
            }
        }
      Err(e)=>
        {
          false
        }
        }
    }
}


#[wasm_bindgen]
pub fn
add_img(w: u32, h: u32, data: Vec<u8>)
{
    unsafe
    {
      SYMTBL.add_img("image",w,h,data);
    }
}


#[wasm_bindgen]
pub fn
setup(freq: u32)-> Option<String>
{
    unsafe
    {
        match SYMTBL.generate_exec()
        {
      Ok(exec)=>
        {
          EXEC = exec;

          A_MACHINE.reset(0,freq as usize,&mut EXEC,"main",0);

          let  mut buf = String::new();

          EXEC.print_text_to(&mut buf);

          Some(buf)
        }
      Err(e)=>
        {
          ERR_MSG = e.to_string();

          None
        }
        }
    }
}




