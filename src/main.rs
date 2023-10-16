use clap::Parser;

/// Struct for reverb option and input.
#[derive(Parser)]
struct Arguments {

    /// Whether to print the input on one line or multiple.
    #[arg(short = 's', long = "separate", action = clap::ArgAction::Count)]
    separate: u8,

    /// Input to be printed.
    input: Vec<String>
}

fn main() {
    // Parse the given arguments.
    let arguments = Arguments::parse();

    // Check if `separate` option is used.
    // 0 = do not separate.
    // 0 < = separate.
    if arguments.separate == 1 {
        print_input_separate(arguments.input);
    } else {
        print_input_joined(arguments.input);
    }
}

fn print_input_joined(input: Vec<String>) {
    // Print input joined with spaces.
    println!("{}", input.join(" "));
}

fn print_input_separate(input: Vec<String>) {
    // Print input separately one at a time.
    for string in input.iter() {
        println!("{}", string);
    };
}