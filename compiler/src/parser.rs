// auto-generated: "lalrpop 0.19.8"
// sha3: 6bd07c642019dff20430e1e6df0583bc4f3b99d8782cbab26d62325065a28a72
use lalrpop_util::ParseError;
use crate::DictionaryError;
use crate::ProgramInformation;
use crate::SpanMaker;
use crate::Spanned as SpannedNode;
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use lalrpop_util::ParseError;
    use crate::DictionaryError;
    use crate::ProgramInformation;
    use crate::SpanMaker;
    use crate::Spanned as SpannedNode;
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(CardEntry),
        Variant2(alloc::vec::Vec<CardEntry>),
        Variant3(DefinitionVariable),
        Variant4(alloc::vec::Vec<DefinitionVariable>),
        Variant5(SpannedNode<Definition>),
        Variant6(alloc::vec::Vec<SpannedNode<Definition>>),
        Variant7(u64),
        Variant8(usize),
        Variant9(bool),
        Variant10(core::option::Option<CardEntry>),
        Variant11(SpannedNode<(i64, i64)>),
        Variant12(core::option::Option<SpannedNode<(i64, i64)>>),
        Variant13(SpannedNode<u64>),
        Variant14(core::option::Option<SpannedNode<u64>>),
        Variant15(Definition),
        Variant16(DefinitionFaction),
        Variant17(core::option::Option<DefinitionVariable>),
        Variant18(Vec<DefinitionVariable>),
        Variant19(String),
        Variant20(i64),
        Variant21(Program),
        Variant22(SpannedNode<DefinitionFaction>),
        Variant23(SpannedNode<Vec<DefinitionVariable>>),
        Variant24(SpannedNode<String>),
        Variant25(SpannedNode<Vec<CardEntry>>),
        Variant26(Vec<CardEntry>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        //     (<Spanned<Definition>>)+ = (*) (<Spanned<Definition>>)+ Spanned<Definition> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     (<Spanned<Definition>>)+ = (*) Spanned<Definition> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Definition = (*) Spanned<DefinitionFaction> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Definition = (*) Spanned<DefinitionVariables> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionFaction = (*) KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionVariables = (*) KeywordVariables "{" TrailingComma<DefinitionVariable> "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Program = (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Program = (*) (<Spanned<Definition>>)+ ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Definition> = (*) Definition ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<DefinitionFaction> = (*) DefinitionFaction ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<DefinitionVariables> = (*) DefinitionVariables ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     __Program = (*) Program ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        3,  // on KeywordFaction, goto 2
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        25,  // on KeywordVariables, goto 24

        // State 1
        //     (<Spanned<Definition>>)+ = (<Spanned<Definition>>)+ (*) Spanned<Definition> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Definition = (*) Spanned<DefinitionFaction> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Definition = (*) Spanned<DefinitionVariables> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionFaction = (*) KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionVariables = (*) KeywordVariables "{" TrailingComma<DefinitionVariable> "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Program = (<Spanned<Definition>>)+ (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Definition> = (*) Definition ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<DefinitionFaction> = (*) DefinitionFaction ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<DefinitionVariables> = (*) DefinitionVariables ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        3,  // on KeywordFaction, goto 2
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        25,  // on KeywordVariables, goto 24

        // State 2
        //     DefinitionFaction = KeywordFaction (*) Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 3
        //     (<DefinitionVariable> ",")+ = (*) (<DefinitionVariable> ",")+ DefinitionVariable "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     (<DefinitionVariable> ",")+ = (*) DefinitionVariable "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionVariable = (*) Spanned<Identifier> "=" Spanned<Integer> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionVariables = KeywordVariables "{" (*) TrailingComma<DefinitionVariable> "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (*) (<DefinitionVariable> ",")+ ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (*) (<DefinitionVariable> ",")+ DefinitionVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (*) DefinitionVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -67,  // on "}", reduce `TrailingComma<DefinitionVariable> =  => ActionFn(108);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 4
        //     (<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+ (*) DefinitionVariable "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionVariable = (*) Spanned<Identifier> "=" Spanned<Integer> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+ (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+ (*) DefinitionVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -69,  // on "}", reduce `TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+ => ActionFn(110);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 5
        //     DefinitionVariable = Spanned<Identifier> "=" (*) Spanned<Integer> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Integer> = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 6
        //     (<CardEntry> ",")+ = (*) (<CardEntry> ",")+ CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     (<CardEntry> ",")+ = (*) CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" (*) Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<NumberOrVariable> = (*) NumberOrVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<TrailingComma<CardEntry>> = (*) TrailingComma<CardEntry> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -63,  // on "}", reduce `TrailingComma<CardEntry> =  => ActionFn(98);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 7
        //     (<CardEntry> ",")+ = (<CardEntry> ",")+ (*) CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<NumberOrVariable> = (*) NumberOrVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (<CardEntry> ",")+ (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (<CardEntry> ",")+ (*) CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -65,  // on "}", reduce `TrailingComma<CardEntry> = (<CardEntry> ",")+ => ActionFn(100);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 8
        //     CardEntry = Spanned<NumberOrVariable> (*) Spanned<NumberOrVariable> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> (*) Spanned<NumberOrVariable> CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> (*) Spanned<NumberOrVariable> CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> (*) Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<NumberOrVariable> = (*) NumberOrVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 9
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> (*) CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> (*) CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> (*) CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ".." ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryPoints = (*) Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = (*) KeywordPoints "(" NumberOrVariable ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -24,  // on ",", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable> => ActionFn(106);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -24,  // on "}", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable> => ActionFn(106);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        54,  // on KeywordDifficulty, goto 53
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        55,  // on KeywordPoints, goto 54
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 10
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints (*) CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" NumberOrVariableSigned ".." ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = (*) KeywordDifficulty "(" ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -23,  // on ",", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints => ActionFn(105);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -23,  // on "}", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints => ActionFn(105);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        54,  // on KeywordDifficulty, goto 53
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 11
        //     CardEntryDifficulty = KeywordDifficulty "(" (*) NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" (*) NumberOrVariableSigned ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" (*) NumberOrVariableSigned ".." ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" (*) ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        15,  // on "..", goto 14
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 12
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints "(" (*) NumberOrVariable ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 13
        //     (<CardEntry> ",")+ = (*) (<CardEntry> ",")+ CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     (<CardEntry> ",")+ = (*) CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" (*) Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<NumberOrVariable> = (*) NumberOrVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<TrailingComma<CardEntry>> = (*) TrailingComma<CardEntry> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -63,  // on "}", reduce `TrailingComma<CardEntry> =  => ActionFn(98);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 14
        //     CardEntryDifficulty = KeywordDifficulty "(" ".." (*) NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 15
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ".." (*) NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ".." (*) ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariableSigned = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        68,  // on ")", goto 67
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 16
        //     (<CardEntry> ",")+ = (*) (<CardEntry> ",")+ CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     (<CardEntry> ",")+ = (*) CardEntry "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntry = (*) Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" (*) Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Identifier = (*) IdentifierRegex ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Integer = (*) r#"[0-9]+"# ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Integer ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     NumberOrVariable = (*) Spanned<Identifier> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<Identifier> = (*) Identifier ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<NumberOrVariable> = (*) NumberOrVariable ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     Spanned<TrailingComma<CardEntry>> = (*) TrailingComma<CardEntry> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) (<CardEntry> ",")+ CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (*) CardEntry ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -63,  // on "}", reduce `TrailingComma<CardEntry> =  => ActionFn(98);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        41,  // on r#"[0-9]+"#, goto 40
        0,  // on r#"\\.[0-9]*"#, error
        29,  // on IdentifierRegex, goto 28
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 17
        //     Spanned<Definition> = Definition (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -54,  // on KeywordFaction, reduce `Spanned<Definition> = Definition => ActionFn(90);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -54,  // on KeywordVariables, reduce `Spanned<Definition> = Definition => ActionFn(90);`

        // State 18
        //     Spanned<DefinitionFaction> = DefinitionFaction (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -55,  // on KeywordFaction, reduce `Spanned<DefinitionFaction> = DefinitionFaction => ActionFn(91);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -55,  // on KeywordVariables, reduce `Spanned<DefinitionFaction> = DefinitionFaction => ActionFn(91);`

        // State 19
        //     Spanned<DefinitionVariables> = DefinitionVariables (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -56,  // on KeywordFaction, reduce `Spanned<DefinitionVariables> = DefinitionVariables => ActionFn(92);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -56,  // on KeywordVariables, reduce `Spanned<DefinitionVariables> = DefinitionVariables => ActionFn(92);`

        // State 20
        //     __Program = Program (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 21
        //     (<Spanned<Definition>>)+ = Spanned<Definition> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -14,  // on KeywordFaction, reduce `(<Spanned<Definition>>)+ = Spanned<Definition> => ActionFn(68);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -14,  // on KeywordVariables, reduce `(<Spanned<Definition>>)+ = Spanned<Definition> => ActionFn(68);`

        // State 22
        //     Definition = Spanned<DefinitionFaction> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -38,  // on KeywordFaction, reduce `Definition = Spanned<DefinitionFaction> => ActionFn(3);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -38,  // on KeywordVariables, reduce `Definition = Spanned<DefinitionFaction> => ActionFn(3);`

        // State 23
        //     Definition = Spanned<DefinitionVariables> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -37,  // on KeywordFaction, reduce `Definition = Spanned<DefinitionVariables> => ActionFn(2);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -37,  // on KeywordVariables, reduce `Definition = Spanned<DefinitionVariables> => ActionFn(2);`

        // State 24
        //     DefinitionVariables = KeywordVariables (*) "{" TrailingComma<DefinitionVariable> "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        4,  // on "{", goto 3
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 25
        //     (<Spanned<Definition>>)+ = (<Spanned<Definition>>)+ Spanned<Definition> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -15,  // on KeywordFaction, reduce `(<Spanned<Definition>>)+ = (<Spanned<Definition>>)+, Spanned<Definition> => ActionFn(69);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -15,  // on KeywordVariables, reduce `(<Spanned<Definition>>)+ = (<Spanned<Definition>>)+, Spanned<Definition> => ActionFn(69);`

        // State 26
        //     Spanned<Identifier> = Identifier (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -57,  // on ")", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        -57,  // on ",", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        -57,  // on "..", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        -57,  // on "=", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        0,  // on "false", error
        0,  // on "true", error
        -57,  // on "{", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        -57,  // on "}", reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -57,  // on r#"[0-9]+"#, reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        0,  // on r#"\\.[0-9]*"#, error
        -57,  // on IdentifierRegex, reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        -57,  // on KeywordDifficulty, reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -57,  // on KeywordPoints, reduce `Spanned<Identifier> = Identifier => ActionFn(93);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 27
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> (*) "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        30,  // on "{", goto 29
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 28
        //     Identifier = IdentifierRegex (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -45,  // on ")", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        -45,  // on ",", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        -45,  // on "..", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        -45,  // on "=", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        0,  // on "false", error
        0,  // on "true", error
        -45,  // on "{", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        -45,  // on "}", reduce `Identifier = IdentifierRegex => ActionFn(21);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -45,  // on r#"[0-9]+"#, reduce `Identifier = IdentifierRegex => ActionFn(21);`
        0,  // on r#"\\.[0-9]*"#, error
        -45,  // on IdentifierRegex, reduce `Identifier = IdentifierRegex => ActionFn(21);`
        -45,  // on KeywordDifficulty, reduce `Identifier = IdentifierRegex => ActionFn(21);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -45,  // on KeywordPoints, reduce `Identifier = IdentifierRegex => ActionFn(21);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 29
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" (*) KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        34,  // on KeywordLeaders, goto 33
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 30
        //     (<DefinitionVariable> ",")+ = DefinitionVariable (*) "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = DefinitionVariable (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        36,  // on ",", goto 35
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -66,  // on "}", reduce `TrailingComma<DefinitionVariable> = DefinitionVariable => ActionFn(107);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 31
        //     DefinitionVariable = Spanned<Identifier> (*) "=" Spanned<Integer> ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        6,  // on "=", goto 5
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 32
        //     DefinitionVariables = KeywordVariables "{" TrailingComma<DefinitionVariable> (*) "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        37,  // on "}", goto 36
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 33
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders (*) "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        7,  // on "{", goto 6
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 34
        //     (<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+ DefinitionVariable (*) "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+ DefinitionVariable (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        38,  // on ",", goto 37
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -68,  // on "}", reduce `TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+, DefinitionVariable => ActionFn(109);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 35
        //     (<DefinitionVariable> ",")+ = DefinitionVariable "," (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -9,  // on "}", reduce `(<DefinitionVariable> ",")+ = DefinitionVariable, "," => ActionFn(64);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        -9,  // on IdentifierRegex, reduce `(<DefinitionVariable> ",")+ = DefinitionVariable, "," => ActionFn(64);`
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 36
        //     DefinitionVariables = KeywordVariables "{" TrailingComma<DefinitionVariable> "}" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -43,  // on KeywordFaction, reduce `DefinitionVariables = KeywordVariables, "{", TrailingComma<DefinitionVariable>, "}" => ActionFn(4);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -43,  // on KeywordVariables, reduce `DefinitionVariables = KeywordVariables, "{", TrailingComma<DefinitionVariable>, "}" => ActionFn(4);`

        // State 37
        //     (<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+ DefinitionVariable "," (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -10,  // on "}", reduce `(<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+, DefinitionVariable, "," => ActionFn(65);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        -10,  // on IdentifierRegex, reduce `(<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+, DefinitionVariable, "," => ActionFn(65);`
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 38
        //     Spanned<Integer> = Integer (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -58,  // on ",", reduce `Spanned<Integer> = Integer => ActionFn(94);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -58,  // on "}", reduce `Spanned<Integer> = Integer => ActionFn(94);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 39
        //     DefinitionVariable = Spanned<Identifier> "=" Spanned<Integer> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -40,  // on ",", reduce `DefinitionVariable = Spanned<Identifier>, "=", Spanned<Integer> => ActionFn(16);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -40,  // on "}", reduce `DefinitionVariable = Spanned<Identifier>, "=", Spanned<Integer> => ActionFn(16);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 40
        //     Integer = r#"[0-9]+"# (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -46,  // on ")", reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        -46,  // on ",", reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        -46,  // on "..", reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -46,  // on "}", reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -46,  // on r#"[0-9]+"#, reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        0,  // on r#"\\.[0-9]*"#, error
        -46,  // on IdentifierRegex, reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        -46,  // on KeywordDifficulty, reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -46,  // on KeywordPoints, reduce `Integer = r#"[0-9]+"# => ActionFn(18);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 41
        //     (<CardEntry> ",")+ = CardEntry (*) "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = CardEntry (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        49,  // on ",", goto 48
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -62,  // on "}", reduce `TrailingComma<CardEntry> = CardEntry => ActionFn(97);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 42
        //     NumberOrVariable = Integer (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -47,  // on ")", reduce `NumberOrVariable = Integer => ActionFn(12);`
        -47,  // on ",", reduce `NumberOrVariable = Integer => ActionFn(12);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -47,  // on "}", reduce `NumberOrVariable = Integer => ActionFn(12);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -47,  // on r#"[0-9]+"#, reduce `NumberOrVariable = Integer => ActionFn(12);`
        0,  // on r#"\\.[0-9]*"#, error
        -47,  // on IdentifierRegex, reduce `NumberOrVariable = Integer => ActionFn(12);`
        -47,  // on KeywordDifficulty, reduce `NumberOrVariable = Integer => ActionFn(12);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -47,  // on KeywordPoints, reduce `NumberOrVariable = Integer => ActionFn(12);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 43
        //     Spanned<NumberOrVariable> = NumberOrVariable (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -59,  // on ",", reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -59,  // on "}", reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -59,  // on r#"[0-9]+"#, reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        0,  // on r#"\\.[0-9]*"#, error
        -59,  // on IdentifierRegex, reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        -59,  // on KeywordDifficulty, reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -59,  // on KeywordPoints, reduce `Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 44
        //     NumberOrVariable = Spanned<Identifier> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -48,  // on ")", reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        -48,  // on ",", reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -48,  // on "}", reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -48,  // on r#"[0-9]+"#, reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        0,  // on r#"\\.[0-9]*"#, error
        -48,  // on IdentifierRegex, reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        -48,  // on KeywordDifficulty, reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        -48,  // on KeywordPoints, reduce `NumberOrVariable = Spanned<Identifier> => ActionFn(13);`
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 45
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> (*) "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        50,  // on "}", goto 49
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 46
        //     Spanned<TrailingComma<CardEntry>> = TrailingComma<CardEntry> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -60,  // on "}", reduce `Spanned<TrailingComma<CardEntry>> = TrailingComma<CardEntry> => ActionFn(96);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 47
        //     (<CardEntry> ",")+ = (<CardEntry> ",")+ CardEntry (*) "," ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     TrailingComma<CardEntry> = (<CardEntry> ",")+ CardEntry (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        51,  // on ",", goto 50
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -64,  // on "}", reduce `TrailingComma<CardEntry> = (<CardEntry> ",")+, CardEntry => ActionFn(99);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 48
        //     (<CardEntry> ",")+ = CardEntry "," (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -4,  // on "}", reduce `(<CardEntry> ",")+ = CardEntry, "," => ActionFn(60);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -4,  // on r#"[0-9]+"#, reduce `(<CardEntry> ",")+ = CardEntry, "," => ActionFn(60);`
        0,  // on r#"\\.[0-9]*"#, error
        -4,  // on IdentifierRegex, reduce `(<CardEntry> ",")+ = CardEntry, "," => ActionFn(60);`
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 49
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" (*) KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        56,  // on KeywordHeroes, goto 55
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 50
        //     (<CardEntry> ",")+ = (<CardEntry> ",")+ CardEntry "," (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -5,  // on "}", reduce `(<CardEntry> ",")+ = (<CardEntry> ",")+, CardEntry, "," => ActionFn(61);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        -5,  // on r#"[0-9]+"#, reduce `(<CardEntry> ",")+ = (<CardEntry> ",")+, CardEntry, "," => ActionFn(61);`
        0,  // on r#"\\.[0-9]*"#, error
        -5,  // on IdentifierRegex, reduce `(<CardEntry> ",")+ = (<CardEntry> ",")+, CardEntry, "," => ActionFn(61);`
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 51
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryDifficulty (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -22,  // on ",", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryDifficulty => ActionFn(104);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -22,  // on "}", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryDifficulty => ActionFn(104);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 52
        //     CardEntryPoints = Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -33,  // on ",", reduce `CardEntryPoints = Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> => ActionFn(7);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -33,  // on "}", reduce `CardEntryPoints = Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> => ActionFn(7);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        -33,  // on KeywordDifficulty, reduce `CardEntryPoints = Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> => ActionFn(7);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 53
        //     CardEntryDifficulty = KeywordDifficulty (*) "(" NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty (*) "(" NumberOrVariableSigned ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty (*) "(" NumberOrVariableSigned ".." ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty (*) "(" ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        12,  // on "(", goto 11
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 54
        //     Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints (*) "(" NumberOrVariable ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        13,  // on "(", goto 12
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 55
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes (*) "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        14,  // on "{", goto 13
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 56
        //     CardEntry = Spanned<NumberOrVariable> Spanned<NumberOrVariable> CardEntryPoints CardEntryDifficulty (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -21,  // on ",", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints, CardEntryDifficulty => ActionFn(103);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -21,  // on "}", reduce `CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints, CardEntryDifficulty => ActionFn(103);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 57
        //     NumberOrVariableSigned = Integer (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -49,  // on ")", reduce `NumberOrVariableSigned = Integer => ActionFn(14);`
        0,  // on ",", error
        -49,  // on "..", reduce `NumberOrVariableSigned = Integer => ActionFn(14);`
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 58
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned (*) ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned (*) ".." NumberOrVariableSigned ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned (*) ".." ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        63,  // on ")", goto 62
        0,  // on ",", error
        16,  // on "..", goto 15
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 59
        //     NumberOrVariableSigned = Spanned<Identifier> (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        -50,  // on ")", reduce `NumberOrVariableSigned = Spanned<Identifier> => ActionFn(15);`
        0,  // on ",", error
        -50,  // on "..", reduce `NumberOrVariableSigned = Spanned<Identifier> => ActionFn(15);`
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 60
        //     Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints "(" NumberOrVariable (*) ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        65,  // on ")", goto 64
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 61
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> (*) "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        66,  // on "}", goto 65
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 62
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ")" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -27,  // on ",", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, ")" => ActionFn(85);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -27,  // on "}", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, ")" => ActionFn(85);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 63
        //     CardEntryDifficulty = KeywordDifficulty "(" ".." NumberOrVariableSigned (*) ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        69,  // on ")", goto 68
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 64
        //     Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints "(" NumberOrVariable ")" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -53,  // on ",", reduce `Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints, "(", NumberOrVariable, ")" => ActionFn(89);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -53,  // on "}", reduce `Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints, "(", NumberOrVariable, ")" => ActionFn(89);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        -53,  // on KeywordDifficulty, reduce `Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints, "(", NumberOrVariable, ")" => ActionFn(89);`
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 65
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" (*) KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        70,  // on KeywordUnits, goto 69
        0,  // on KeywordVariables, error

        // State 66
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ".." NumberOrVariableSigned (*) ")" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        71,  // on ")", goto 70
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 67
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ".." ")" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -28,  // on ",", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", ")" => ActionFn(86);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -28,  // on "}", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", ")" => ActionFn(86);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 68
        //     CardEntryDifficulty = KeywordDifficulty "(" ".." NumberOrVariableSigned ")" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -29,  // on ",", reduce `CardEntryDifficulty = KeywordDifficulty, "(", "..", NumberOrVariableSigned, ")" => ActionFn(87);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -29,  // on "}", reduce `CardEntryDifficulty = KeywordDifficulty, "(", "..", NumberOrVariableSigned, ")" => ActionFn(87);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 69
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits (*) "{" Spanned<TrailingComma<CardEntry>> "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        17,  // on "{", goto 16
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 70
        //     CardEntryDifficulty = KeywordDifficulty "(" NumberOrVariableSigned ".." NumberOrVariableSigned ")" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        -30,  // on ",", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", NumberOrVariableSigned, ")" => ActionFn(88);`
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        -30,  // on "}", reduce `CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", NumberOrVariableSigned, ")" => ActionFn(88);`
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 71
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> (*) "}" "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        73,  // on "}", goto 72
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 72
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" (*) "}" ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        74,  // on "}", goto 73
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        0,  // on KeywordFaction, error
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        0,  // on KeywordVariables, error

        // State 73
        //     DefinitionFaction = KeywordFaction Spanned<Identifier> "{" KeywordLeaders "{" Spanned<TrailingComma<CardEntry>> "}" KeywordHeroes "{" Spanned<TrailingComma<CardEntry>> "}" KeywordUnits "{" Spanned<TrailingComma<CardEntry>> "}" "}" (*) ["(", ")", ",", "..", "=", "false", "true", "{", "}", r#"\"[^\"]*\""#, r#"'[^']*'"#, r#"[0-9]+"#, r#"\\.[0-9]*"#, IdentifierRegex, KeywordDifficulty, KeywordFaction, KeywordHeroes, KeywordLeaders, KeywordPoints, KeywordUnits, KeywordVariables, EOF]
        0,  // on "(", error
        0,  // on ")", error
        0,  // on ",", error
        0,  // on "..", error
        0,  // on "=", error
        0,  // on "false", error
        0,  // on "true", error
        0,  // on "{", error
        0,  // on "}", error
        0,  // on r#"\"[^\"]*\""#, error
        0,  // on r#"'[^']*'"#, error
        0,  // on r#"[0-9]+"#, error
        0,  // on r#"\\.[0-9]*"#, error
        0,  // on IdentifierRegex, error
        0,  // on KeywordDifficulty, error
        -39,  // on KeywordFaction, reduce `DefinitionFaction = KeywordFaction, Spanned<Identifier>, "{", KeywordLeaders, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordHeroes, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordUnits, "{", Spanned<TrailingComma<CardEntry>>, "}", "}" => ActionFn(5);`
        0,  // on KeywordHeroes, error
        0,  // on KeywordLeaders, error
        0,  // on KeywordPoints, error
        0,  // on KeywordUnits, error
        -39,  // on KeywordVariables, reduce `DefinitionFaction = KeywordFaction, Spanned<Identifier>, "{", KeywordLeaders, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordHeroes, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordUnits, "{", Spanned<TrailingComma<CardEntry>>, "}", "}" => ActionFn(5);`

    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 21 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -51,  // on EOF, reduce `Program =  => ActionFn(70);`

        // State 1
        -52,  // on EOF, reduce `Program = (<Spanned<Definition>>)+ => ActionFn(71);`

        // State 2
        0,  // on EOF, error

        // State 3
        0,  // on EOF, error

        // State 4
        0,  // on EOF, error

        // State 5
        0,  // on EOF, error

        // State 6
        0,  // on EOF, error

        // State 7
        0,  // on EOF, error

        // State 8
        0,  // on EOF, error

        // State 9
        0,  // on EOF, error

        // State 10
        0,  // on EOF, error

        // State 11
        0,  // on EOF, error

        // State 12
        0,  // on EOF, error

        // State 13
        0,  // on EOF, error

        // State 14
        0,  // on EOF, error

        // State 15
        0,  // on EOF, error

        // State 16
        0,  // on EOF, error

        // State 17
        -54,  // on EOF, reduce `Spanned<Definition> = Definition => ActionFn(90);`

        // State 18
        -55,  // on EOF, reduce `Spanned<DefinitionFaction> = DefinitionFaction => ActionFn(91);`

        // State 19
        -56,  // on EOF, reduce `Spanned<DefinitionVariables> = DefinitionVariables => ActionFn(92);`

        // State 20
        -70,  // on EOF, reduce `__Program = Program => ActionFn(0);`

        // State 21
        -14,  // on EOF, reduce `(<Spanned<Definition>>)+ = Spanned<Definition> => ActionFn(68);`

        // State 22
        -38,  // on EOF, reduce `Definition = Spanned<DefinitionFaction> => ActionFn(3);`

        // State 23
        -37,  // on EOF, reduce `Definition = Spanned<DefinitionVariables> => ActionFn(2);`

        // State 24
        0,  // on EOF, error

        // State 25
        -15,  // on EOF, reduce `(<Spanned<Definition>>)+ = (<Spanned<Definition>>)+, Spanned<Definition> => ActionFn(69);`

        // State 26
        0,  // on EOF, error

        // State 27
        0,  // on EOF, error

        // State 28
        0,  // on EOF, error

        // State 29
        0,  // on EOF, error

        // State 30
        0,  // on EOF, error

        // State 31
        0,  // on EOF, error

        // State 32
        0,  // on EOF, error

        // State 33
        0,  // on EOF, error

        // State 34
        0,  // on EOF, error

        // State 35
        0,  // on EOF, error

        // State 36
        -43,  // on EOF, reduce `DefinitionVariables = KeywordVariables, "{", TrailingComma<DefinitionVariable>, "}" => ActionFn(4);`

        // State 37
        0,  // on EOF, error

        // State 38
        0,  // on EOF, error

        // State 39
        0,  // on EOF, error

        // State 40
        0,  // on EOF, error

        // State 41
        0,  // on EOF, error

        // State 42
        0,  // on EOF, error

        // State 43
        0,  // on EOF, error

        // State 44
        0,  // on EOF, error

        // State 45
        0,  // on EOF, error

        // State 46
        0,  // on EOF, error

        // State 47
        0,  // on EOF, error

        // State 48
        0,  // on EOF, error

        // State 49
        0,  // on EOF, error

        // State 50
        0,  // on EOF, error

        // State 51
        0,  // on EOF, error

        // State 52
        0,  // on EOF, error

        // State 53
        0,  // on EOF, error

        // State 54
        0,  // on EOF, error

        // State 55
        0,  // on EOF, error

        // State 56
        0,  // on EOF, error

        // State 57
        0,  // on EOF, error

        // State 58
        0,  // on EOF, error

        // State 59
        0,  // on EOF, error

        // State 60
        0,  // on EOF, error

        // State 61
        0,  // on EOF, error

        // State 62
        0,  // on EOF, error

        // State 63
        0,  // on EOF, error

        // State 64
        0,  // on EOF, error

        // State 65
        0,  // on EOF, error

        // State 66
        0,  // on EOF, error

        // State 67
        0,  // on EOF, error

        // State 68
        0,  // on EOF, error

        // State 69
        0,  // on EOF, error

        // State 70
        0,  // on EOF, error

        // State 71
        0,  // on EOF, error

        // State 72
        0,  // on EOF, error

        // State 73
        -39,  // on EOF, reduce `DefinitionFaction = KeywordFaction, Spanned<Identifier>, "{", KeywordLeaders, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordHeroes, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordUnits, "{", Spanned<TrailingComma<CardEntry>>, "}", "}" => ActionFn(5);`

    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 7,
            5 => 4,
            8 => 1,
            13 => match state {
                 // on CardEntry, goto 47
                7 => 47,
                _ => 41,
            },
            15 => match state {
                 // on CardEntryDifficulty, goto 56
                10 => 56,
                _ => 51,
            },
            17 => 10,
            20 => 17,
            21 => 18,
            22 => match state {
                 // on DefinitionVariable, goto 34
                4 => 34,
                _ => 30,
            },
            24 => 19,
            26 => 26,
            27 => match state {
                 // on Integer, goto 38
                5 => 38,
                 // on Integer, goto 57
                11 | 14..=15 => 57,
                _ => 42,
            },
            28 => match state {
                 // on NumberOrVariable, goto 60
                12 => 60,
                _ => 43,
            },
            29 => match state {
                 // on NumberOrVariableSigned, goto 63
                14 => 63,
                 // on NumberOrVariableSigned, goto 66
                15 => 66,
                _ => 58,
            },
            30 => 20,
            31 => 52,
            32 => match state {
                 // on Spanned<Definition>, goto 25
                1 => 25,
                _ => 21,
            },
            33 => 22,
            34 => 23,
            35 => match state {
                 // on Spanned<Identifier>, goto 27
                2 => 27,
                 // on Spanned<Identifier>, goto 31
                3..=4 => 31,
                 // on Spanned<Identifier>, goto 59
                11 | 14..=15 => 59,
                _ => 44,
            },
            36 => 39,
            37 => match state {
                 // on Spanned<NumberOrVariable>, goto 9
                8 => 9,
                _ => 8,
            },
            38 => match state {
                 // on Spanned<TrailingComma<CardEntry>>, goto 61
                13 => 61,
                 // on Spanned<TrailingComma<CardEntry>>, goto 71
                16 => 71,
                _ => 45,
            },
            40 => 46,
            41 => 32,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""..""###,
            r###""=""###,
            r###""false""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
            r###"r#"\"[^\"]*\""#"###,
            r###"r#"'[^']*'"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"\\.[0-9]*"#"###,
            r###"IdentifierRegex"###,
            r###"KeywordDifficulty"###,
            r###"KeywordFaction"###,
            r###"KeywordHeroes"###,
            r###"KeywordLeaders"###,
            r###"KeywordPoints"###,
            r###"KeywordUnits"###,
            r###"KeywordVariables"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, '__1, '__2>
    where 'input: '__2
    {
        program_information: &'__1 mut ProgramInformation,
        span_maker: &'__2 mut SpanMaker<'input>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input, '__1, '__2> __state_machine::ParserDefinition for __StateMachine<'input, '__1, '__2>
    where 'input: '__2
    {
        type Location = usize;
        type Error = DictionaryError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Program;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 21 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.program_information,
                self.span_maker,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(9, _) if true => Some(6),
            Token(10, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(0, _) if true => Some(9),
            Token(1, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(2, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(17, __tok0) | Token(2, __tok0) | Token(13, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            program_information: &mut ProgramInformation,
            span_maker: &mut SpanMaker<'input>,
            input: &'input str,
        ) -> Result<Program, __lalrpop_util::ParseError<usize, Token<'input>, DictionaryError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    program_information,
                    span_maker,
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token<'input>, DictionaryError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                // CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", NumberOrVariableSigned, ")" => ActionFn(88);
                assert!(__symbols.len() >= 6);
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant20(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant20(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = match super::__action88::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant11(__nt), __end));
                (6, 15)
            }
            30 => {
                __reduce30(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                // DefinitionFaction = KeywordFaction, Spanned<Identifier>, "{", KeywordLeaders, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordHeroes, "{", Spanned<TrailingComma<CardEntry>>, "}", KeywordUnits, "{", Spanned<TrailingComma<CardEntry>>, "}", "}" => ActionFn(5);
                assert!(__symbols.len() >= 16);
                let __sym15 = __pop_Variant0(__symbols);
                let __sym14 = __pop_Variant0(__symbols);
                let __sym13 = __pop_Variant25(__symbols);
                let __sym12 = __pop_Variant0(__symbols);
                let __sym11 = __pop_Variant0(__symbols);
                let __sym10 = __pop_Variant0(__symbols);
                let __sym9 = __pop_Variant25(__symbols);
                let __sym8 = __pop_Variant0(__symbols);
                let __sym7 = __pop_Variant0(__symbols);
                let __sym6 = __pop_Variant0(__symbols);
                let __sym5 = __pop_Variant25(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant24(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym15.2.clone();
                let __nt = match super::__action5::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10, __sym11, __sym12, __sym13, __sym14, __sym15) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant16(__nt), __end));
                (16, 21)
            }
            39 => {
                __reduce39(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                // NumberOrVariable = Spanned<Identifier> => ActionFn(13);
                let __sym0 = __pop_Variant24(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action13::<>(program_information, span_maker, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant7(__nt), __end));
                (1, 28)
            }
            48 => {
                __reduce48(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                // NumberOrVariableSigned = Spanned<Identifier> => ActionFn(15);
                let __sym0 = __pop_Variant24(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action15::<>(program_information, span_maker, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant20(__nt), __end));
                (1, 29)
            }
            50 => {
                __reduce50(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(program_information, span_maker, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant21(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(program_information, span_maker, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CardEntry, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Definition, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, DefinitionFaction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, DefinitionVariable, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<(i64, i64)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<Definition>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<DefinitionFaction>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<Vec<CardEntry>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<Vec<DefinitionVariable>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SpannedNode<u64>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<CardEntry>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<DefinitionVariable>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<CardEntry>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<DefinitionVariable>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<CardEntry>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<DefinitionVariable>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<SpannedNode<(i64, i64)>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<SpannedNode<u64>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<CardEntry> ",") = CardEntry, "," => ActionFn(55);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action55::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<CardEntry> ",")* =  => ActionFn(53);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action53::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<CardEntry> ",")* = (<CardEntry> ",")+ => ActionFn(54);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<CardEntry> ",")+ = CardEntry, "," => ActionFn(60);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action60::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<CardEntry> ",")+ = (<CardEntry> ",")+, CardEntry, "," => ActionFn(61);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action61::<>(program_information, span_maker, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<DefinitionVariable> ",") = DefinitionVariable, "," => ActionFn(50);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action50::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<DefinitionVariable> ",")* =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<DefinitionVariable> ",")* = (<DefinitionVariable> ",")+ => ActionFn(49);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<DefinitionVariable> ",")+ = DefinitionVariable, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<DefinitionVariable> ",")+ = (<DefinitionVariable> ",")+, DefinitionVariable, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(program_information, span_maker, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spanned<Definition>>) = Spanned<Definition> => ActionFn(42);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spanned<Definition>>)* =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spanned<Definition>>)* = (<Spanned<Definition>>)+ => ActionFn(41);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spanned<Definition>>)+ = Spanned<Definition> => ActionFn(68);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spanned<Definition>>)+ = (<Spanned<Definition>>)+, Spanned<Definition> => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (KeywordPoints "(" <NumberOrVariable> ")") = KeywordPoints, "(", NumberOrVariable, ")" => ActionFn(28);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action28::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(26);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action26::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(25);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action25::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints, CardEntryDifficulty => ActionFn(103);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action103::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 13)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryDifficulty => ActionFn(104);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action104::<>(program_information, span_maker, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable>, CardEntryPoints => ActionFn(105);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action105::<>(program_information, span_maker, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry = Spanned<NumberOrVariable>, Spanned<NumberOrVariable> => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action106::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry? = CardEntry => ActionFn(51);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntry? =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, ")" => ActionFn(85);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action85::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryDifficulty = KeywordDifficulty, "(", NumberOrVariableSigned, "..", ")" => ActionFn(86);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action86::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryDifficulty = KeywordDifficulty, "(", "..", NumberOrVariableSigned, ")" => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant20(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (5, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryDifficulty? = CardEntryDifficulty => ActionFn(29);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryDifficulty? =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryPoints = Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryPoints? = CardEntryPoints => ActionFn(31);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CardEntryPoints? =  => ActionFn(32);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action32::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CharLiteral = r#"'[^']*'"# => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = Spanned<DefinitionVariables> => ActionFn(2);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = Spanned<DefinitionFaction> => ActionFn(3);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DefinitionVariable = Spanned<Identifier>, "=", Spanned<Integer> => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(program_information, span_maker, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DefinitionVariable? = DefinitionVariable => ActionFn(46);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DefinitionVariable? =  => ActionFn(47);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action47::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DefinitionVariables = KeywordVariables, "{", TrailingComma<DefinitionVariable>, "}" => ActionFn(4);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action4::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (4, 24)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Float = Integer, r#"\\.[0-9]*"# => ActionFn(17);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action17::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Identifier = IdentifierRegex => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Integer = r#"[0-9]+"# => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NumberOrVariable = Integer => ActionFn(12);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NumberOrVariableSigned = Integer => ActionFn(14);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(70);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action70::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = (<Spanned<Definition>>)+ => ActionFn(71);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<(KeywordPoints "(" <NumberOrVariable> ")")> = KeywordPoints, "(", NumberOrVariable, ")" => ActionFn(89);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action89::<>(program_information, span_maker, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 31)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<Definition> = Definition => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<DefinitionFaction> = DefinitionFaction => ActionFn(91);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<DefinitionVariables> = DefinitionVariables => ActionFn(92);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<Identifier> = Identifier => ActionFn(93);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<Integer> = Integer => ActionFn(94);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<NumberOrVariable> = NumberOrVariable => ActionFn(95);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spanned<TrailingComma<CardEntry>> = TrailingComma<CardEntry> => ActionFn(96);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StringLiteral = r#"\"[^\"]*\""# => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<CardEntry> = CardEntry => ActionFn(97);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<CardEntry> =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (0, 40)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<CardEntry> = (<CardEntry> ",")+, CardEntry => ActionFn(99);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action99::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 40)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<CardEntry> = (<CardEntry> ",")+ => ActionFn(100);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action100::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<DefinitionVariable> = DefinitionVariable => ActionFn(107);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action107::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<DefinitionVariable> =  => ActionFn(108);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action108::<>(program_information, span_maker, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 41)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+, DefinitionVariable => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109::<>(program_information, span_maker, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 41)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        program_information: &mut ProgramInformation,
        span_maker: &mut SpanMaker<'input>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<DefinitionVariable> = (<DefinitionVariable> ",")+ => ActionFn(110);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action110::<>(program_information, span_maker, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 41)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use lalrpop_util::ParseError;
    use crate::DictionaryError;
    use crate::ProgramInformation;
    use crate::SpanMaker;
    use crate::Spanned as SpannedNode;
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\"[\0-!\\#-\u{10ffff}]*\")", false),
            ("^('[\0-\\&\\(-\u{10ffff}]*')", false),
            ("^(\\.[0-9]*)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^(\\.\\.)", false),
            ("^(=)", false),
            ("^(false)", false),
            ("^(true)", false),
            ("^(\\{)", false),
            ("^(\\})", false),
            ("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]*)", true),
            ("^([0-9A-Z_a-zªµºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬˮ\u{300}-ʹͶ-ͷͺ-ͽͿΆΈ-ΊΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙՠ-ֈ\u{591}-\u{5bd}\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}א-תׯ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ\u{7fd}ࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࡰ-ࢇࢉ-ࢎ\u{898}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রলশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ\u{9fe}\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}ਖ਼-ੜਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b55}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ\u{bd7}௦-௯\u{c00}-ఌఎ-ఐఒ-నప-హ\u{c3c}-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౝౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೝ-ೞೠ-\u{ce3}೦-೯ೱ-ೳ\u{d00}-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿ\u{d81}-ඃඅ-ඖක-නඳ-රලව-ෆ\u{dca}\u{dcf}-\u{dd4}\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄຆ-ຊຌ-ຣລວ-ຽເ-ໄໆ\u{ec8}-\u{ece}໐-໙ໜ-ໟༀ\u{f18}-\u{f19}༠-༩\u{f35}\u{f37}\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇჍა-ჺჼ-ቈቊ-ቍቐ-ቖቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-᜕ᜟ-᜴ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}\u{180f}-᠙ᠠ-ᡸᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ\u{1ab0}-\u{1ace}\u{1b00}-ᭌ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈᲐ-ᲺᲽ-Ჿ\u{1cd0}-\u{1cd2}\u{1cd4}-ᳺᴀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙὛὝὟ-ώᾀ-ᾴᾶ-ᾼιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔ⁱⁿₐ-ₜ\u{20d0}-\u{20f0}ℂℇℊ-ℓℕℙ-ℝℤΩℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎⅠ-ↈⒶ-ⓩⰀ-ⳤⳫ-ⳳⴀ-ⴥⴧⴭⴰ-ⵧⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄯㄱ-ㆎㆠ-ㆿㇰ-ㇿ㐀-䶿一-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꟊꟐ-ꟑꟓꟕ-ꟙꟲ-ꠧ\u{a82c}ꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻꣽ-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭩꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐕰-𐕺𐕼-𐖊𐖌-𐖒𐖔-𐖕𐖗-𐖡𐖣-𐖱𐖳-𐖹𐖻-𐖼𐘀-𐜶𐝀-𐝕𐝠-𐝧𐞀-𐞅𐞇-𐞰𐞲-𐞺𐠀-𐠅𐠈𐠊-𐠵𐠷-𐠸𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨵\u{10a38}-\u{10a3a}\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𐴀-\u{10d27}𐴰-𐴹𐺀-𐺩\u{10eab}-\u{10eac}𐺰-𐺱\u{10efd}-𐼜𐼧𐼰-\u{10f50}𐽰-\u{10f85}𐾰-𐿄𐿠-𐿶𑀀-\u{11046}𑁦-𑁵\u{1107f}-\u{110ba}\u{110c2}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅄-𑅇𑅐-\u{11173}𑅶\u{11180}-𑇄\u{111c9}-\u{111cc}𑇎-𑇚𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{11241}𑊀-𑊆𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133b}-𑍄𑍇-𑍈𑍋-𑍍𑍐\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙\u{1145e}-𑑡𑒀-𑓅𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄𑙐-𑙙𑚀-𑚸𑛀-𑛉𑜀-𑜚\u{1171d}-\u{1172b}𑜰-𑜹𑝀-𑝆𑠀-\u{1183a}𑢠-𑣩𑣿-𑤆𑤉𑤌-𑤓𑤕-𑤖𑤘-𑤵𑤷-𑤸\u{1193b}-\u{11943}𑥐-𑥙𑦠-𑦧𑦪-\u{119d7}\u{119da}-𑧡𑧣-𑧤𑨀-\u{11a3e}\u{11a47}𑩐-\u{11a99}𑪝𑪰-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𑵠-𑵥𑵧-𑵨𑵪-𑶎\u{11d90}-\u{11d91}𑶓-𑶘𑶠-𑶩𑻠-𑻶\u{11f00}-𑼐𑼒-\u{11f3a}𑼾-\u{11f42}𑽐-𑽙𑾰𒀀-𒎙𒐀-𒑮𒒀-𒕃𒾐-𒿰𓀀-𓐯\u{13440}-\u{13455}𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖩰-𖪾𖫀-𖫉𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖹀-𖹿𖼀-𖽊\u{16f4f}-𖾇\u{16f8f}-𖾟𖿠-𖿡𖿣-\u{16fe4}𖿰-𖿱𗀀-𘟷𘠀-𘳕𘴀-𘴈𚿰-𚿳𚿵-𚿻𚿽-𚿾𛀀-𛄢𛄲𛅐-𛅒𛅕𛅤-𛅧𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1cf00}-\u{1cf2d}\u{1cf30}-\u{1cf46}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}𝼀-𝼞𝼥-𝼪\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞀰-𞁭\u{1e08f}𞄀-𞄬\u{1e130}-𞄽𞅀-𞅉𞅎𞊐-\u{1e2ae}𞋀-𞋹𞓐-𞓹𞟠-𞟦𞟨-𞟫𞟭-𞟮𞟰-𞟾𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-𞥋𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤𞸧𞸩-𞸲𞸴-𞸷𞸹𞸻𞹂𞹇𞹉𞹋𞹍-𞹏𞹑-𞹒𞹔𞹗𞹙𞹛𞹝𞹟𞹡-𞹢𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉🯰-🯹𠀀-𪛟𪜀-𫜹𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀𰀀-𱍊𱍐-𲎯\u{e0100}-\u{e01ef}]+)", false),
            ("^(\\#[\0-\t\u{b}-\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("^(//[\0-\t\u{b}-\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("^(/\\*[\0-\\)\\+-\u{10ffff}]*[\0-\\.0-\u{10ffff}]*(\\*/)[\n\r]*)", true),
            ("^([0-9]+)", false),
            ("^(difficulty)", false),
            ("^(faction)", false),
            ("^(heroes)", false),
            ("^(leaders)", false),
            ("^(points)", false),
            ("^(units)", false),
            ("^(variables)", false),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, definitions, _): (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize),
) -> Program
{
    Program { definitions, name: program_information.name.clone() }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<Vec<DefinitionVariable>>, usize),
) -> Definition
{
    Definition::Variables(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<DefinitionFaction>, usize),
) -> Definition
{
    Definition::Faction(__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, variables, _): (usize, Vec<DefinitionVariable>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<DefinitionVariable>
{
    variables
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, SpannedNode<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, leaders, _): (usize, SpannedNode<Vec<CardEntry>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, heroes, _): (usize, SpannedNode<Vec<CardEntry>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, units, _): (usize, SpannedNode<Vec<CardEntry>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Result<DefinitionFaction,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    match name.as_ref() {
        "NorthernKingdom" | "Monster" | "Scoiatael" |
        "Nilfgaardian" | "Skellige" => Ok(DefinitionFaction { name, leaders, heroes, units }),
        _ => Err(ParseError::User { error: DictionaryError::UnknownFaction(name) })
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, count, _): (usize, SpannedNode<u64>, usize),
    (_, index, _): (usize, SpannedNode<u64>, usize),
    (_, points, _): (usize, core::option::Option<SpannedNode<u64>>, usize),
    (_, difficulty, _): (usize, core::option::Option<SpannedNode<(i64, i64)>>, usize),
) -> CardEntry
{
    CardEntry { index, count, points, difficulty }
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<u64>, usize),
) -> SpannedNode<u64>
{
    __0
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, l, _): (usize, usize, usize),
    (_, above, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    SpannedNode::new((above,-1), span_maker.span(l, r, "card_entry_difficulty"))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, l, _): (usize, usize, usize),
    (_, above, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    SpannedNode::new((above,-1), span_maker.span(l, r, "card_entry_difficulty"))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, l, _): (usize, usize, usize),
    (_, below, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    SpannedNode::new((-1,below), span_maker.span(l, r, "card_entry_difficulty"))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, l, _): (usize, usize, usize),
    (_, above, _): (usize, i64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, below, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Result<SpannedNode<(i64, i64)>,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    if above >= below {
    Err(ParseError::User { error: DictionaryError::InvalidRequirement(SpannedNode::new((above,below), span_maker.span(l, r, "card_entry_difficulty"))) })
  } else {
    Ok(SpannedNode::new((above,below), span_maker.span(l, r, "card_entry_difficulty")))
  }
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, u64, usize),
) -> u64
{
    __0
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<String>, usize),
) -> Result<u64,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    match program_information.get_variable(&__0) {
    Some(value) => Ok(value),
    None => Err(ParseError::User { error: DictionaryError::UnknownVariable(__0) }),
  }
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, u64, usize),
) -> i64
{
    __0.try_into().expect("Entered number too big for an i64")
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<String>, usize),
) -> Result<i64,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    match program_information.get_variable(&__0) {
      Some(value) => Ok(value as i64),
      None => Err(ParseError::User { error: DictionaryError::UnknownVariable(__0) }),
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, name, _): (usize, SpannedNode<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, SpannedNode<u64>, usize),
) -> DefinitionVariable
{
    program_information.register_variable(name, value)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, int, _): (usize, u64, usize),
    (_, n, _): (usize, &'input str, usize),
) -> String
{
    format!("{int}{n}")
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    __0.parse().expect("integer parsing error")
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    false
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, u64, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<u64>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, u64, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<u64>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, u64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> u64
{
    __0
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<(i64, i64)>, usize),
) -> core::option::Option<SpannedNode<(i64, i64)>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<SpannedNode<(i64, i64)>>
{
    None
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<u64>, usize),
) -> core::option::Option<SpannedNode<u64>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<SpannedNode<u64>>
{
    None
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, u64, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<u64>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, Vec<CardEntry>, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<Vec<CardEntry>>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<CardEntry>, usize),
    (_, e, _): (usize, core::option::Option<CardEntry>, usize),
) -> Vec<CardEntry>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<String>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<DefinitionVariable>, usize),
    (_, e, _): (usize, core::option::Option<DefinitionVariable>, usize),
) -> Vec<DefinitionVariable>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, DefinitionFaction, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<DefinitionFaction>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, Vec<DefinitionVariable>, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<Vec<DefinitionVariable>>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize),
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    v
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<Definition>, usize),
) -> SpannedNode<Definition>
{
    __0
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, Definition, usize),
    (_, r, _): (usize, usize, usize),
) -> SpannedNode<Definition>
{
    SpannedNode::new(val, span_maker.span(l, r, "spanned"))
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, SpannedNode<Definition>, usize),
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize),
    (_, e, _): (usize, SpannedNode<Definition>, usize),
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, DefinitionVariable, usize),
) -> core::option::Option<DefinitionVariable>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<DefinitionVariable>
{
    None
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<DefinitionVariable>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<DefinitionVariable>, usize),
) -> alloc::vec::Vec<DefinitionVariable>
{
    v
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, DefinitionVariable, usize),
    (_, _, _): (usize, &'input str, usize),
) -> DefinitionVariable
{
    __0
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, CardEntry, usize),
) -> core::option::Option<CardEntry>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<CardEntry>
{
    None
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<CardEntry>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<CardEntry>, usize),
) -> alloc::vec::Vec<CardEntry>
{
    v
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, CardEntry, usize),
    (_, _, _): (usize, &'input str, usize),
) -> CardEntry
{
    __0
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, CardEntry, usize),
) -> alloc::vec::Vec<CardEntry>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<CardEntry>, usize),
    (_, e, _): (usize, CardEntry, usize),
) -> alloc::vec::Vec<CardEntry>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, __0, _): (usize, DefinitionVariable, usize),
) -> alloc::vec::Vec<DefinitionVariable>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<DefinitionVariable>, usize),
    (_, e, _): (usize, DefinitionVariable, usize),
) -> alloc::vec::Vec<DefinitionVariable>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, CardEntry, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<CardEntry>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action55(
        program_information,
        span_maker,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<CardEntry>, usize),
    __1: (usize, CardEntry, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<CardEntry>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action55(
        program_information,
        span_maker,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, core::option::Option<CardEntry>, usize),
) -> Vec<CardEntry>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action53(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<CardEntry>, usize),
    __1: (usize, core::option::Option<CardEntry>, usize),
) -> Vec<CardEntry>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        program_information,
        span_maker,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, DefinitionVariable, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<DefinitionVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        program_information,
        span_maker,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<DefinitionVariable>, usize),
    __1: (usize, DefinitionVariable, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<DefinitionVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action50(
        program_information,
        span_maker,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, core::option::Option<DefinitionVariable>, usize),
) -> Vec<DefinitionVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<DefinitionVariable>, usize),
    __1: (usize, core::option::Option<DefinitionVariable>, usize),
) -> Vec<DefinitionVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        program_information,
        span_maker,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<Definition>, usize),
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize),
    __1: (usize, SpannedNode<Definition>, usize),
) -> alloc::vec::Vec<SpannedNode<Definition>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        program_information,
        span_maker,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action40(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<SpannedNode<Definition>>, usize),
) -> Program
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action41(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u64, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, usize, usize),
) -> SpannedNode<u64>
{
    let __start0 = __1.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action28(
        program_information,
        span_maker,
        input,
        __1,
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, usize, usize),
    __4: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, usize, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, i64, usize),
    __4: (usize, usize, usize),
    __5: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, i64, usize),
    __5: (usize, usize, usize),
    __6: (usize, &'input str, usize),
) -> Result<SpannedNode<(i64, i64)>,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, u64, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> SpannedNode<u64>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Definition, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<Definition>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, DefinitionFaction, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<DefinitionFaction>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Vec<DefinitionVariable>, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<Vec<DefinitionVariable>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, u64, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<u64>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, u64, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<u64>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Vec<CardEntry>, usize),
    __1: (usize, usize, usize),
) -> SpannedNode<Vec<CardEntry>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        program_information,
        span_maker,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action87<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, i64, usize),
    __4: (usize, &'input str, usize),
) -> SpannedNode<(i64, i64)>
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, i64, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, i64, usize),
    __5: (usize, &'input str, usize),
) -> Result<SpannedNode<(i64, i64)>,__lalrpop_util::ParseError<usize,Token<'input>,DictionaryError>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, u64, usize),
    __3: (usize, &'input str, usize),
) -> SpannedNode<u64>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Definition, usize),
) -> SpannedNode<Definition>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, DefinitionFaction, usize),
) -> SpannedNode<DefinitionFaction>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action92<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Vec<DefinitionVariable>, usize),
) -> SpannedNode<Vec<DefinitionVariable>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action93<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, String, usize),
) -> SpannedNode<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action94<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, u64, usize),
) -> SpannedNode<u64>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action95<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, u64, usize),
) -> SpannedNode<u64>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action96<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, Vec<CardEntry>, usize),
) -> SpannedNode<Vec<CardEntry>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action97<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, CardEntry, usize),
) -> Vec<CardEntry>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action98<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<CardEntry>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action52(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action99<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<CardEntry>, usize),
    __1: (usize, CardEntry, usize),
) -> Vec<CardEntry>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action51(
        program_information,
        span_maker,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action100<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<CardEntry>, usize),
) -> Vec<CardEntry>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action52(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
    __2: (usize, core::option::Option<SpannedNode<u64>>, usize),
    __3: (usize, SpannedNode<(i64, i64)>, usize),
) -> CardEntry
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action29(
        program_information,
        span_maker,
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
    __2: (usize, core::option::Option<SpannedNode<u64>>, usize),
) -> CardEntry
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action103<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
    __2: (usize, SpannedNode<u64>, usize),
    __3: (usize, SpannedNode<(i64, i64)>, usize),
) -> CardEntry
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        program_information,
        span_maker,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action104<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
    __2: (usize, SpannedNode<(i64, i64)>, usize),
) -> CardEntry
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action32(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action105<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
    __2: (usize, SpannedNode<u64>, usize),
) -> CardEntry
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        program_information,
        span_maker,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, SpannedNode<u64>, usize),
    __1: (usize, SpannedNode<u64>, usize),
) -> CardEntry
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        program_information,
        span_maker,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, DefinitionVariable, usize),
) -> Vec<DefinitionVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action46(
        program_information,
        span_maker,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<DefinitionVariable>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action47(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        program_information,
        span_maker,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<DefinitionVariable>, usize),
    __1: (usize, DefinitionVariable, usize),
) -> Vec<DefinitionVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        program_information,
        span_maker,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'input,
>(
    program_information: &mut ProgramInformation,
    span_maker: &mut SpanMaker<'input>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<DefinitionVariable>, usize),
) -> Vec<DefinitionVariable>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        program_information,
        span_maker,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        program_information,
        span_maker,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, DictionaryError>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, DictionaryError>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), DictionaryError>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, DictionaryError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
