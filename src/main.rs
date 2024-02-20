mod token;
mod source_file;
mod syntax;
mod language;
mod ir;
mod debug;

use std::env;


fn
open_and_print_tokens()
{
  let  args: Vec<String> = env::args().collect();

    for i in 1..args.len()
    {
      let  arg = &args[i];

        if let Ok(src) = crate::source_file::SourceFile::open(&arg)
        {
          println!("{} is opened",&arg);

            if let Ok(toks) = crate::token::tokenize::tokenize(&src)
            {
//              crate::token::print_token_string(&toks);
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
print_help()
{
  println!("e <expression> -- evaluate <expression> and print its result.");
  println!("x <statement> -- execute <statement>.");
  println!("t <...> -- make type from <...>.");
  println!("h -- print this text.");
  println!("q -- quit this program.");
}


fn
divide(s: &str)-> (&str,&str)
{
  let  mut  it = s.chars();
  let  mut pos = 0;

    while let Some(c) = it.next()
    {
        if ((c >= 'a') && (c <= 'z'))
        || ((c >= 'A') && (c <= 'Z'))
        ||  (c >= '_')
        {
          pos += 1;

          continue;
        }


      break;
    }


  (&s[0..pos],&s[pos..])
}


fn
evaluate(s: &str)
{
  use crate::language::expression::Expression;

//    if let Ok(e) = Expression::make_from_string(s)
    {
//      e.print();

/*
      let  v = e.to_value(None);

      print!(" = ");

      v.print();
*/
      print!("\n");
    }
}


fn
execute(s: &str)
{
  use crate::language::statement::Statement;

//    if let Ok(st) = Statement::make_from_string(s)
    {
//      st.print(0);

      print!("\n");
    }
}


fn
execute_program(s: &str)
{
  use crate::language::declaration::Space;

//    if let Ok(lib) = Library::make_from_string(s)
    {
//      lib.print();

      print!("\n");
    }
}


fn
load()
{
  use crate::language::declaration::Space;
  use crate::language::typesystem::TypeCode;

TypeCode::new_enum("Test",vec![("a",0),("v",2)]).print();

  let  args: Vec<String> = env::args().collect();

  let  mut buf = String::new();

    for i in 1..args.len()
    {
      buf.push_str(&args[i]);
    }


/*
  let  unff = Unff::from(&buf);

  unff.print();

  println!("");

  let  s = unff.to_string();

  println!("{}",&s);
*/

//  let  new_unff = Unff::from(&s);

//  new_unff.print();

  println!("");
/*
  let  mut lib = Library::new();

  use std::io::Read;

    if let Ok(mut f) = std::fs::File::open("test.g")
    {
      let  mut s = String::new();

      let  _ = f.read_to_string(&mut s);

        if let Ok(lib) = Library::make_from_string(&s)
        {
          lib.print();

          print!("\n");

            if let Ok(coll) = language::compile::compile(&lib)
            {
              coll.print();
            }
        }
    }
*/

}


fn
type_make(s: &str)
{
  use crate::language::typesystem::TypeItem;

//    if let Ok(t) = TypeItem::make_from_string(s)
    {
//      t.print();

      print!("\n");
    }
}


fn
main()
{
load();

return; 
/*
  println!("--GBCC Interactive Interpreter--");
  println!("type <h>, list command.");

  let  mut buf = String::new();

    loop
    {
      use std::io;

        if let Ok(sz) = io::stdin().read_line(&mut buf)
        {
            if (sz != 0) && (buf != "\n")
            {
              let  (cmd,arg) = divide(buf.trim());

                if cmd == "q"
                {
                  println!("");

                  return;
                }


              println!("\n\n** result **");

                if cmd == "e"
                {
                  evaluate(arg);
                }

              else
                if cmd == "t"
                {
                  type_make(arg);
                }

              else
                if cmd == "x"
                {
                  execute(arg);
                }

              else
                if cmd == "p"
                {
                  execute_program(arg);
                }

              else
                if cmd == "l"
                {
                  load();
                }

              else
                if cmd == "h"
                {
                  print_help();
                }

              else
                {
                  println!("{} is unknown command.",cmd);
                }


              println!("** end of result **\n\n");
            }


          buf.clear();
        }
    }
*/
}




