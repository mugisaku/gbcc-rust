mod tokenizer;
mod token;
mod source_file;
mod syntax;
mod language;
mod ir;
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

parameter = IDENTIFIER & ":" & type_expression;

function_signature = "(" & [parameter & {"," & parameter}] & ")" & ["->" & type_expression];

function_definition = "fn" & IDENTIFIER & function_signature & block_statement;

variable_declaration = "var" & parameter;

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
  crate::ir::test::test_for();
/*
  let  dic_f = source_file::SourceFile::from(dic_s);
  let  txt_f = source_file::SourceFile::from(txt_s);

  let  dic = syntax::dictionary::Dictionary::from(&dic_f);

//  dic.print();

    if let Ok(toks) = tokenizer::tokenize(&txt_f)
    {
    }
*/
}




