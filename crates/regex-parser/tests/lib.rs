mod test{
    use super::parser;

    #[test]
    fn test_vim_regex_pattern(){
        vim_regex::pattern(r"abc \| abc").unwrap();
        vim_regex::pattern(r"abc \& abc").unwrap();
        vim_regex::pattern(r"[abc]*").unwrap();
    }

    #[test]
    fn test_vim_regex_piece(){
        vim_regex::piece(r"[abc]*").unwrap();
    }

    #[test]
    fn test_vim_regex_multi(){
        vim_regex::multi(r"*").unwrap();
        vim_regex::multi(r"\+").unwrap();
        vim_regex::multi(r"\?").unwrap();
        vim_regex::multi(r"\=").unwrap();
        vim_regex::multi(r"\@=").unwrap();
        vim_regex::multi(r"\@!").unwrap();
        vim_regex::multi(r"\@<=").unwrap();
        vim_regex::multi(r"\@<!").unwrap();
        vim_regex::multi(r"\@1<=").unwrap();
        vim_regex::multi(r"\@123<!").unwrap();
        vim_regex::multi(r"\@4<=").unwrap();
        vim_regex::multi(r"\@555555<!").unwrap();
        vim_regex::multi(r"\@>").unwrap();
        vim_regex::multi(r"\{1,6}").unwrap();
        vim_regex::multi(r"\{1,}").unwrap();
        vim_regex::multi(r"\{,6}").unwrap();
        vim_regex::multi(r"\{16}").unwrap();
        vim_regex::multi(r"\{1}").unwrap();
        vim_regex::multi(r"\{6}").unwrap();
        vim_regex::multi(r"\{16}").unwrap();
        vim_regex::multi(r"\{-1,6}").unwrap();
        vim_regex::multi(r"\{-1,}").unwrap();
        vim_regex::multi(r"\{-,6}").unwrap();
        vim_regex::multi(r"\{-16}").unwrap();
        vim_regex::multi(r"\{-1}").unwrap();
        vim_regex::multi(r"\{-6}").unwrap();
        vim_regex::multi(r"\{-16}").unwrap();
        vim_regex::multi(r"\{-}").unwrap();
        vim_regex::multi(r"\{}").unwrap();
    }

    #[test]
    fn test_vim_regex_special_char_remainder(){
        vim_regex::special_char_remainder(r"[abc]").unwrap();
    }

    parser!{
        pub grammar ccc() for str {
            pub rule start() -> () = "b" / "a" ** " " {}
        }
    }
    #[test]
    fn test_peg(){
        ccc::start("a a").unwrap();
        ccc::start("b").unwrap();
    }
}
