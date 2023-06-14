

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#expression

operand_core: .Identifier | .Integer | .Floating | .Character | .String | ("(" & expression & ")");

prefix_operator: "!" | "++" | "--" | "-" | "~" | "*" | "&";

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

assign_operator:
    "="
  | "+="
  | "-="
  | "*="
  | "/="
  | "%="
  | "|=" 
  | "&="
  | "^="
  | "<<="
  | ">>="
  ;

name_resolution: "::" & .Identifier;
access         : "." & .Identifier;
subscript      : "[" & expression & "]";
call           : "(" & [expression & [{"," & expression}]] & ")";
increment      : "++";
decrement      : "++";

postfix_operator: access | subscript | call | name_resolution | increment | decrement;

operand: [{prefix_operator}] & operand_core & [{postfix_operator}];

expression_tail: binary_operator & operand;

expression: operand & [{expression_tail}] & [assign_operator & operand & [{expression_tail}]];

"##;



pub fn
get_dictionary()-> &'static Dictionary
{
  static  mut dic_opt: Option<Dictionary> = None;

    unsafe
    {
        if let None = dic_opt
        {
            if let Ok(mut tmp_dic) = Dictionary::make_from_string(&DIC_S)
            {
                if tmp_dic.test().is_ok()
                {
                  dic_opt = Some(tmp_dic);
                }
            }
        }


        if let Some(dic) = &dic_opt
        {
          return dic;
        }
    }

 
  panic!("expression dictionary initialize error");
}




