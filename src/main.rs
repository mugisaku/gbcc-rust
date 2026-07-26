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
  use crate::language::exec::*;
  use crate::language::machine::*;

    match decl::DeclSet::read(s)
    {
   Ok(mut root)=>
    {
        match root.finalize()
        {
      Ok(())=>
        {
          root.print();

            match root.generate_exec()
            {
          Ok(mut exec)=> 
            {
              println!("");

              exec.print_text();

              println!("");

              let  mut m = Machine::new();

              m.set_verbose();

              m.reset(1024,&mut exec,"main");

              println!("\n  ****");

              println!("machine runs");

              m.keep_run();

              println!("machine is finished");

              println!("\n  ****");

              exec.print_memory();

              println!("");
            }
          Err(e)=>{e.print();}
            }
        }
      Err(e)=>{e.print();}
        }
    }
  Err(e)=>{e.print();}
    }
}




fn
main()
{
  let  codes =
r#"

class Test{

enum{
  Apple, Grape, Orange

}

}


const  c1 = 24;
const  c2 = c1+60;

static s_obj;
static x_obj[80];
static y_obj = Test::Grape;


/*
fn
main()
{
  sys.spawn(test,123,9);
  sys.input();

  return Test::Apple;
}
*/




static video;

static sin_freq;
static sin_vol ;
static squ_freq;
static squ_vol ;
static saw_freq;
static saw_vol ;
static tri_freq;
static tri_vol ;
static noi_vol ;

static report;

static video_field[4*VIDEO_W*VIDEO_H];

const VIDEO_W = 400;
const VIDEO_H = 200;

const    UP_KEY = 0b00000001;
const  LEFT_KEY = 0b00000010;
const RIGHT_KEY = 0b00000100;
const  DOWN_KEY = 0b00001000;
const     Z_KEY = 0b00010000;
const     X_KEY = 0b00100000;
const     C_KEY = 0b01000000;
const     V_KEY = 0b10000000;

const FRONT_DIR = 0;
const  LEFT_DIR = 1;
const RIGHT_DIR = 2;
const  BACK_DIR = 3;

const  CHR_W = 24;
const  CHR_H = 40;




fn
sleep(tm)
{
  var  base = sys.timer();

    loop
    {
        if (sys.timer()-base) >= tm
        {
          break;
        }


      halt;
    }
}


fn
dot(x,y,pixel)
{
  (video+(4*VIDEO_W*y)+(4*x)).u32 = pixel;
}


fn
draw_rect(x,y,w,h,pixel)
{
    for off in w
    {
      dot(x+off,y    ,pixel);
      dot(x+off,y+h-1,pixel);
    }


    for off in h
    {
      dot(x    ,y+off,pixel);
      dot(x+w-1,y+off,pixel);
    }
}


fn
print_int(i,f,x,y,w,pixel)
{
    if i == 0
    {
      f('0',x,y,pixel);

      return;
    }


    while i
    {
      f('0'+(i%10),x,y,pixel);

      x -=  w;
      i /= 10;
    }
}


fn
print8_unicode(u,x,y,pixel)
{
  var  ptr = FONT8_START+(8*u);

    for y_off in 8
    {
      var  bits = ptr.u8;
                  ptr += 1;

        for x_off in 8
        {
            if bits&0x80
            {
              dot(x+x_off,y+y_off,pixel);
            }


          bits <<= 1;
        }
    }
}


fn
print14_unicode(u,x,y,pixel)
{
  var  ptr = FONT14_START+(2*14*u);

    for y_off in 14
    {
      var  bits = ptr.u16;
                  ptr += 2;

        for x_off in 14
        {
            if bits&0x8000
            {
              dot(x+x_off,y+y_off,pixel);
            }


          bits <<= 1;
        }
    }
}


fn
print14_unicode_s(s,x,y,pixel)
{
    loop
    {
      var  u = s.u16;
               s += 2;

        if u == 0
        {
          break;
        }


      print14_unicode(u,x,y,pixel);

      x += 16;
    }
}


fn
print_character(dir,anim,dst_x,dst_y)
{
  var  src_x = 0;
  var  src_y = 0;
  var  rev = 0;

       if (dir == LEFT_DIR) {src_y = CHR_H;}
  else if (dir == RIGHT_DIR){src_y = CHR_H;  rev = 1;}
  else if (dir == BACK_DIR) {src_y = CHR_H*2;}

       if anim == 1{src_x = CHR_W;}
  else if anim == 3{src_x = CHR_W*2;}

  var  dst_ptr_base = video+(4*VIDEO_W*dst_y)+(4*dst_x);

  var  src_pitch = 4*(image+4).u32;
  var  src_ptr_base = image+8+(src_pitch*src_y)+(4*src_x);

    for y in CHR_H
    {
      var  dst_ptr = dst_ptr_base;
                     dst_ptr_base += 4*VIDEO_W;

      var  src_ptr = src_ptr_base;
                     src_ptr_base += src_pitch;

        if rev
        {
          dst_ptr += 4*CHR_W;

            for x in CHR_W
            {
              dst_ptr.u32 = src_ptr.u32;

              dst_ptr -= 4;
              src_ptr += 4;
            }
        }

      else
        {
            for x in CHR_W
            {
              dst_ptr.u32 = src_ptr.u32;

              dst_ptr += 4;
              src_ptr += 4;
            }
        }
    }
}


fn
fill(x,y,w,h,pixel)
{
  var  base_ptr = video+(4*VIDEO_W*y)+(4*x);

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
test(x,y)
{
    while x < (VIDEO_W-32)
    {
      draw_rect(x,y,32,32,0xFFFFFFFF);

      x += 8;

      halt;
    }
}


static  dir = 0;

static  x = 80;
static  y = 80;
static  anim = 0;
static  x_move_value = 0;
static  y_move_value = 0;


fn
video_proc()
{
  loop
  {
    fill(0,0,VIDEO_W,VIDEO_H,0);

    print_int(x,print14_unicode,16*8, 0,16,0xFFFFFFFF);
    print_int(y,print14_unicode,16*8,16,16,0xFFFFFFFF);

    print_character(dir,anim>>1&3,x,y);

    halt;
  }
}


fn
object_proc()
{
  loop
  {
    var  input = sys.input();

      if input&Z_KEY
      {
        sys.spawn(test,x,y);
      }


      if (x_move_value == 0) && (y_move_value == 0)
      {
             if (input&UP_KEY   ){dir =  BACK_DIR;  y_move_value = 24;}
        else if (input&DOWN_KEY ){dir = FRONT_DIR;  y_move_value = 24;}
             if (input&LEFT_KEY ){dir =  LEFT_DIR;  x_move_value = 24;}
        else if (input&RIGHT_KEY){dir = RIGHT_DIR;  x_move_value = 24;}
      }

    else
      {
          for _ in 2
          {
                 if dir ==  BACK_DIR{y_move_value -= 2;  y -= 2;}
            else if dir == FRONT_DIR{y_move_value -= 2;  y += 2;}
                 if dir ==  LEFT_DIR{x_move_value -= 2;  x -= 2;}
            else if dir == RIGHT_DIR{x_move_value -= 2;  x += 2;}

            anim += 1;
          }
      }


    halt;
  }
}


fn
main()
{
  video = video_field;

  sys.spawn(video_proc);
  sys.spawn(object_proc);

}


"#;


  compile_and_run(codes);
}




