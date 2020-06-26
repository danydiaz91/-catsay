extern crate colored;
use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What  does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    // Make the cat appear dead.
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message.bright_yellow().underline().on_purple());
    println!("  \\");
    println!("   \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.red().bold());
    println!("    =( I )=");
}
