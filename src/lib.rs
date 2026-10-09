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
use crate::language::project::*;
use crate::language::exec::*;


#[wasm_bindgen]
extern "C"{
pub fn  check(s: &str);
}


static mut PJ: Project = Project::new();
static mut EXEC: Exec = Exec::new();
static mut ERR_MSG: String = String::new();
static mut MACHINE: Machine = Machine::new();

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
get_u16(off: usize)-> u16
{unsafe{EXEC.get_u16(off)}}

#[wasm_bindgen]
pub fn
put_u16(off: usize, v: u16)
{unsafe{EXEC.put_u16(off,v);}}

#[wasm_bindgen]
pub fn
get_word_hi(off: usize)-> u32
{unsafe{(EXEC.get_u64(off)>>32) as u32}}

#[wasm_bindgen]
pub fn
get_word_lo(off: usize)-> u32
{unsafe{EXEC.get_u64(off) as u32}}

#[wasm_bindgen]
pub fn
put_word(off: usize, hi: u32, lo: u32)
{unsafe{EXEC.put_u64(off,((hi as u64)<<32)|(lo as u64));}}

#[wasm_bindgen]
pub fn
set_input(v: u32)
{unsafe{MACHINE.set_input(v as u64);}}

#[wasm_bindgen]
pub fn
get_input()-> u32
{unsafe{MACHINE.get_input() as u32}}




#[wasm_bindgen]
pub fn
get_data_address(s: &str)-> u32
{
  unsafe{
    EXEC.find_data_address(s).unwrap() as u32
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
process(freq: u32, tm: u32)
{
  unsafe{
    MACHINE.run(freq as usize, tm as usize);
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
add_source(name: &str, s: &str)-> bool
{
  unsafe{
    match PJ.add_source(name,s)
    {
  Ok(())=>{true}
  Err(msg)=>
    {
      ERR_MSG = msg.to_string();

      false
    }
    }
  }
}


#[wasm_bindgen]
pub fn
clear()
{
    unsafe
    {
      PJ.clear();
    }
}


#[wasm_bindgen]
pub fn
compile()-> bool
{
    unsafe
    {
        match PJ.compile()
        {
      Ok(())=>
        {
            match PJ.generate_exec()
            {
          Ok(exec)=>
            {
              EXEC = exec;

              true
            }
          Err(msg)=>
            {
              ERR_MSG = msg.to_string();

              false
            }
            }
        }
      Err(e)=>
        {
          ERR_MSG = e.to_string();

          false
        }
        }
    }
}


#[wasm_bindgen]
pub fn
setup()-> String
{
    unsafe
    {
      MACHINE.reset(&mut EXEC,"main");

      let  mut buf = String::new();

      buf.push_str("\n  [data]  \n");

      EXEC.print_memory_to(&mut buf);

      buf.push_str("\n  [text]  \n");

      EXEC.print_text_to(&mut buf);

      buf
    }
}




