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

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

use std::{
    fs,
    path::Path,
};

use pest::Parser;

#[derive(Parser)]
#[grammar = "meml.pest"]
pub struct MemlParser {}

pub fn parse_manifest(manifest_path: &str) {
    if !Path::new(manifest_path).is_file() {
        panic!("Manifest `{}` is not a file.", manifest_path);
    }

    let raw_content = fs::read_to_string(manifest_path)
        .expect("Could not read manifest.");

    let parser_result = MemlParser::parse(Rule::meml, &raw_content);
    if parser_result.is_err() {
        panic!("{}", parser_result.unwrap());
    }

    let (local_defs, ext_defs, unparsed) = parser::get_definitions(parser_result.unwrap());
}

#[cfg(test)]
mod tests;

