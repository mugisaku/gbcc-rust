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

    match decl::Decl::read_as_root(s)
    {
   Ok(root)=>
    {
        match SymbolTable::build(root)
        {
       Ok(mut symtbl)=>
        {
          let  mut tmp = Vec::<u8>::new();

          tmp.resize(4*384*384,0);

          symtbl.add_img("image",384,384,tmp);

//          symtbl.print();

            match symtbl.generate_exec()
            {
          Ok(mut exec)=> 
            {
              println!("");

              let  mut m = Machine::new();

              m.reset(0,1024*1024*16,&mut exec,"main",0);

    //m.set_verbose();

              println!("machine runs");
    //loop{
              m.keep_run();
    //}
              println!("machine is finished");

//              exec.print_memory();
    //          exec.print_text();
            }
          Err(e)=>{e.print();}
            }
        }
      Err(e)=>
        {
          println!("build is failed");

          e.print();
        }
        }
    }
  Err(e)=>
    {
      e.print();

      println!("");
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

field video_field 4*VIDEO_W*VIDEO_H;

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
print_character(src_x,src_y,src_w,src_h,dst_x,dst_y)
{
//  var  dst_ptr_base = video+(4*VIDEO_W*dst_y)+(4*dst_x);

//  var  src_pitch = (image+4).u32;
//  var  src_ptr_base = image+8+(4*src_pitch*src_y)+(4*src_x);
/*
    for y in src_h
    {
      var  dst_ptr = dst_ptr_base;
                     dst_ptr_base += 4*VIDEO_W;

      var  src_ptr = src_ptr_base;
                     src_ptr_base += 4*src_pitch;

        for x in src_w
        {
          dst_ptr.u32 = src_ptr.u32;

          dst_ptr += 4;
          src_ptr += 4;
        }
    }
*/
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
main()
{
  video = video_field;

  var  x = 80;

  loop
  {
    ill(0,0,VIDEO_W,VIDEO_H,0);

         if (input&UP_KEY   )&&(y >=                8){y -= 8;}
    else if (input&DOWN_KEY )&&((y+24) <= (VIDEO_H-8)){y += 8;}
         if (input&LEFT_KEY )&&(x >=                8){x -= 8;}
    else if (input&RIGHT_KEY)&&((x+24) <= (VIDEO_W-8)){x += 8;}

    print_int(x,print14_unicode,16*8, 0,16,0xFFFFFFFF);
    print_int(y,print14_unicode,16*8,16,16,0xFFFFFFFF);

    print_character(0,0,24,24,x,y);

    halt;
  }


  loop{halt;}
}
"#;


  compile_and_run(codes);
}




