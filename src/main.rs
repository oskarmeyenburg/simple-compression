use std::env;
use std::fs;

fn read(file: &str) -> Vec<u8> {
    fs::read(file).expect("Failed to read the file")
}

#[derive(Debug)] // allows printing
struct Options {
    valid: bool,
    input_path: Option<String>,
    output_path: Option<String>,
    compress: i32,
}

enum ArgType {
    Help,
    Value,
    UnknownValue,
    InputFilePath,
    OutputFilePath,
    ExecutablePath,
    Compress,
}

fn print_help() {
    println!(
        "\n\
        Usage:\n  \
          cmpr <path> [options]\n\n\
        Options:\n  \
          -h, --help        Show help\n  \
          -o, --out [path]  Path to output file\n  \
          -c, --compress    Compress the selected file\n  \
          -d, --decompress  Decompress the selected file
        "
    );
}

fn match_args(options: &mut Options, argument: &str) -> ArgType {
    match argument {
        "--help" | "-h" => {
            options.valid = false;
            print_help();
            ArgType::Help
        }
        "--out" | "-o" => ArgType::OutputFilePath,
        "--compress" | "-c" => {
            options.compress += 1;
            ArgType::Compress
        }
        "--decompress" | "-d" => {
            options.compress += 2;
            ArgType::Compress
        }
        _ => {
            if argument.starts_with('-') {
                options.valid = false;
                println!("Unknown option: {argument}. Try using the `--help` option for more information.");
                ArgType::UnknownValue
            } else {
                ArgType::Value
            }
        }
    }
}

fn parse_args() -> Options {
    let args: Vec<String> = env::args().collect();

    let mut options = Options {
        valid: true,
        input_path: Option::None,
        output_path: Option::None,
        compress: -1,
    };

    let mut expected_argument = Some(ArgType::ExecutablePath);

    for arg in &args {
        // work in progress; returned values are not handled yet an not important
        let arg_type = match_args(&mut options, &arg);
        match arg_type {
            ArgType::Help | ArgType::UnknownValue => {
                return options;
            }
            ArgType::OutputFilePath => {
                if matches!(expected_argument, Some(ArgType::OutputFilePath)) {
                    println!("Expected value for `--out` option, found {arg} instead. Try using the `--help` option for more information.");
                    return options;
                }
                if options.output_path.is_some() {
                    println!("Expected {arg} option only once. Try using the `--help` option for more information.");
                    return options;
                }
                expected_argument = Some(ArgType::OutputFilePath);
            }
            ArgType::Value => {
                if let Some(ref field) = expected_argument {
                    match field {
                        ArgType::ExecutablePath => {
                            expected_argument = Some(ArgType::InputFilePath);
                        }
                        ArgType::InputFilePath => {
                            options.input_path = Some(arg.clone());
                            expected_argument = None;
                        }
                        ArgType::OutputFilePath => {
                            options.output_path = Some(arg.clone());
                            if options.input_path.is_none() {
                                expected_argument = Some(ArgType::InputFilePath);
                            } else {
                                expected_argument = None;
                            }
                        }
                        _ => {}
                    }
                } else {
                    println!("Unexpected positional argument: {arg}. Try using the `--help` option for more information.")
                }
            }
            _ => {
                if matches!(expected_argument, Some(ArgType::OutputFilePath)) {
                    println!("Expected value for `--out` option, found {arg} instead. Try using the `--help` option for more information.");
                    return options;
                }
            }
        }
    }

    if options.valid {
        if expected_argument.is_some() {
            options.valid = false;
            if options.input_path.is_none() {
                println!("Expected path to input file as a positional argument. Try using the `--help` option for more information.")
            } else if options.output_path.is_none() {
                println!("Expected value following `--out` option. Try using the `--help` option for more information.")
            }
        } else if options.compress == -1 {
            println!("Expected `--compress` or `--decompress` option.");
            options.valid = false;
        } else if options.compress == 2 {
            println!("Expected either `--compress` or `--decompress` option, not both.");
            options.valid = false;
        }
    }

    return options;
}

fn main() {
    let options = parse_args();
    if !options.valid {
        return;
    }

    println!("{:#?}", options);

    let file = "data.txt";
    let data = read(file);
    // println!("{:#?}", data)
}
