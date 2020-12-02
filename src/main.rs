use clap::clap_app;
use std::fs;

mod ast;
mod eval;
mod parse;

fn run(input: &str, output: Option<&str>) -> anyhow::Result<()> {
    let text = fs::read_to_string(input)?;
    let ast = parse::program(&text)?;
    ast.run()?;
    let textout = String::new();
    if let Some(output) = output {
        fs::write(output, textout)?;
    } else {
        print!("{}", textout);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let m = clap_app!(etree =>
        (version: "0.1.0")
        (author: "Giles Edkins <edkins@gmail.com>")
        (about: "Processes expression tree language")
        (@arg verbosity: -v ... "Increases message verbosity")
        (@arg quiet: -q "Silence all messages")
        (@arg INPUT: +required "Input file")
        (@arg OUTPUT: -o  +takes_value "Output file")
    ).get_matches();

    let verbosity = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");
    let input = m.value_of("INPUT").unwrap();
    let output = m.value_of("OUTPUT");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbosity)
        .init()?;

    run(input, output)
}
