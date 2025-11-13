use std::{env, fs::File, io::{BufRead, BufReader}};
use std::time::Instant;
use serde_json::Value;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: parse <input_path>");
        std::process::exit(1);
    }
    let path = &args[1];
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    let start = Instant::now();
    let mut lines: u64 = 0;
    while {
        buf.clear();
        reader.read_line(&mut buf)? > 0
    } {
        if buf.trim().is_empty() { continue; }
        let _v: Value = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => { eprintln!("parse error: {}", e); continue; }
        };
        lines += 1;
    }
    let dur = start.elapsed();
    let ms = dur.as_secs_f64() * 1000.0;
    println!("rust_parse lines={} duration_ms={:.2}", lines, ms);
    Ok(())
}
