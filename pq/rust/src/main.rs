// Core
use std::path::{PathBuf, Path};
use std::fs::File;

// External Crates
use polars::prelude::*;
use polars::sql::SQLContext;
use clap::Parser;

#[allow(dead_code, unused_imports)]
#[derive(Parser, Debug)]
#[command(name="pq")]
#[command(version="1.0.0")]
#[command(about="pq", long_about=None)]
struct Cli {
    #[arg(short, long, help = "first")]
    first: Option<String>,

    #[arg(short, long, help = "last")]
    last: Option<String>,

    #[arg(short, long, help = "info")]
    info: bool,

    #[arg(short, long, help = "query")]
    query: Option<String>,

    file: PathBuf,
}

fn read_parquet_file(file: &Path) -> DataFrame {
    let df = ParquetReader::new(File::open(file).unwrap());
    df.finish().unwrap()
}

fn _head(df_path: &Path, n: Option<&str>) {
    let count = n.unwrap().parse::<usize>().unwrap();
    let df = read_parquet_file(df_path);
    
    println!("{:?}", df.head(Some(count)));
}

fn _tail(df_path: &Path, n: Option<&str>) {
    let count = n.unwrap().parse::<usize>().unwrap();
    let df = read_parquet_file(df_path);
    
    println!("{:?}", df.tail(Some(count)));
}

fn _info(df_path: &Path) {
    let df = read_parquet_file(df_path);

    println!("schema : {:?}", df.schema());
    println!("columns: {:?}", df.shape());
}

fn _query(df_path: &Path, query: Option<&str>) {
    let df = read_parquet_file(df_path);
    
    let mut ctx = SQLContext::new();
    ctx.register("tbl", df.lazy());

    let sql_query = format!("SELECT * FROM tbl WHERE {};", query.unwrap());
    println!("query: {sql_query}");

    let result = ctx.execute(&sql_query).unwrap().collect().unwrap();
    println!("result: {:?}", result);
}

fn parse_args() -> Cli {
    let args = Cli::parse();

    let first = args.first.as_deref();
    let last = args.last.as_deref();
    let info = args.info;
    let query = args.query.as_deref();

    // check one of the above is defined
    let filename = args.file.as_path();

    if first.is_some() {
        _head(filename, first);
    } else if last.is_some() {
        _tail(filename, last);
    } else if info {
        _info(filename);
    } else if query.is_some() {
        _query(filename, query);
    } else {
        panic!("Please specify either --first, --last, --info, or --query");
    }

    if !filename.is_file() {
        println!("{:?} must exist!", filename);
        std::process::exit(1);
    }

    args
}


fn main() {
    parse_args();
}
