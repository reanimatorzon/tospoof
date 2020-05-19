//! Parses 'aliases.yaml' file located in the same directory as this binary.

use std::collections::HashMap;
#[allow(unused)]
use std::fs;

use anyhow::bail;
use cfg_if::cfg_if;
use yaml_rust::{Yaml, YamlLoader};

use crate::global::{parse_function_args, Result};

const ALIASES_FILE_NAME: &str = "aliases.yaml";

/// Alias aka Function
#[derive(Debug, Eq)]
pub struct Function {
    pub args: Vec<String>,
    pub list: Vec<String>,
}

/// Aliases storage
pub type AliasDictionary = HashMap<String, Function>;

type AliasEntry = (String, Function);

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.args.eq(&other.args) && self.list.eq(&other.list)
    }
}

pub fn parse_aliases_config(dir: &str) -> Result<AliasDictionary> {
    let doc = load_yaml(dir, ALIASES_FILE_NAME)?;
    let dict = parse_aliases(&doc);
    check_deps(&dict)?;
    Ok(dict)
}

fn get_aliases_file_contents(_path: String) -> Result<String> {
    cfg_if! {
        if #[cfg(not(test))] {
            Ok(fs::read_to_string(_path)?)
        } else {
            Ok(String::from(&_path[0.._path.len() - ALIASES_FILE_NAME.len() - 1]))
        }
    }
}

fn load_yaml(dir: &str, filename: &str) -> Result<Option<Yaml>> {
    let path = format!("{}/{}", dir, filename);
    let content = &get_aliases_file_contents(path)?;

    let mut docs = YamlLoader::load_from_str(content)?;
    assert!(docs.len() <= 1,);

    match docs.len() {
        0 => Ok(None),
        1 => Ok(Some(docs.remove(0))),
        _ => bail!("several documents aren't supported in 'aliases.yaml' config"),
    }
}

fn parse_aliases(doc: &Option<Yaml>) -> AliasDictionary {
    if let Some(doc) = doc {
        if let Some(aliases) = doc.as_hash() {
            return aliases.iter().map(parse_function).collect();
        }
    }
    AliasDictionary::with_capacity(0)
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
            Some(X::CURRENT) => bail!("circular dependency found in alias: {}", sign),
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

// @see src/mocks.rs::ALIASES_FILE_CONTENT
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some() {
        let aliases: String = "\
            my-project-hosted-in-local-network-addr: 192.168.1.101\n\
            my-prj: my-project-hosted-in-local-network-addr\n\
            local(third, fourth): 127.0.<third>.<fourth>\n\
            dev(site): <site>.dev\n\
            localhost(last): local(0, <last>)\n\
            my-domains(lang):\n\
              - <lang>.example.com\n\
              - <lang>.test.net\n\
            my-domains-full-list:\n\
              - us.example.com\n\
              - my-domains(de)\n\
              - my-domains(static)\n\
            some-domains-as-one-line-array: [example.com, example.net]\n\
            preset-example:\n\
              - localhost(1)\n\
              - dev(example.local)\n\
              - my-domains-full-list\n\
            preset-as-array-example: ['local(0, 1)', my-prj]\n\
        "
        .to_string();

        let actual = parse_aliases_config(&aliases);

        let mut expected = HashMap::new();

        expected.insert(
            "my-project-hosted-in-local-network-addr".to_string(),
            Function {
                args: vec![],
                list: vec!["192.168.1.101".to_string()],
            },
        );

        expected.insert(
            "my-prj".to_string(),
            Function {
                args: vec![],
                list: vec!["my-project-hosted-in-local-network-addr".to_string()],
            },
        );

        expected.insert(
            "local(2)".to_string(),
            Function {
                args: vec!["third".to_string(), "fourth".to_string()],
                list: vec!["127.0.<third>.<fourth>".to_string()],
            },
        );

        expected.insert(
            "dev(1)".to_string(),
            Function {
                args: vec!["site".to_string()],
                list: vec!["<site>.dev".to_string()],
            },
        );

        expected.insert(
            "localhost(1)".to_string(),
            Function {
                args: vec!["last".to_string()],
                list: vec!["local(0, <last>)".to_string()],
            },
        );

        expected.insert(
            "my-domains(1)".to_string(),
            Function {
                args: vec!["lang".to_string()],
                list: vec![
                    "<lang>.example.com".to_string(),
                    "<lang>.test.net".to_string(),
                ],
            },
        );

        expected.insert(
            "my-domains-full-list".to_string(),
            Function {
                args: vec![],
                list: vec![
                    "us.example.com".to_string(),
                    "my-domains(de)".to_string(),
                    "my-domains(static)".to_string(),
                ],
            },
        );

        expected.insert(
            "some-domains-as-one-line-array".to_string(),
            Function {
                args: vec![],
                list: vec!["example.com".to_string(), "example.net".to_string()],
            },
        );

        expected.insert(
            "preset-example".to_string(),
            Function {
                args: vec![],
                list: vec![
                    "localhost(1)".to_string(),
                    "dev(example.local)".to_string(),
                    "my-domains-full-list".to_string(),
                ],
            },
        );

        expected.insert(
            "preset-as-array-example".to_string(),
            Function {
                args: vec![],
                list: vec!["local(0, 1)".to_string(), "my-prj".to_string()],
            },
        );

        assert_eq!(actual.unwrap(), expected);
    }
}
