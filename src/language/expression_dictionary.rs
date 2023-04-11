

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"


operand_core: .Identifier | .Integer | .Floating | .Character | .String | ("(" & expression & ")");

prefix_operator: "!" | "++" | "--" | "-" | "~" | "*" | "&";

binary_operator:
    "+=" |"+"
  | "-=" | "-"
  | "*=" | "*"
  | "/=" | "/"
  | "%=" | "%"
  | "==" | "="
  | "|=" | "||" | "|"
  | "&=" | "&&" | "&"
  | "^=" | "^"
  | "<<=" |"<<" | "<=" | "<"
  | ">>=" |">>" | ">=" | ">"
  | "!="
  ;

access   : "." & .Identifier;
subscript: "[" & expression & "]";
call     : "(" & [expression & [{"," & expression}]] & ")";

postfix_operator: access | subscript | call;

operand: [{prefix_operator}] & operand_core & [{postfix_operator}];

expression_tail: binary_operator & operand;

expression: operand & [{expression_tail}];

"##;



pub fn
get_dictionary()-> Dictionary
{
    if let Ok(mut dic) = Dictionary::make_from_string(&DIC_S)
    {
        if dic.test().is_ok()
        {
            if dic.set_main("expression").is_ok()
            {
              return dic;
            }
        }


      println!("expression dictionary test is failed");
    }


   panic!("expression dictionary making error");
}




