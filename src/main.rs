use std::io::Write;

use latex::Document;
use postamble::write_postamble;
use preamble::write_preamble;

mod postamble;
mod preamble;

fn main() {
    write_preamble();
    hello_world();
    write_postamble();
}

fn hello_world() -> () {
    let mut doc = Document::new(latex::DocumentClass::Part);
    let mut section = latex::Section::new("Hello World");
    section.push("Hello World");
    doc.push(latex::Element::Section(section));

    let rendered = latex::print(&doc).unwrap();

    let mut file = std::fs::File::create("./src/hello_world.tex").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
