use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short = 'n', help = "omit printing new line")]
    new_line: bool,

    #[arg(help = "Text to echo", num_args = 1.., required = true, value_name="TEXT")]
    text: Vec<String>,
}

fn main() {
    let cli = Args::parse();
    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.new_line { "" } else { "\n" }
    );
}
