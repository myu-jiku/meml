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

use once_cell::sync::OnceCell;

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

pub fn get_definitions<'a>(
    pairs: Pairs<'a, Rule>,
    external_definitions: &DefinitionMap<'a>,
) -> (DefinitionMap<'a>, DefinitionMap<'a>, Vec<Pair<'a, Rule>>) {
    let mut local_definitions = HashMap::from([
        ("strings".to_string(), HashMap::new()),
        ("elements".to_string(), HashMap::new()),
        ("functions".to_string(), HashMap::new()),
    ]);
    let mut exports = local_definitions.clone();
    let mut remaining = Vec::<Pair<Rule>>::new();

    for pair in pairs {
        let (strings, elements, functions) = eval_definition(
            pair,
            external_definitions,
            &mut local_definitions,
            &mut exports,
            &mut remaining,
        );
        local_definitions
            .get_mut("strings")
            .unwrap()
            .extend(strings);
        local_definitions
            .get_mut("elements")
            .unwrap()
            .extend(elements);
        local_definitions
            .get_mut("functions")
            .unwrap()
            .extend(functions);
    }

    return (local_definitions, exports, remaining);
}

fn eval_definition<'a>(
    pair: Pair<'a, Rule>,
    external_definitions: &DefinitionMap<'a>,
    local_definitions: &mut DefinitionMap<'a>,
    exports: &mut DefinitionMap<'a>,
    remaining: &mut Vec<Pair<'a, Rule>>,
) -> (
    HashMap<String, Definition<'a>>,
    HashMap<String, Definition<'a>>,
    HashMap<String, Definition<'a>>,
) {
    let mut strings = HashMap::new();
    let mut elements = HashMap::new();
    let mut functions = HashMap::new();

    match pair.as_rule() {
        Rule::string_const_def => {
            let mut inner_rules = pair.into_inner();
            let key = inner_rules.next().unwrap().as_str().to_string();
            let val = Definition::String(parse_string(
                inner_rules.next().unwrap(),
                &local_definitions,
                None,
            ));
            strings.insert(key, val);
        }
        Rule::element_const_def => {
            let mut inner_rules = pair.into_inner();
            elements.insert(
                inner_rules.next().unwrap().as_str().to_string(),
                Definition::Element(Element::factory(inner_rules.next().unwrap())),
            );
        }
        Rule::func_def => {
            let mut inner_rules = pair.into_inner();
            let name = inner_rules.next().unwrap().as_str().to_string();
            let arg_names = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .map(|pair| pair.as_str().to_string())
                .collect();
            functions.insert(
                name,
                Definition::Function(Element::function(inner_rules.next().unwrap(), arg_names)),
            );
        }
        Rule::export => {
            let (strings, elements, functions) = eval_definition(
                pair.into_inner().next().unwrap(),
                external_definitions,
                local_definitions,
                exports,
                remaining,
            );
            exports.get_mut("strings").unwrap().extend(strings);
            exports.get_mut("elements").unwrap().extend(elements);
            exports.get_mut("functions").unwrap().extend(functions);
        }
        Rule::include => {
            let span = pair.as_span();
            let mut inner_rules = pair.into_inner();
            let def_type = inner_rules.next().unwrap().as_str();
            let def_category = format!("{}s", def_type);
            let def_name = inner_rules.next().unwrap().as_str().to_string();
            if let Some(value) = external_definitions
                .get(&def_category)
                .unwrap()
                .get(&def_name)
            {
                local_definitions
                    .get_mut(&def_category)
                    .unwrap()
                    .insert(def_name, value.clone());
            } else {
                panic!(
                    "{}",
                    Error::new_from_span(
                        ErrorVariant::<()>::CustomError {
                            message: format!(
                                "undefined {} constant; available values: {}",
                                def_type,
                                external_definitions
                                    .get(&def_category)
                                    .unwrap()
                                    .keys()
                                    .map(|item| format!("`{}`", item))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            )
                        },
                        span,
                    )
                );
            }
        }
        Rule::EOI => (),
        _ => remaining.push(pair),
    }

    return (strings, elements, functions);
}

pub fn get_contents(pairs: Vec<Pair<Rule>>, local_definitions: DefinitionMap) -> Vec<Element> {
    let mut root = Vec::new();
    let mut element_container = OnceCell::with_value(Element::default());

    for pair in pairs {
        match pair.as_rule() {
            Rule::element => {
                root.push(Element::construct(pair, &local_definitions, None));
            }
            Rule::const_use => {
                let elem = element_container.get_mut().unwrap();
                elem.eval_child(pair, &local_definitions, None);
                root.append(&mut elem.children);
            }
            Rule::func_use => {}
            _ => unreachable!(),
        }
    }

    return root;
}
