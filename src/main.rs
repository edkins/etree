use clap::clap_app;
use log::warn;

fn main() -> anyhow::Result<()> {
    let m = clap_app!(etree =>
        (version: "0.1.0")
        (author: "Giles Edkins <edkins@gmail.com>")
        (about: "Processes expression tree language")
        (@arg verbosity: -v ... "Increases message verbosity")
        (@arg quiet: -q "Silence all messages")
    ).get_matches();

    let verbosity = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbosity)
        .init()?;

    warn!("Hello");
    Ok(())
}
