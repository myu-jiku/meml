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
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use pest::Parser;

use parser::{MemlParser, Rule};

pub fn parse_manifest(manifest_path: &str) {
    let manifest_file = Path::new(manifest_path);

    // Panic if the Path is not a file
    if !manifest_file.is_file() {
        panic!("Manifest `{}` is not a file.", manifest_path);
    }

    let raw_content = fs::read_to_string(manifest_path).expect("Could not read manifest file.");
    let manifest_rules = parser::parse_raw(&raw_content);

    let (manifest_definitions, manifest_exports, manifest_contents) =
        parser::get_definitions(manifest_rules, &HashMap::new());

    let root_dir = manifest_file.parent().unwrap();

    for section in parser::get_contents(manifest_contents, manifest_definitions) {
        let mut action = String::new();
        let mut directories = Vec::<String>::new();
        let mut files = Vec::<String>::new();
        let mut extension = String::new();
        let mut target = String::new();

        for (name, value) in section.arguments {
            match name.as_str() {
                "action" => action = value.to_string(),
                "directory" => directories.push(value),
                "file" => files.push(value),
                "change_extension" => extension = value.to_string(),
                "target" => target = value.to_string(),
                _ => panic!(
                    "Unexpected property `{}` in section `{}`. Expected one of `action`, `directory`, `file`, `change_extension` and `target`.",
                    name,
                    section.name,
                ),
            }
        }

        // Sort the files and remove duplicates
        files.sort_unstable();
        files.dedup();

        let is_action_none = action == "none";

        if action.is_empty() {
            panic!("Section `{}`: No action specified. Add `action: \"none\"` as a section property to disable this check.", section.name);
        } else if target.is_empty() && !is_action_none {
            panic!(
                "Section `{}`: No target directory specified in.",
                section.name
            );
        } else {
            if !(directories.is_empty() && files.is_empty()) {
                let mut file_paths = Vec::new();

                for directory in directories {
                    let path = root_dir.join(directory);
                    if path.is_dir() {
                        file_paths.append(
                            &mut fs::read_dir(path)
                                .unwrap()
                                .map(|item| item.unwrap().path())
                                .filter(|item| {
                                    let ext = item.extension();
                                    ext.is_some() && ext.unwrap() == "meml"
                                })
                                .collect(),
                        );
                    } else {
                        panic!(
                            "Section `{}`: Directory `{}` not found.",
                            section.name,
                            path.display()
                        );
                    }
                }

                file_paths.append(&mut files.iter().map(|item| root_dir.join(item)).collect());

                println!("{:#?}", file_paths);

                if !is_action_none {
                    fs::create_dir_all(root_dir.join(&target)).expect(&format!(
                        "Section `{}`: Could not create target directories.",
                        section.name
                    ));
                }

                for path in file_paths {
                    let basename = path.file_stem().unwrap().to_str().unwrap();

                    let raw_content = fs::read_to_string(&path).expect(&format!(
                        "Section `{}`: Could not read file `{}`.",
                        section.name,
                        path.display()
                    ));
                    let rules = parser::parse_raw(&raw_content);

                    let (definitions, exports, contents) =
                        parser::get_definitions(rules, &manifest_exports);

                    let elements = parser::get_contents(contents, definitions);

                    let target_path = if !is_action_none {
                        root_dir.join(&target).join(format!(
                            "{}.{}",
                            basename,
                            if extension.is_empty() {
                                ".meml"
                            } else {
                                &extension
                            }
                        ))
                    } else {
                        PathBuf::new()
                    };

                    match action.as_str() {
                        "xml" => {
                            let content = elements
                                .iter()
                                .map(|item| item.as_xml())
                                .collect::<Vec<String>>()
                                .join("");

                            if !target_path.is_file()
                                || (content != fs::read_to_string(&target_path).unwrap())
                            {
                                fs::write(&target_path, content).expect(&format!(
                                    "Section `{}`: Could not write to `{}`",
                                    section.name,
                                    target_path.display()
                                ));
                            }
                        }
                        "none" => (),
                        _ => panic!(
                            "Section `{}`: Invalid action `{}`. Possible values: `xml`, `none`",
                            section.name, action
                        ),
                    }

                    println!(
                        "{}",
                        elements
                            .iter()
                            .map(|i| i.as_xml())
                            .collect::<Vec<String>>()
                            .join("")
                    );
                }
            } else {
                panic!("Section `{}`: No input specified. Please add one or more of either `file` or `directory` as a property.", section.name);
            }
        }
    }
}

#[cfg(test)]
mod tests;
