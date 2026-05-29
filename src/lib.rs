mod token;
mod source_file;
mod syntax;
mod language;
mod node;
mod object;
mod debug;


use wasm_bindgen::prelude::*;
use crate::language::machine::*;


#[wasm_bindgen]
extern "C"{
pub fn  check(s: &str);
}


static mut MEM: Vec<u8> = Vec::new();
static mut MACHINE: Machine = Machine::new();

const VIDEO_START: usize = 0;
const  WIDTH: usize = 400;
const HEIGHT: usize = 200;
const VIDEO_BUFFER_SIZE: usize = (WIDTH*HEIGHT);

const INPUT_UP:    u8 = 0b000001;
const INPUT_LEFT:  u8 = 0b000010;
const INPUT_RIGHT: u8 = 0b000100;
const INPUT_DOWN:  u8 = 0b001000;
const INPUT_ENTER: u8 = 0b010000;


static mut R: u64 = 12345678;


#[wasm_bindgen]
pub fn  get_width()->  u32{WIDTH as u32}

#[wasm_bindgen]
pub fn  get_height()-> u32{HEIGHT as u32}


fn
get_rand()-> u64
{
  unsafe
  {
    R ^= R<<9;
    R ^= R>>7;

    R
  }
}


#[wasm_bindgen]
pub fn
get_pixel(x: u32, y: u32)-> u8
{
  let  i = ((WIDTH*(y as usize))+(x as usize));

  unsafe{*MEM.get_unchecked(i)}
}


#[wasm_bindgen]
pub fn
get_audio_freq()-> f64
{
  if (get_rand()&1) == 0
  {
    440.0
  }else{880.0}
}


#[wasm_bindgen]
pub fn
get_audio_volume()-> f64
{
  if (get_rand()&1) == 0
  {
    0.05
  }else{0.1}
}


static mut INPUT: u8 = 0;

#[wasm_bindgen]
pub fn
set_input_up()
{unsafe{INPUT |= INPUT_UP;}}

#[wasm_bindgen]
pub fn
unset_input_up()
{unsafe{INPUT &= !INPUT_UP;}}

#[wasm_bindgen]
pub fn
set_input_left()
{unsafe{INPUT |= INPUT_LEFT;}}

#[wasm_bindgen]
pub fn
unset_input_left()
{unsafe{INPUT &= !INPUT_LEFT;}}

#[wasm_bindgen]
pub fn
set_input_right()
{unsafe{INPUT |= INPUT_RIGHT;}}

#[wasm_bindgen]
pub fn
unset_input_right()
{unsafe{INPUT &= !INPUT_RIGHT;}}

#[wasm_bindgen]
pub fn
set_input_down()
{unsafe{INPUT |= INPUT_DOWN;}}

#[wasm_bindgen]
pub fn
unset_input_down()
{unsafe{INPUT &= !INPUT_DOWN;}}

#[wasm_bindgen]
pub fn
set_input_enter()
{unsafe{INPUT |= INPUT_ENTER;}}

#[wasm_bindgen]
pub fn
unset_input_enter()
{unsafe{INPUT &= !INPUT_ENTER;}}


static mut X_POS: usize = 0;
static mut Y_POS: usize = 0;

#[wasm_bindgen]
pub fn
process()
{
  unsafe
  {
      if ((INPUT&INPUT_UP)    != 0) && (Y_POS != 0)        {Y_POS -= 1;}
      if ((INPUT&INPUT_LEFT)  != 0) && (X_POS != 0)        {X_POS -= 1;}
      if ((INPUT&INPUT_RIGHT) != 0) && (X_POS < (WIDTH -1)){X_POS += 1;}
      if ((INPUT&INPUT_DOWN)  != 0) && (Y_POS < (HEIGHT-1)){Y_POS += 1;}


    *MEM.get_unchecked_mut((WIDTH*Y_POS)+X_POS) = 255;
  }
}


#[wasm_bindgen]
pub fn
setup(s: &str)
{
/*
    if let Ok(root) = decl::Decl::read_as_root(s)
    {
        if let Ok(mut symtbl) = SymbolTable::build(root)
        {
          let  exec = symtbl.generate_exec(&mi);

          symtbl.print();

          println!("");

          let  mut m = Machine::new(&mi);

          m.reset(&exec);
        }

      else
        {
          println!("build is failed");
        }
    }
*/
}




