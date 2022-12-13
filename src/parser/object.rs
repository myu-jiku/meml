/*
meml – XML replacement written in Rust with the pest library <https://pest.rs>.
Developed to be used in ygo_destiny <https://github.com/myu-jiku/ygo_destiny/>.
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
    pub properties: HashMap<String, String>,
    pub children: Vec<Self>,
    pub content: String,
}

impl Object {
    pub fn construct(
        pair: Pair<Rule>,
        local_defs: &HashMap<String, Definition>,
        ext_defs: &HashMap<String, Definition>
    ) -> Object {
        let mut object = Self::default();
        for component in pair.into_inner() {
            match component.as_rule() {
                Rule::property => {
                    let mut inner_rules = component.into_inner();
                    object.properties.insert(
                        inner_rules.next().unwrap().as_str().to_string(),
                        parse_string(inner_rules.next().unwrap(), local_defs, ext_defs),
                    );
                },
                Rule::object => object.children.push(Self::construct(component, local_defs, ext_defs)),
                Rule::content => object.content.push_str(&parse_string(
                    component.into_inner().next().unwrap(),
                    local_defs,
                    ext_defs,
                )),
                _ => unreachable!(),
            }
        }
        return object;
    }
}
