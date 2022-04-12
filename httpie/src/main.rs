use clap::{AppSettings, Parser};
use anyhow::Result;
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version="1.0", author="forfd")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post)
}

#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn main() {
    println!("httpie starts...");
    let opts: Opts = Opts::parse();
    println!("options: {:?}", opts);
}
