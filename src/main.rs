use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let parsed_str: Vec<String> = parse_markdown_file(&args[1]);
            write_markdown_file(&args[1], parsed_str);
            println!("[ INFO ] Parsing complete!");
        }
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }
}

fn usage() {
    print_long_banner();
}

fn parse_markdown_file(_filename: &str) -> Vec<String> {
    print_short_banner();
    println!("[ INFO ] Starting parser!");
    let input_filename = Path::new(_filename);
    let file = match File::open(&input_filename) {
        Err(err) => panic!("Couldn't open file: {}", err),
        Ok(value) => value,
    };

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut output_line = String::new();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                let mut next_str = String::from("\n\n<h1>");
                next_str.push_str(&line_contents[2..]);
                output_line.push_str(&next_str);
            }
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    tokens
}

fn write_markdown_file(_filename: &str, _parsed_str: Vec<String>) {
    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");
    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");
    for line in &_parsed_str {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str("(v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}
