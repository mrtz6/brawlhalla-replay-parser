use clap::Parser;
use std::fs::File;
use std::io::Write;
use brparser::ReplayParser;

/// A simple-to-use CLI to decompress, decipher and parse Brawlhalla .replay files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the replay file
    #[arg(short, long)]
    replay: String,

    /// Path to the generated output file
    #[arg(short, long, default_value = "output.json")]
    output: String,
}

fn main() {
    let args = Args::parse();
    
    let replay = match ReplayParser::parse_from_file(&args.replay) {
        Ok(replay) => replay,
        Err(err) => {
            eprintln!("Failed to parse replay file: {}", err);
            return;
        }
    };
    
    let serialized_replay = match serde_json::to_string_pretty(&replay) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to serialize replay file: {}", err);
            return;
        }
    };
    
    let mut output_file = match File::create(&args.output) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create output file: {}", err);
            return;
        }
    };

    match output_file.write_all(serialized_replay.as_bytes()) {
        Ok(_) => println!("Replay written to {}", args.output),
        Err(err) => eprintln!("Failed to write to output file: {}", err),
    }
}
