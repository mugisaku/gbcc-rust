mod token;
mod source_file;
mod syntax;
//mod language;
//mod ir;
mod debug;

use std::env;


static dic_s: &'static str =
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


type_expression: .Identifier;

parameter: .Identifier & ":" & type_expression;

function_signature: "(" & [parameter & {"," & parameter}] & ")" & ["->" & type_expression];

function_definition: 'fn -> .Identifier & function_signature & block_statement;

variable_declaration: 'var & parameter;

struct_definition: 'struct -> .Identifier & "{" & [{}] & "}";

enum_definition: 'enum -> .Identifier & "{" & [{}] &  "}";


operand: .Identifier | .Integer | .Floating | .Character | .String | ("(" & expression & ")");

unary_operator: "!" | "++" | "--" | "-" | "~" | "*" | "&";

binary_operator:
  "+=" | "+" |
  "-=" | "-" |
  "*=" | "*" |
  "/=" | "/" |
  "%=" | "%" |
  "==" | "!=" | "=" |
  "||" | "|=" | "|" |
  "&&" | "&=" | "&" |
  "^=" | "^" |
  "<<=" |"<<" | "<=" | "<" |
  ">>=" |">>" | ">=" | ">" ;

access   : "." & .Identifier;
subscript: "[" & expression & "]";
call     : "(" & [expression & [{"," & expression}]] & ")";

primary_operation: access | subscript | call;

unary_operation: [{unary_operator}] & operand & [{primary_operation}];

expression: unary_operation & [{binary_operator & unary_operation}];

"##;


static txt_s: &'static str =
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
main()
{
  use crate::syntax::dictionary::{
    Dictionary,
  };

    if let Ok(dic) = Dictionary::make_from_string(&dic_s)
    {
      dic.print();
    }

  else
    {
      println!("");
    }
}




