use crate::global::{parse_function_args, Result};
use std::collections::HashMap;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Function {
    pub args: Vec<String>,
    pub list: Vec<String>,
}

pub type AliasDictionary = HashMap<String, Function>;
type AliasEntry = (String, Function);

pub fn parse_aliases_config(dir: &str) -> Result<AliasDictionary> {
    let doc = &load_yaml(dir, "aliases.yaml")?;
    let dict = parse_aliases(doc);
    check_deps(&dict)?;
    Ok(dict)
}

fn load_yaml(dir: &str, filename: &str) -> Result<Yaml> {
    let path = format!("{}/{}", dir, filename);
    let content =
        &fs::read_to_string(path).unwrap_or_else(|_| panic!("no '{}' found in {}", filename, dir));

    let mut docs = YamlLoader::load_from_str(content)?;
    assert_eq!(1, docs.len(), "only one document supported in YAML config");

    Ok(docs.remove(0))
}

fn parse_aliases(doc: &Yaml) -> AliasDictionary {
    match doc.as_hash() {
        Some(aliases) => aliases.iter().map(parse_function).collect(),
        None => AliasDictionary::new(),
    }
}

fn parse_function((sign, list): (&Yaml, &Yaml)) -> AliasEntry {
    let sign = sign.as_str().expect("alias key should be a string");
    let sign = parse_function_args(&sign);
    let list = if list.is_array() {
        list.as_vec()
            .unwrap()
            .iter()
            .map(Yaml::as_str)
            .map(Option::unwrap)
            .map(String::from)
            .collect()
    } else {
        vec![String::from(list.as_str().unwrap())]
    };
    (sign.0, Function { args: sign.1, list })
}

fn check_deps(dict: &AliasDictionary) -> Result<()> {
    enum X {
        CURRENT,
        VISITED,
    }

    fn check(visited: &mut HashMap<String, X>, sign: String, dict: &AliasDictionary) -> Result<()> {
        match visited.get(&sign) {
            Some(X::VISITED) => return Ok(()),
            Some(X::CURRENT) => panic!("circular dependency found in alias: {}", sign),
            _ => (),
        }

        if let Some(func) = dict.get(&sign) {
            visited.insert(sign.clone(), X::CURRENT);
            for next in func.list.iter() {
                check(visited, parse_function_args(&next).0 /* sign */, dict)?
            }
            visited.insert(sign.clone(), X::VISITED);
        }

        Ok(())
    }

    let visited = &mut HashMap::default();
    for sign in dict.keys() {
        check(visited, String::from(sign), dict)?;
    }
    Ok(())
}
