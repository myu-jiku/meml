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

use std::{collections::HashMap, fs, path::Path};

use pest::Parser;

#[derive(Parser)]
#[grammar = "meml.pest"]
pub struct MemlParser {}

pub fn parse_manifest(manifest_path: &str) {
    if !Path::new(manifest_path).is_file() {
        panic!("Manifest `{}` is not a file.", manifest_path);
    }

    let raw_content = fs::read_to_string(manifest_path).expect("Could not read manifest.");

    let parser_result = MemlParser::parse(Rule::meml, &raw_content);
    if parser_result.is_err() {
        panic!("{}", parser_result.unwrap());
    }

    let temp_maps = (HashMap::new(), HashMap::new());
    let (local_defs, ext_defs, unparsed) =
        parser::get_definitions(parser_result.unwrap(), &temp_maps.0, &temp_maps.1);

    let mut meta_properties: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let root_path = Path::new(manifest_path).parent().unwrap();
    for section in parser::get_content(unparsed, local_defs, &temp_maps.0, &temp_maps.1) {
        println!("{}", section.as_xml());

        let mut action = "".to_string();
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut extension = "".to_string();
        let mut target = "".to_string();
        for (name, value) in section.properties.iter() {
            match name.as_str() {
                "action" => action = value.to_string(),
                "directory" => directories.push(value),
                "file" => files.push(value),
                "override_ext" => extension = value.to_string(),
                "target" => target = value.to_string(),
                _ => println!("{}", name),
            }
        }

        if action.is_empty() {
        } else if target.is_empty() {
        } else {
            if !(directories.is_empty() && files.is_empty()) {
                if let Some(x) = meta_properties.insert(section.name.to_string(), HashMap::new()) {
                    panic!();
                }

                let mut file_paths = Vec::new();
                for directory in directories {
                    let path = root_path.join(directory);
                    if path.is_dir() {
                        file_paths.append(
                            &mut fs::read_dir(path)
                                .unwrap()
                                .map(|i| i.unwrap().path())
                                .filter(|i| {
                                    let ext = i.extension();
                                    ext.is_some() && ext.unwrap() == "meml"
                                })
                                .collect(),
                        );
                    } else {
                        println!("Directory `{}` not found.", path.display());
                    }
                }

                file_paths.append(&mut files.iter().map(|i| root_path.join(i)).collect());

                println!("{:#?}", file_paths);

                fs::create_dir_all(root_path.join(&target)).expect("Could not create directory.");

                meta_properties
                    .get_mut(&section.name)
                    .unwrap()
                    .insert("BASENAME".to_string(), Vec::new());
                for path in file_paths {
                    let basename = path.file_stem().unwrap().to_str().unwrap();
                    meta_properties
                        .get_mut(&section.name)
                        .unwrap()
                        .get_mut("BASENAME")
                        .unwrap()
                        .push(basename.to_string());

                    let raw_content = fs::read_to_string(&path)
                        .expect(&format!("Could not read file `{}`.", path.display()));

                    let parser_result = MemlParser::parse(Rule::meml, &raw_content);
                    if parser_result.is_err() {
                        panic!("{}", parser_result.unwrap());
                    }

                    let (defs, exports, unparsed) = parser::get_definitions(
                        parser_result.unwrap(),
                        &ext_defs,
                        &meta_properties,
                    );
                    let target_path = root_path.join(&target).join(format!(
                        "{}.{}",
                        basename.to_string(),
                        if extension.is_empty() {
                            ".meml"
                        } else {
                            &extension
                        }
                    ));

                    let content = parser::get_content(unparsed, defs, &ext_defs, &meta_properties)
                        .iter()
                        .map(|i| i.as_xml())
                        .collect::<Vec<String>>()
                        .join("");

                    if !target_path.is_file()
                        || (content != fs::read_to_string(&target_path).unwrap())
                    {
                        fs::write(&target_path, content)
                            .expect(&format!("Could not write to `{}`.", target_path.display()));
                    }
                }

                println!("{:#?}", meta_properties);
            }
        }
    }
}

#[cfg(test)]
mod tests;
