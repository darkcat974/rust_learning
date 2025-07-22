use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufWriter};
use std::path::Path;
use csv::{Reader, Writer};
use serde_json::{json, Value};
use std::error::Error;
use std::collections::BTreeSet;

fn json_to_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader)?;
    let json_array = json_value.as_array()
        .ok_or("Le JSON doit être un tableau d'objets")?;
    let mut headers = BTreeSet::new();

    for obj in json_array {
        if let Some(map) = obj.as_object() {
            for key in map.keys() {
                headers.insert(key.clone());
            }
        } else {
            return Err("Le JSON doit être un tableau d'objets JSON".into());
        }
    }
    let csv_path = std::path::Path::new(file_path).with_extension("csv");
    let file = File::create(&csv_path)?;
    let mut wtr = Writer::from_writer(BufWriter::new(file));
    let headers_vec: Vec<String> = headers.into_iter().collect();

    wtr.write_record(&headers_vec)?;

    for obj in json_array {
        let obj_map = obj.as_object().unwrap();
        let mut row = Vec::new();
        for key in &headers_vec {
            let cell = obj_map.get(key)
                .map(|v| {
                    if v.is_string() {
                        v.as_str().unwrap().to_string()
                    } else {
                        v.to_string()
                    }
                })
                .unwrap_or_else(|| "".to_string());
            row.push(cell);
        }
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    println!("CSV file created at {}", csv_path.display());
    Ok(())
}

fn detect_type(field: &str) -> Value {
    let trimmed = field.trim();

    if let Ok(int_val) = trimmed.parse::<i64>() {
        return json!(int_val);
    }
    if let Ok(float_val) = trimmed.parse::<f64>() {
        return json!(float_val);
    }
    match trimmed.to_lowercase().as_str() {
        "true" | "yes" | "1" => return json!(true),
        "false" | "no" | "0" => return json!(false),
        _ => {}
    }
    return json!(field);
}

fn csv_to_json(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    println!("Reading CSV file: {}", file_path);
    let mut json_records = Vec::new();
    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        let record = result?;
        let mut json_obj = serde_json::Map::new();
        for (i, field) in record.iter().enumerate() {
            let key = headers.get(i)
                .map(|h| h.to_string())
                .unwrap_or_else(|| format!("{}", i));
            json_obj.insert(key, detect_type(field));
        }
        json_records.push(json!(json_obj));
    }

    let json_array = serde_json::to_string_pretty(&json_records)?;
    let path = Path::new(file_path);
    let json_path = path.with_extension("json");
    let mut file = File::create(&json_path)?;
    file.write_all(json_array.as_bytes())?;

    println!("JSON written to: {}", json_path.display());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [file_path, file_path2, ...]", args[0]);
        return;
    };
    for i in 1..args.len() {
        let file_path = &args[i];
        if file_path.ends_with(".csv") {
            let _ = csv_to_json(file_path);
        } else if file_path.ends_with(".json") {
            if let Err(e) = json_to_csv(file_path) {
                eprintln!("Erreur lors de la conversion JSON -> CSV: {}", e);
            }
        } else {
            println!("{}: Invalid file type. Use '.csv' or '.json'.", file_path);
        }
    }
}
