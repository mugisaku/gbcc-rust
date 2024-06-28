

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#statement

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


expression_or_assign: expression::expression & [assign_operator & expression::expression];


statement: ";"
  | break
  | continue
  | if
  | while
  | for
  | loop
  | statement_list
  | return
  | fn
  | var
  | static
  | const
  | struct
  | union
  | enum
  | alias
  | expression_or_assign
  ;


break   : 'break;
continue: 'continue;
return  : 'return -> [expression::expression];


if: 'if -> expression::expression & statement_list & [{'else & 'if & expression::expression & statement_list} & ['else & statement_list]];

statement_list: "{" & [{statement}] & "}";

loop : 'loop -> statement_list;
while: 'while -> expression::expression & statement_list;
for  : 'for;


parameter: .Identifier & ":" & typesystem::type;
parameter_list: "(" & [parameter & [{"," & parameter}]] & ")";

fn    : 'fn -> .Identifier & parameter_list & ["->" & typesystem::type] & statement_list;
var   : 'var -> .Identifier & [":" & typesystem::type] & ["=" & expression::expression];
static: 'static -> .Identifier & ":" & typesystem::type & "=" & expression::expression;
const : 'const  -> .Identifier & ":" & typesystem::type & "=" & expression::expression;
struct: 'struct -> .Identifier & member_list;
union : 'union -> .Identifier & member_list;
enum  : 'enum -> .Identifier & enumerator_list;
alias : 'alias -> .Identifier & ":" & typesystem::type;

member_list: "{" & [{parameter & [","]}] & "}";

enumerator: .Identifier & ["=" & expression::expression];
enumerator_list: "{" & [{enumerator & [","]}] & "}";

primary_statement: fn | var | static | const | struct | union | enum | alias;


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


   panic!("statement dictionary making error");
}




