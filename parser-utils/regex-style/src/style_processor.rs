// use std::marker::Sized;

pub enum Target {
    Perl,
    Emca,
}

pub static TARGET_MODE: Target = Target::Perl;

// pub trait RegexConverter: Sized {
pub trait RegexConverter {
    fn process_branches(&self, branches: Vec<String>) -> String;
    fn process_concats(concats: Vec<String>) -> String;
    fn process_picese(picese: Vec<String>) -> String;
}
