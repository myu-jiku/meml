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

use std::collections::HashMap;

use pest::iterators::{
    Pair,
    Pairs,
};

use crate::Rule;

pub struct Object {
    pub properties: HashMap<String, String>,
    pub children: HashMap<String, Self>,
    pub content: String,
}

pub enum Definition {
    Constant(String),
    Object(Object),
    Macro,
}

pub fn get_definitions(meml: Pairs<Rule>) -> (
    HashMap<String, Definition>,
    HashMap<String, Definition>,
    Vec<Pair<Rule>>,
) {
    let mut local_defs = HashMap::new();
    let mut ext_defs = HashMap::new();
    let mut remaining = Vec::new();

    for pair in meml {
        Lmatch pair.as_rule() {
            Rule::const_def_local => (),
            Rule::const_def_extern => (),
            Rule::object_def_local => (),
            Rule::object_def_extern => (),
            Rule::list_def_local => (),
            Rule::list_def_extern => (),
            Rule::macro_def_local => (),
            Rule::macro_def_extern => (),
            Rule::EOI => (),
            _ => remaining.push(pair),
        }
    }

    println!("{:#?}", remaining);
    return (local_defs, ext_defs, remaining);
}

