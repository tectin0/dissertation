use latex::*;

pub(crate) fn write_postamble() -> () {
    let mut doc = Document::new(DocumentClass::Part);

    let end_document = Element::UserDefined("\\end{document}".to_string());

    doc.push(end_document);

    let rendered = latex::print(&doc).unwrap();

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("./src/_postamble.tex").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
