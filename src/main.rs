use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

/// Print input to stdout.
#[derive(Parser)]
struct Arguments {

    /// Copy the stdout print to your clipboard. Cannot be used with separate.
    /// Copy will take priority over separate.
    #[arg(short = 'c', long = "copy", action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    copy: bool,

    /// Whether to print the input on one line or multiple.
    /// Does not take effect if used with copy option.
    #[arg(short = 's', long = "separate", action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    separate: bool,

    /// Input to be printed.
    input: Vec<String>
}

fn main() {
    // Parse arguments.
    let arguments = Arguments::parse();

    if arguments.copy {
        // Set up ClipboardContext.
        let mut clipboard_context = ClipboardContext::new().unwrap();
        // Join input to one string.
        let output = arguments.input.join(" ");

        // Set joined input to clipboard.
        clipboard_context.set_contents(output.to_owned()).unwrap();
        // Print result.
        println!("{}", output)
    }

    if arguments.separate {
        // Print input separately one at a time.
        for string in arguments.input.iter() {
            println!("{}", string);
        }

        return
    }

    println!("{}", arguments.input.join(" "));
}