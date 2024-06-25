
mod defs;

use markdown::{mdast::{Node, Text}, to_mdast, ParseOptions};
use std::fs;

fn main() {
    let file_content: &str = &fs::read_to_string("README.md").unwrap();
    let root = to_mdast(file_content, &ParseOptions::default()).unwrap();
    let tree = root.children().unwrap();
    for i in tree {
        if let Node::Paragraph(target) = i {
            println!("{:?}", target.children);
        }
    }

    //println!("{:?}", tree);
}
