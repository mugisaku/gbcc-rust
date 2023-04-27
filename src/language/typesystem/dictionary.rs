

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#typesystem


function_pointer: 'fn & "(" & [type_note_list] & ")" & ["->" & type_note];

tuple: "(" & [type_note_list] & ")";

primitive:
    'bool
  | 'i8 | 'i16 | 'i32 | 'i64 | 'usize
  | 'u8 | 'u16 | 'u32 | 'u64 | 'isize
  | 'f32 | 'f64
  ;


type_note: primitive | tuple | function_pointer | .Identifier;

type_note_list: type_note & [{"," & type_note}];


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


   panic!("typesystem dictionary making error");
}




