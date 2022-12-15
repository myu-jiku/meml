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

#[derive(Debug, Default)]
pub struct Object {
    pub name: String,
    pub properties: HashMap<String, String>,
    pub children: Vec<Self>,
    pub content: String,
}

impl Object {
    pub fn construct(
        pair: Pair<Rule>,
        local_defs: &HashMap<String, Definition>,
        ext_defs: &HashMap<String, Definition>,
    ) -> Object {
        let mut inner_rules = pair.into_inner();
        let mut object = Self::default();
        object.name = inner_rules.next().unwrap().as_str().to_string();
        for component in inner_rules {
            match component.as_rule() {
                Rule::property => {
                    let mut inner_rules = component.into_inner();
                    object.properties.insert(
                        inner_rules.next().unwrap().as_str().to_string(),
                        parse_string(inner_rules.next().unwrap(), local_defs, ext_defs),
                    );
                }
                Rule::object => object
                    .children
                    .push(Self::construct(component, local_defs, ext_defs)),
                Rule::content => {
                    if !object.content.is_empty() {
                        object.content.push_str("\n");
                    };
                    object.content.push_str(&parse_string(
                        component.into_inner().next().unwrap(),
                        local_defs,
                        ext_defs,
                    ));
                }
                _ => unreachable!(
                    "Rule `{:?}` with content `{}`.",
                    component.as_rule(),
                    component.as_str()
                ),
            }
        }
        return object;
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
