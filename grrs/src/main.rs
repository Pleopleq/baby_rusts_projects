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
    let file_content = std::fs::read_to_string(&args.path)
        .expect("Could not read file");
    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
