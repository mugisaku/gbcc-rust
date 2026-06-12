mod token;
mod source_file;
mod syntax;
mod language;
mod node;
mod object;
mod debug;

use std::env;


fn
compile_and_run(s: &str)
{
  use crate::language::*;
  use crate::language::decl::*;
  use crate::language::symbol_table::*;
  use crate::language::machine::*;

    if let Ok(root) = decl::Decl::read_as_root(s)
    {
        if let Ok(mut symtbl) = SymbolTable::build(root)
        {
          let  mut exec = symtbl.generate_exec();

          exec.print_text();
//          symtbl.print();

          println!("");

          let  mut m = Machine::new();

          m.reset(0,1024*1024*16,&mut exec,"main",0);

//m.set_verbose();

          println!("machine runs");
//loop{
          m.keep_run();
//}
          println!("machine is finished");

          exec.print_memory();
        }

      else
        {
          println!("build is failed");
        }
    }
}




fn
open_and_print_tokens()
{
  let  args: Vec<String> = env::args().collect();

    for i in 1..args.len()
    {
      let  arg = &args[i];

        if let Ok(src) = crate::source_file::SourceFile::from_file(&arg)
        {
          println!("{} is opened",&arg);

            if let Ok(toks) = crate::token::tokenize::tokenize(&src)
            {
              crate::token::print_token_string(&toks);

              println!("\n--");

              crate::token::restore_token_string(&toks);
            }

          else
            {
              println!("tokenize is failed");

              return;
            }
        }
    }
}


fn
main()
{
  let  codes =
r#"

io input;
io video;
io time;

io sin_freq;
io sin_vol ;
io squ_freq;
io squ_vol ;
io saw_freq;
io saw_vol ;
io tri_freq;
io tri_vol ;
io noi_vol ;

io report;

str font u8 = {
0b00000000,
0b00011100,
0b00100010,
0b01000101,
0b01001001,
0b01010001,
0b00100010,
0b00011100,

0b00000000,
0b00001100,
0b00111100,
0b00001100,
0b00001100,
0b00001100,
0b00001100,
0b00011110,

0b00000000,
0b00011100,
0b01100011,
0b00000011,
0b00000011,
0b00001100,
0b00110000,
0b01111111,

0b00000000,
0b00011100,
0b01100011,
0b00000011,
0b00001100,
0b00000011,
0b01100011,
0b00011100,

0b00000000,
0b00000110,
0b00001110,
0b00010110,
0b00100110,
0b01111111,
0b00000110,
0b00000110,

0b00000000,
0b01111111,
0b01100000,
0b01111100,
0b01100011,
0b00000011,
0b01100011,
0b00011100,

0b00000000,
0b00011100,
0b01100011,
0b01100000,
0b01111110,
0b01100011,
0b01100011,
0b00011100,

0b00000000,
0b01111111,
0b00000011,
0b00000011,
0b00000110,
0b00011000,
0b00110000,
0b01100000,

0b00000000,
0b00011100,
0b01100011,
0b01100011,
0b00011100,
0b01100011,
0b01100011,
0b00011100,

0b00000000,
0b00011100,
0b01100011,
0b01100011,
0b00011111,
0b00000011,
0b01100011,
0b00011100,

};

field video_field 4*VIDEO_W*VIDEO_H;

const VIDEO_W = 400;
const VIDEO_H = 200;

const    UP_KEY = 0b0001;
const  LEFT_KEY = 0b0010;
const RIGHT_KEY = 0b0100;
const  DOWN_KEY = 0b1000;


fn
sleep(tm)
{
  var  base = time;

    loop
    {
        if (time-base) >= tm
        {
          break;
        }


      halt;
    }
}


fn
fill(x,y,w,h,pixel)
{
  var  base_ptr = video.ptr+(4*VIDEO_W*y)+(4*x);

    for y_off in h
    {
      var  ptr = base_ptr             ;
                 base_ptr += 4*VIDEO_W;

        for x_off in w
        {
          ptr.u32 = pixel;

          ptr += 4;
        }
    }
}


fn
main()
{
  video = video_field.ptr;

  fill(0,0,VIDEO_W,VIDEO_H,0xFF00FF);

  loop{halt;}
}


"#;


  compile_and_run(codes);
}




