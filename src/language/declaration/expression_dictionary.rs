

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#expression

path: .Identifier & [{"::" & .Identifier}];

operand_core: path | .Integer | .Floating | .Character | .String | ("(" & expression & ")");

unary_operator: "!" | "++" | "--" | "-" | "~" | "*" | "&";

binary_operator:
    "+"
  | "-"
  | "*"
  | "/"
  | "%"
  | "=="
  | "||" | "|"
  | "&&" | "&"
  | "^"
  | "<<" | "<=" | "<"
  | ">>" | ">=" | ">"
  | "!="
  ;


access         : "." & .Identifier;
subscript      : "[" & expression & "]";
call           : "(" & [expression & [{"," & expression}]] & ")";
increment      : "++";
decrement      : "++";

postfix_operator: access | subscript | call | increment | decrement;

operand: [{unary_operator}] & operand_core & [{postfix_operator}];

expression: operand & [{binary_operator & operand}];

"##;



pub fn
get_dictionary()-> &'static Dictionary
{
  static  mut DIC_OPT: Option<Dictionary> = None;

    unsafe
    {
        if let None = DIC_OPT
        {
            if let Ok(mut tmp_dic) = Dictionary::make_from_string(&DIC_S)
            {
                if tmp_dic.test().is_ok()
                {
                  DIC_OPT = Some(tmp_dic);
                }
            }
        }


        if let Some(dic) = &DIC_OPT
        {
          return dic;
        }
    }

 
  panic!("expression dictionary initialize error");
}




