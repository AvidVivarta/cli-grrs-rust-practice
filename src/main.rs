use std::{fmt, fs::File };
use std::io::{Write,BufRead, BufReader, BufWriter, stdout};
use clap::Parser;
use anyhow::{Result, Context};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long)] // can add code inside args to set default value ->, default_value_t=String::from("")
    pattern: String, 
    #[arg(short='P', long)]
    path: std::path::PathBuf
}

impl fmt::Display for CliArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pattern: {}\nPath: {}", self.pattern, self.path.to_str().unwrap_or("")) 
    }
}


fn main() ->Result<()>{
    let cli_args = CliArgs::parse();
    println!("{}", cli_args); 
    let file: File = File::open(&cli_args.path).with_context(|| format!("Could not read file `{:?}`", &cli_args.path))?;
    let reader: BufReader<File> = BufReader::new(file);
    let stdout = stdout().lock();
    let mut writer: BufWriter<_> = BufWriter::new(stdout);

    for line in reader.lines() {
        let line = line.context("failed to read line")?;
        if line.contains(&cli_args.pattern) { 
            writeln!(writer, "{}", &line).context("unable to write the line")?;
        }
    }

    Ok(())
}
