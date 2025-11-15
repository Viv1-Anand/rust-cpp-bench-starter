use rand::{Rng, distributions::Alphanumeric};
use std::{env, fs::{OpenOptions, create_dir_all}, io::{BufWriter, Write}};
use time::OffsetDateTime;

mod json_record;

use json_record::{LogLevel, JsonRecord};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: gen <num_lines> <output_path>");
        std::process::exit(1);
    }
    let num: usize = args[1].parse().expect("num_lines must be a number");
    let full_path = &args[2];

    // Create output directory if it doesn't exist
    let dir_path = std::path::Path::new(full_path).parent().expect("invalid output path");
    create_dir_all(dir_path)?;

    // Open output file for writing and truncate if it exists
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(full_path)?;

    // Create a buffered writer and random number generator
    let mut w = BufWriter::new(f);
    let mut rng = rand::thread_rng();

    // Generate log records
    for i in 0..num {
        let ts = OffsetDateTime::now_utc().unix_timestamp();
        let msg: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
        let rec = JsonRecord {
            id: i,
            level: if i % 10 == 0 { LogLevel::WARN } else { LogLevel::INFO },
            msg,
            ts: ts as usize,
        };
        serde_json::to_writer(&mut w, &rec).unwrap();
        w.write_all(b"\n")?;
    }
    w.flush()?;
    Ok(())
}
