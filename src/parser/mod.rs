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

mod macro_fn;
mod object;

use core::fmt::Debug;
use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::Rule;
use macro_fn::MacroFn;
use object::Object;

#[derive(Debug)]
pub enum Definition {
    Constant(String),
    Object(Object),
    List(Vec<String>),
    Macro(MacroFn),
}

pub fn get_definitions<'a>(
    meml: Pairs<'a, Rule>,
    ext_defs: &'a HashMap<String, Definition>,
    meta_properties: &'a HashMap<String, HashMap<String, Vec<String>>>,
) -> (
    HashMap<String, Definition>,
    HashMap<String, Definition>,
    Vec<Pair<'a, Rule>>,
) {
    let mut local_defs = HashMap::new();
    let mut exports = HashMap::new();
    let mut remaining = Vec::new();

    for pair in meml {
        match pair.as_rule() {
            Rule::const_def_local => {
                let mut inner_rules = pair.into_inner();
                local_defs.insert(
                    format!("${}", inner_rules.next().unwrap().as_str()),
                    Definition::Constant(parse_string(
                        inner_rules.next().unwrap(),
                        &local_defs,
                        &ext_defs,
                    )),
                );
            }
            Rule::const_def_extern => {
                let mut inner_rules = pair.into_inner().next().unwrap().into_inner();
                exports.insert(
                    format!("${}", inner_rules.next().unwrap().as_str()),
                    Definition::Constant(parse_string(
                        inner_rules.next().unwrap(),
                        &local_defs,
                        &ext_defs,
                    )),
                );
            }
            Rule::object_def_local => {
                let mut inner_rules = pair.into_inner();
                local_defs.insert(
                    format!("/{}", inner_rules.next().unwrap()),
                    Definition::Object(Object::construct(
                        inner_rules.next().unwrap(),
                        &local_defs,
                        &ext_defs,
                    )),
                );
            }
            Rule::object_def_extern => {
                let mut inner_rules = pair.into_inner().next().unwrap().into_inner();
                exports.insert(
                    format!("/{}", inner_rules.next().unwrap()),
                    Definition::Object(Object::construct(
                        inner_rules.next().unwrap(),
                        &local_defs,
                        &ext_defs,
                    )),
                );
            }
            Rule::list_def_local => {
                let mut inner_rules = pair.into_inner();
                local_defs.insert(
                    format!("*{}", inner_rules.next().unwrap().as_str().to_string()),
                    Definition::List(
                        inner_rules
                            .map(|p| parse_string(p, &local_defs, &ext_defs))
                            .collect(),
                    ),
                );
            }
            Rule::list_def_extern => {
                let mut inner_rules = pair.into_inner().next().unwrap().into_inner();
                exports.insert(
                    format!("*{}", inner_rules.next().unwrap().as_str().to_string()),
                    Definition::List(
                        inner_rules
                            .map(|p| parse_string(p, &local_defs, &ext_defs))
                            .collect(),
                    ),
                );
            }
            Rule::macro_def_local => {
                let object_builder = MacroFn::construct(pair);
                local_defs.insert(object_builder.0, object_builder.1);
            }
            Rule::macro_def_extern => {
                let object_builder = MacroFn::construct(pair.into_inner().next().unwrap());
                exports.insert(object_builder.0, object_builder.1);
            }
            Rule::EOI => (),
            _ => remaining.push(pair),
        }
    }

    println!("defs {:#?}", local_defs);
    return (local_defs, exports, remaining);
}

pub fn get_content(
    pairs: Vec<Pair<Rule>>,
    local_defs: HashMap<String, Definition>,
    ext_defs: &HashMap<String, Definition>,
    meta_properties: &HashMap<String, HashMap<String, Vec<String>>>,
) -> Vec<Object> {
    let mut root = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::object => {
                println!("{:#?}", pair);
                root.push(Object::construct(pair, &local_defs, &ext_defs));
            }
            _ => unreachable!(),
        }
    }

    println!("{:#?}", root);
    return root;
}

fn parse_string(
    pair: Pair<Rule>,
    local_defs: &HashMap<String, Definition>,
    ext_defs: &HashMap<String, Definition>,
) -> String {
    let mut result = String::new();

    for component in pair.into_inner() {
        match component.as_rule() {
            Rule::text => result.push_str(component.as_str()),
            _ => {
                let defs = match component.as_rule() {
                    Rule::const_use_local => local_defs,
                    Rule::const_use_extern => ext_defs,
                    _ => unreachable!(),
                };

                let span = component.as_span();
                let name = component.into_inner().next().unwrap().as_str();
                if let Some(value) = defs.get(&format!("${}", name)) {
                    if let Definition::Constant(def) = value {
                        result.push_str(def.as_str());
                    }
                } else {
                    println!("{:#?}", span);
                }
            }
        }
    }
    return result;
}
