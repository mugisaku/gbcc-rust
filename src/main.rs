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

              m.reset(&mut exec,"main");

              println!("\n  ****");

              println!("machine runs");

              m.keep_run(800);

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

mod Test{

enum{
  Apple, Grape, Orange

}

}


const  c1 = 24;
const  c2 = c1+60;

static x_obj[80];
static y_obj = Test::Grape;


fn
main()
{
  var  tmp[5]: u8{0xFFFF};

  var  ptr = tmp.ptr;

  return tmp[0];
}


"#;


  compile_and_run(codes);
}




