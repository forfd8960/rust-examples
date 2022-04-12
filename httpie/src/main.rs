use std::str::FromStr;
use clap::{AppSettings, Parser};
use anyhow::{Result, anyhow};
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
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<String>,
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("=");
        let err = || anyhow!(format!("failed to parse{}", s));
        Ok(Self {
            k: (splits.next().ok_or_else(err)?).to_string(),
            v: (splits.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
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
