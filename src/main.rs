use std::{collections::HashMap, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Kanji {
    strokes: u32,
    meanings: Vec<String>,
    readings_on: Vec<String>,
    readings_kun: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <strokes>");
    } else {
        let stroke_amount: u32 = args[1].parse().unwrap();
        let file = File::open("kanji.json")?;
        let reader = BufReader::new(file);
        let kanji: HashMap<String, Kanji> = serde_json::from_reader(reader)?;
        let mut matching: bool = false;
        // Iterate over each kanji element and filter by strokes
        for (kanji_char, info) in &kanji {
            if info.strokes == stroke_amount {
                matching = true;
                println!("Kanji: {}", kanji_char);
                println!("Meanings: {:?}", info.meanings);
                println!("Readings On: {:?}", info.readings_on);
                println!("Readings Kun: {:?}", info.readings_kun);
                println!();
            }
        }
        if !matching {
            println!("No kanji found with {} strokes.", stroke_amount);
        }

    }
        Ok(())
}
