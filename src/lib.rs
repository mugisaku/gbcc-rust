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


static mut MEM: Vec<u8> = Vec::new();
static mut MACHINE: Machine = Machine::new();

fn
get_byte(off: usize)-> u8
{unsafe{*MEM.get_unchecked(off%MEM.len())}}

fn
put_byte(off: usize, v: u8)
{unsafe{*MEM.get_unchecked_mut(off%MEM.len()) = v;}}

fn
get_word(off: usize)-> u64
{unsafe{*(MEM.as_ptr().add(off) as *const u64)}}




const VIDEO_START: usize = 0;
const  WIDTH: usize = 400;
const HEIGHT: usize = 200;

const INPUT_UP:    u8 = 0b00001;
const INPUT_LEFT:  u8 = 0b00010;
const INPUT_RIGHT: u8 = 0b00100;
const INPUT_DOWN:  u8 = 0b01000;
const INPUT_ENTER: u8 = 0b10000;


#[wasm_bindgen]
pub fn  get_width()->  u32{WIDTH as u32}

#[wasm_bindgen]
pub fn  get_height()-> u32{HEIGHT as u32}


#[wasm_bindgen]
pub fn
get_pixel(x: u32, y: u32)-> u32
{
  let  base = get_word(8) as usize;

  let  i = (base+(3*WIDTH*(y as usize))+(3*x as usize));

   ((get_byte(i+0) as u32)<<24)
  |((get_byte(i+1) as u32)<<16)
  |((get_byte(i+2) as u32)<< 8)
}


#[wasm_bindgen]
pub fn
get_audio_freq()-> f64
{
  440.0
}


#[wasm_bindgen]
pub fn
get_audio_volume()-> f64
{
  0.05
}


#[wasm_bindgen]
pub fn
set_input_up()
{put_byte(0,get_byte(0)|INPUT_UP);}

#[wasm_bindgen]
pub fn
unset_input_up()
{put_byte(0,get_byte(0)& !INPUT_UP);}

#[wasm_bindgen]
pub fn
set_input_left()
{put_byte(0,get_byte(0)|INPUT_LEFT);}

#[wasm_bindgen]
pub fn
unset_input_left()
{put_byte(0,get_byte(0)& !INPUT_LEFT);}

#[wasm_bindgen]
pub fn
set_input_right()
{put_byte(0,get_byte(0)|INPUT_RIGHT);}

#[wasm_bindgen]
pub fn
unset_input_right()
{put_byte(0,get_byte(0)& !INPUT_RIGHT);}

#[wasm_bindgen]
pub fn
set_input_down()
{put_byte(0,get_byte(0)|INPUT_DOWN);}

#[wasm_bindgen]
pub fn
unset_input_down()
{put_byte(0,get_byte(0)& !INPUT_DOWN);}

#[wasm_bindgen]
pub fn
set_input_enter()
{put_byte(0,get_byte(0)|INPUT_ENTER);}

#[wasm_bindgen]
pub fn
unset_input_enter()
{put_byte(0,get_byte(0)& !INPUT_ENTER);}


#[wasm_bindgen]
pub fn
process()
{
  let  b = get_byte(0);

  check(&format!("{}",b));

  unsafe{MACHINE.run();}
}


#[wasm_bindgen]
pub fn
setup(s: &str)-> Option<String>
{
    unsafe
    {
        if let Ok(root) = Decl::read_as_root(s)
        {
            if let Ok(mut symtbl) = SymbolTable::build(root)
            {
              let  exec = symtbl.generate_exec();

              MEM = exec.generate_memory();

              MACHINE.connect_memory(MEM.as_mut_ptr(),MEM.len());

              MACHINE.reset(128,&exec,"main");


              let  mut buf = String::new();

              exec.print_text_to(&mut buf);

              return Some(buf);
            }
        }
    }


  None
}




