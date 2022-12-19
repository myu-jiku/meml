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

pub fn parse_manifest(manifest_path: &str) {}

pub fn test() {
    let input = r#"
{ LICENSE

YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

LICENSE }


def property(name content): property {
    name: "${name}"
    "${content}"
}

def class_name: "YGOWindow"
def parent_class: "AdwApplicationWindow"

interface { template {
    class: "$(class_name)"
    parent: "$(parent_class)"
    property("width-request" "640")
    property("height-request" "480")
    property {
        name: "content"
        toast_overlay
    }
}}

def toast_overlay: object {
    class: "AdwToastOverlay"
    id: "toast_overlay"

    child { object {
        class: "AdwLeaflet"
        id: "leaflet"
        property("can-navigate-back" "true")
        property("can-unfold" "false")
        property("transition-type" "slide")
        child { object {
            class: "AdwLeafletPage"
            property {
                name: "child"
                main_box
            }
        }}
    }}
}

def main_box: object {
    class: "GtkBox"
    property("orientation" "vertical")
    property("vexpand" "true")
    property("hexpand" "true")
    header_bar
    child { object {
        class: "GtkScrolledWindow"
        property("min-content-height" "200")
        property("hscrollbar-policy" "never")
        property("vexpand" "true")
        child { object {
            class: "AdwClamp"
            property("maximum-size" "800")
            property("orientation" "horizontal")
            collection_list
        }}
    }}
}

def header_bar: child { object {
    class: "AdwHeaderBar"
    child { object {
        class: "GtkButton"
        property("icon-name" "open-menu-symbolic")
    }}
    property {
        name: "title-widget"
        object {
            class: "AdwWindowTitle"
            property("title" "YGO Destiny")
        }
    }
}}

def collection_list: child { object {
    class: "YGOCollectionList"
    id: "collection_list"
    property("orientation" "vertical")
    property("vexpand" "true")
    property("hexpand" "true")
    property("valign" "center")
}}
    "#;

    let pairs = parser::parse_raw(input);
    let (local_definitions, unparsed) = parser::get_definitions(pairs);
    parser::get_content(unparsed, local_definitions);
}

#[cfg(test)]
mod tests;
