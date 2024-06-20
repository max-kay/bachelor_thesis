use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn print_help() {
    println!("Usage: find-pairs [OPTIONS] <input> <output>");
    println!();
    println!("This tool calculates the pair multiplicities for the specified file.");
    println!("If no output path is given the result is printed to stdout");
    println!();
    println!("Options:");
    println!("  -h, --help       Print this help message");
    println!("  -e, --example    Print an example input file");
}

fn make_output<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let (group, positions, bounds, construct_ab_pairs) = crystallography::objects::from_file(path)?;
    let expansions =
        crystallography::objects::calculate_pairs(group, positions, bounds, construct_ab_pairs);
    Ok(crystallography::objects::produce_output_string(&expansions))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
            }
            "-e" | "--example" => {
                println!(include_str!("../../files/input/commented_example"))
            }
            path => {
                println!("{}", make_output(path)?);
            }
        }
    } else if args.len() == 3 {
        let string = make_output(&args[1])?;
        let output = &args[2];
        let mut file = File::create(output)?;
        write!(file, "{}", string)?;
        file.flush()?;
    } else {
        print_help();
    }
    Ok(())
}
