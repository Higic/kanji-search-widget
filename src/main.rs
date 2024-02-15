use std::{collections::HashMap, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

// Kanji struct to describe the json data
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
        let stroke_amount: u32 = args[1].parse()?;
        search_kanji_by_stroke(stroke_amount)?;
    }
        Ok(())
}

// Used for searching kanji by stroke amount
fn search_kanji_by_stroke(stroke_amount: u32) -> Result<(), Box<dyn std::error::Error>> {
        let kanji = load_kanji_data()?;
        let mut amount = 0;
        // Iterate over each kanji element and filter by strokes
        println!("Kanji with {} strokes: ", stroke_amount);
        for (kanji_char, info) in &kanji {
            if info.strokes == stroke_amount {

                // Command line display
                display_kanji_info(kanji_char, info);
                amount += 1;
            }
        }
        if amount == 0{
            println!("No kanji found with {} strokes.", stroke_amount);
        } else {
            println!("Total kanji found: {}", amount);
        }

        Ok(())
}

// Used for displaying command line output
fn display_kanji_info(kanji_char: &str, info: &Kanji) {
    println!("Kanji: {}", kanji_char);
    println!("Meanings: {:?}", info.meanings);
    println!("Readings On: {:?}", info.readings_on);
    println!("Readings Kun: {:?}", info.readings_kun);
    println!();
}

// Get the data from the kanji.json file
fn load_kanji_data() -> Result<HashMap<String, Kanji>, Box<dyn std::error::Error>> {
    let file = File::open("kanji.json")?;
    let reader = BufReader::new(file);
    let kanji: HashMap<String, Kanji> = serde_json::from_reader(reader)?;
    Ok(kanji)
}