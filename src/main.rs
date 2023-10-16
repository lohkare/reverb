use clap::Parser;

#[derive(Parser)]
struct Arguments {

    /// Should we print given words one by one?
    #[arg(short = 's', long = "separate", action = clap::ArgAction::Count)]
    separate: u8,

    /// String to print.
    input: Vec<String>
}

fn main() {
    let arguments = Arguments::parse();
    if arguments.separate == 1 {
        print_input_separate(arguments.input);
    } else {
        print_input_joined(arguments.input);
    }
}

fn print_input_joined(input: Vec<String>) {
    println!("{}", input.join(" "));
}

fn print_input_separate(input: Vec<String>) {
    for string in input.iter() {
        println!("{}", string);
    };
}