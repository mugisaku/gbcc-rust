

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#statement

statement: ";"
  |  if
  | while
  | for
  | break
  | continue
  | block
  | return
  | fn
  | var
  | struct
  | union
  | enum
  | expression::expression
  ;

if      : 'if -> expression::expression & block & [{'else & ['if] & block}];
while   : 'while -> block;
for     : 'for -> block;
break   : 'break;
continue: 'continue;
block   : "{" & [statement & {statement}] & "}";
return  : 'return -> [expression::expression];

parameter: .Identifier & ":" & typesystem::type_note;
parameter_list: "(" & [parameter & [{"," & parameter}]] & ")";

fn    : 'fn -> .Identifier & parameter_list & ["->" & typesystem::type_note] & block;
var   : 'var -> .Identifier & [":" & typesystem::type_note] & ["=" & expression::expression] & ";";
struct: 'struct -> .Identifier;
union : 'union -> .Identifier;
enum  : 'enum -> .Identifier;

primary_statement: fn | var | struct | union | enum;


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


   panic!("statement dictionary making error");
}




