mod analyzer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Használat: cargo run -- szoveg.txt [--search szó] [--json]");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("Nem sikerült beolvasni a fájlt");

    let mut search = None;
    let mut json_output = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--search" if i + 1 < args.len() => {
                search = Some(args[i + 1].as_str());
                i += 1;
            }
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }

    analyzer::analyze(&content, search, json_output);
}