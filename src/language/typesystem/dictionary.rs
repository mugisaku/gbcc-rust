

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"


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
get_dictionary()-> Dictionary
{
    if let Ok(mut dic) = Dictionary::make_from_string(&DIC_S)
    {
        if dic.test().is_ok()
        {
            if dic.set_main("type_note").is_ok()
            {
              return dic;
            }
        }


      println!("typesystem dictionary test is failed");
    }


   panic!("typesystem dictionary making error");
}




