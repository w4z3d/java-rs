use clap::Clap;

use java_rs_base::error::Error;

mod helpers;
mod commands;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "S7 Studio")]
struct Opts {
    input: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1.0")]
    Dump(Dump),
    #[clap(version = "0.1.0")]
    Replicate(Output),
    #[clap(version = "0.1.0")]
    Obfuscate(Obfuscation),
    #[clap(version = "0.1.0")]
    Deobfuscate(Obfuscation),
}

#[derive(Clap)]
struct Dump {
    #[clap(long)]
    pub ugly_print: bool,
    #[clap(short)]
    pub output: Option<String>,
}

#[derive(Clap)]
struct Output {
    #[clap(short)]
    pub output: String,
}

#[derive(Clap)]
struct Obfuscation {
    #[clap(short)]
    pub output: String,
    #[clap(long)]
    pub no_remap_instructions: bool,
    #[clap(long)]
    pub no_correct_strings: bool,
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let input = &opts.input;

    match opts.subcmd {
        SubCommand::Dump(Dump {
            ugly_print,
            output,
        }) => commands::dump::execute(input, ugly_print, output),
        SubCommand::Replicate(Output { output }) => commands::replicate::execute(input, output),
        SubCommand::Obfuscate(Obfuscation { output, no_remap_instructions, no_correct_strings }) => commands::obfuscate::execute(input, output, no_remap_instructions, no_correct_strings),
        SubCommand::Deobfuscate(Obfuscation { output, no_remap_instructions, no_correct_strings }) => commands::deobfuscate::execute(input, output, no_remap_instructions, no_correct_strings),
    }

    Ok(())
}
