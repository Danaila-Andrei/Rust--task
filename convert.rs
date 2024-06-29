//Lru Cache implementation
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use lru::LruCache;

// Converts a binary string to its hexadecimal representation
fn bin_to_hex(bin: &str) -> String {
    bin.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| {
            let mut hex_val = 0;
            for (i, &c) in chunk.iter().enumerate() {
                if c == '1' {
                    hex_val += 1 << (3 - i);
                }
            }
            format!("{:X}", hex_val)
        })
        .collect()
}

// Converts a hexadecimal string to its binary representation
fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let mut bin_str = String::new();
            let mut num = c.to_digit(16).unwrap();
            for _ in 0..4 {
                bin_str.insert(0, if num % 2 == 1 { '1' } else { '0' });
                num /= 2;
            }
            bin_str
        })
        .collect::<Vec<_>>()
        .join("")
}

// Converts mat.in to mat.in.x
fn convert_mat_in_to_mat_in_x(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut cache = LruCache::new(1000);

    let start_time = Instant::now();
    for line in reader.lines() {
        let line = line?;
        if let Some(cached) = cache.get(&line) {
            writeln!(writer, "{}", cached)?;
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let dimension = parts[0];
        let bin_str = parts[1];
        let hex_str = bin_to_hex(bin_str);
        let result = format!("{}:{}", dimension, hex_str);
        cache.put(line.clone(), result.clone());
        writeln!(writer, "{}", result)?;
    }
    let duration = start_time.elapsed();
    println!("Convertirea mat.in în mat.in.x a durat: {:?}", duration);

    Ok(())
}

// Converts mat.in.x to mat.in
fn convert_mat_in_x_to_mat_in(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut cache = LruCache::new(1000);

    let start_time = Instant::now();
    for line in reader.lines() {
        let line = line?;
        if let Some(cached) = cache.get(&line) {
            writeln!(writer, "{}", cached)?;
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let dimension = parts[0];
        let hex_str = parts[1];
        let bin_str = hex_to_bin(hex_str);
        let result = format!("{}:{}", dimension, bin_str);
        cache.put(line.clone(), result.clone());
        writeln!(writer, "{}", result)?;
    }
    let duration = start_time.elapsed();
    println!("Convertirea mat.in.x în mat.in a durat: {:?}", duration);

    Ok(())
}

fn main() {
    let input_path = "mat.in";
    let output_path = "mat.in.x";

    if let Err(e) = convert_mat_in_to_mat_in_x(input_path, output_path) {
        eprintln!("Eroare la convertirea mat.in în mat.in.x: {}", e);
    }

    let input_path_x = "mat.in.x";
    let output_path_x = "mat.in.out";

    if let Err(e) = convert_mat_in_x_to_mat_in(input_path_x, output_path_x) {
        eprintln!("Eroare la convertirea mat.in.x în mat.in: {}", e);
    }
}
//Non cache implemetation
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;


fn bin_to_hex(bin: &str) -> String {
    bin.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| {
            let mut hex_val = 0;
            for (i, &c) in chunk.iter().enumerate() {
                if c == '1' {
                    hex_val += 1 << (3 - i);
                }
            }
            format!("{:X}", hex_val)
        })
        .collect()
}


fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let mut bin_str = String::new();
            let mut num = c.to_digit(16).unwrap();
            for _ in 0..4 {
                bin_str.insert(0, if num % 2 == 1 { '1' } else { '0' });
                num /= 2;
            }
            bin_str
        })
        .collect::<Vec<_>>()
        .join("")
}


fn convert_mat_in_to_mat_in_x(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    let start_time = Instant::now();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        let dimension = parts[0];
        let bin_str = parts[1];
        let hex_str = bin_to_hex(bin_str);
        let result = format!("{}:{}", dimension, hex_str);
        writeln!(writer, "{}", result)?;
    }
    let duration = start_time.elapsed();
    println!("Convertirea mat.in în mat.in.x a durat: {:?}", duration);

    Ok(())
}


fn convert_mat_in_x_to_mat_in(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    let start_time = Instant::now();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        let dimension = parts[0];
        let hex_str = parts[1];
        let bin_str = hex_to_bin(hex_str);
        let result = format!("{}:{}", dimension, bin_str);
        writeln!(writer, "{}", result)?;
    }
    let duration = start_time.elapsed();
    println!("Convertirea mat.in.x în mat.in a durat: {:?}", duration);

    Ok(())
}

fn main() {
    let input_path = "mat.in";
    let output_path = "mat.in.x";

    if let Err(e) = convert_mat_in_to_mat_in_x(input_path, output_path) {
        eprintln!("Eroare la convertirea mat.in în mat.in.x: {}", e);
    }

    let input_path_x = "mat.in.x";
    let output_path_x = "mat.in.out";

    if let Err(e) = convert_mat_in_x_to_mat_in(input_path_x, output_path_x) {
        eprintln!("Eroare la convertirea mat.in.x în mat.in: {}", e);
    }
}
