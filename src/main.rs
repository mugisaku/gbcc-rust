mod tokenizer;
mod token;
mod source_file;
mod syntax;
mod language;
mod debug;


static dic_s: &'static str =
r#"
top_element = {expression};

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
 1*2+3
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
use language::expression::ProcessedExpression;
use syntax::parser::Directory;
use syntax::parser::Cursor;
        if let Some(dir) = syntax::parser::parse(&dic,&toks)
        {
          dir.print();
          let mut  cur = Cursor::from(&dir);

            while let Some(te_dir) = cur.seek_directory("top_element")
            {
              let mut  te_cur = Cursor::from(&te_dir);

                if let Some(e_dir) = te_cur.seek_directory("expression")
                {
                  let  e = Expression::from(&e_dir);

                  e.print();
let  pe = ProcessedExpression::from(e);
println!("");
pe.print();
let v = pe.evaluate();
println!("");
v.print();
println!("");
                }


              cur.advance(1);
            }
        }
    }
}




