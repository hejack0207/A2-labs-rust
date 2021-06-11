// #[macro_use]
// extern crate peg;
use peg::{self, parser};
// use std::marker::Sized;

use crate::style_processor::{TARGET_MODE, Target, RegexConverter};

pub struct EmcaConverter{
    verbose: bool,
}

pub struct PerlConverter;

pub enum Converter{
    EmcaConverter,
    PerlConverter,
}

impl RegexConverter for EmcaConverter {
    fn process_branches(&self, branches: Vec<String>) -> String{
        let ret = branches.join(" | ");
        println!("{}", ret);
        ret
        // EmcaConverter::process_branches(branches)
    }
    fn process_concats(concats: Vec<String>) -> String{
        concats.join(" & ")
    }
    fn process_picese(picese: Vec<String>) -> String{
        picese.join("")
    }
}

impl RegexConverter for PerlConverter {
    fn process_branches(&self, branches: Vec<String>) -> String{
        let ret = branches.join(" | ");
        println!("{}", ret);
        ret
        // EmcaConverter::process_branches(branches)
    }
    fn process_concats(concats: Vec<String>) -> String{
        concats.join(" & ")
    }
    fn process_picese(picese: Vec<String>) -> String{
        picese.join("")
    }
}

impl RegexConverter for Converter {
    fn process_branches(&self, branches: Vec<String>) -> String{
        match self {
            Converter::EmcaConverter => EmcaConverter{ verbose: true}.process_branches(branches),
            _ => "".to_string(),
        }  
    }
    fn process_concats(concats: Vec<String>) -> String{
        EmcaConverter::process_concats(concats)
    }
    fn process_picese(picese: Vec<String>) -> String{
        EmcaConverter::process_picese(picese)
    }
}

pub static mut CONVERTER: Converter = Converter::EmcaConverter;

struct Holder<T: RegexConverter>{
    pub data: T,
}

trait HolderX<T: RegexConverter> {
    fn set_converter(t: &T) -> Holder<T>;

    fn get_converter(&self);
}

parser!{
    pub grammar vim_regex() for str { //c: &Converter

        pub rule pattern() -> String = _ vb:( branch() ** (_ r"\|" _ )) _ { 
                // match TARGET_MODE {
                //     Target::Perl => vb.join(" | "),
                //     Target::Emca => "".to_string(),
                // }
                unsafe {
                    CONVERTER.process_branches(vb)
                    // c.process_branches(vb)
                    // holder.data.process_branches(vb)
                    // match CONVERTER {
                    //     Converter::EmcaConverter => Converter::EmcaConverter.process_branches(vb),
                    // }
                }
            }

        pub rule branch() -> String = vc:(concat() ** ( _ r"\&" _ )) {
                Converter::process_concats(vc)
                // EmcaConverter::process_concats(vc)
            }

        pub rule concat() -> String = vp:(piece() *) {
                Converter::process_picese(vp)
                // EmcaConverter::process_picese(vp)
            }

        pub rule piece() -> String = a:atom() m:multi() {
                format!("{}{}", a, m)
            }
            / a:atom() {
                a.to_string()
            }

        pub rule atom() -> &'input str = a:ordinary_atom()  { a }
            / r"\(" _ pattern() _ r"\)"  { "" }
            / r"\%(" _ pattern() _ r"\)" { "" }
            / r"\z(" _ pattern() _ r"\)" { "" }

        pub rule ordinary_atom() ->&'input str = c:( special_char() 
            / character_class() 
            / character_class_nl() 
            / special_char_remainder() 
            / engine_flag() ) { c }

        pub rule special_char() -> &'input str = ( r"^" / r"\^" / r"\_^"
            / r"$" / r"\$" / r"\_$"
            / r"." / r"\_."
            / r"\<" / r"\>"
            / r"\zs" / r"\ze"
            / r"\%^" / r"\%$" / r"\%V" / r"\%#" / r"\%'m" / r"\%" digit()+ ['l'|'c'|'v'] ) {"special_char"}

        pub rule character_class() -> &'input str = ( r"\i" / r"\I" / r"\k" / r"\K" / r"\f" / r"\F" / r"\p" / r"\P" 
            / r"\s" / r"\S" / r"\d" / r"\D" / r"\x" / r"\X" / r"\o" / r"\O" / r"\w" / r"\W" 
            / r"\h" / r"\H" / r"\a" / r"\A" / r"\l" / r"\L" / r"\u" / r"\U" ) { "character_class" }

        pub rule character_class_nl() -> &'input str = ( r"\_i" / r"\_I" / r"\_k" / r"\_K" / r"\_f" / r"\_F" / r"\_p" / r"\_P" 
            / r"\_s" / r"\_S" / r"\_d" / r"\_D" / r"\_x" / r"\_X" / r"\_o" / r"\_O" / r"\_w" / r"\_W" 
            / r"\_h" / r"\_H" / r"\_a" / r"\_A" / r"\_l" / r"\_L" / r"\_u" / r"\_U" ) { "character_class_nl" }

        // "\g,\j,\q,\y"
        pub rule special_char_remainder() -> &'input str = c:$( r"\e" / r"\t" / r"\r" / r"\b" / r"\n"
            / r"\" ['0'..='9']
            / r"\z" ['0'..='9']
            / ['a'..='z' | 'A'..='Z' | '0'..='9']
            / r"[" "^"? ['a'..='z' | 'A'..='Z' | '0'..='9']* "]"
            / r"\%[" ['a'..='z' | 'A'..='Z' | '0'..='9'] "]"
            / r"\c" / r"\C" / r"\Z" / r"\m" / r"\M" / r"\v" / r"\V"
            / r"\%" ['d'|'x'|'o'|'u'|'U'|'C'] digit()+ ) { c }

        pub rule multi() -> &'input str = "*"  { "*" }
            / r"\+"  { "+" }
            / r"\="  { "?" }
            / r"\?"  { "?" }
            / r"\{" rul:("-"?) min:$(digit()+)? ","? max:$(digit()+)? r"}" { "tbd" }
            / r"\@=" { "tdb" }
            / r"\@!"  { "tbd" }
            / r"\@" prechars:$(digit()+)? "<" ['='|'!']  { "tbd" }
            / r"\@>"  { "tbd" }

        rule _() -> String = [' ' | '\n']* { "".to_string() }

        pub rule engine_flag() ->&'input str = $(r"\%#=" ['0'|'1'|'2'])

        pub rule digit() = ['0'..='9']
    }
}

