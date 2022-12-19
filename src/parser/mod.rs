/*
meml – XML replacement written in Rust with the pest library <https://pest.rs>.
Developed to be used in ygo_destiny <https://github.com/myuujiku/ygo_destiny/>.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published
by the Free Software Foundation, either version 3 of the License,
or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod element;
mod function;
mod string;

use std::collections::HashMap;

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};

use element::{Element, ElementFactory};
use function::Function;
use string::parse_string;

#[derive(Parser)]
#[grammar = "meml.pest"]
pub struct MemlParser {}

#[derive(Clone, Debug)]
pub enum Definition<'a> {
    String(String),
    Element(ElementFactory<'a>),
    Function(Function<'a>),
}

pub type Arguments = HashMap<String, String>;
pub type DefinitionMap<'a> = HashMap<String, HashMap<String, Definition<'a>>>;

pub fn parse_raw(raw_input: &str) -> Pairs<Rule> {
    let pairs = MemlParser::parse(Rule::meml, raw_input);
    match pairs {
        Ok(x) => x,
        x => panic!("{}", x.unwrap_err()),
    }
}

pub fn get_definitions(pairs: Pairs<Rule>) -> (DefinitionMap, Vec<Pair<Rule>>) {
    let mut local_definitions = HashMap::from([
        ("strings".to_string(), HashMap::new()),
        ("elements".to_string(), HashMap::new()),
        ("functions".to_string(), HashMap::new()),
    ]);
    let mut remaining = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::string_const_def => {
                let mut inner_rules = pair.into_inner();
                let key = inner_rules.next().unwrap().as_str().to_string();
                let val = Definition::String(parse_string(
                    inner_rules.next().unwrap(),
                    &local_definitions,
                    None,
                ));
                local_definitions
                    .get_mut("strings")
                    .unwrap()
                    .insert(key, val);
            }
            Rule::element_const_def => {
                let mut inner_rules = pair.into_inner();
                local_definitions.get_mut("elements").unwrap().insert(
                    inner_rules.next().unwrap().as_str().to_string(),
                    Definition::Element(Element::factory(inner_rules.next().unwrap())),
                );
            }
            Rule::func_def => {
                let mut inner_rules = pair.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let arg_names = inner_rules.next().unwrap().into_inner().map(|pair| pair.as_str().to_string()).collect();
                local_definitions.get_mut("functions").unwrap().insert(
                    name,
                    Definition::Function(Element::function(inner_rules.next().unwrap(), arg_names)),
                );
            }
            Rule::EOI => (),
            _ => remaining.push(pair),
        }
    }

    return (local_definitions, remaining);
}

pub fn get_content(pairs: Vec<Pair<Rule>>, local_definitions: DefinitionMap) {
    let mut root = Vec::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::element => {
                root.push(Element::construct(pair, &local_definitions, None));
            }
            _ => unreachable!(),
        }
    }
    
    println!("{:#?}", root);
    println!("{}", root
        .iter()
        .map(|i| i.as_xml())
        .collect::<Vec<String>>()
        .join(""));
}
