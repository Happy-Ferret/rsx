/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg(any(feature = "rsx-macro", feature = "css-macro"))]
#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[cfg(feature = "css-macro")]
extern crate regex;
extern crate rsx_primitives;

#[cfg(feature = "rsx-macro")]
#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use rsx_primitives::rsx_dom::rsx_parser::parse as parse_rsx;

    let source = input.to_string();
    let (parsed, ..) = parse_rsx(&source).unwrap();

    let expanded = quote! {
        #parsed.into()
    };

    expanded.parse().unwrap()
}

#[cfg(feature = "css-macro")]
#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use regex::Regex;
    use rsx_primitives::rsx_stylesheet::servo_css_parser::parse as parse_css;
    use rsx_primitives::rsx_stylesheet::servo_css_parser::types::*;
    use rsx_primitives::rsx_stylesheet::types::Stylesheet;
    use std::env;
    use std::fs::File;
    use std::io::Read;

    let source = input.to_string();
    let len = source.len();

    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();
    let url = Url::parse("about::inline").unwrap();

    let css = if source.chars().nth(0).unwrap() == '"' {
        let file_path = env::current_dir()
            .unwrap()
            .join(&source[1..len - 1]);

        let mut result = String::new();
        File::open(file_path.clone())
            .expect(&format!(
                "Couldn't open file {}",
                file_path.to_string_lossy()
            ))
            .read_to_string(&mut result)
            .expect(&format!(
                "Couldn't read file {}",
                file_path.to_string_lossy()
            ));

        result
    } else {
        // When converting TokenStreams to Strings, a whitespace is inserted between
        // each and every token. This unfortunately means that whitespace is also
        // inserted in selectors, turning ".foo" into ". foo" which isn't valid CSS.
        // Same goes to rule names, such as "margin-left" becoming "margin - left".
        // Crudely find and fix those occurences.

        let re_selectors = Regex::new(r"(?P<type>[.#])\s(?P<name>[a-zA-Z0-9]+)").unwrap();
        let result = re_selectors.replace_all(&source, "$type$name");

        let re_rules = Regex::new(r"(?P<start>[a-zA-Z0-9]+)\s-\s(?P<end>[a-zA-Z0-9]+)\s").unwrap();
        let result = re_rules.replace_all(&result, "$start-$end");

        result.into_owned()
    };

    let parsed = parse_css(&css, url, origin, qm, media);
    let stylesheet: Stylesheet = parsed.into();

    let expanded = quote! {
        #stylesheet
    };

    expanded.parse().unwrap()
}
