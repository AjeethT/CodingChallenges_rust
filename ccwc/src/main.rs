use clap::{command, Parser};

/// ccwc tool for counting words, lines, and characters
#[derive(Parser, Debug)]
#[command(name="ccwc")]
#[command( about="ccwc tool for counting words, lines, and characters", long_about = None)]
struct Args {
    /// input file to count words, lines, or characters from
    #[arg(short='p', long, help = "input file path"  )]
    path: String,

    /// option to specify the desired operation: 'c' for character count, 'w' for word count, or 'l' for line count
    #[arg(short='o', long, help = "operation to perform: 'c' for character count, 'w' for word count, or 'l' for line count"  )]
    option: char,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(args.path).expect("Failed to read file");

    if args.option == 'c' {
        println!("{}", content.len());
    }
    else if args.option == 'w' {
        println!("{}", content.split_whitespace().count());
    }
    else if args.option == 'l' {
        println!("{}", content.lines().count());
    }
    else {
        println!("Invalid option. Use -c for character count, -w for word count, or -l for line count.");
    }
        

}