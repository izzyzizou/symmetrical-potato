use std::env;
use std::io::{self, Write};

const POTATO: [&str; 11] = [
    "          .-''''-.",
    "        .'  .--.  '.",
    "       /   /    \\   \\",
    "      ;   |  ()  |   ;",
    "      |   |      |   |",
    "      ;   |  ..  |   ;",
    "       \\   \\____/   /",
    "        '.        .'",
    "          '-.__.-'",
    "          /  ||  \\",
    "         /___||___\\",
];

fn main() -> io::Result<()> {
    let plain = env::args().any(|argument| argument == "--plain");
    let mut output = io::BufWriter::new(io::stdout().lock());

    for (index, line) in POTATO.iter().enumerate() {
        if plain {
            writeln!(output, "{line}")?;
            continue;
        }

        let color = match index {
            0..=1 | 7..=8 => "\x1b[38;5;220m",
            2..=6 => "\x1b[38;5;178m",
            _ => "\x1b[38;5;101m",
        };
        writeln!(output, "{color}{line}\x1b[0m")?;
    }

    writeln!(
        output,
        "\x1b[38;5;114m        fresh from the terminal\x1b[0m"
    )?;
    Ok(())
}
