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
use crate::language::exec::*;


#[wasm_bindgen]
extern "C"{
pub fn  check(s: &str);
}


static mut EX_IMG_W: u32 = 0;
static mut EX_IMG_H: u32 = 0;
static mut EX_IMG_DATA: Vec<u8> = Vec::new();

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
get_word(off: usize)-> u32
{unsafe{EXEC.get_u32(off)}}

#[wasm_bindgen]
pub fn
put_word(off: usize, v: u32)
{unsafe{EXEC.put_u32(off,v);}}

#[wasm_bindgen]
pub fn
set_input(v: u32)
{unsafe{MACHINE.set_input(v as u64);}}




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
    MACHINE.run();
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
transfer_ex_img(w: u32, h: u32, data: Vec<u8>)
{
  unsafe{
    EX_IMG_W    = w;
    EX_IMG_H    = h;
    EX_IMG_DATA = data;
  }
}


#[wasm_bindgen]
pub fn
compile(s: &str)-> bool
{
    unsafe
    {
        match DeclSet::read(s)
        {
      Ok(mut root)=>
        {
            match root.finalize()
            {
          Ok(())=>
            {
              root.add_ex_img("image",EX_IMG_W,EX_IMG_H,&EX_IMG_DATA);

                match root.generate_exec()
                {
              Ok(exec)=>
                {
                  EXEC = exec;

                  true
                }
              Err(e)=>
                {
                  ERR_MSG = e.to_string();

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
setup(freq: u32)-> String
{
    unsafe
    {
      MACHINE.reset(freq as usize,&mut EXEC,"main");

      let  mut buf = String::new();

      EXEC.print_text_to(&mut buf);

      buf
    }
}




