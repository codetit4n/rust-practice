use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Speed {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    Slow,
}
/// A command line tool to do things
#[derive(Parser, Debug)]
#[command(author , long_about = None)]
#[command(version = "0.0.1")]
#[command(author = "codeTIT4N")]
#[command(after_help = "Goodbye!")]

struct Cli {
    #[arg(long, default_value = "8080", value_parser = port_in_range)]
    port: u16,

    #[arg(long, value_enum)]
    speed: Speed,
}

const PORT_RANGE: std::ops::RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;

    if port == 80 || port == 443 {
        return Err(format!("port {} is reserved", port));
    }

    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

#[test]
fn reserve_port_80_and_443() {
    let cli = Cli::try_parse_from(&["cli-app", "slow", "--port", "80"]);
    if cli.is_ok() {
        panic!("port 80 should be reserved");
    }
}

#[test]
fn speed() {
    let cli = Cli::try_parse_from(&["cli-app", "slow", "--speed", "fast"]);
    if cli.is_ok() {
        panic!("speed fast should be out of range");
    }
}
