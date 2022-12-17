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

use std::str::FromStr;

#[derive(Clone)]
pub struct MacroFn {
    pub arg_count: usize,
    blueprint: String,
    delimiter: String,
    name: String,
}

impl Debug for MacroFn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}[{}]", self.name, self.arg_count)
    }
}

impl MacroFn {
    pub fn construct(pair: Pair<Rule>) -> (String, Definition) {
        let mut inner_rules = pair.into_inner();
        let name = inner_rules.next().unwrap().as_str().to_string();
        let arg_count = usize::from_str(inner_rules.next().unwrap().as_str()).unwrap();
        let mut result = String::new();

        let delimiter = if let Some(next) = inner_rules.next() {
            match next.as_rule() {
                Rule::delim => next.as_str().to_string(),
                Rule::object => {
                    result.push_str(next.as_str());
                    ",".to_string()
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        };

        result.push_str(inner_rules.as_str());

        (
            format!("@{}", name),
            Definition::Macro(Self {
                arg_count: arg_count,
                blueprint: result,
                delimiter: delimiter,
                name: name,
            }),
        )
    }

    pub fn call(&self, raw_args: String, delim_arg: String) -> String {
        let args: Vec<&str> = raw_args
            .split(&match delim_arg.is_empty() {
                true => self.delimiter.clone(),
                false => delim_arg,
            })
            .collect();

        if args.len() != self.arg_count {
            // TODO: Prettify with span
            panic!(
                "Call of macro `{}`: expected {} arguments but got {}.",
                self.name,
                self.arg_count,
                args.len()
            );
        }

        let mut result = self.blueprint.clone();

        for arg_n in 1..=self.arg_count {
            let placeholder = format!("#{}", arg_n);
            if !result.contains(&placeholder) {
                // TODO: Prettify with span
                panic!(
                    "Macro `{}`: expected place holder `{}`.",
                    self.name, placeholder
                );
            } else {
                result = result.replace(&placeholder, args[arg_n - 1].trim());
            }
        }

        return result;
    }
}
