use indoc::indoc;
use latex::*;

pub(crate) fn write_preamble() -> () {
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.use_package("fontspec");
    doc.preamble.use_package("unicode-math");
    doc.preamble.use_package("graphicx");

    let font = indoc! {"
    \\setmainfont{texgyrepagella}[
        Extension = .otf,
        UprightFont = *-regular,
        BoldFont = *-bold,
        ItalicFont = *-italic,
        BoldItalicFont = *-bolditalic,
    ]"};

    let font = PreambleElement::UserDefined(font.to_string());

    let math_font = indoc! {"
    \\setmathfont{texgyrepagella-math.otf}"
    };

    let math_font = PreambleElement::UserDefined(math_font.to_string());

    let graphics_path = indoc! {"
    \\graphicspath{ {../images/} }"
    };

    let graphics_path = PreambleElement::UserDefined(graphics_path.to_string());

    doc.preamble.push(font);
    doc.preamble.push(math_font);
    doc.preamble.push(graphics_path);

    let rendered = latex::print(&doc).unwrap();

    // remove second to last line
    let rendered = rendered.split("\n").collect::<Vec<&str>>()
        [0..rendered.split("\n").collect::<Vec<&str>>().len() - 2]
        .join("\n");

    use std::fs::File;
    use std::io::Write;

    // write rendered to "./src/report.tex" file if exists overwrite
    let mut file = File::create("./src/_preamble.tex").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
