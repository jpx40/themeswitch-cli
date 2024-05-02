use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "conf.pest"]
pub struct CONFParser;

enum CONFValue {}

pub fn parse_conf(path: &str) -> Result<Conf, String> {}
