use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "conf.pest"]
pub struct CONFParser;
