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

pub fn parse_string(
    pair: Pair<Rule>,
    local_definitions: &DefinitionMap,
    function_arguments: Option<&Arguments>,
) -> String {
    let mut result = String::new();

    for string_component in pair.into_inner().next().unwrap().into_inner() {
        match string_component.as_rule() {
            Rule::qtext => result.push_str(string_component.as_str()),
            Rule::sconst => {
                let pair = string_component.into_inner().next().unwrap();
                let name = pair.as_str();

                if let Some(value) = local_definitions.get("strings").unwrap().get(name) {
                    if let Definition::String(def) = value {
                        result.push_str(def.as_str());
                    }
                } else {
                    panic!(
                        "{}",
                        Error::new_from_span(
                            ErrorVariant::<()>::CustomError {
                                message: "undefined constant".to_string()
                            },
                            pair.as_span(),
                        )
                    );
                }
            }
            Rule::sarg => {
                if let Some(arguments) = function_arguments {
                    let pair = string_component.into_inner().next().unwrap();
                    let name = pair.as_str();

                    if let Some(value) = arguments.get(name) {
                        result.push_str(value.as_str());
                    } else {
                        panic!(
                            "{}",
                            Error::new_from_span(
                                ErrorVariant::<()>::CustomError {
                                    message: "undefined argument".to_string()
                                },
                                pair.as_span(),
                            )
                        )
                    }
                } else {
                    panic!("{}", Error::new_from_span(
                        ErrorVariant::<()>::CustomError{
                            message: "unexpected function argument (to access a constant use parentheses instead)".to_string()
                        },
                        string_component.as_span(),
                    ))
                }
            }
            _ => unimplemented!(),
        }
    }

    return result;
}
