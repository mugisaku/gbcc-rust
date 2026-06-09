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


static mut EXEC: Exec = Exec::new();
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
get_info()-> u32
{
  unsafe{
    B_MACHINE.get_pc() as u32
  }
}


#[wasm_bindgen]
pub fn
process()
{
  unsafe{
    A_MACHINE.run();
    B_MACHINE.run();
  }
}


#[wasm_bindgen]
pub fn
setup(s: &str, memsz: u32, freq: u32)-> Option<String>
{
    unsafe
    {
        if let Ok(root) = Decl::read_as_root(s)
        {
            if let Ok(mut symtbl) = SymbolTable::build(root)
            {
              EXEC = symtbl.generate_exec(memsz as usize);

              A_MACHINE.reset(0,freq as usize,&mut EXEC,"a_main",0);
              B_MACHINE.reset(1,freq as usize,&mut EXEC,"b_main",1024);

              let  mut buf = String::new();

              EXEC.print_text_to(&mut buf);

              return Some(buf);
            }
        }
    }


  None
}




