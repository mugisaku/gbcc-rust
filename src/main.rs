mod tokenizer;
mod token;
mod source_file;
mod syntax;
mod language;
mod virtual_machine;
mod debug;


static dic_s: &'static str =
r#"
top_element =
  function_definition |
    struct_definition |
      enum_definition |
  variable_definition;

statement =
                 ";" |
        if_statement |
     while_statement |
     break_statement |
  continue_statement |
     block_statement |
     return_statement;

if_statement       = "if" & expression & block_statement & [{"else" & ["if"] & block_statement}];
while_statement    = "while" & block_statement;
break_statement    = "break";
continue_statement = "continue";
block_statement    = "{" & [{statement | (variable_declaration & ["=" & expression]) | expression}] &"}";
return_statement   = "return" & [expression];


type_expression = IDENTIFIER;

parameter_list = "(" & [variable_declaration & {"," & variable_declaration}] & ")";

return_value_type = "->" & type_expression;

function_definition = "fn" & IDENTIFIER & parameter_list & [return_value_type] & block_statement;

variable_declaration = "var" & IDENTIFIER & ":" & type_expression;

struct_definition = "struct" & IDENTIFIER & "{" & [{}] & "}";

enum_definition = "enum" & IDENTIFIER & "{" & [{}] &  "}";


operand = IDENTIFIER | INTEGER_LITERAL | FLOATING_LITERAL | LETTER_LITERAL | CHARACTER_LITERAL | STRING_LITERAL | ("(" & expression & ")");

unary_operator = "!" | "++" | "--" | "-" | "~" | "*" | "&";

binary_operator =
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

access    = "." & IDENTIFIER;
subscript = "[" & expression & "]";
call      = "(" & [expression & [{"," & expression}]] & ")";

primary_operation = access | subscript | call;

unary_operation = [{unary_operator}] & operand & [{primary_operation}];

expression = unary_operation & [{binary_operator & unary_operation}];

"#;


static txt_s: &'static str =
r#"

fn
test()-> slen
{
  return 1+2*3
}


"#;


fn
main()
{
  let  dic_f = source_file::SourceFile::from(dic_s);
  let  txt_f = source_file::SourceFile::from(txt_s);

  let  dic = syntax::dictionary::Dictionary::from(&dic_f);

//  dic.print();

    if let Ok(toks) = tokenizer::tokenize(&txt_f)
    {
use language::expression::Expression;
use language::space::*;
use syntax::parser::Directory;
use syntax::parser::Cursor;
use virtual_machine::assembly::*;
use virtual_machine::processor::Processor;
        if let Some(dir) = syntax::parser::parse(&dic,&toks)
        {
//dir.print();
          let  sp = Space::from(&dir);
//sp.print();
//let  _ = crate::virtual_machine::object::compile(&sp);
let mut  note = Note::new();
note.put_label("start");
note.put_relpos16("function");
note.put_pshu32(80);
note.put_pshu32(20);
note.put_pshu32(24);
note.put_cal();
note.put_pshu8(16);
note.put_ssp();
note.put_prnu64();
note.put_hlt();
note.put_label("function");

note.put_psh8();
note.put_maa();
note.put_ld64();
note.put_pshu8(16);
note.put_maa();
note.put_ld64();
note.put_addu();

note.put_psh8();
note.put_retd();
note.put_hlt();


note.print();
print!("\n");
  if let Ok(img) = note.assemble()
  {
    print_as_machine_code(&img);

let mut  proc = Processor::new();

proc.renew_memory(1024);
proc.load_image(&img);
proc.reset();
while !proc.is_halted()
{
proc.step();
}
  }


        }
    }
}




