use docx_rs::documents::Paragraph;
use docx_rs::DocxFile;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    file_path: String,
}

fn main() {
    let args = Args::from_args();
}
