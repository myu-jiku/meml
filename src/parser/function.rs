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
pub struct Function<'a> {
    pub pair: Pair<'a, Rule>,
    pub arg_names: Vec<String>,
}

impl Function<'_> {
    pub fn call(&self, arguments: Vec<String>, local_definitions: &DefinitionMap) -> Element {
        let args_len = arguments.len();
        let names_len = self.arg_names.len();

        let args = if args_len < names_len {
            panic!()
        } else if args_len > names_len {
            panic!()
        } else {
            self.arg_names.clone().into_iter().zip(arguments).collect()
        };

        Element::construct(self.pair.clone(), local_definitions, Some(&args))
    }
}
