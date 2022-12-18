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

use super::*;

use crate::{pest::Parser, MemlParser};

#[derive(Clone, Debug)]
pub struct ObjectBuilder<'a> {
    pair: Pair<'a, Rule>,
}

impl ObjectBuilder<'_> {
    pub fn build(
        &self,
        local_defs: &HashMap<String, Definition>,
        ext_defs: &HashMap<String, Definition>,
    ) -> Object {
        Object::construct(self.pair.clone(), local_defs, ext_defs)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub name: String,
    pub properties: HashMap<String, String>,
    pub children: Vec<Self>,
    pub content: String,
}

impl Object {
    pub fn builder(pair: Pair<Rule>) -> ObjectBuilder {
        ObjectBuilder { pair: pair }
    }

    pub fn construct(
        pair: Pair<Rule>,
        local_defs: &HashMap<String, Definition>,
        ext_defs: &HashMap<String, Definition>,
    ) -> Object {
        let mut inner_rules = pair.into_inner();
        let mut object = Self::default();
        object.name = inner_rules.next().unwrap().as_str().to_string();
        object.eval_components(inner_rules, local_defs, ext_defs);
        return object;
    }

    pub fn eval_components(
        &mut self,
        components: Pairs<Rule>,
        local_defs: &HashMap<String, Definition>,
        ext_defs: &HashMap<String, Definition>,
    ) {
        for component in components {
            match component.as_rule() {
                Rule::property => {
                    let mut inner_rules = component.into_inner();
                    self.properties.insert(
                        inner_rules.next().unwrap().as_str().to_string(),
                        parse_string(inner_rules.next().unwrap(), local_defs, ext_defs),
                    );
                }
                Rule::object => self
                    .children
                    .push(Self::construct(component, local_defs, ext_defs)),
                Rule::content => {
                    if !self.content.is_empty() {
                        self.content.push_str("\n");
                    };
                    self.content.push_str(&parse_string(
                        component.into_inner().next().unwrap(),
                        local_defs,
                        ext_defs,
                    ));
                }
                Rule::object_use_local => {
                    let name = component.into_inner().next().unwrap().as_str();
                    if let Some(Definition::Object(object)) = local_defs.get(&format!("!{}", name))
                    {
                        self.children.push(object.build(local_defs, ext_defs));
                    }
                }
                Rule::macro_expand_local => {
                    let mut inner_rules = component.into_inner();
                    let name = inner_rules.next().unwrap().as_str();
                    let (args, delim) = {
                        let next = inner_rules.next().unwrap();
                        match next.as_rule() {
                            Rule::string => (next.as_str(), ""),
                            Rule::delim => (inner_rules.next().unwrap().as_str(), next.as_str()),
                            _ => unimplemented!(),
                        }
                    };
                    if let Some(Definition::Macro(macro_fn)) = local_defs.get(&format!("@{}", name))
                    {
                        let raw_object = macro_fn.call(args.to_string(), delim.to_string());
                        let mut object = Self::construct(
                            MemlParser::parse(Rule::indented_object, &raw_object)
                                .unwrap()
                                .next()
                                .unwrap(),
                            local_defs,
                            ext_defs,
                        );
                        object.eval_components(inner_rules, local_defs, ext_defs);
                        self.children.push(object);
                    }
                }
                _ => unreachable!(
                    "Rule `{:?}` with content `{}`.",
                    component.as_rule(),
                    component.as_str(),
                ),
            }
        }
    }

    pub fn as_xml(&self) -> String {
        let options = self.properties.iter().fold(String::new(), |init, item| {
            format!("{} {}=\"{}\"", init, item.0, item.1)
        });

        let content = if !self.children.is_empty() {
            self.children
                .iter()
                .map(|o| o.as_xml())
                .collect::<Vec<String>>()
                .join("")
        } else {
            self.content.clone()
        };

        if content.is_empty() {
            format!("<{}{} />", self.name, options)
        } else {
            format!("<{name}{}>{}</{name}>", options, content, name = self.name)
        }
    }
}
