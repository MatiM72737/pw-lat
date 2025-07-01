use clap::{Parser, ArgAction};

/// PipeWire latency tweak helper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Quantum value
    quantum: Option<u32>,

    /// Rate value
    rate: Option<u32>,

    /// Reset to defaults
    #[arg(short = 'd', long = "default", action = ArgAction::SetTrue)]
    reset: bool,
}

fn main() {
    let args = Args::parse();

    if args.reset {
        println!("Quantum and Rate changing to default (0)");
        apply("clock.force-quantum", 0);
        apply("clock.force-rate", 0);
    } else if let Some(q) = args.quantum {
        println!("Quantum changing to: {}", q);
        apply("clock.force-quantum", q);
        if let Some(r) = args.rate {
            println!("Rate changing to: {}", r);
            apply("clock.force-rate", r);
        }
    } else {
        println!("Usage: lat [quantum] [rate]");
        println!("       lat -d | --default  # reset quantum and rate to 0");
    }
}

fn apply(setting: &str, value: u32) {
    let _ = std::process::Command::new("pw-metadata")
        .args(["-n", "settings", "0", setting, &value.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}
