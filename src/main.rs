extern crate docx;

use docx::document::Paragraph;
use docx::DocxFile;

fn main() {
    let input_file: DocxFile = DocxFile::from_file("input.docx").unwrap();
    let input_data = input_file.parse().unwrap();
    println!("{}", input_data);
}
