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

#[derive(Clone, Debug)]
pub struct ElementFactory<'a> {
    pub pair: Pair<'a, Rule>,
}

impl ElementFactory<'_> {
    pub fn construct_element(&self, local_definitions: &DefinitionMap) -> Element {
        Element::construct(self.pair.clone(), local_definitions, None)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub namespace: String,
    pub name: String,
    pub arguments: Vec<(String, String)>,
    pub children: Vec<Self>,
    pub content: String,
}

impl Element {
    pub fn factory(pair: Pair<Rule>) -> ElementFactory {
        ElementFactory { pair: pair }
    }

    pub fn function(pair: Pair<Rule>, arg_names: Vec<String>) -> Function {
        Function {
            pair: pair,
            arg_names: arg_names,
        }
    }

    pub fn construct(
        pair: Pair<Rule>,
        local_definitions: &DefinitionMap,
        function_arguments: Option<&Arguments>,
    ) -> Self {
        let mut inner_rules = pair.into_inner();
        let mut element = Self::default();

        element.namespace = inner_rules.next().unwrap().as_str().to_string();
        element.name = inner_rules.next().unwrap().as_str().to_string();

        element.eval_contents(inner_rules, local_definitions, function_arguments);
        return element;
    }

    pub fn eval_contents(
        &mut self,
        mut pairs: Pairs<Rule>,
        local_definitions: &DefinitionMap,
        function_arguments: Option<&Arguments>,
    ) {
        for attribute in pairs.next().unwrap().into_inner() {
            let mut inner_rules = attribute.into_inner();
            self.arguments.push((
                inner_rules.next().unwrap().as_str().to_string(),
                parse_string(
                    inner_rules.next().unwrap(),
                    local_definitions,
                    function_arguments,
                ),
            ));
        }

        for child in pairs.next().unwrap().into_inner() {
            self.eval_child(child, local_definitions, function_arguments);
        }

        if let Some(string) = pairs.next().unwrap().into_inner().next() {
            self.content = parse_string(string, local_definitions, function_arguments);
        }
    }

    pub fn eval_child(
        &mut self,
        child: Pair<Rule>,
        local_definitions: &DefinitionMap,
        function_arguments: Option<&Arguments>,
    ) {
        match child.as_rule() {
            Rule::element => self.children.push(Element::construct(
                child,
                local_definitions,
                function_arguments,
            )),
            Rule::const_use => {
                if let Some(value) = local_definitions
                    .get("elements")
                    .unwrap()
                    .get(child.as_str())
                {
                    if let Definition::Element(def) = value {
                        self.children.push(def.construct_element(local_definitions));
                    }
                } else {
                    panic!(
                        "{}",
                        Error::new_from_span(
                            ErrorVariant::<()>::CustomError {
                                message: "undefined element constant".to_string()
                            },
                            child.as_span(),
                        )
                    );
                }
            }
            Rule::func_use => {
                let mut inner_rules = child.into_inner();
                let name = inner_rules.next().unwrap().as_str();
                let args = inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|pair| parse_string(pair, local_definitions, None))
                    .collect();
                if let Some(value) = local_definitions.get("functions").unwrap().get(name) {
                    if let Definition::Function(def) = value {
                        self.children.push(def.call(args, &local_definitions))
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn as_xml(&self) -> String {
        let name = if self.namespace.is_empty() {
            self.name.to_string()
        } else {
            format!("{}:{}", self.namespace, self.name)
        };

        let arguments = self
            .arguments
            .iter()
            .map(|item| format!(" {}=\"{}\"", item.0, item.1))
            .collect::<Vec<String>>()
            .join("");

        let contents = if self.children.is_empty() {
            self.content.to_string()
        } else {
            self.children
                .iter()
                .map(|child| child.as_xml())
                .collect::<Vec<String>>()
                .join("")
        };

        if contents.is_empty() {
            format!("<{}{}/>", name, arguments)
        } else {
            format!("<{name}{}>{}</{name}>", arguments, contents, name = name)
        }
    }
}
