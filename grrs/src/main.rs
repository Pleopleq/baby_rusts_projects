use structopt::StructOpt;

#[derive(Debug)]
#[derive(StructOpt)]
struct CLI {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::from_args();
    println!("{:?}", args.pattern);
    println!("{:?}", args.path);
}
