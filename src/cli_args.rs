use clap::Parser;

#[derive(Parser, Default, Debug)]
struct ProgramArgs {
    // An optional value is denoted by an Option<>
    name: Option<String>,

    // A flag is a simple bool.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let args = ProgramArgs::parse();
    if let Some(name) = args.name.as_deref() {
        println!("Hello, {}!", name);
    }
    else {
        println!("Hello, world!");
    }

    if args.verbose {
        println!("Someone wanted me to be verbose, but I don't have much to say.¯\\_(ツ)_/¯");
    }
    println!("Your arguments are: {:?}", args);
}
