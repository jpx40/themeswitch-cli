use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "conf.pest"]
pub struct CONFParser;

enum CONFValue {
    Object(Vec<(&'a str, CONFValue<'a>)>),
    Array(Vec<CONFValue<'a>>),
    Char(&'a str),
    // could be an miskake
    //
    //this too
    Path(CONFValue<'a>),
    Wallpaper(CONFValue<'a>),
    Color(CONFValue<'a>),
    Cmd(CONFValue<'a>),
    Profile(Vec<CONFValue<'a>>),
    Name(&'a str),
    String(&'a str),
    Val(&'a str),
    Value(&'a str),
    Path(CONFValue<'a>),
    Params(Vec<CONFValue<'a>>),
    File(Vec<CONFValue<'a>>),
    Import(CONFValue<'a>),
    Exec(Vec<CONFValue<'a>>),
    Script(Vec<CONFValue<'a>>),
    Variable(CONFValue<'a>, CONFValue<'a>),
    Comment,
    Args(CONFValue<'a>),
    Out(CONFValue<'a>),
}

pub fn parse_conf(path: &str) -> Result<Conf, String> {}
