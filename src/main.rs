#![allow(dead_code, unused_imports)]
mod unperceive; 
use unperceive::*; 

use std::fs::File; 
use std::path::Path; 
use std::time::Instant; 

use colored::*; 
use clap::{App, AppSettings, Arg, ArgGroup}; 
use color_eyre::eyre::{Report, Result}; 
use tracing::instrument; 
use paris::Logger; 



#[instrument]
fn main() ->Result<(), Report>{
    color_eyre::install()?; 
    let mut log = Logger::new(); 
    let cli = App::new("[̲̅u][̲̅n][̲̅p][̲̅e][̲̅r][̲̅c][̲̅e][̲̅i][̲̅v][̲̅e]") 
        .setting(AppSettings::ColorAlways)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("\n0.1.0")
        .author("Xavier F. <https://github.com/Xavulu/nekew>")
        .about("because being perceived should be optional")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("input photo for processing")
            .takes_value(true)
            .value_name("INPUTFILE")
            .required(true)
            ) 
        .arg(Arg::with_name("dark")
            .short("d")
            .long("dark")
            .help("use the dark face cover")
            .takes_value(false))
        .arg(Arg::with_name("light")
            .short("l")
            .long("light")
            .help("use the light face cover")
            .takes_value(false)
        )
        .arg(Arg::with_name("custom")
            .short("c")
            .long("custom")
            .help("provide a custom face cover, should be transparent png")
            .takes_value(true))
        .group(ArgGroup::with_name("filters")
            .args(&["light", "dark", "custom"])
            .required(true)
            ) 
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("output directory for modified image")
            .takes_value(true)
            .value_name("OUTPUT")
            .required(true)
        ).get_matches(); 

        let input = cli.value_of("input").unwrap(); 
        let output = cli.value_of("output").unwrap(); 
        





    Ok(())
}
