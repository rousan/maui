extern crate maui;
use clap::{App, Arg};
use colored::*;
use maui::constants;

fn main() {
    let matches = App::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .version_short("v")
        .author(constants::APP_AUTHOR)
        .about(constants::APP_DESCRIPTION)
        .arg(
            Arg::with_name("filePath")
                .help("A file path to see size")
                .value_name("FILE_PATH")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Specify a format to see file sie e.g. 'kb' or 'b'")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let file_path = matches.value_of("filePath").unwrap();
    let format = matches.value_of("format").unwrap();

    match maui::file_size::human_readable_file_size(file_path.as_ref(), format) {
        Ok(size) => println!("File size: {}", size),
        Err(err) => eprintln!("{} {}", "error:".red(), err),
    }
}
