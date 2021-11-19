use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct CLI {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()>  {
    let args = CLI::from_args();
    let file_content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{:#?}`", &args.path))?;

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
