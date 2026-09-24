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
  use crate::language::project::*;
  use crate::language::exec::*;
  use crate::language::machine::*;

  let  mut pj = Project::new();

    match pj.add_source("",s)
    {
   Ok(())=>
    {
        match pj.compile()
        {
      Ok(())=>
        {
          pj.print();

          println!("");

            match pj.generate_exec()
            {
          Ok(mut exec)=> 
            {
              exec.print_text();

              println!("");

              let  mut m = Machine::new();

              m.set_verbose();

              m.reset(&mut exec,"main");

              println!("\n  ****");

              println!("machine runs");

              m.keep_run(800,0);

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




static    screen_address;
static offscreen_address;

static screen_status;

static video_field[VIDEO_W*VIDEO_H]: u32;

static  mouse_state;

fn  mouse_get_x(){return mouse_state>>16&0xFFFF;}
fn  mouse_get_y(){return mouse_state&0xFFFF;}
fn  mouse_is_pressing_l(){return mouse_state>>32&1;}
fn  mouse_is_pressing_r(){return mouse_state>>32&2;}


static rand_state;

fn
xorshift()
{
  rand_state ^= rand_state<<7;
  rand_state ^= rand_state>>9;

  return rand_state;
}




const SCREEN_W = 400;
const SCREEN_H = 320;
const VIDEO_W = SCREEN_W*2;
const VIDEO_H = SCREEN_H*2;


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
dot(x,y,pixel)
{
  (screen_address+(4*VIDEO_W*y)+(4*x)).u32ref = pixel;
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
fill_rect(x,y,w,h,pixel)
{
    for y_off in h{
    for x_off in w{
      dot(x+x_off,y+y_off,pixel);
    }}
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
      var  bits = ptr.u8ref;
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
      var  bits = ptr.u16ref;
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
      var  u = s.u16ref;
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
fill(x,y,w,h,pixel)
{
  var  base_ptr = screen_address+(4*VIDEO_W*y)+(4*x);

    for y_off in h
    {
      var  ptr = base_ptr             ;
                 base_ptr += 4*VIDEO_W;

        for x_off in w
        {
          ptr.u32ref = pixel;

          ptr += 4;
        }
    }
}


const
PIECE_SIZE = 8;

static
a_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
b_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
c_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
d_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
e_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,

}

static
f_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,0,0,0,0,1,1,
  1,1,0,0,0,0,1,1,

}

static
g_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,0,0,0,0,1,1,
  1,1,0,0,0,0,1,1,

}

static
h_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,1,1,1,
  0,0,0,0,1,1,1,1,
  0,0,0,1,1,1,1,1,
  0,0,1,1,1,1,1,1,
  0,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
i_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,1,1,1,1,
  0,0,0,0,1,1,1,1,
  0,0,0,1,1,1,1,1,
  0,0,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}

static
j_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,0,1,1,
  0,0,0,0,0,1,1,1,
  0,0,0,0,0,1,1,1,
  0,0,0,0,0,1,1,1,
  0,0,0,0,0,1,1,1,
  0,0,0,0,1,1,1,1,

}

static
k_piece[PIECE_SIZE*PIECE_SIZE]: u8{
  0,0,0,0,1,1,1,1,
  0,0,0,0,1,1,1,1,
  0,0,0,1,1,1,1,1,
  0,0,0,1,1,1,1,1,
  0,0,1,1,1,1,1,1,
  0,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

}


static
codes[80]: u32{};




static
pieces[PIECE_SIZE*PIECE_SIZE*80]: u8{
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,

  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,
  1,1,1,1,1,1,1,1,

  1,1,0,0,0,0,1,1,
  1,1,0,0,0,0,1,1,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,
  1,1,0,0,0,0,1,1,
  1,1,0,0,0,0,1,1,

};


const
WHITE = 0xFFFFFFFF;

const
BLUE = 0x00FF0000;

const
GREEN = 0x000FF00;

const
LGREEN = 0x003FFF3F;

const
TABLE_SIZE = 16;

static
table[TABLE_SIZE*TABLE_SIZE]: u8;

static
g_number_of_pieces;


fn
draw_table()
{
    for y in TABLE_SIZE{
    for x in TABLE_SIZE{
      var  i = get_value_unchecked(x,y);

      draw(i,x,y);
    }}


  draw_rect(8*cursor_x,8*cursor_y,8,8,WHITE);

  screen_status = 1;
}


fn
piece_get_unchecked(ptr,x,y)
{
  return ptr.u8ref[(PIECE_SIZE*y)+x];
}


fn
piece_get(ptr,x,y)
{
    if (x >= 0)
    && (y >= 0)
    && (x < PIECE_SIZE)
    && (y < PIECE_SIZE)
    {
      return piece_get_unchecked(ptr,x,y);
    }


  return 0;
}


fn
get_piece_ptr(i)
{
  return pieces[PIECE_SIZE*PIECE_SIZE*i].ptr;
}


fn
piece_put_unchecked(ptr,x,y,v)
{
  ptr.u8ref[(PIECE_SIZE*y)+x] = v;
}


fn
piece_copy(dst_ptr,src_ptr)
{
    for y in PIECE_SIZE{
    for x in PIECE_SIZE{
      var  v = piece_get_unchecked(src_ptr,x,y);

      piece_put_unchecked(dst_ptr,x,y,v);
    }}
}


fn
piece_copy_and_rot(dst_ptr,src_ptr)
{
    for y in PIECE_SIZE{
    for x in PIECE_SIZE{
      var  v = piece_get_unchecked(src_ptr,x,y);

      piece_put_unchecked(dst_ptr,PIECE_SIZE-1-y,x,v);
    }}
}


fn
draw(i,dst_x,dst_y)
{
  var  ptr = get_piece_ptr(i);

    for y in PIECE_SIZE{
    for x in PIECE_SIZE{
      var  v = piece_get_unchecked(ptr,x,y);

      var  pixel;

           if v == 0{pixel =   BLUE;}
      else if v == 1{pixel =  GREEN;}
      else if v == 2{pixel = LGREEN;}

      dot((PIECE_SIZE*dst_x)+x,(PIECE_SIZE*dst_y)+y,pixel);
    }}
}


fn
get_value_unchecked(x,y)
{
  return table[(TABLE_SIZE*y)+x];
}


fn
get_value(x,y)
{
    if (x >= 0)
    && (y >= 0)
    && (x <  TABLE_SIZE)
    && (y <  TABLE_SIZE)
    {
      return get_value_unchecked(x,y);
    }


  return 0;
}


fn
put_value_unchecked(x,y,v)
{
  table[(TABLE_SIZE*y)+x] = v;
}


fn
put_value(x,y,v)
{
    if (x >= 0)
    && (y >= 0)
    && (x <  TABLE_SIZE)
    && (y <  TABLE_SIZE)
    {
      put_value_unchecked(x,y,v);
    }
}


fn
get_code(i)
{
  var  ptr = get_piece_ptr(i);

  var  code = 0;

    for n in PIECE_SIZE
    {
      code <<= 1;

        if piece_get_unchecked(ptr,           n,           0) == 1{code |= 1<<24;}
        if piece_get_unchecked(ptr,           0,           n) == 1{code |= 1<<16;}
        if piece_get_unchecked(ptr,PIECE_SIZE-1,           n) == 1{code |= 1<< 8;}
        if piece_get_unchecked(ptr,           n,PIECE_SIZE-1) == 1{code |= 1    ;}
    }


  return code;
}


fn
find(bits)
{
    for i in g_number_of_pieces
    {
      var  code = codes[i];

      var  t = 0;

        if (code>>24)&0xFF == 0xFF{t |= 0b1000;}
        if (code>>16)&0xFF == 0xFF{t |= 0b0100;}
        if (code>> 8)&0xFF == 0xFF{t |= 0b0010;}
        if (code    )&0xFF == 0xFF{t |= 0b0001;}

        if t == bits
        {
          return i;
        }
    }


  return 0;
}


fn
find2(code)
{
    for i in g_number_of_pieces
    {
        if codes[i] == code
        {
          return i;
        }
    }


  return 0;
}


fn
process_edge(i)
{
  var  ptr = get_piece_ptr(i);

    for y in PIECE_SIZE{
    for x in PIECE_SIZE{
      var  i = piece_get_unchecked(ptr,x,y);

        if i == 1
        {
            if (piece_get(ptr,x  ,y-1) == 0)
            || (piece_get(ptr,x-1,y  ) == 0)
            || (piece_get(ptr,x+1,y  ) == 0)
            || (piece_get(ptr,x  ,y+1) == 0)
            {
              piece_put_unchecked(ptr,x,y,2);
            }
        }
    }}
}


fn
process_table()
{
    for y in TABLE_SIZE{
    for x in TABLE_SIZE{
      var  i = get_value_unchecked(x,y);

        if i == 0
        {
          var  bits = 0;

            if get_value(x  ,y-1) == 1{bits |= 0b1000;}
            if get_value(x-1,y  ) == 1{bits |= 0b0100;}
            if get_value(x+1,y  ) == 1{bits |= 0b0010;}
            if get_value(x  ,y+1) == 1{bits |= 0b0001;}


            if bits
            {
              var  new_i = find(bits);

              put_value_unchecked(x,y,new_i);
            }
        }
    }}
}


fn
process_table_stage2()
{
    for y in TABLE_SIZE{
    for x in TABLE_SIZE{
      var  i = get_value_unchecked(x,y);

        if i == 0
        {
          var  u = (codes[get_value(x  ,y-1)]<<24)&0xFF000000;
          var  l = (codes[get_value(x-1,y  )]<< 8)&0x00FF0000;
          var  r = (codes[get_value(x+1,y  )]>> 8)&0x0000FF00;
          var  d = (codes[get_value(x  ,y+1)]>>24)&0x000000FF;

          var  new_i = find2(u|l|r|d);

            if new_i
            {
              put_value_unchecked(x,y,new_i);
            }
        }
    }}
}


static cursor_x;
static cursor_y;


fn
flip()
{
  var  tmp = offscreen_address                       ;
             offscreen_address = screen_address      ;
                                 screen_address = tmp;

  screen_status = 1;
}


static g_key_lock;

fn
process_input()
{
    if mouse_is_pressing_l()
    {
      var  x = mouse_get_x()/PIECE_SIZE;
      var  y = mouse_get_y()/PIECE_SIZE;

        if (x < TABLE_SIZE)
        && (y < TABLE_SIZE)
        {
          var  ptr = table[(TABLE_SIZE*y)+x].ptr;

            if ptr.u8ref != 1
            {
              ptr.u8ref = 1;

              draw_table();
            }
        }
    }

  else
    if mouse_is_pressing_r()
    {
      var  x = mouse_get_x()/PIECE_SIZE;
      var  y = mouse_get_y()/PIECE_SIZE;

        if (x < TABLE_SIZE)
        && (y < TABLE_SIZE)
        {
          var  ptr = table[(TABLE_SIZE*y)+x].ptr;

            if ptr.u8ref != 0
            {
              ptr.u8ref = 0;

              draw_table();
            }
        }
    }


  var  input = sys.input();

    if input&Z_KEY
    {
      var  ptr = table[(TABLE_SIZE*cursor_y)+cursor_x].ptr;

        if ptr.u8ref != 1
        {
          ptr.u8ref = 1;

          draw_table();
        }
    }

  else
    if input&X_KEY
    {
      var  ptr = table[(TABLE_SIZE*cursor_y)+cursor_x].ptr;

        if ptr.u8ref != 0
        {
          ptr.u8ref = 0;

          draw_table();
        }
    }

  else
    if input&C_KEY
    {
        if g_key_lock == 0
        {
          process_table();

          draw_table();

          g_key_lock = 1;
        }
    }

  else
    if input&V_KEY
    {
        if g_key_lock == 0
        {
          process_table_stage2();

          draw_table();

          g_key_lock = 1;
        }
    }

  else
    {
      g_key_lock = 0;

      var  flag = 0;

           if (input& LEFT_KEY) && (cursor_x >              0){  cursor_x -= 1;  flag = 1;}
      else if (input&RIGHT_KEY) && (cursor_x < (TABLE_SIZE-1)){  cursor_x += 1;  flag = 1;}

           if (input&  UP_KEY) && (cursor_y >              0){  cursor_y -= 1;  flag = 1;}
      else if (input&DOWN_KEY) && (cursor_y < (TABLE_SIZE-1)){  cursor_y += 1;  flag = 1;}

        if flag
        {
          draw_table();
        }
    }
}


fn
process(i,ptr)
{
  piece_copy(get_piece_ptr(i),ptr);
  piece_copy_and_rot(get_piece_ptr(i+1),get_piece_ptr(i  ));
  piece_copy_and_rot(get_piece_ptr(i+2),get_piece_ptr(i+1));
  piece_copy_and_rot(get_piece_ptr(i+3),get_piece_ptr(i+2));

  return i+4;
}


fn
init()
{
  for y in TABLE_SIZE{
  for x in TABLE_SIZE{
    put_value(x,y,xorshift()%(g_number_of_pieces-1));
  }}
}


fn
main()
{
     screen_address = video_field.ptr             ;
  offscreen_address = video_field.ptr+(4*SCREEN_W);

  var  i = process(3,a_piece.ptr);
       i = process(i,b_piece.ptr);
       i = process(i,c_piece.ptr);
       i = process(i,d_piece.ptr);
       i = process(i,e_piece.ptr);
       i = process(i,f_piece.ptr);
       i = process(i,g_piece.ptr);
       i = process(i,h_piece.ptr);
       i = process(i,i_piece.ptr);
  var  n = process(i,j_piece.ptr);
       n = process(n,k_piece.ptr);

  g_number_of_pieces = n;

    for ii in i
    {
      draw(ii,ii,0);
    }


  draw(i  ,0,1);
  draw(i+4,0,2);

  draw(i+1  ,2,1);
  draw(i+1+4,1,1);
  draw(i+3  ,1,2);
  draw(i+3+4,2,2);

  draw(i+2  ,3,2);
  draw(i+2+4,3,1);

//  init();

  draw_table();

    for i in g_number_of_pieces
    {
      codes[i] = get_code(i);
    }


  var  last;

    loop
    {
      var  now = sys.timer();

        if now > (last+80)
        {
          last = now;

          process_input();
        }


      halt;
    }
}




"#;


  compile_and_run(codes);
}




