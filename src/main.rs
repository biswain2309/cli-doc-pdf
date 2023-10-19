use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    name: String,
    count: u8,
}

fn main() {
    let args = Args::from_args();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
