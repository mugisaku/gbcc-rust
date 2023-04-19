mod token;
mod source_file;
mod syntax;
mod language;
//mod ir;
mod debug;

use std::env;


static DIC_S: &'static str =
r##"
top_element:
  function_definition |
    struct_definition |
      enum_definition |
  variable_definition;

statement:
                 ";" |
        if_statement |
     while_statement |
     break_statement |
  continue_statement |
     block_statement |
     return_statement;

if_statement      : 'if -> expression & block_statement & [{'else & ['if] & block_statement}];
while_statement   : 'while -> block_statement;
break_statement   : 'break;
continue_statement: 'continue;
block_statement   : "{" & [{statement | (variable_declaration & ["=" & expression]) | expression}] &"}";
return_statement  : 'return -> [expression];


function_definition: 'fn -> .Identifier & function_signature & block_statement;

variable_declaration: 'var & parameter;


"##;


static TXT_S: &'static str =
r"

fn
test()-> slen
{
  return 1+2*3
}


";


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
  use crate::language::context::Context;

    if let Ok(e) = Expression::make_from_string(s)
    {
      e.print();

      let  v = e.to_value(None);

      print!(" = ");

      v.print();

      print!("\n");
    }
}


fn
type_make(s: &str)
{
  use crate::language::typesystem::TypeNote;

    if let Ok(t) = TypeNote::make_from_string(s)
    {
      t.print();

      print!("\n");
    }
}


fn
main()
{
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
}




