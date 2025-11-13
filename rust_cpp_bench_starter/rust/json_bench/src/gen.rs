use rand::{Rng, distributions::Alphanumeric};
use std::{env, fs::File, io::{BufWriter, Write}};
use time::OffsetDateTime;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: gen <num_lines> <output_path>");
        std::process::exit(1);
    }
    let num: usize = args[1].parse().expect("num_lines must be a number");
    let path = &args[2];
    let f = File::create(path)?;
    let mut w = BufWriter::new(f);
    let mut rng = rand::thread_rng();

    for i in 0..num {
        let ts = OffsetDateTime::now_utc().unix_timestamp();
        let msg: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
        let rec = serde_json::json!({
            "ts": ts,
            "level": if i % 10 == 0 { "warn" } else { "info" },
            "id": i,
            "msg": msg
        });
        serde_json::to_writer(&mut w, &rec).unwrap();
        w.write_all(b"\n")?;
    }
    w.flush()?;
    Ok(())
}
