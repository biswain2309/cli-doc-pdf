extern crate docx_rs as docx;
use docx::read_docx;
use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    file_path: String,
}

fn main() {
    let args = Args::from_args();
    let file = match File::open(args.file_path) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    let buf_reader = BufReader::new(file);
    let mut file_contents = Vec::new();
    buf_reader.read_to_end(&mut file_contents)?;
    for paragraph in docx.paragraphs() {
        println!("{}", paragraph.text())
    }
}
