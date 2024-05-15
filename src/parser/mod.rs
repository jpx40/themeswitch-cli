use crate::store::PROFILE;
use crate::util::path::expand_path;
use crate::{parser, template, util};
use crate::{store::*, wallpaper};

use crate::wallpaper::wpaper::check_paper;
use camino::Utf8Path;
use itertools::{cloned, ProcessResults};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::path::{self, Path};
use std::process::Command;
use std::string::String;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wpaper {
    pub engine: Option<String>,
    pub path: Option<String>,
    pub wallpaper: Option<Vec<String>>,
    pub config: Option<HashMap<String, String>>,
}

pub struct File {
    pub path: String,

    pub import: Vec<Import>,

    pub profiles: HashMap<String, Profile>,

    pub global_variables: HashMap<String, Variable>,
}

impl Wpaper {
    fn new() -> Self {
        Wpaper {
            engine: None,
            path: None,
            wallpaper: None,
            config: None,
        }
    }
    fn set_path(&mut self, path: String) {
        self.path = Some(path)
    }
    pub fn set_config(&mut self, config: HashMap<String, String>) {
        self.config = Some(config);
    }

    fn set_engine(&mut self, engine: String) {
        self.engine = Some(engine);
    }
    fn set_wallpaper(&mut self, wallpaper: Vec<String>) {
        self.wallpaper = Some(wallpaper);
    }
}
//
// pub fn parse_conf(path: &str) -> Result<Conf, String> {
//     let mut ast = vec![];
// }
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub variables: Option<HashMap<String, Variable>>,
    pub exec: Option<Vec<Exec>>,
    pub script: Option<Vec<Script>>,
    pub template: Option<Vec<Template>>,
    pub color: Option<Color>,
    pub wallpaper: Option<wallpaper::wpaper::Paper>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Variable {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imports {
    pub file: String,
    pub import: Vec<Import>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub path: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub path: Option<String>,
    pub arg: Option<Vec<String>>,
    pub params: Option<Vec<HashMap<String, String>>>,
}

pub struct Param {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmd {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(HashMap<String, String>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub path: String,
    pub out: String,
    pub color: Option<Color>,
    pub arg: Option<Vec<String>>,
    pub params: Option<Vec<HashMap<String, String>>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exec {
    pub arg: Option<Vec<String>>,
    pub path: Option<String>,
    pub cmd: Option<String>,
    pub params: Option<Vec<HashMap<String, String>>>,
}

pub fn parse_conf(path: &str) -> Result<(), String> {
    let unparsed_file = std::fs::read_to_string(path).expect("cannot read file");

    let file = CONFParser::parse(Rule::file, &unparsed_file)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .expect("file rule is missing");

    //    println!("{:#?}", file);
    let mut current_section_name = "";

    fn get_path(path: &str) -> &str {
        let mut path = path.split('=').collect::<Vec<&str>>()[1];
        if path.contains('\"') {
            path = path.trim_matches('\"');
        }

        path
    }

    fn get_args(args: Pairs<Rule>) -> Vec<String> {
        let mut arg: Vec<String> = Vec::new();
        for line in args {
            let line_nr = line.line_col();
            if line.as_rule() == Rule::array {
                let line_nr = line.line_col();
                for line in line.into_inner() {
                    if line.as_rule() == Rule::string {
                        let line_nr = line.line_col();
                        for line in line.into_inner() {
                            let line_nr = line.line_col();
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
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ParamsResult {
        Path(String),
        Params(HashMap<String, String>),
    }
    //
    fn get_params(params: Pairs<Rule>) -> Result<ParamsResult, String> {
        let mut map: Option<HashMap<String, String>> = None;
        let mut path: Option<String> = None;
        for line in params {
            match line.as_rule() {
                Rule::path => {
                    let line_nr = line.line_col();
                    let path_temp =
                        expand_path(get_path(line.as_str())).unwrap_or_else(|err| panic!("{err}"));

                    path = Some(path_temp)
                }
                Rule::object => {
                    let line_nr = line.line_col();
                    for line in line.into_inner() {
                        let line_nr = line.line_col();
                        if line.as_rule() == Rule::o_pair {
                            let line_nr = line.line_col();
                            let mut key = String::new();
                            let mut value = String::new();
                            for line in line.into_inner() {
                                let line_nr = line.line_col();
                                match line.as_rule() {
                                    Rule::key => {
                                        let line_nr = line.line_col();
                                        key = line.into_inner().as_str().to_string();
                                    }
                                    Rule::string => {
                                        let line_nr = line.line_col();
                                        value = line.into_inner().as_str().to_string();
                                    }
                                    _ => {}
                                }
                            }
                            if let Some(m) = map.as_mut() {
                                m.insert(key, value);
                            } else if !key.is_empty() && !value.is_empty() {
                                map = Some(HashMap::from([(key, value)]));
                            } else if key.is_empty() {
                                return Err("Empty key in HashMap".to_string());
                            } else if value.is_empty() {
                                return Err("Empty value in HashMap".to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(path) = path {
            Ok(ParamsResult::Path(path))
        } else if let Some(map) = map {
            Ok(ParamsResult::Params(map))
        } else {
            Err("Failed to get params".to_string())
        }
    }
    fn get_variable(rules: Pairs<Rule>) -> Variable {
        let mut name = String::new();
        let mut value = String::new();
        for line in rules {
            let line_nr = line.line_col();
            match line.as_rule() {
                Rule::var => {
                    let line_nr = line.line_col();
                    name = line.as_str().to_string();
                }
                Rule::string => {
                    let line_nr = line.line_col();
                }
                _ => {}
            };
        }
        Variable { name, value }
    }
    fn get_wallpaper(wallpaper: Pairs<Rule>) -> Wpaper {
        let mut w = Wpaper::new();
        let mut map: Option<HashMap<String, String>> = None;
        //   println!("{:#?}", wallpaper);
        for line in wallpaper {
            let line_nr = line.line_col();
            match line.as_rule() {
                Rule::path => {
                    let line_nr = line.line_col();
                    let path = get_path(line.as_str());

                    let path = expand_path(path).unwrap_or_else(|err| panic!("{err}"));
                    // let path = if let Some(path) = Utf8Path::new(path)
                    //     .canonicalize()
                    //     .unwrap_or_else(|err| panic!("{err}"))
                    //     .to_str()
                    // {
                    //     path.to_string()
                    // } else {
                    //     panic!("not a valid path")
                    // };

                    w.set_path(path.to_string());
                }
                Rule::array => {
                    let line_nr = line.line_col();
                    let mut array: Vec<String> = Vec::new();
                    for line in line.into_inner() {
                        let line_nr = line.line_col();
                        let mut s = String::new();
                        if line.as_rule() == Rule::string {
                            let line_nr = line.line_col();

                            s = line.into_inner().as_str().to_string();
                        }
                        if !s.is_empty() {
                            array.push(s);
                        }
                    }

                    w.set_wallpaper(array);
                }
                Rule::engine => {
                    let line_nr = line.line_col();
                    let mut engine = String::new();

                    for line in line.into_inner() {
                        let line_nr = line.line_col();
                        if Rule::string == line.as_rule() {
                            let line_nr = line.line_col();
                            let line = line.into_inner();
                            engine = line.as_str().to_string();
                        }
                    }
                    if !engine.is_empty() {
                        w.set_engine(engine)
                    }
                }
                Rule::config => {
                    let line_nr = line.line_col();
                    let line = if let Some(line) = line.into_inner().next() {
                        line
                    } else {
                        panic!("config is empty")
                    };
                    if line.as_rule() == Rule::object {
                        let line_nr = line.line_col();
                        for line in line.into_inner() {
                            let line_nr = line.line_col();
                            if line.as_rule() == Rule::o_pair {
                                let line_nr = line.line_col();
                                let mut key = String::new();
                                let mut value = String::new();
                                for line in line.into_inner() {
                                    let line_nr = line.line_col();
                                    match line.as_rule() {
                                        Rule::key => {
                                            let line_nr = line.line_col();
                                            key = line.into_inner().as_str().to_string();
                                        }
                                        Rule::string => {
                                            let line_nr = line.line_col();
                                            value = line.into_inner().as_str().to_string();
                                        }
                                        _ => {}
                                    }
                                }
                                if let Some(m) = map.as_mut() {
                                    m.insert(key, value);
                                } else if !key.is_empty() && !value.is_empty() {
                                    map = Some(HashMap::from([(key, value)]));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        w
    }

    //
    let mut imports: Vec<Import> = Vec::new();
    let mut profiles: Vec<Profile> = Vec::new();
    let mut global_variables: HashMap<String, Variable> = HashMap::new();
    fn get_color(color: Pairs<Rule>) -> Result<HashMap<String, String>, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        match get_params(color) {
            Ok(p) => match p {
                ParamsResult::Params(p) => Ok(p),

                ParamsResult::Path(p) => match util::file_to_hashmap(&p) {
                    Ok(c) => Ok(c),
                    Err(err) => Err(err),
                },
            },

            Err(err) => Err(err),
        }
    }
    fn get_cmd(cmd: &str) -> &str {
        cmd.split('=').collect::<Vec<&str>>()[1]
    }

    fn get_imports(import: Pairs<Rule>) -> String {
        let mut import_out = String::new();
        for line in import {
            let line_nr = line.line_col();
            if line.as_rule() == Rule::key {
                let line_nr = line.line_col();
                import_out = line.as_str().to_string();
            }
        }
        import_out
    }
    file.into_inner().for_each(|line| match line.as_rule() {
        Rule::profile => {
            let line_nr = line.line_col();
            let mut profile = Profile::new();
            for line in line.into_inner() {
                match line.as_rule() {
                    Rule::name => {
                        profile.set_name(
                            line.as_str().split('=').collect::<Vec<&str>>()[1].to_string(),
                        );
                    }

                    Rule::variable => profile.add_variable(get_variable(line.into_inner())),

                    Rule::script => {
                        let mut script = Script::new();
                        for line in line.into_inner() {
                            match line.as_rule() {
                                Rule::arg => {
                                    script.add_args(get_args(line.into_inner()));
                                }
                                Rule::path => {
                                    let path = get_path(line.as_str());
                                    let path =
                                        expand_path(path).unwrap_or_else(|err| panic!("{err}"));
                                    script.set_path(path.to_string());
                                }
                                Rule::param => {
                                    match get_params(line.into_inner())
                                        .unwrap_or_else(|err| panic!("{err}"))
                                    {
                                        ParamsResult::Path(path) => {
                                            script.add_param(
                                                util::file_to_hashmap(&path)
                                                    .unwrap_or_else(|err| panic!("{err}")),
                                            );
                                        }

                                        ParamsResult::Params(map) => script.add_param(map),
                                    }
                                }
                                _ => (),
                            }
                        }
                        profile.add_script(script);
                    }
                    Rule::exec => {
                        let line_nr = line.line_col();
                        let mut exec = Exec::new();
                        for line in line.into_inner() {
                            let line_nr = line.line_col();
                            match line.as_rule() {
                                Rule::cmd => exec.set_cmd(get_cmd(line.as_str()).to_string()),
                                Rule::arg => {
                                    exec.add_args(get_args(line.into_inner()));
                                }
                                Rule::path => {
                                    let line_nr = line.line_col();
                                    let path = get_path(line.as_str());
                                    exec.set_path(path.to_string());
                                }
                                Rule::param => {
                                    let line_nr = line.line_col();
                                    match get_params(line.into_inner())
                                        .unwrap_or_else(|err| panic!("{err}"))
                                    {
                                        ParamsResult::Path(path) => {
                                            exec.add_param(util::json_to_hashmap(&path));
                                        }

                                        ParamsResult::Params(map) => exec.add_param(map),
                                    }
                                }
                                _ => (),
                            }
                        }
                        profile.add_exec(exec);
                    }
                    Rule::wallpaper => {
                        let line_nr = line.line_col();
                        let wallpaper = get_wallpaper(line.into_inner());

                        profile.set_wallpaper(wallpaper);
                    }

                    Rule::template => {
                        let line_nr = line.line_col();
                        let mut template = Template::new();
                        for line in line.into_inner() {
                            match line.as_rule() {
                                Rule::arg => {
                                    let line_nr = line.line_col();
                                    template.add_args(get_args(line.into_inner()));
                                }
                                Rule::path => {
                                    let line_nr = line.line_col();
                                    let path = get_path(line.as_str());
                                    template.set_path(path.to_string());
                                }
                                Rule::param => {
                                    let line_nr = line.line_col();
                                    match get_params(line.into_inner())
                                        .unwrap_or_else(|err| panic!("{err}"))
                                    {
                                        ParamsResult::Path(path) => {
                                            template.add_param(
                                                util::file_to_hashmap(&path)
                                                    .unwrap_or_else(|err| panic!("{err}")),
                                            );
                                        }

                                        ParamsResult::Params(map) => template.add_param(map),
                                    }
                                }
                                Rule::color => {
                                    let line_nr = line.line_col();
                                    template.add_color(
                                        get_color(line.into_inner())
                                            .unwrap_or_else(|err| panic!("{err}")),
                                    )
                                }
                                _ => (),
                            }
                        }
                        profile.add_template(template)
                    }
                    Rule::color => profile.add_color(
                        get_color(line.into_inner()).unwrap_or_else(|err| panic!("{err}")),
                    ),

                    _ => {}
                }
            }

            profiles.push(profile);
        }
        Rule::import => {
            let line_nr = line.line_col();
            let import = Import::new(get_imports(line.into_inner()));
            imports.push(import)
        }
        Rule::variable => {
            let line_nr = line.line_col();
            let var = get_variable(line.into_inner());

            global_variables.insert(var.name.clone(), var);
        }
        Rule::EOI => (),
        _ => unreachable!(),
    });

    let profiles_map: HashMap<String, Profile> =
        HashMap::from_iter(profiles.into_iter().map(|p| (p.name.clone(), p)));

    if let Ok(mut store) = PROFILE.lock() {
        *store = profiles_map.clone();
    }

    if let Ok(mut store) = GLOBAL_VARIABEL.lock() {
        store.extend(global_variables.clone());
    }

    let mut file = File::new();
    file.add_filename(path.to_string());
    file.add_profiles(profiles_map);
    file.add_imports(imports.clone());
    file.add_global_variables(global_variables);

    let imports: Imports = Imports {
        import: imports,
        file: path.to_string(),
    };
    if let Ok(mut store) = IMPORT.lock() {
        store.push(imports.clone());
    }
    Ok(())
}

impl File {
    fn new() -> Self {
        File {
            path: String::new(),
            import: vec![],
            profiles: HashMap::new(),
            global_variables: HashMap::new(),
        }
    }
    pub fn add_filename(&mut self, path: String) {
        self.path = path
    }
    pub fn add_profiles(&mut self, profile: HashMap<String, Profile>) {
        if self.profiles.is_empty() {
            self.profiles = profile
        } else {
            self.profiles.extend(profile)
        }
    }
    pub fn add_global_variables(&mut self, global_variables: HashMap<String, Variable>) {
        if self.global_variables.is_empty() {
            self.global_variables = global_variables
        } else {
            self.global_variables.extend(global_variables)
        }
    }
    pub fn add_import(&mut self, import: Import) {
        self.import.push(import);
    }
    pub fn add_imports(&mut self, imports: Vec<Import>) {
        if self.import.is_empty() {
            self.import = imports;
        } else {
            self.import.extend(imports);
        }
    }
    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn add_global_variable(&mut self, variable: Variable) {
        self.global_variables
            .insert(variable.name.clone(), variable);
    }
}

impl Color {
    fn new() -> Self {
        Color(HashMap::new())
    }
    fn make_new(key: String, value: String) -> Color {
        Color(HashMap::from([(key, value)]))
    }
    fn add(&mut self, name: String, value: String) {
        self.0.insert(name, value);
    }
    fn get(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(|s| s.as_str())
    }
    fn get_all(&self) -> HashMap<String, String> {
        self.0.clone()
    }

    fn merge(&mut self, map_ref: Color) {
        self.0
            .extend(map_ref.0.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }
}
impl Imports {
    pub fn new() -> Self {
        Imports {
            import: vec![],
            file: String::new(),
        }
    }

    pub fn add(&mut self, path: String) {
        self.import.push(Import::new(path));
    }
    pub fn set_filename(&mut self, file: String) {
        self.file = file;
    }
}

impl Variable {
    pub fn new(name: String, value: String) -> Self {
        Variable { name, value }
    }
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            name: String::new(),
            variables: None,
            exec: None,
            script: None,
            template: None,
            color: None,
            wallpaper: None,
        }
    }

    pub fn set_wallpaper(&mut self, wallpaper: Wpaper) {
        let wallpaper = wallpaper::wpaper::Paper::Wpaper(wallpaper);
        let wallpaper = match check_paper(wallpaper) {
            Ok(w) => w,
            Err(e) => panic!("{e}"),
        };
        self.wallpaper = Some(wallpaper);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_variable(&mut self, variable: Variable) {
        if self.variables.is_some() {
            self.variables
                .as_mut()
                .unwrap()
                .insert(variable.name.clone(), variable);
        } else if self.variables.is_none() {
            self.variables = Some(HashMap::from([(variable.name.clone(), variable)]));
        }
    }

    pub fn add_exec(&mut self, exec: Exec) {
        if self.exec.is_some() {
            self.exec.as_mut().unwrap().push(exec);
        } else if self.exec.is_none() {
            self.exec = Some(vec![exec]);
        }
    }

    pub fn add_script(&mut self, script: Script) {
        if self.script.is_some() {
            self.script.as_mut().unwrap().push(script);
        } else if self.script.is_none() {
            self.script = Some(vec![script]);
        }
    }
    fn add_color(&mut self, color: HashMap<String, String>) {
        let color = Color(color);
        self.color = Some(color);
    }
    pub fn add_template(&mut self, template: Template) {
        if self.template.is_some() {
            self.template.as_mut().unwrap().push(template);
        } else if self.template.is_none() {
            self.template = Some(vec![template]);
        }
    }
}
impl Template {
    pub fn new() -> Self {
        Template {
            path: String::new(),
            out: String::new(),
            color: None,
            arg: None,
            params: None,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_out(&mut self, out: String) {
        self.out = out;
    }
    fn add_param(&mut self, param: HashMap<String, String>) {
        if self.params.is_some() {
            self.params.as_mut().unwrap().push(param);
        } else if self.arg.is_none() {
            self.params = Some(vec![param]);
        }
    }
    fn add_color(&mut self, color: HashMap<String, String>) {
        let color = Color(color);
        self.color = Some(color);
    }
    fn add_args(&mut self, args: Vec<String>) {
        self.arg = Some(args);
    }
    fn add_arg(&mut self, arg: String) {
        if self.arg.is_some() {
            self.arg.as_mut().unwrap().push(arg);
        } else if self.arg.is_none() {
            self.arg = Some(vec![arg]);
        }
    }
}
impl Exec {
    pub fn new() -> Self {
        Self {
            path: None,
            arg: None,
            params: None,
            cmd: None,
        }
    }
    pub fn set_cmd(&mut self, cmd: String) {
        self.cmd = Some(cmd);
    }
    fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn add_arg(&mut self, arg: String) {
        if self.arg.is_some() {
            self.arg.as_mut().unwrap().push(arg);
        } else if self.arg.is_none() {
            self.arg = Some(vec![arg]);
        }
    }
    fn add_args(&mut self, args: Vec<String>) {
        self.arg = Some(args);
    }
    fn add_param(&mut self, param: HashMap<String, String>) {
        if self.params.is_some() {
            self.params.as_mut().unwrap().push(param);
        } else if self.arg.is_none() {
            self.params = Some(vec![param]);
        }
    }
}

impl Script {
    pub fn new() -> Self {
        Self {
            path: None,
            arg: None,
            params: None,
        }
    }

    fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }
    fn add_args(&mut self, args: Vec<String>) {
        self.arg = Some(args);
    }
    fn add_arg(&mut self, arg: String) {
        if self.arg.is_some() {
            self.arg.as_mut().unwrap().push(arg);
        } else if self.arg.is_none() {
            self.arg = Some(vec![arg]);
        }
    }
    fn add_param(&mut self, param: HashMap<String, String>) {
        if self.params.is_some() {
            self.params.as_mut().unwrap().push(param);
        } else if self.arg.is_none() {
            self.params = Some(vec![param]);
        }
    }
}

impl Import {
    pub fn new(path: String) -> Self {
        Import { path }
    }
}
