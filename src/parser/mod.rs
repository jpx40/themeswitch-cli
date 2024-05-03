use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "conf.pest"]
pub struct CONFParser;

enum CONFValue<'a> {
    Object(Vec<(&'a str, CONFValue<'a>)>),
    Array(Vec<CONFValue<'a>>),
    Char(&'a str),
    Path(&'a str),
    Wallpaper(Vec<CONFValue<'a>>),
    Color(&'a str),
    Cmd(&'a str),
    Profile(Vec<CONFValue<'a>>),
    Name(&'a str),
    String(&'a str),
    Val(&'a str),
    Value(&'a str),
    Params(Vec<CONFValue<'a>>),
    File(Vec<CONFValue<'a>>),
    Import(Vec<CONFValue<'a>>),
    Exec(Vec<CONFValue<'a>>),
    Script(Vec<CONFValue<'a>>),
    Variable(&'a str, &'a str),
    Comment,
    Args(Vec<CONFValue<'a>>),
    Out(&'a str),
    Null,
}
//
// pub fn parse_conf(path: &str) -> Result<Conf, String> {
//     let mut ast = vec![];
// }
//

pub fn parse_conf(path: &str) {
    let unparsed_file = std::fs::read_to_string(path).expect("cannot read file");

    let file = CONFParser::parse(Rule::file, &unparsed_file)
        .expect("parse error")
        .next()
        .expect("file rule is missing");

    //    println!("{:#?}", file);
    let mut current_section_name = "";

    fn get_path(path: &str) -> &str {
        path.split('=').collect::<Vec<&str>>()[1]
    }

    fn get_args(args: Pairs<Rule>) -> Vec<String> {
        let mut arg: Vec<String> = Vec::new();
        for line in args {
            if line.as_rule() == Rule::array {
                for line in line.into_inner() {
                    if line.as_rule() == Rule::string {
                        for line in line.into_inner() {
                            if !matches!(line.as_str(), "[]") {
                                arg.push(line.as_str().to_string());
                            }
                        }
                    }
                }
            }
        }
        arg
    }
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::profile => {
                for line in line.into_inner() {
                    match line.as_rule() {
                        Rule::name => {
                            let name: Vec<&str> = line.as_str().split('=').collect();
                            let out = name[1];
                        }

                        Rule::script => {
                            for line in line.into_inner() {
                                match line.as_rule() {
                                    Rule::exec => println!("{:?}", line.as_str()),
                                    Rule::arg => {
                                        let args = get_args(line.into_inner());
                                        println!("{:?}", args);
                                    }
                                    Rule::path => {
                                        let path = get_path(line.as_str());
                                    }
                                    _ => (),
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }
            Rule::variable => {
                let mut inner_rules = line.into_inner();
                let name = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
