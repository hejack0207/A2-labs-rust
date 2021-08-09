use peg::{self, parser};

parser!{
    pub grammar vim_syntax() for str {
        pub rule syntax_file() = ""

        rule syn_keyword() = r"syntax" _ "keyword" _ group_name() _ syn_arguments_common()? _ keyword()+ _ syn_arguments_common()?

        rule syn_match() = r"syntax" _ "match" _ group_name() _ syn_arguments_match()? _ syn_excludenl()? _ syn_keepend()? _ syn_pattern() syn_arguments_match()?

        rule syn_region() = r"syntax" _ "region" _ group_name() _ syn_arguments_region()? _ matchgroup()? 
            _ syn_keepend()? _ syn_extend()?
            _ syn_excludenl()? _ (r"start=" _? syn_pattern() _ )+ ( r"skip=" syn_pattern() )? _ (r"end=" _? syn_pattern() _)+ syn_arguments_region()?

        rule matchgroup() = r"matchgroup=" (group_name() / "NONE")

        rule syn_extend() = r"extend"

        rule group_name() =

        rule group_name_pattern() = group_name() / syn_pattern()

        rule syn_arguments_common() = "conceal" / "cchar="['\x20'..='\x7f'] / "contained" 
            / "transparent" 
            / "containedin=" (group_name() ",")* group_name() 
            / "nextgroup=" (group_name() ",")* group_name() 
            / "skipwhite" / "skipnl" / "skipempty"

        rule syn_arguments_match() = syn_arguments_common() 
            / "contains=" ("ALL" / "ALLBUT" (group_name_pattern() ",")* group_name_pattern() 
                / ( "TOP" / "CONTAINED") ((group_name_pattern() ",")* group_name_pattern())? ) 
            / "fold" / "display" / "extend"

        rule syn_arguments_region() = syn_arguments_match() / "oneline" / "concealends"

        rule syn_excludenl() = r"excludenl"

        rule syn_keepend() = r"keepend"

        rule syn_pattern() = (ps:syn_pattern_delimiter() ['a'..='z']* pe:syn_pattern_delimiter()) syn_pattern_offset()?

        rule syn_pattern_offset() = (( "ms" / "me" / "hs" / "he" / "rs" / "re" )
                "=" 
                ( "s" (("+" / "-") digit()+ )? / "e" (("+" / "-") digit()+ )? ) )
             / ( "lc=" digit()+ )

        rule syn_cluster() = "syntax" _ "cluster" _ cluster_name() 
            _ ("contain=" group_name()+)? 
            _ ("add=" group_name()+)?
            _ ("remove=" group_name()+)?

        rule cluster_name() = 

        rule digit() = ['0'..='9']

        rule syn_conceal_implicit() = r"syntax" _ "conceal" _ ( "on" / "off" )

        rule keyword() = ['a'..='z' | 'A'..='Z']*<,80>

        rule syn_pattern_delimiter() -> &'input str = "a" {""}

        rule _() = [' ' | '\n']+
    }
}